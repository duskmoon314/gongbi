#[derive(Clone, Debug, Default, PartialEq)]
pub enum Palette {
    #[default]
    Pastel1,
}

pub trait PaletteExt: plotters::style::Palette
where
    Self: Sized,
{
    fn idx2color(&self, idx: usize) -> plotters::style::PaletteColor<Self> {
        <Self as plotters::style::Palette>::pick(idx)
    }
}

impl<T: plotters::style::Palette> PaletteExt for T {}

pub struct Pastel1;

impl plotters::style::Palette for Pastel1 {
    const COLORS: &'static [(u8, u8, u8)] = &[
        (251, 180, 174),
        (179, 205, 227),
        (204, 235, 197),
        (222, 203, 228),
        (254, 217, 166),
        (255, 255, 204),
        (229, 216, 189),
        (253, 218, 236),
        (242, 242, 242),
    ];
}
