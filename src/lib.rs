//! # Gongbi (工笔)
//!
//! ## Overview
//!
//! Gongbi is a data visualization crate based on [plotters](https:://crates.io/crates/plotters), and inspired by [ggplot2](https://ggplot2.tidyverse.org/).
//!
//! The name "Gongbi" (工笔) is a Chinese painting technique that uses highly detailed brushstrokes to create realistic images.
//!
//! ## Usage
//!
//! Like `ggplot2`, you can start with [`plot!`], supply a data source and aesthetics ([`aes!`]), and add layers ([`geom_point!`] or [`geom_line!`]) and labels ([`labs!`]).
//!
//! ```ignore
//! # use gongbi::*;
//! # use polars::prelude::*;
//! let mpg = CsvReadOptions::default()
//!     .with_has_header(true)
//!     .try_into_reader_with_file_path(Some("examples/mpg.csv".into()))?
//!     .finish()?;
//!
//! let plot = plot!(mpg, aes!("displ", "hwy"), save = "gongbi.svg")
//!     + geom_point!()
//!     + labs!(caption = "Demo of geom_point");
//!
//! plot.draw()?;
//! ```
//!
#![doc = include_str!("../gongbi.svg")]
//!

use std::path::PathBuf;

use layer::MappedElements;

/// Re-export plotters crate
pub use plotters;
use plotters::prelude::*;

pub mod aes;
pub mod data;
pub mod geom;
pub mod label;
pub mod layer;

/// # Plot: The main object to create a plot
///
/// It is recommended to use the [`plot!`] macro or [`Plot::builder`] to create a [`Plot`] object.
#[derive(Debug, Default, typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct Plot {
    /// The data source for the plot
    ///
    /// If not specified, the data source must be supplied in each layer.
    ///
    /// See [`data::Data`] for all supported data sources.
    #[builder(setter(transform = |x: impl Into<data::Data>| Some(Box::new(x.into()))))]
    pub data: Option<Box<data::Data>>,

    /// The default aesthetics mapping for the plot
    ///
    /// If not specified, the aesthetics mapping must be supplied in each layer.
    ///
    /// See [`aes::Aes`] for all supported aesthetics.
    pub mapping: aes::Aes,

    /// The layers of the plot
    ///
    /// This is the main part of the plot, layers like points, lines, and bars are added here. When [`Plot::draw`] is called, all layers are drawn on the plot.
    ///
    /// Usually, layers are added using the `+` operator. See [`layer::Layer`] for more information on each layer.
    pub layers: Vec<layer::Layer>,

    /// The label of the plot
    ///
    /// This is used to add a caption, x-axis label, and y-axis label to the plot.
    pub label: label::Label,

    /// The size of the plot
    ///
    /// This is optional and defaults to (1024, 768).
    #[builder(default = (1024, 768))]
    pub size: (u32, u32),

    /// The path to save the plot
    #[builder(default = "gongbi.png".into())]
    pub save: PathBuf,
}

/// # plot!: A macro to create a [`Plot`] object
///
/// This macro is used to create a [`Plot`] object in a more concise way. Check [`Plot`] for more information on each argument.
///
/// ## Usage
///
/// ```ignore
/// plot!(
///     data = <Data>,
///     mapping = aes!(...),
///     size = (<width>, <height>),
///     save = "path/to/file.png"
/// )
/// ```
///
/// ## Arguments
///
/// ### data
///
/// The default data source for the plot. If not specified, the data source must be supplied in each layer.
///
/// The `data =` part can be omitted if the data source is the first argument.
///
/// ### mapping
///
/// The default aesthetics mapping for the plot. If not specified, the aesthetics mapping must be supplied in each layer.
///
/// The `mapping =` part can be omitted if the data source and aesthetics mapping are the first two arguments.
///
/// ### size
///
/// The size of the plot. This is optional and defaults to (1024, 768).
///
/// ### save
///
/// The path to save the plot. This is optional and defaults to "gongbi.png".
///
/// Only PNG and SVG file formats are supported at the moment.
#[macro_export]
macro_rules! plot {
    ($($arg:ident = $val:expr),* $(,)?) => {
        $crate::Plot::builder()
            $(.$arg($val))*
            .build()
    };

    ($data:expr $(, $($arg:ident = $val:expr),+ $(,)?)?) => {
        $crate::plot!(data = $data $(, $($arg = $val),+)?)
    };

    ($data:expr, $aes:expr $(, $($arg:ident = $val:expr),+ $(,)?)?) => {
        $crate::plot!(data = $data, mapping = $aes $(, $($arg = $val),+)?)
    }
}

impl<L> core::ops::Add<L> for Plot
where
    L: Into<layer::Layer>,
{
    type Output = Plot;

    fn add(self, rhs: L) -> Self::Output {
        // println!("self.mapping: {:?}", self.mapping);

        let mut rhs: layer::Layer = rhs.into();
        rhs.data_mut()
            .get_or_insert(self.data.as_ref().unwrap().clone());
        rhs.mapping_mut().inherit(&self.mapping);

        let mut layers = self.layers;
        layers.push(rhs);

        Plot { layers, ..self }
    }
}

impl core::ops::Add<label::Label> for Plot {
    type Output = Plot;

    fn add(self, rhs: label::Label) -> Self::Output {
        let label = self.label + rhs;

        Plot { label, ..self }
    }
}

impl Plot {
    pub fn draw(&self) -> anyhow::Result<()> {
        use plotters::prelude::*;

        let save = self.save.clone();
        let ext = save.extension().unwrap().to_str().unwrap();

        match ext {
            "png" => {
                let root = BitMapBackend::new(&save, self.size);

                self.draw_inner(root)?;
            }
            "svg" => {
                let root = SVGBackend::new(&save, self.size);

                self.draw_inner(root)?;
            }
            _ => panic!("Unsupported file extension"),
        }

        Ok(())
    }

    fn draw_inner<DB: DrawingBackend>(&self, db: DB) -> anyhow::Result<()>
    where
        <DB as plotters::prelude::DrawingBackend>::ErrorType: 'static,
    {
        use plotters::prelude::*;

        let root = db.into_drawing_area();

        root.fill(&WHITE)?;

        let mut elements: Vec<DynElement<DB, (f64, f64)>> = Vec::new();
        let mut x_range = (f64::INFINITY, -f64::INFINITY);
        let mut y_range = (f64::INFINITY, -f64::INFINITY);

        self.layers.iter().for_each(|layer| {
            let MappedElements {
                x_range: x,
                y_range: y,
                elements: element,
            } = layer.mapping_data::<DB>();

            x_range = (x_range.0.min(x.0), x_range.1.max(x.1));

            y_range = (y_range.0.min(y.0), y_range.1.max(y.1));

            elements.extend(element);
        });

        let x_range_len = x_range.1 - x_range.0;
        let y_range_len = y_range.1 - y_range.0;

        x_range = (
            x_range.0 - 0.025 * x_range_len,
            x_range.1 + 0.025 * x_range_len,
        );
        y_range = (
            y_range.0 - 0.025 * y_range_len,
            y_range.1 + 0.025 * y_range_len,
        );

        let mut chart = ChartBuilder::on(&root);

        chart
            .margin(5)
            .x_label_area_size(10.percent())
            .y_label_area_size(10.percent());

        if let Some(caption) = self.label.caption.as_ref() {
            chart.caption(caption, ("sans-serif", 32).into_font());
        }

        let mut chart = chart.build_cartesian_2d(x_range.0..x_range.1, y_range.0..y_range.1)?;

        let mut mesh = chart.configure_mesh();
        mesh.axis_desc_style(("sans-serif", 24).into_font())
            .x_label_style(("sans-serif", 16).into_font())
            .y_label_style(("sans-serif", 16).into_font());

        if let Some(x_label) = self.label.x.as_ref() {
            mesh.x_desc(x_label);
        } else if let Some(x_label) = self.mapping.x.as_ref() {
            mesh.x_desc(x_label);
        } else {
            mesh.x_desc("x");
        }

        if let Some(y_label) = self.label.y.as_ref() {
            mesh.y_desc(y_label);
        } else if let Some(y_label) = self.mapping.y.as_ref() {
            mesh.y_desc(y_label);
        } else {
            mesh.y_desc("y");
        }

        mesh.draw()?;

        chart.draw_series(elements)?;

        root.present()?;

        Ok(())
    }
}
