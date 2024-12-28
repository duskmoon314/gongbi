//! # Layer module
//!
//! This module contains the [`Layer`] trait and many geometric layers that implement it.
//!
//! ## List of geometric layers
//!
//! - [`geom::point`]
//! - [`geom::line`]

use std::{fmt::Debug, rc::Rc};

use dyn_clone::DynClone;
use plotters::{
    chart::ChartContext,
    coord::types::RangedCoordf64,
    prelude::{BitMapBackend, Cartesian2d, SVGBackend},
};

pub mod geom;

/// # Layer trait
///
/// This trait abstracts the concept of a layer in a plot.
pub trait Layer: DynClone + Debug {
    /// Get the mutable reference to the mapping.
    ///
    /// This method is called when adding a layer to a plot.
    /// It allows the layer inheriting the mapping from the plot.
    fn mapping_mut(&mut self) -> &mut crate::aes::Aes;

    /// Get the mutable reference to the data.
    ///
    /// This method is called when adding a layer to a plot.
    /// It allows the layer inheriting the data from the plot.
    fn data_mut(&mut self) -> &mut Option<Rc<dyn crate::data::Data>>;

    /// Get the range of the x-axis and y-axis needed to draw the layer.
    ///
    /// This method is called when drawing the layer.
    fn range_2d(&self) -> (f64, f64, f64, f64);

    /// Draw the layer on a 2D SVG chart context.
    fn draw_svg_2d<'a>(
        &'a self,
        chart: &mut ChartContext<'a, SVGBackend<'a>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    ) -> anyhow::Result<()>;

    /// Draw the layer on a 2D PNG chart context.
    fn draw_png_2d<'a>(
        &'a self,
        chart: &mut ChartContext<
            'a,
            BitMapBackend<'a>,
            Cartesian2d<RangedCoordf64, RangedCoordf64>,
        >,
    ) -> anyhow::Result<()>;
}

dyn_clone::clone_trait_object!(Layer);
