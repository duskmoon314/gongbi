use gongbi::{aes, geom_point, labs, plot};
use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    let mpg = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("mpg.csv".into()))?
        .finish()?;

    let plot = plot!(mpg.clone(), aes!("displ", "hwy"), save = "mpg_point.svg")
        + geom_point!()
        + labs!(caption = "Demo of geom_point");

    plot.draw()?;

    let plot = plot!(
        mpg.clone(),
        aes!("displ", "hwy", color = "blue"),
        save = "mpg_point_color.svg"
    ) + geom_point!();
    plot.draw()?;

    Ok(())
}
