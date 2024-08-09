use plotters::prelude::*;

use crate::layer::MappedElements;

#[derive(Clone, Debug, PartialEq, typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct Point {
    #[builder(setter(strip_option))]
    pub data: Option<Box<crate::data::Data>>,

    pub mapping: crate::aes::Aes,
}

#[macro_export]
macro_rules! geom_point {
    ($($arg:ident = $val:expr),* $(,)?) => {
        $crate::geom::point::Point::builder()
            $(.$arg($val))*
            .build()
    };

    ($aes:expr $(, $($arg:ident = $val:expr),+ $(,)?)?) => {
        $crate::geom_point!(mapping = $aes $(, $($arg = $val),+)?)
    }
}

impl Point {
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
        let data = self.data.as_ref().expect("data is required for geom_point");
        let mapping = &self.mapping;

        let x = mapping.x.as_ref().expect("x is required for geom_point");
        let y = mapping.y.as_ref().expect("y is required for geom_point");

        let x_range = data.column_range_f64(x);
        let y_range = data.column_range_f64(y);

        let points: Vec<(f64, f64)> = data
            .column_f64(x)
            .into_iter()
            .zip(data.column_f64(y))
            .collect();

        let mut color = mapping.color.clone().unwrap_or_default();
        if mapping.alpha.is_some() {
            *color.alpha_mut() = mapping.alpha.unwrap();
        }

        let elements = points
            .iter()
            .map(|(x, y)| {
                Circle::new(
                    (*x, *y),
                    mapping.size.unwrap_or(5),
                    if mapping.fill.unwrap_or(true) {
                        color.0.filled()
                    } else {
                        color.0.stroke_width(1)
                    },
                )
                .into_dyn()
            })
            .collect();

        MappedElements {
            x_range,
            y_range,
            elements,
        }
    }
}
