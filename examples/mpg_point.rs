use std::{env, path::PathBuf};

use gongbi::{aes, geom_point, labs, plot};
use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    let examples_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("examples");
    let mpg = examples_dir.join("mpg.csv");

    let mpg = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(mpg))?
        .finish()?;

    let plot = plot!(mpg.clone(), aes!("displ", "hwy"))
        + geom_point!()
        + labs!(caption = "Demo of geom_point");

    plot.to_svg(examples_dir.join("mpg_point.svg"), (1024, 768))?;
    // plot.to_png(examples_dir.join("mpg_point.png"), (1024, 768))?;

    let plot = plot!(mpg.clone(), aes!("displ", "hwy", color = "blue"))
        + geom_point!()
        + labs!(caption = "Demo of geom_point with color");

    plot.to_svg(examples_dir.join("mpg_point_color.svg"), (1024, 768))?;

    let plot = plot!(mpg.clone(), aes!("displ", "hwy"))
        + geom_point!(aes!(shape = 0))
        + labs!(caption = "Demo of geom_point with shape");

    plot.to_svg(examples_dir.join("mpg_point_shape.svg"), (1024, 768))?;

    Ok(())
}
