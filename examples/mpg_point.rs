use gongbi::{aes, geom_point, plot};
use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    let mpg = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("examples/mpg.csv".into()))?
        .finish()?;

    let plot = plot!(mpg, aes!("displ", "hwy"), save = "gongbi.svg") + geom_point!();

    plot.draw()?;

    Ok(())
}
