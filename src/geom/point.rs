use plotters::{coord::Shift, prelude::*, style::Color};

use crate::aes::plaette::Pastel1;

use super::GeomMethod;

// use super::Geom;

#[derive(Clone, Debug, PartialEq, typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct GeomPoint {
    pub aes: Option<crate::aes::Aes>,
}

#[macro_export]
macro_rules! geom_point {
    ($($arg:ident = $val:expr),* $(,)?) => {
        $crate::geom::point::GeomPoint::builder()
            $(.$arg($val))*
            .build()
    };

    ($aes: expr $(, $($arg:ident = $val:expr),+ $(,)?)?) => {
        $crate::geom_point!(aes = $aes $(, $($arg = $val),+)?)
    }
}

impl GeomMethod for GeomPoint {
    fn aes_mut(&mut self) -> &mut Option<crate::aes::Aes> {
        &mut self.aes
    }

    fn draw<DB: DrawingBackend>(
        &self,
        area: &DrawingArea<DB, Shift>,
        data: &crate::data::Data,
    ) -> Result<(), crate::PlotError<DB::ErrorType>> {
        let aes = self.aes.as_ref().expect("aes is required for geom_point");

        let x = aes.x.as_ref().expect("x is required for geom_point");
        let y = aes.y.as_ref().expect("y is required for geom_point");

        let x_range: (f64, f64) = data.column_range(x);
        let y_range: (f64, f64) = data.column_range(y);

        let mut chart = ChartBuilder::on(area)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .margin((1).percent())
            .build_cartesian_2d(x_range.0..x_range.1, y_range.0..y_range.1)?;

        chart.draw_series([Rectangle::new(
            [(x_range.0, y_range.0), (x_range.1, y_range.1)],
            BLACK.mix(0.2).filled(),
        )])?;

        let mut mesh = chart.configure_mesh();
        mesh.x_desc(x)
            .y_desc(y)
            .light_line_style(TRANSPARENT)
            .x_labels(x_range.1 as usize - x_range.0 as usize)
            .y_labels(y_range.1 as usize - y_range.0 as usize);

        mesh.draw()?;

        let points: Vec<(f64, f64)> = data.column(x).into_iter().zip(data.column(y)).collect();

        let color = match &aes.color {
            Some(crate::aes::color::Color::Column(c)) => data.column_to_color(c, &Pastel1),
            Some(crate::aes::color::Color::Rgb(rgb)) => vec![rgb.filled(); points.len()],
            None => vec![BLACK.filled(); points.len()],
        };

        chart.draw_series(
            points
                .iter()
                .zip(color)
                .map(|((x, y), style)| Circle::new((*x, *y), aes.size.unwrap_or(5), style)),
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aes;

    #[test]
    fn geom_point_macro() {
        let geom_point = geom_point!();
        assert_eq!(geom_point, GeomPoint { aes: None });

        let geom_point = geom_point!(aes!("x"));
        assert_eq!(
            geom_point,
            GeomPoint {
                aes: Some(aes!("x"))
            }
        );
    }
}
