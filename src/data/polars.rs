use polars::prelude::*;

use super::DataMethod;

impl DataMethod for DataFrame {
    fn column_f64(&self, column: &str) -> Vec<f64> {
        self.column(column)
            .unwrap_or_else(|e| panic!("Column {column} not found: {e}"))
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
