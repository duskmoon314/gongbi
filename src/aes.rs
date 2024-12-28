//! Aesthetic mappings

use std::ops::{Add, AddAssign};

use derive_builder::Builder;

pub mod color;

/// Aesthetic mappings
///
/// Aesthetic mappings describe how variables in the data are mapped to visual
/// properties (aesthetics) of geoms. Aesthetic mappings are constructed with
/// the [`aes!`](crate::aes!) marco or [`Aes::builder`] method.
///
/// TODO: some aesthetics are not used as "mapping" but as "constant" values.
#[derive(Clone, Debug, Default, PartialEq, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct Aes {
    /// The column name to map to the x-axis
    pub x: Option<&'static str>,

    /// The column name to map to the y-axis
    pub y: Option<&'static str>,

    /// The color aesthetic
    pub color: Option<color::Color>,

    /// The fill aesthetic
    pub fill: Option<bool>,

    /// The size aesthetic
    pub size: Option<i32>,

    /// The shape aesthetic
    pub shape: Option<u8>,

    /// The label aesthetic
    pub label: Option<String>,
}

impl Aes {
    /// Create a new [`Aes`] object via the builder pattern
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
        self.shape = self.shape.or(rhs.shape);
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
    /// Set the color aesthetic, alias for [`AesBuilder::color`]
    pub fn colour<T: Into<color::Color>>(&mut self, colour: T) -> &mut Self {
        self.color(colour)
    }

    /// Set the color aesthetic, alias for [`AesBuilder::color`]
    pub fn col<T: Into<color::Color>>(&mut self, col: T) -> &mut Self {
        self.color(col)
    }
}

// Trick to hide internal implementation details from the docs
macro_rules! __aes {
    ($aes: item) => {
        /// # aes!: Construct Aesthetic Mappings
        ///
        /// This macro is used to create an [`Aes`] object in a more concise way
        /// like `ggplot2`. It is a wrapper around [`Aes::builder`].
        ///
        /// ## Usage
        ///
        /// ```ignore
        /// aes!(
        ///     x = <X_COLUMN>,
        ///     y = <Y_COLUMN>,
        ///     ...
        /// )
        /// ```
        ///
        /// ### Arguments
        ///
        /// #### `x` and `y`
        ///
        /// The column names to map to the x-axis and y-axis, respectively.
        ///
        /// If they are the first two arguments, they can be passed without the
        /// named argument.
        ///
        /// ```
        /// # use gongbi::*;
        /// let a1 = aes!(x);
        /// let a2 = aes!(x, y);
        ///
        /// assert_eq!(a1.x, Some("x"));
        /// assert_eq!(a2.x, Some("x"));
        /// assert_eq!(a2.y, Some("y"));
        /// ```
        ///
        /// #### Other Aesthetics
        ///
        /// Other aesthetics can be set with the named argument.
        ///
        /// - `color`
        /// - `size`
        /// - `shape`
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
