//! Label module.

use std::ops::Add;

use derive_builder::Builder;

/// # Label layer
///
/// This layer is used to add important information to the plot.
#[derive(Clone, Debug, Default, PartialEq, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct Label {
    /// The caption of the label.
    pub caption: Option<String>,

    /// The x-axis description.
    pub x: Option<String>,

    /// The y-axis description.
    pub y: Option<String>,
}

impl Label {
    /// Create a [`Label`] via the builder pattern.
    pub fn builder() -> LabelBuilder {
        LabelBuilder::default()
    }
}

/// # labs!: Construct a new [`Label`] layer.
///
/// This macro is used to construct a new [`Label`] layer in a more concise like `ggplot2`.
/// It is a wrapper around the [`Label::builder`] method.
///
/// ## Usage
///
/// ```ignore
/// labs!(
///     caption = <Caption>,
///     x = <X>,
///     y = <Y>,
/// )
/// ```
///
/// ### Arguments
///
/// - `caption`: Also known as the title of the plot.
/// - `x`: The x-axis description.
/// - `y`: The y-axis description.
#[macro_export]
macro_rules! labs {
    ($($arg: ident = $val: expr),* $(,)?) => {
        $crate::label::Label::builder()
            $(.$arg($val))*
            .build()
            .unwrap()
    }
}

impl Add for Label {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            caption: self.caption.or(rhs.caption),
            x: self.x.or(rhs.x),
            y: self.y.or(rhs.y),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_label_macro() {
        let label = labs!(caption = "Caption", x = "X", y = "Y");

        assert_eq!(label.caption, Some("Caption".to_string()));
        assert_eq!(label.x, Some("X".to_string()));
        assert_eq!(label.y, Some("Y".to_string()));
    }
}
