use plotters::prelude::DrawingBackend;

use crate::layer::MappedElements;

pub mod line;
pub mod point;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
#[non_exhaustive]
pub enum Geom {
    Point(point::Point),
    Line(line::Line),
}

impl Geom {
    pub fn data_mut(&mut self) -> &mut Option<Box<crate::data::Data>> {
        match self {
            Geom::Point(p) => p.data_mut(),
            Geom::Line(l) => l.data_mut(),
        }
    }

    pub fn mapping_mut(&mut self) -> &mut crate::aes::Aes {
        match self {
            Geom::Point(p) => p.mapping_mut(),
            Geom::Line(l) => l.mapping_mut(),
        }
    }

    pub fn mapping_data<'a, DB>(&self) -> MappedElements<'a, DB>
    where
        DB: DrawingBackend,
    {
        match self {
            Geom::Point(p) => p.mapping_data(),
            Geom::Line(l) => l.mapping_data(),
        }
    }
}
