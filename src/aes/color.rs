use plotters::style::RGBColor;

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Column(String),
    Rgb(RGBColor),
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        Color::Column(value.to_string())
    }
}

impl From<RGBColor> for Color {
    fn from(value: RGBColor) -> Self {
        Color::Rgb(value)
    }
}

pub fn str_to_rgb(value: &str) -> RGBColor {
    let value = value.trim_start_matches('#');
    let r = u8::from_str_radix(&value[0..2], 16).unwrap();
    let g = u8::from_str_radix(&value[2..4], 16).unwrap();
    let b = u8::from_str_radix(&value[4..6], 16).unwrap();
    RGBColor(r, g, b)
}

#[macro_export]
macro_rules! rgb {
    ($r: expr, $g: expr, $b: expr) => {
        $crate::aes::color::Color::Rgb($crate::plotters::prelude::RGBColor($r, $g, $b))
    };

    ($str: literal) => {
        Color::Rgb(str_to_rgb($str))
    };
}
