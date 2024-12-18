use std::rc::Rc;

use derive_builder::Builder;

use super::Layer;

mod line;
mod point;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum GeomKind {
    Point,
    Line,
}

#[derive(Clone, Debug, Builder)]
pub struct Geom {
    kind: GeomKind,

    #[builder(default)]
    mapping: crate::aes::Aes,

    #[builder(default, setter(strip_option))]
    data: Option<Rc<dyn crate::data::Data>>,
}

impl Geom {
    pub fn builder() -> GeomBuilder {
        GeomBuilder::default()
    }
}

impl Layer for Geom {
    fn mapping_mut(&mut self) -> &mut crate::aes::Aes {
        &mut self.mapping
    }

    fn data_mut(&mut self) -> &mut Option<Rc<dyn crate::data::Data>> {
        &mut self.data
    }

    fn range_2d(&self) -> (f64, f64, f64, f64) {
        let x = self
            .mapping
            .x
            .unwrap_or_else(|| panic!("Layer {:?} does not have x mapping", self.kind));

        match self.mapping.y {
            Some(y) => {
                let data = self.data.as_ref().expect("data is not set");

                let x_range = data.column_range_f64(x);
                let y_range = data.column_range_f64(y);

                (x_range.0, x_range.1, y_range.0, y_range.1)
            }
            None => {
                let data = self.data.as_ref().expect("data is not set");

                let x_len = data.column_len(x);
                let x_range = data.column_range_f64(x);

                (0.0, x_len as f64, x_range.0, x_range.1)
            }
        }
    }

    fn draw_svg_2d(
        &self,
        chart: &mut plotters::prelude::ChartContext<
            '_,
            plotters::prelude::SVGBackend<'_>,
            plotters::prelude::Cartesian2d<
                plotters::coord::types::RangedCoordf64,
                plotters::coord::types::RangedCoordf64,
            >,
        >,
    ) -> anyhow::Result<()> {
        use plotters::prelude::*;

        let data = self.data.as_ref().expect("data is not set");
        let mapping = &self.mapping;

        match self.kind {
            GeomKind::Point => {
                let x = mapping.x.expect("x is not set");
                let y = mapping.y.expect("y is not set");

                let points: Vec<(f64, f64)> = data
                    .column_f64(x)
                    .into_iter()
                    .zip(data.column_f64(y))
                    .collect();

                let color = mapping.color.clone().unwrap_or_default().as_rgb();

                let style = match mapping.fill {
                    Some(false) => color.stroke_width(1),
                    _ => color.filled(),
                };

                let anno = chart.draw_series(PointSeries::of_element(
                    points,
                    mapping.size.unwrap_or(5),
                    style,
                    &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st),
                ))?;

                if let Some(label) = &mapping.label {
                    anno.label(label)
                        .legend(move |(x, y)| Circle::new((x, y), 5, color));
                }
            }

            GeomKind::Line => {
                let x = mapping.x.expect("x is not set");
                let y = mapping.y;

                let points: Vec<(f64, f64)> = match y {
                    Some(y) => data
                        .column_f64(x)
                        .into_iter()
                        .zip(data.column_f64(y))
                        .collect(),
                    None => {
                        let x = data.column_f64(x);

                        (0..x.len()).map(|u| u as f64).zip(x).collect()
                    }
                };

                let color = mapping.color.clone().unwrap_or_default().as_rgb();

                let anno = chart.draw_series(LineSeries::new(points, color))?;

                if let Some(label) = &mapping.label {
                    anno.label(label)
                        .legend(move |(x, y)| PathElement::new([(x, y), (x + 15, y)], color));
                }
            }
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! geom {
    ($kind:ident $(, $($arg:ident = $val:expr),* $(,)?)?) => {
        $crate::layer::geom::Geom::builder()
            .kind($crate::layer::geom::GeomKind::$kind)
            $( $(. $arg($val) )* )*
            .build()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{geom_point, *};

    use super::*;

    #[test]
    fn geom_macro() {
        let geom = geom!(Point);

        assert_eq!(geom.kind, GeomKind::Point);

        let point = geom_point!(aes!(x));

        assert_eq!(point.kind, GeomKind::Point);
        assert_eq!(point.mapping.x, Some("x"));
    }
}
