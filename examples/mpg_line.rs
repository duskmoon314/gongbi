use gongbi::{aes, geom_line, plot};
use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    let mpg = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("examples/mpg.csv".into()))?
        .finish()?;

    let plot = plot!(mpg, aes!("hwy"), save = "gongbi.svg") + geom_line!();

    plot.draw()?;

    Ok(())
}
