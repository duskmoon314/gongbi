use plotters::{coord::Shift, prelude::*};

pub mod point;

#[derive(Debug, derive_more::From)]
pub enum Geom {
    Point(point::GeomPoint),
}

impl Geom {
    pub fn draw<DB: DrawingBackend>(
        &self,
        area: &DrawingArea<DB, Shift>,
        data: &crate::data::Data,
    ) -> Result<(), crate::PlotError<DB::ErrorType>> {
        match self {
            Geom::Point(p) => p.draw(area, data),
        }
    }
}

pub trait GeomMethod {
    fn aes_mut(&mut self) -> &mut Option<crate::aes::Aes>;

    fn draw<DB: DrawingBackend>(
        &self,
        area: &DrawingArea<DB, Shift>,
        data: &crate::data::Data,
    ) -> Result<(), crate::PlotError<DB::ErrorType>>;
}
