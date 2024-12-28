//! # Point geom layer
//!
//! The point geom is used to create scatter plot. The scatter plot is used to
//! visualize the relationship between two continuous variables.
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
//!     + geom_point!();
//!
//! p.to_svg("geom_point.svg", (800, 600))?;
//! # Ok(())
//! # }
//! ```
//!
//! See [`geom_point!`](crate::geom_point!) or [`Point::builder`] for more details.

use std::rc::Rc;

use derive_builder::Builder;
use plotters::{
    chart::ChartContext,
    coord::types::RangedCoordf64,
    prelude::{
        Cartesian2d, Circle, Cross, DrawingBackend, EmptyElement, IntoDynElement, Rectangle,
        TriangleMarker,
    },
    series::PointSeries,
    style::Color,
};

use crate::layer::Layer;

use super::range_2d_x_xy;

/// # Point layer
///
/// The point geom layer is used to draw scatter plot.
///
/// To create a point layer, [`geom_point!`](crate::geom_point!) and [`Point::builder`] can be used.
#[derive(Clone, Debug, Default, Builder)]
pub struct Point {
    /// The data source for the layer.
    #[builder(default, setter(strip_option))]
    data: Option<Rc<dyn crate::data::Data>>,

    /// The aes mapping for the layer.
    #[builder(default)]
    mapping: crate::aes::Aes,
}

impl Point {
    /// Create a new [`Point`] via the builder pattern.
    pub fn builder() -> PointBuilder {
        PointBuilder::default()
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

        let column_x = mapping.x.expect("x must be provided for geom_point");
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

        let style = match mapping.fill {
            Some(false) => color.stroke_width(1),
            _ => color.filled(),
        };

        let anno = chart.draw_series(PointSeries::of_element(
            points,
            mapping.size.unwrap_or(5),
            style,
            &|c, s, st| {
                EmptyElement::at(c)
                    + match mapping.shape {
                        Some(0) => Rectangle::new([(-s, -s), (s, s)], st).into_dyn(),
                        None | Some(1) => Circle::new((0, 0), s, st).into_dyn(),
                        Some(2) => TriangleMarker::new((0, 0), s, st).into_dyn(),
                        Some(3) => Cross::new((0, 0), s, st).into_dyn(),

                        _ => todo!(),
                    }
            },
        ))?;

        if let Some(label) = &mapping.label {
            anno.label(label).legend(move |(x, y)| match mapping.shape {
                Some(0) => Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], color).into_dyn(),
                None | Some(1) => Circle::new((x, y), 5, color).into_dyn(),
                Some(2) => TriangleMarker::new((x, y), 5, color).into_dyn(),
                Some(3) => Cross::new((x, y), 5, color).into_dyn(),
                _ => todo!(),
            });
        }

        Ok(())
    }
}

/// # geom_point!: Construct a [`Point`] layer
///
/// This macro is used to create a [`Point`] layer in a more concise way like `ggplot2`.
/// It is a wrapper around [`Point::builder`].
///
/// ## Usage
///
/// ```ignore
/// geom_point!(
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
macro_rules! geom_point {
    ($($param: ident = $value: expr),* $(,)?) => {
        $crate::layer::geom::point::Point::builder()
            $(.$param($value))*
            .build()
            .unwrap()
    };

    ($mapping: expr $(, $($param: ident = $value: expr),+ $(,)?)?) => {
        geom_point!(mapping = $mapping $(, $($param = $value),+)?)
    };
}

impl Layer for Point {
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
