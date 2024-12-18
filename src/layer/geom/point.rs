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

use super::Geom;

#[macro_export]
macro_rules! geom_point {
    ($aes:expr $(, $($param: tt)*)?) => {
        $crate::geom!(Point, mapping = $aes $(, $($param)*)?)
    };

    ($($param: tt)*) => {
        $crate::geom!(Point, $($param)*)
    };
}

impl Geom {
    pub fn draw_point_2d<'a, DB>(
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
            anno.label(label);
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
