//! # Line geom layer
//!
//! The line geom is used to connects the data points in th order on the x-axis.
//!
//! ## Example
//!
//! ```no_run
//! # use gongbi::*;
//! # use polars::prelude::*;
//! # fn main() -> anyhow::Result<()> {
//! # let mpg = CsvReadOptions::default()
//! #     .with_has_header(true)
//! #     .try_into_reader_with_file_path(Some("examples/mpg.csv".into()))?
//! #     .finish()?;
//! let p = plot!(mpg, aes!(displ, hwy))
//!     + geom_line!();
//!
//! p.to_svg("geom_line.svg", (800, 600))?;
//! # Ok(())
//! # }
//! ```
//!
//! See [`geom_line!`](crate::geom_line) or [`Line::builder`] for more details.

use std::rc::Rc;

use derive_builder::Builder;
use plotters::{
    chart::ChartContext,
    coord::types::RangedCoordf64,
    element::DashedPathElement,
    prelude::{Cartesian2d, DrawingBackend, IntoDynElement, PathElement},
    series::{DashedLineSeries, LineSeries},
};

use crate::layer::Layer;

use super::range_2d_x_xy;

/// # Line layer
///
/// The line geom is used to connects the data points in th order on the x-axis.
///
/// To create a line layer, use [`geom_line!`](crate::geom_line) or [`Line::builder`].
#[derive(Clone, Debug, Default, Builder)]
pub struct Line {
    /// The data source for the layer.
    #[builder(default, setter(strip_option))]
    data: Option<Rc<dyn crate::data::Data>>,

    /// The aes mapping for the layer.
    #[builder(default)]
    mapping: crate::aes::Aes,
}

impl Line {
    /// Create a new [`Line`] via the builder pattern.
    pub fn builder() -> LineBuilder {
        LineBuilder::default()
    }

    fn draw_2d<'a, DB>(
        &'a self,
        chart: &mut ChartContext<'a, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    ) -> anyhow::Result<()>
    where
        DB: DrawingBackend + 'a,
        <DB as plotters::prelude::DrawingBackend>::ErrorType: 'static,
    {
        // If only x is provided, we use it as y, and use index as x
        // If x and y are both provided, we use both

        let data = self.data.as_ref().expect("data is not provided");
        let mapping = &self.mapping;

        let column_x = mapping.x.expect("x must be provided for geom_line");
        let column_y = mapping.y;

        let points: Vec<(f64, f64)> = match column_y {
            Some(column_y) => data
                .column_f64(column_x)
                .into_iter()
                .zip(data.column_f64(column_y))
                .collect(),
            None => {
                let x = data.column_f64(column_x);

                (0..x.len()).map(|u| u as f64).zip(x).collect()
            }
        };

        let color = mapping.color.clone().unwrap_or_default().as_rgb();

        let anno = match mapping.shape {
            None | Some(1) => chart.draw_series(LineSeries::new(points, color))?,
            Some(2) => chart.draw_series(DashedLineSeries::new(points, 5, 5, color.into()))?,

            _ => todo!(),
        };

        if let Some(label) = &mapping.label {
            anno.label(label).legend(move |(x, y)| match mapping.shape {
                None | Some(1) => PathElement::new([(x, y), (x + 20, y)], color).into_dyn(),
                Some(2) => DashedPathElement::new([(x, y), (x + 20, y)], 5, 5, color).into_dyn(),
                _ => todo!(),
            });
        }

        Ok(())
    }
}

/// # geom_line!: Construct a [`Line`] layer
///
/// The macro is used to create a [`Line`] layer in a more concise way like `ggplot2`.
/// It is a wrapper around [`Line::builder`].
///
/// ## Usage
///
/// ```ignore
/// geom_line!(
///     mapping = aes!(...),
///     data = <Data>
/// )
/// ```
///
/// ### Arguments
///
/// #### mapping
///
/// Set of aesthetic mappings created by [`aes!`](crate::aes!) or [`Aes::builder`](crate::aes::Aes::builder).
///
/// #### data
///
/// The data to be displayed in this layer.
///
/// If not provided, it will be inherited from the [`Plot`](crate::Plot).
///
/// If provided, it will override the data from the [`Plot`](crate::Plot).
#[macro_export]
macro_rules! geom_line {
    ($($param: ident = $value: expr),* $(,)?) => {
        $crate::layer::geom::line::Line::builder()
            $(.$param($value))*
            .build()
            .unwrap()
    };

    ($mapping: expr $(, $($param: ident = $value: expr),+ $(,)?)?) => {
        geom_line!(mapping = $mapping $(, $($param = $value),+)?)
    };
}

impl Layer for Line {
    fn data_mut(&mut self) -> &mut Option<Rc<dyn crate::data::Data>> {
        &mut self.data
    }

    fn mapping_mut(&mut self) -> &mut crate::aes::Aes {
        &mut self.mapping
    }

    fn range_2d(&self) -> (f64, f64, f64, f64) {
        let data = self.data.as_ref().expect("data is not provided");

        range_2d_x_xy(data, &self.mapping)
    }

    fn draw_svg_2d<'a>(
        &'a self,
        chart: &mut ChartContext<
            'a,
            plotters::prelude::SVGBackend<'a>,
            Cartesian2d<RangedCoordf64, RangedCoordf64>,
        >,
    ) -> anyhow::Result<()> {
        self.draw_2d(chart)
    }

    fn draw_png_2d<'a>(
        &'a self,
        chart: &mut ChartContext<
            'a,
            plotters::prelude::BitMapBackend<'a>,
            Cartesian2d<RangedCoordf64, RangedCoordf64>,
        >,
    ) -> anyhow::Result<()> {
        self.draw_2d(chart)
    }
}
