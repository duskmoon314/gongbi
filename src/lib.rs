use std::path::PathBuf;

pub use plotters;

use geom::GeomMethod;

pub mod aes;
pub mod data;
pub mod geom;

#[derive(Default, typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into, strip_option)))]
pub struct Plot {
    pub data: Option<data::Data>,

    pub aes: Option<aes::Aes>,

    pub size: Option<(u32, u32)>,

    #[builder(setter(!strip_option))]
    pub geoms: Vec<geom::Geom>,

    pub save: Option<PathBuf>,
}

#[macro_export]
macro_rules! plot {
    ($data:ident, $aes:expr $(, $($arg:ident = $val:expr),+ $(,)?)?) => {
        $crate::Plot::builder()
            .data($data)
            .aes($aes)
            $(.$($arg($val))+)?
            .build()
    };

    ($data:ident $(, $($arg:ident = $val:expr),+ $(,)?)?) => {
        $crate::plot!($data, $crate::aes!() $(, $($arg = $val),+)?)
    };
}

impl<G> core::ops::Add<G> for Plot
where
    G: Into<geom::Geom> + GeomMethod,
{
    type Output = Plot;

    fn add(self, rhs: G) -> Self::Output {
        let mut rhs = rhs;
        rhs.aes_mut()
            .get_or_insert(self.aes.clone().unwrap_or_default());

        let mut geoms = self.geoms;
        geoms.push(rhs.into());

        Plot { geoms, ..self }
    }
}

impl Plot {
    pub fn draw(&self) -> anyhow::Result<()> {
        use plotters::prelude::*;

        let save = self.save.clone().unwrap_or("gongbi.png".into());
        let ext = save.extension().unwrap().to_str().unwrap();

        match ext {
            "png" => {
                let root =
                    BitMapBackend::new(&save, self.size.unwrap_or((1024, 768))).into_drawing_area();

                root.fill(&WHITE)?;

                for geom in &self.geoms {
                    geom.draw(&root, self.data.as_ref().unwrap())?;
                }

                root.present()?;
            }
            "svg" => {
                let root =
                    SVGBackend::new(&save, self.size.unwrap_or((1024, 768))).into_drawing_area();

                root.fill(&WHITE)?;

                for geom in &self.geoms {
                    geom.draw(&root, self.data.as_ref().unwrap())?;
                }

                root.present()?;
            }
            _ => panic!("Unsupported file extension"),
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PlotError<E>
where
    E: std::error::Error + Send + Sync,
{
    #[error("Plotters error: {0}")]
    Plotters(#[from] plotters::prelude::DrawingAreaErrorKind<E>),
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn plot_macro() -> anyhow::Result<()> {
        // let map: HashMap<String, Vec<u32>> = HashMap::new();\
        let df = polars::df!(
            "a" => [1, 2, 3],
            "b" => [4, 5, 6],
        )?;

        let plot = plot!(df);
        assert!(plot.data.is_some());
        assert_eq!(plot.aes, Some(aes::Aes::default()));

        Ok(())
    }
}
