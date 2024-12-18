use polars::prelude::*;

impl super::Data for DataFrame {
    fn column_f64(&self, column_name: &str) -> Vec<f64> {
        let series = self
            .column(column_name)
            .unwrap_or_else(|_| panic!("Column {column_name} not found in data"))
            .as_series()
            .expect("Only support Series for now")
            .rechunk();

        let values = series
            .iter()
            .map(|any_val| any_val.try_extract::<f64>().expect("Failed to extract f64"));

        values.collect()
    }

    fn column_range_f64(&self, column_name: &str) -> (f64, f64) {
        let series = self
            .column(column_name)
            .unwrap_or_else(|_| panic!("Column {column_name} not found in data"))
            .as_series()
            .expect("Only support Series for now");

        let min = series.min().unwrap().unwrap();
        let max = series.max().unwrap().unwrap();

        (min, max)
    }

    fn column_len(&self, column_name: &str) -> usize {
        self.column(column_name).unwrap().len()
    }
}
