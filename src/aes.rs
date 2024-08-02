#[derive(Clone, Debug, Default, PartialEq, typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct Aes {
    pub x: Option<String>,
    pub y: Option<String>,
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
    use super::*;

    #[test]
    fn aes_macro() {
        let aes = aes!("x");
        assert_eq!(
            aes,
            Aes {
                x: Some(String::from("x")),
                y: None,
            }
        );

        let aes = aes!("x", "y");
        assert_eq!(
            aes,
            Aes {
                x: Some(String::from("x")),
                y: Some(String::from("y")),
            }
        );

        let aes = aes!("x", y = "y");
        assert_eq!(
            aes,
            Aes {
                x: Some(String::from("x")),
                y: Some(String::from("y")),
            }
        );

        let aes = aes!(x = "x", y = "y");
        assert_eq!(
            aes,
            Aes {
                x: Some(String::from("x")),
                y: Some(String::from("y")),
            }
        );
    }
}
