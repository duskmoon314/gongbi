use std::fmt::Debug;

use plotters::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Layer {
    Geom(crate::geom::Geom),
}

impl Layer {
    pub fn data_mut(&mut self) -> &mut Option<Box<crate::data::Data>> {
        match self {
            Layer::Geom(geom) => geom.data_mut(),
        }
    }

    pub fn mapping_mut(&mut self) -> &mut crate::aes::Aes {
        match self {
            Layer::Geom(geom) => geom.mapping_mut(),
        }
    }

    pub fn mapping_data<'a, DB>(&self) -> MappedElements<'a, DB>
    where
        DB: DrawingBackend,
    {
        match self {
            Layer::Geom(geom) => geom.mapping_data(),
        }
    }
}

impl<G> From<G> for Layer
where
    G: Into<crate::geom::Geom>,
{
    fn from(geom: G) -> Self {
        Layer::Geom(geom.into())
    }
}

pub struct MappedElements<'a, DB: DrawingBackend> {
    pub x_range: (f64, f64),
    pub y_range: (f64, f64),
    pub elements: Vec<DynElement<'a, DB, (f64, f64)>>,
}
