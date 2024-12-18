use std::ops::{Add, AddAssign};

use derive_builder::Builder;

pub mod color;

#[derive(Clone, Debug, Default, PartialEq, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct Aes {
    pub x: Option<&'static str>,

    pub y: Option<&'static str>,

    pub color: Option<color::Color>,

    pub fill: Option<bool>,

    pub size: Option<i32>,

    pub label: Option<String>,
}

impl Aes {
    pub fn builder() -> AesBuilder {
        AesBuilder::default()
    }
}

impl AddAssign for Aes {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x.or(rhs.x);
        self.y = self.y.or(rhs.y);
        self.color = self.color.clone().or(rhs.color);
        self.fill = self.fill.or(rhs.fill);
        self.size = self.size.or(rhs.size);
        self.label = self.label.clone().or(rhs.label);
    }
}

impl Add for Aes {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self.clone();
        res += rhs;
        res
    }
}

impl AesBuilder {
    pub fn colour<T: Into<color::Color>>(&mut self, colour: T) -> &mut Self {
        self.color(colour)
    }

    pub fn col<T: Into<color::Color>>(&mut self, col: T) -> &mut Self {
        self.color(col)
    }
}

// Trick to hide internal implementation details from the docs
macro_rules! __aes {
    ($aes: item) => {
        /// Construct Aesthetic Mappings
        $aes
    };
}

#[cfg(doc)]
__aes![
    #[macro_export]
    macro_rules! aes {
        ($($param: tt)*) => {
            ...
        };
    }
];

#[cfg(not(doc))]
__aes![
    #[macro_export]
    macro_rules! aes {
        // The terminal case, call builder's build method
        (@impl $aes: expr,) => {{
            $aes.build().unwrap()
        }};

        // ===== Handle different aesthetics =====
        (@impl $aes: expr, $key: ident = $value: ident $(, $($rest: tt)*)?) => {{
            $aes.$key(stringify!($value));
            aes!(@impl $aes, $($($rest)*)?)
        }};

        (@impl $aes: expr, $key: ident = $value: expr $(, $($rest: tt)*)?) => {{
            $aes.$key($value);
            aes!(@impl $aes, $($($rest)*)?)
        }};

        // ===== Extend the Main Macro =====

        // Extend the macro to accept x and y without named
        ($x: ident, $y: ident $(, $($param: tt)*)?) => {
            aes!(x = $x, y = $y $(, $($param)+)?)
        };

        ($x: literal, $y: literal $(, $($param: tt)*)?) => {
            aes!(x = $x, y = $y $(, $($param)+)?)
        };

        // Extend the macro to accept x without named
        ($x: ident $(, $($param: tt)*)?) => {
            aes!(x = $x $(, $($param)+)?)
        };

        ($x: literal $(, $($param: tt)*)?) => {
            aes!(x = $x $(, $($param)+)?)
        };

        // ===== The Main Macro Entry =====

        ($($param: tt)*) => {{
            let mut aes = $crate::aes::Aes::builder();
            let aes = aes!(@impl &mut aes, $($param)*);
            aes
        }};
    }
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aes_builder() {
        let mut aes = Aes::builder();
        aes.x("mpg");
        let aes = aes.build().unwrap();
        assert_eq!(
            aes,
            Aes {
                x: Some("mpg"),
                ..Default::default()
            }
        );
    }

    #[test]
    fn aes_macro() {
        let aes = aes!(x = mpg);
        assert_eq!(
            aes,
            Aes {
                x: Some("mpg"),
                ..Default::default()
            }
        );

        let aes = aes!(mpg);
        assert_eq!(
            aes,
            Aes {
                x: Some("mpg"),
                ..Default::default()
            }
        );
    }
}
