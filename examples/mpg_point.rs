use gongbi::{aes, geom_point, labs, plot};
use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    let mpg = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("mpg.csv".into()))?
        .finish()?;

    let plot = plot!(mpg.clone(), aes!("displ", "hwy"))
        + geom_point!()
        + labs!(caption = "Demo of geom_point");

    plot.to_svg("mpg_point.svg", (1024, 768))?;

    let plot = plot!(mpg.clone(), aes!("displ", "hwy", color = "blue"))
        + geom_point!()
        + labs!(caption = "Demo of geom_point with color");

    plot.to_svg("mpg_point_color.svg", (1024, 768))?;

    Ok(())
}
