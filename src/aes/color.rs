use std::str::FromStr;

use plotters::style::RGBColor;

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    RGB(RGBColor),
}

impl Color {
    pub fn as_rgb(&self) -> RGBColor {
        match self {
            Color::RGB(rgb) => *rgb,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::RGB(RGBColor(0, 0, 0))
    }
}

impl From<RGBColor> for Color {
    fn from(color: RGBColor) -> Self {
        Color::RGB(color)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Color::RGB(RGBColor(value.0, value.1, value.2))
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "black" => Ok(Color::RGB(RGBColor(0, 0, 0))),
            "silver" => Ok(Color::RGB(RGBColor(192, 192, 192))),
            "gray" => Ok(Color::RGB(RGBColor(128, 128, 128))),
            "white" => Ok(Color::RGB(RGBColor(255, 255, 255))),
            "maroon" => Ok(Color::RGB(RGBColor(128, 0, 0))),
            "red" => Ok(Color::RGB(RGBColor(255, 0, 0))),
            "purple" => Ok(Color::RGB(RGBColor(128, 0, 128))),
            "fuchsia" => Ok(Color::RGB(RGBColor(255, 0, 255))),
            "green" => Ok(Color::RGB(RGBColor(0, 128, 0))),
            "lime" => Ok(Color::RGB(RGBColor(0, 255, 0))),
            "olive" => Ok(Color::RGB(RGBColor(128, 128, 0))),
            "yellow" => Ok(Color::RGB(RGBColor(255, 255, 0))),
            "navy" => Ok(Color::RGB(RGBColor(0, 0, 128))),
            "blue" => Ok(Color::RGB(RGBColor(0, 0, 255))),
            "teal" => Ok(Color::RGB(RGBColor(0, 128, 128))),
            "aqua" => Ok(Color::RGB(RGBColor(0, 255, 255))),
            s if s.starts_with("#") => {
                if s.len() != 7 {
                    return Err("Invalid color value".to_string());
                }
                let r = u8::from_str_radix(&s[1..3], 16).expect("Invalid red value");
                let g = u8::from_str_radix(&s[3..5], 16).expect("Invalid green value");
                let b = u8::from_str_radix(&s[5..7], 16).expect("Invalid blue value");
                Ok(Color::RGB(RGBColor(r, g, b)))
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
