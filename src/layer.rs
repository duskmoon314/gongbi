use std::rc::Rc;

use dyn_clone::DynClone;
use plotters::{
    chart::ChartContext,
    coord::types::RangedCoordf64,
    prelude::{Cartesian2d, SVGBackend},
};

pub mod geom;

pub trait Layer: DynClone {
    fn mapping_mut(&mut self) -> &mut crate::aes::Aes;

    fn data_mut(&mut self) -> &mut Option<Rc<dyn crate::data::Data>>;

    fn range_2d(&self) -> (f64, f64, f64, f64);

    fn draw_svg_2d(
        &self,
        chart: &mut ChartContext<'_, SVGBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    ) -> anyhow::Result<()>;
}

dyn_clone::clone_trait_object!(Layer);
