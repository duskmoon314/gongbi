use std::ops::Add;

use derive_builder::Builder;

#[derive(Clone, Debug, Default, PartialEq, Builder)]
#[builder(default, setter(into, strip_option))]
pub struct Label {
    pub caption: Option<String>,

    pub x: Option<String>,

    pub y: Option<String>,
}

impl Label {
    pub fn builder() -> LabelBuilder {
        LabelBuilder::default()
    }
}

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
