pub mod color;
pub mod plaette;

#[derive(Clone, Debug, Default, PartialEq, typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct Aes {
    /// The `x` dimension to use
    pub x: Option<String>,

    /// The `y` dimension to use
    pub y: Option<String>,

    pub alpha: Option<f64>,

    pub color: Option<color::Color>,

    #[builder(default = true, setter(!strip_option))]
    pub fill: bool,

    pub size: Option<i32>,
}

/// The builder for the `Aes` struct
pub fn aes() -> AesBuilder {
    Aes::builder()
}

#[macro_export]
macro_rules! aes {
    // The main implementation of the aes macro
    ($($arg:ident = $val:expr),* $(,)?) => {
        $crate::aes::Aes::builder()
            $(.$arg($val))*
            .build()
    };

    // Extend the macro to accept x without named
    ($x: literal $(, $($arg:ident = $val:expr),* $(,)?)?) => {
        $crate::aes!(x = $x $(, $($arg = $val),+)?)
    };

    // Extend the macro to accept x and y without named
    ($x: literal, $y: literal $(, $($arg:ident = $val:expr),* $(,)?)?) => {
        $crate::aes!(x = $x, y = $y $(, $($arg = $val),+)?)
    };
}

#[cfg(test)]
mod tests {
    use crate::rgb;

    use super::*;

    #[test]
    fn aes_macro() {
        let aes = aes!("x");
        assert_eq!(
            aes,
            Aes {
                x: Some(String::from("x")),
                ..Default::default()
            }
        );

        let aes = aes!("x", "y");
        assert_eq!(
            aes,
            Aes {
                x: Some(String::from("x")),
                y: Some(String::from("y")),
                ..Default::default()
            }
        );

        let aes = aes!("x", y = "y");
        assert_eq!(
            aes,
            Aes {
                x: Some(String::from("x")),
                y: Some(String::from("y")),
                ..Default::default()
            }
        );

        let aes = aes!(x = "x", y = "y");
        assert_eq!(
            aes,
            Aes {
                x: Some(String::from("x")),
                y: Some(String::from("y")),
                ..Default::default()
            }
        );

        let aes = aes!("x", color = rgb!(255, 0, 0));
    }

    #[test]
    fn aes_builder_fn() {
        let aes = aes().x("x").build();
        assert_eq!(
            aes,
            Aes {
                x: Some(String::from("x")),
                y: None,
                ..Default::default()
            }
        );
    }
}
