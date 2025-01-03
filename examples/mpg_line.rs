use std::{env, path::PathBuf};

use gongbi::{aes, geom_line, plot};
use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    let examples_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("examples");
    let mpg = examples_dir.join("mpg.csv");

    let mpg = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(mpg))?
        .finish()?;

    let plot = plot!(mpg.clone(), aes!(hwy)) + geom_line!(aes!(label = "hwy"));

    plot.to_svg(examples_dir.join("mpg_line.svg"), (1024, 768))?;

    let plot = plot!(mpg.clone(), aes!(hwy)) + geom_line!(aes!(shape = 2));

    plot.to_svg(examples_dir.join("mpg_line_shape.svg"), (1024, 768))?;

    Ok(())
}
