use core::ops::Add;

#[derive(Clone, Debug, Default, PartialEq, typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct Label {
    /// The caption of the plot.
    pub caption: Option<String>,

    /// The x-axis label of the plot.
    pub x: Option<String>,

    /// The y-axis label of the plot.
    pub y: Option<String>,
}

impl Add for Label {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            caption: rhs.caption.or(self.caption),
            x: rhs.x.or(self.x),
            y: rhs.y.or(self.y),
        }
    }
}

#[macro_export]
macro_rules! labs {
    ($($arg:ident = $val:expr),* $(,)?) => {
        $crate::label::Label::builder()
            $(.$arg($val))*
            .build()
    }
}
