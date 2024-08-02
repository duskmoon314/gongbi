use plotters::style::{Palette, PaletteColor};

pub trait PaletteExt: Palette
where
    Self: Sized,
{
    fn idx2color(&self, idx: usize) -> PaletteColor<Self> {
        <Self as Palette>::pick(idx)
    }
}

impl<T: Palette> PaletteExt for T {}

pub struct Pastel1;

impl Palette for Pastel1 {
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
