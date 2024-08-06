pub mod color;
pub mod plaette;

#[derive(Clone, Debug, Default, PartialEq, typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct Aes {
    /// Which column is used for the x-axis
    pub x: Option<String>,

    /// Which column is used for the y-axis
    pub y: Option<String>,

    /// The alpha value for the color
    pub alpha: Option<f64>,

    /// The color to use
    pub color: Option<color::Color>,

    /// Whether to fill the shape
    pub fill: Option<bool>,

    /// The size of the shape
    pub size: Option<i32>,
}

impl Aes {
    pub fn inherit(&mut self, parent_aes: &Aes) {
        self.x = self.x.clone().or(parent_aes.x.clone());
        self.y = self.y.clone().or(parent_aes.y.clone());
        self.alpha = self.alpha.or(parent_aes.alpha);
        self.color = self.color.clone().or(parent_aes.color.clone());
        self.fill = self.fill.or(parent_aes.fill);
        self.size = self.size.or(parent_aes.size);
    }
}

#[macro_export]
macro_rules! aes {
    // The main implementation
    ($($arg:ident = $val:expr),* $(,)?) => {
        $crate::aes::Aes::builder()
            $(.$arg($val))*
            .build()
    };

    // Extend the macro to accept x without named
    ($x: expr $(, $($arg:ident = $val:expr),* $(,)?)?) => {
        $crate::aes!(x = $x $(, $($arg = $val),+)?)
    };

    // Extend the macro to accept x and y without named
    ($x: expr, $y: expr $(, $($arg:ident = $val:expr),* $(,)?)?) => {
        $crate::aes!(x = $x, y = $y $(, $($arg = $val),+)?)
    };
}

#[cfg(test)]
mod tests {
    use crate::rgb;

    // use super::*;

    #[test]
    fn aes_macro() {
        let _ = aes!("x");
        let _ = aes!("x", "y");
        let _ = aes!("x", y = "y");
        let _ = aes!(x = "x", y = "y");
        let _ = aes!("x", color = rgb!(255, 0, 0));
    }
}
