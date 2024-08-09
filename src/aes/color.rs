use core::panic;
use std::str::FromStr;

use plotters::style::{RGBAColor, RGBColor};

/// Color type for aesthetics
#[derive(Clone, Debug, PartialEq)]
pub struct Color(pub RGBAColor);

impl Color {
    pub fn alpha_mut(&mut self) -> &mut f64 {
        &mut self.0 .3
    }
}

impl Default for Color {
    fn default() -> Self {
        Color(RGBAColor(0, 0, 0, 1.0))
    }
}

impl From<RGBAColor> for Color {
    fn from(value: RGBAColor) -> Self {
        Color(value)
    }
}

impl From<RGBColor> for Color {
    fn from(value: RGBColor) -> Self {
        Color(value.into())
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Color(RGBAColor(value.0, value.1, value.2, 1.0))
    }
}

impl From<(u8, u8, u8, f64)> for Color {
    fn from(value: (u8, u8, u8, f64)) -> Self {
        Color(RGBAColor(value.0, value.1, value.2, value.3))
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "black" => Ok(Color(RGBAColor(0, 0, 0, 1.0))),
            "silver" => Ok(Color(RGBAColor(192, 192, 192, 1.0))),
            "gray" => Ok(Color(RGBAColor(128, 128, 128, 1.0))),
            "white" => Ok(Color(RGBAColor(255, 255, 255, 1.0))),
            "maroon" => Ok(Color(RGBAColor(128, 0, 0, 1.0))),
            "red" => Ok(Color(RGBAColor(255, 0, 0, 1.0))),
            "purple" => Ok(Color(RGBAColor(128, 0, 128, 1.0))),
            "fuchsia" => Ok(Color(RGBAColor(255, 0, 255, 1.0))),
            "green" => Ok(Color(RGBAColor(0, 128, 0, 1.0))),
            "lime" => Ok(Color(RGBAColor(0, 255, 0, 1.0))),
            "olive" => Ok(Color(RGBAColor(128, 128, 0, 1.0))),
            "yellow" => Ok(Color(RGBAColor(255, 255, 0, 1.0))),
            "navy" => Ok(Color(RGBAColor(0, 0, 128, 1.0))),
            "blue" => Ok(Color(RGBAColor(0, 0, 255, 1.0))),
            "teal" => Ok(Color(RGBAColor(0, 128, 128, 1.0))),
            "aqua" => Ok(Color(RGBAColor(0, 255, 255, 1.0))),
            s if s.starts_with("#") => {
                if s.len() != 7 {
                    return Err("Invalid color value".to_string());
                }
                let r = u8::from_str_radix(&s[1..3], 16).expect("Invalid red value");
                let g = u8::from_str_radix(&s[3..5], 16).expect("Invalid green value");
                let b = u8::from_str_radix(&s[5..7], 16).expect("Invalid blue value");
                Ok(Color(RGBAColor(r, g, b, 1.0)))
            }
            _ => panic!("Unsupported color"),
        }
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        Color::from_str(value).unwrap()
    }
}

impl From<String> for Color {
    fn from(value: String) -> Self {
        Color::from_str(&value).unwrap()
    }
}

#[macro_export]
macro_rules! rgb {
    ($r: expr, $g: expr, $b: expr) => {
        $crate::aes::color::Color($crate::plotters::style::RGBColor($r, $g, $b).into())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_from_str() {
        let red = Color::from("red");
        assert_eq!(red, Color(RGBAColor(255, 0, 0, 1.0)));

        let red = Color::from("#ff0000");
        assert_eq!(red, Color(RGBAColor(255, 0, 0, 1.0)));
    }
}
