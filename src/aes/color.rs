use plotters::style::{RGBAColor, RGBColor};

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Column(String),
    Rgba(RGBAColor),
}

impl From<String> for Color {
    fn from(value: String) -> Self {
        Color::Column(value)
    }
}

impl From<RGBAColor> for Color {
    fn from(value: RGBAColor) -> Self {
        Color::Rgba(value)
    }
}

impl From<RGBColor> for Color {
    fn from(value: RGBColor) -> Self {
        Color::Rgba(value.into())
    }
}

#[macro_export]
macro_rules! rgb {
    ($r: expr, $g: expr, $b: expr) => {
        $crate::aes::color::Color::Rgba($crate::plotters::style::RGBColor($r, $g, $b).into())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_macro() {
        let color = rgb!(0, 0, 0);
        assert_eq!(color, Color::Rgba(RGBAColor(0, 0, 0, 1.0)));
    }
}
