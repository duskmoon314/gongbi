use plotters::prelude::*;

use crate::layer::MappedElements;

#[derive(Clone, Debug, PartialEq, typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct Line {
    #[builder(setter(strip_option))]
    pub data: Option<Box<crate::data::Data>>,

    pub mapping: crate::aes::Aes,
}

#[macro_export]
macro_rules! geom_line {
    ($($arg:ident = $val:expr),* $(,)?) => {
        $crate::geom::line::Line::builder()
            $(.$arg($val))*
            .build()
    };

    ($aes:expr $(, $($arg:ident = $val:expr),+ $(,)?)?) => {
        $crate::geom_line!(mapping = $aes $(, $($arg = $val),+)?)
    }
}

impl Line {
    pub fn data_mut(&mut self) -> &mut Option<Box<crate::data::Data>> {
        &mut self.data
    }

    pub fn mapping_mut(&mut self) -> &mut crate::aes::Aes {
        &mut self.mapping
    }

    pub fn mapping_data<'a, DB>(&self) -> MappedElements<'a, DB>
    where
        DB: DrawingBackend,
    {
        let data = self.data.as_ref().expect("data is required for geom_line");
        let mapping = &self.mapping;

        let x = mapping.x.as_ref().expect("x is required for geom_line");

        let x_range: (f64, f64);
        let y_range: (f64, f64);

        let points: Vec<(f64, f64)> = match mapping.y.as_ref() {
            Some(y) => {
                x_range = data.column_range_f64(x);
                y_range = data.column_range_f64(y);

                let points: Vec<(f64, f64)> = data
                    .column_f64(x)
                    .into_iter()
                    .zip(data.column_f64(y))
                    .collect();

                points
            }
            None => {
                x_range = (0.0, data.column_len(x) as f64);
                y_range = data.column_range_f64(x);

                let points: Vec<(f64, f64)> = data
                    .column_f64(x)
                    .into_iter()
                    .enumerate()
                    .map(|(i, x)| (i as f64, x))
                    .collect();

                points
            }
        };

        let color = match mapping.color {
            Some(crate::aes::color::Color::Rgba(rgba)) => {
                RGBAColor(rgba.0, rgba.1, rgba.2, mapping.alpha.unwrap_or(rgba.3))
            }
            None => BLACK.mix(1.0),
            _ => unimplemented!(),
        };

        let line_series = LineSeries::new(points.iter().map(|(x, y)| (*x, *y)), &color);

        let elements = line_series.into_iter().collect();

        MappedElements {
            x_range,
            y_range,
            elements,
        }
    }
}
