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
//! ```no_run
//! # use std::path::PathBuf;
//! # use gongbi::*;
//! # use polars::prelude::*;
//! # fn main() -> anyhow::Result<()> {
//! let mpg = CsvReadOptions::default()
//!     .with_has_header(true)
//!     .try_into_reader_with_file_path(Some("examples/mpg.csv".into()))?
//!     .finish()?;
//!
//! let plot = plot!(mpg, aes!("displ", "hwy"))
//!     + geom_point!()
//!     + labs!(caption = "Demo of geom_point");
//!
//! plot.to_svg("gongbi.svg", (1024, 768))?;
//! # Ok(())
//! # }
//! ```
//!

#![warn(missing_docs)]

use std::{
    ops::{Add, Range},
    path::PathBuf,
    rc::Rc,
};

use derive_builder::Builder;
use plotters::{
    chart::ChartContext,
    coord::types::RangedCoordf64,
    prelude::{Cartesian2d, DrawingBackend},
};

pub mod aes;
pub mod data;
pub mod label;
pub mod layer;

/// # Plot: The main object to create a plot
///
/// To create a plot, two steps are required:
/// 1. Create a `Plot` object via [`plot!`] or [`Plot::builder`]
/// 2. Add layers (e.g. [`geom_point!`], [`geom_line!`]) and labels (e.g. [`labs!`]) to the plot
///
/// Finally, call [`Plot::to_svg`] or [`Plot::to_png`] to save the plot to a file
#[derive(Clone, Debug, Builder)]
pub struct Plot {
    /// The data source for the plot
    ///
    /// If not specified, the data source must be supplied in each layer
    #[builder(default, setter(custom))]
    data: Option<Rc<dyn data::Data>>,

    /// The default aesthetics mapping for the plot
    ///
    /// If not specified, the aesthetics mapping must be supplied in each layer
    ///
    /// See [`aes::Aes`] for all supported aesthetics
    #[builder(default)]
    mapping: aes::Aes,

    /// The layers of the plot
    ///
    /// This is the main part of the plot, layers like points, lines, and bars are added here. When [`Plot::draw`] is called, all layers are drawn on the plot.
    ///
    /// Usually, layers are added using the `+` operator.
    #[builder(default, setter(skip))]
    layers: Vec<Box<dyn layer::Layer>>,

    /// The label of the plot
    ///
    /// This is used to add titles, captions, and other labels to the plot
    #[builder(default, setter(skip))]
    label: label::Label,
}

impl PlotBuilder {
    /// Set the data source for the plot
    pub fn data<D>(&mut self, data: D) -> &mut Self
    where
        D: data::Data + 'static,
    {
        self.data = Some(Some(Rc::new(data)));
        self
    }
}

impl Plot {
    /// Create a new [`Plot`] object via the builder pattern
    pub fn builder() -> PlotBuilder {
        PlotBuilder::default()
    }

    fn get_context_range_2d(&self) -> (Range<f64>, Range<f64>) {
        // Get the range of the x and y axis
        let range = self
            .layers
            .iter()
            .map(|layer| layer.range_2d())
            .reduce(|acc, cur| {
                (
                    acc.0.min(cur.0),
                    acc.1.max(cur.1),
                    acc.2.min(cur.2),
                    acc.3.max(cur.3),
                )
            })
            .expect("No layers");

        let x_range_len = range.1 - range.0;
        let y_range_len = range.3 - range.2;

        let x_range = (range.0 - 0.025 * x_range_len)..(range.1 + 0.025 * x_range_len);
        let y_range = (range.2 - 0.025 * y_range_len)..(range.3 + 0.025 * y_range_len);

        (x_range, y_range)
    }

    fn draw_mesh<DB>(
        &self,
        chart: &mut ChartContext<'_, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    ) -> anyhow::Result<()>
    where
        DB: DrawingBackend,
        <DB as plotters::prelude::DrawingBackend>::ErrorType: 'static,
    {
        use plotters::prelude::*;

        let mut mesh = chart.configure_mesh();
        mesh.axis_desc_style(("sans-serif", 24).into_font())
            .x_label_style(("sans-serif", 16).into_font())
            .y_label_style(("sans-serif", 16).into_font());

        if let Some(x_label) = &self.label.x {
            mesh.x_desc(x_label);
        } else if let Some(x_label) = self.mapping.x {
            mesh.x_desc(x_label);
        } else {
            mesh.x_desc("x");
        }

        if let Some(y_label) = &self.label.y {
            mesh.y_desc(y_label);
        } else if let Some(y_label) = self.mapping.y {
            mesh.y_desc(y_label);
        } else {
            mesh.y_desc("y");
        }

        mesh.draw()?;

        Ok(())
    }

    /// Save the plot to an SVG file
    pub fn to_svg<P>(&self, file_path: P, size: (u32, u32)) -> anyhow::Result<()>
    where
        P: Into<PathBuf>,
    {
        use plotters::prelude::*;

        let file_path = file_path.into();

        let root = SVGBackend::new(&file_path, size).into_drawing_area();

        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root);

        chart
            .margin(5)
            .x_label_area_size(10.percent())
            .y_label_area_size(10.percent());

        if let Some(caption) = &self.label.caption {
            chart.caption(caption, ("sans-serif", 32).into_font());
        }

        let (x_range, y_range) = self.get_context_range_2d();

        let mut chart = chart.build_cartesian_2d(x_range, y_range)?;

        self.draw_mesh(&mut chart)?;

        for layer in &self.layers {
            layer.draw_svg_2d(&mut chart)?;
        }

        // chart
        //     .configure_series_labels()
        //     .background_style(WHITE)
        //     .label_font(("sans-serif", 16).into_font())
        //     .draw()?;

        root.present()?;

        Ok(())
    }

    /// Save the plot to a PNG file
    pub fn to_png<P>(&self, file_path: P, size: (u32, u32)) -> anyhow::Result<()>
    where
        P: Into<PathBuf>,
    {
        use plotters::prelude::*;

        let file_path = file_path.into();

        let root = BitMapBackend::new(&file_path, size).into_drawing_area();

        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root);

        chart
            .margin(5)
            .x_label_area_size(10.percent())
            .y_label_area_size(10.percent());

        if let Some(caption) = &self.label.caption {
            chart.caption(caption, ("sans-serif", 32).into_font());
        }

        let (x_range, y_range) = self.get_context_range_2d();

        let mut chart = chart.build_cartesian_2d(x_range, y_range)?;

        self.draw_mesh(&mut chart)?;

        for layer in &self.layers {
            layer.draw_png_2d(&mut chart)?;
        }

        // chart
        //     .configure_series_labels()
        //     .background_style(WHITE)
        //     .label_font(("sans-serif", 16).into_font())
        //     .draw()?;

        root.present()?;

        Ok(())
    }

    /// Save the plot
    ///
    /// The file format is determined by the file extension:
    /// - `.svg`: SVG file
    /// - `.png`: PNG file
    /// - Otherwise, an error is returned
    pub fn save<P>(&self, file_path: P, size: (u32, u32)) -> anyhow::Result<()>
    where
        P: Into<PathBuf>,
    {
        let file_path = file_path.into();
        match file_path.extension().and_then(|ext| ext.to_str()) {
            Some("svg") => self.to_svg(file_path, size),
            Some("png") => self.to_png(file_path, size),
            _ => Err(anyhow::anyhow!("Unsupported file format")),
        }
    }
}

// Trick to hide internal implementation details from the docs
macro_rules! __plot {
    ($plot: item) => {
        /// # plot!: Construct a new [`Plot`] object
        ///
        /// This macro is used to create a [`Plot`] obejct in a more concise way like `ggplot2` in `R`.
        /// It is a wrapper around [`Plot::builder`], so you can see more information of each argument in [`PlotBuilder`].
        ///
        /// ## Usage
        ///
        /// ```ignore
        /// plot!(
        ///     data = <Data>,
        ///     mapping = aes!(...),
        ///     [...Additional arguments]
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
        /// ## Example
        ///
        /// The following examples are equivalent:
        ///
        /// ```
        /// # use gongbi::*;
        /// # use polars::prelude::*;
        /// let df = DataFrame::default();
        ///
        /// let p1 = plot!(df.clone(), aes!("x", "y"));
        /// let p2 = plot!(df.clone(), mapping = aes!("x", "y"));
        /// let p3 = plot!(data = df.clone(), mapping = aes!("x", "y"));
        /// ```
        $plot
    };
}

#[cfg(doc)]
__plot![
    #[macro_export]
    macro_rules! plot {
        ($($param: tt)*) => {
            ...
        };
    }
];

#[cfg(not(doc))]
__plot![
    #[macro_export]
    macro_rules! plot {
        ($($param: ident = $value: expr),* $(,)?) => {
            $crate::Plot::builder()
                $(.$param($value))*
                .build()
                .unwrap()
        };

        ($data: expr $(, $($param: ident = $value: expr),+ $(,)?)?) => {
            plot!(data = $data $(, $($param = $value),+)?)
        };

        ($data: expr, $aes: expr $(, $($param: ident = $value: expr),+ $(,)?)?) => {
            plot!(data = $data, mapping = $aes $(, $($param = $value),+)?)
        };
    }
];

impl<L> Add<L> for Plot
where
    L: layer::Layer + 'static,
{
    type Output = Self;

    fn add(self, rhs: L) -> Self::Output {
        let mut rhs = rhs;

        // Inherit the mapping from the plot
        *rhs.mapping_mut() += self.mapping.clone();

        // If the layer does not have data, use the plot's data
        rhs.data_mut()
            .get_or_insert(self.data.as_ref().expect("Plot does not have data").clone());

        let mut layers = self.layers;
        layers.push(Box::new(rhs));

        Plot { layers, ..self }
    }
}

impl Add<label::Label> for Plot {
    type Output = Self;

    fn add(self, rhs: label::Label) -> Self::Output {
        let label = self.label + rhs;

        Plot { label, ..self }
    }
}
