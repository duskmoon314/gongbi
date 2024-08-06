// use std::collections::HashMap;

// use ::polars::prelude as pl;
// use plotters::style::{Color, Palette};

// use crate::aes::plaette::PaletteExt;

// use super::*;

// impl DataMethod for pl::DataFrame {
//     fn column<T: NumCast>(&self, column: &str) -> Vec<T> {
//         self.column(column)
//             .unwrap_or_else(|e| panic!("Column {column} not found: {e}"))
//             .iter()
//             .map(|any_val| any_val.try_extract().unwrap())
//             .collect()
//     }

//     fn column_range<T: NumCast>(&self, column: &str) -> (T, T) {
//         let series = self
//             .column(column)
//             .unwrap_or_else(|e| panic!("Column {column} not found: {e}"));

//         let min = series
//             .min()
//             .unwrap_or_else(|e| panic!("Failed to get min: {e}"))
//             .unwrap_or_else(|| panic!("Column is empty"));

//         let max = series
//             .max()
//             .unwrap_or_else(|e| panic!("Failed to get max: {e}"))
//             .unwrap_or_else(|| panic!("Column is empty"));

//         (min, max)
//     }

//     fn column_to_color<P: Palette>(&self, column: &str, palette: &P) -> Vec<ShapeStyle> {
//         let series = self
//             .column(column)
//             .unwrap_or_else(|e| panic!("Column {column} not found: {e}"));

//         let mut map = HashMap::new();
//         let mut i = 0;

//         series
//             .iter()
//             .map(|val| {
//                 let idx = map.entry(val).or_insert_with(|| {
//                     let color = palette.idx2color(i);
//                     i += 1;
//                     color.filled()
//                 });
//                 *idx
//             })
//             .collect()
//     }
// }

use polars::prelude::*;

use super::DataMethod;

impl DataMethod for DataFrame {
    fn column_f64(&self, column: &str) -> Vec<f64> {
        self.column(column)
            .unwrap_or_else(|e| panic!("Column {column} not found: {e}"))
            // .f64()
            // .unwrap_or_else(|e| panic!("Failed to convert to f64: {e}"))
            // .to_vec_null_aware()
            // .left()
            // .unwrap_or_else(|| panic!("Null values found in column {column}"))
            .iter()
            .map(|any_val| any_val.try_extract::<f64>().expect("Failed to extract f64"))
            .collect()
    }

    fn column_range_f64(&self, column: &str) -> (f64, f64) {
        let series = self
            .column(column)
            .unwrap_or_else(|e| panic!("Column {column} not found: {e}"));

        let min = series
            .min()
            .unwrap_or_else(|e| panic!("Failed to get min: {e}"))
            .unwrap_or_else(|| panic!("Column is empty"));

        let max = series
            .max()
            .unwrap_or_else(|e| panic!("Failed to get max: {e}"))
            .unwrap_or_else(|| panic!("Column is empty"));

        (min, max)
    }

    fn column_len(&self, column: &str) -> usize {
        self.column(column)
            .unwrap_or_else(|e| panic!("Column {column} not found: {e}"))
            .len()
    }
}
