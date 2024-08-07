// use std::path::PathBuf;

use std::path::PathBuf;

use layer::MappedElements;
/// Re-export plotters crate
pub use plotters;
use plotters::prelude::*;

pub mod aes;
pub mod data;
pub mod geom;
pub mod label;
pub mod layer;

#[derive(Debug, Default, typed_builder::TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct Plot {
    #[builder(setter(transform = |x: impl Into<data::Data>| Some(Box::new(x.into()))))]
    pub data: Option<Box<data::Data>>,

    pub mapping: aes::Aes,

    pub layers: Vec<layer::Layer>,

    pub label: label::Label,

    #[builder(setter(strip_option))]
    pub size: Option<(u32, u32)>,

    #[builder(setter(strip_option))]
    pub save: Option<PathBuf>,
}

#[macro_export]
macro_rules! plot {
    ($($arg:ident = $val:expr),* $(,)?) => {
        $crate::Plot::builder()
            $(.$arg($val))*
            .build()
    };

    ($data:expr $(, $($arg:ident = $val:expr),+ $(,)?)?) => {
        $crate::plot!(data = $data $(, $($arg = $val),+)?)
    };

    ($data:expr, $aes:expr $(, $($arg:ident = $val:expr),+ $(,)?)?) => {
        $crate::plot!(data = $data, mapping = $aes $(, $($arg = $val),+)?)
    }
}

impl<L> core::ops::Add<L> for Plot
where
    L: Into<layer::Layer>,
{
    type Output = Plot;

    fn add(self, rhs: L) -> Self::Output {
        // println!("self.mapping: {:?}", self.mapping);

        let mut rhs: layer::Layer = rhs.into();
        rhs.data_mut()
            .get_or_insert(self.data.as_ref().unwrap().clone());
        rhs.mapping_mut().inherit(&self.mapping);

        let mut layers = self.layers;
        layers.push(rhs);

        Plot { layers, ..self }
    }
}

impl core::ops::Add<label::Label> for Plot {
    type Output = Plot;

    fn add(self, rhs: label::Label) -> Self::Output {
        let label = self.label + rhs;

        Plot { label, ..self }
    }
}

impl Plot {
    pub fn draw(&self) -> anyhow::Result<()> {
        use plotters::prelude::*;

        let save = self.save.clone().unwrap_or("gongbi.png".into());
        let ext = save.extension().unwrap().to_str().unwrap();

        match ext {
            "png" => {
                let root = BitMapBackend::new(&save, self.size.unwrap_or((1024, 768)));

                self.draw_inner(root)?;
            }
            "svg" => {
                let root = SVGBackend::new(&save, self.size.unwrap_or((1024, 768)));

                self.draw_inner(root)?;
            }
            _ => panic!("Unsupported file extension"),
        }

        Ok(())
    }

    fn draw_inner<DB: DrawingBackend>(&self, db: DB) -> anyhow::Result<()>
    where
        <DB as plotters::prelude::DrawingBackend>::ErrorType: 'static,
    {
        use plotters::prelude::*;

        let root = db.into_drawing_area();

        root.fill(&WHITE)?;

        let mut elements: Vec<DynElement<DB, (f64, f64)>> = Vec::new();
        let mut x_range = (f64::INFINITY, -f64::INFINITY);
        let mut y_range = (f64::INFINITY, -f64::INFINITY);

        self.layers.iter().for_each(|layer| {
            let MappedElements {
                x_range: x,
                y_range: y,
                elements: element,
            } = layer.mapping_data::<DB>();

            x_range = (x_range.0.min(x.0), x_range.1.max(x.1));

            y_range = (y_range.0.min(y.0), y_range.1.max(y.1));

            elements.extend(element);
        });

        let x_range_len = x_range.1 - x_range.0;
        let y_range_len = y_range.1 - y_range.0;

        x_range = (
            x_range.0 - 0.025 * x_range_len,
            x_range.1 + 0.025 * x_range_len,
        );
        y_range = (
            y_range.0 - 0.025 * y_range_len,
            y_range.1 + 0.025 * y_range_len,
        );

        let mut chart = ChartBuilder::on(&root);

        chart
            .margin(5)
            .x_label_area_size(10.percent())
            .y_label_area_size(10.percent());

        if let Some(caption) = self.label.caption.as_ref() {
            chart.caption(caption, ("sans-serif", 32).into_font());
        }

        let mut chart = chart.build_cartesian_2d(x_range.0..x_range.1, y_range.0..y_range.1)?;

        let mut mesh = chart.configure_mesh();
        mesh.axis_desc_style(("sans-serif", 24).into_font())
            .x_label_style(("sans-serif", 16).into_font())
            .y_label_style(("sans-serif", 16).into_font());

        if let Some(x_label) = self.label.x.as_ref() {
            mesh.x_desc(x_label);
        } else if let Some(x_label) = self.mapping.x.as_ref() {
            mesh.x_desc(x_label);
        } else {
            mesh.x_desc("x");
        }

        if let Some(y_label) = self.label.y.as_ref() {
            mesh.y_desc(y_label);
        } else if let Some(y_label) = self.mapping.y.as_ref() {
            mesh.y_desc(y_label);
        } else {
            mesh.y_desc("y");
        }

        mesh.draw()?;

        chart.draw_series(elements)?;

        root.present()?;

        Ok(())
    }
}

// #[derive(Debug, thiserror::Error)]
// pub enum PlotError<E>
// where
//     E: std::error::Error + Send + Sync,
// {
//     #[error("Plotters error: {0}")]
//     Plotters(#[from] plotters::prelude::DrawingAreaErrorKind<E>),
// }

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn plot_macro() -> anyhow::Result<()> {
//         // let map: HashMap<String, Vec<u32>> = HashMap::new();\
//         let df = polars::df!(
//             "a" => [1, 2, 3],
//             "b" => [4, 5, 6],
//         )?;

//         let plot = plot!(df);
//         assert!(plot.data.is_some());
//         assert_eq!(plot.aes, Some(aes::Aes::default()));

//         Ok(())
//     }
// }
