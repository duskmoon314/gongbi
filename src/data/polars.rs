use ::polars::prelude as pl;

use super::*;

impl DataMethod for pl::DataFrame {
    fn column<T: NumCast>(&self, column: &str) -> Vec<T> {
        self.column(column)
            .unwrap_or_else(|e| panic!("Column {column} not found: {e}"))
            .iter()
            .map(|any_val| any_val.try_extract().unwrap())
            .collect()
    }

    fn column_range<T: NumCast>(&self, column: &str) -> (T, T) {
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
}
