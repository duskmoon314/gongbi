use std::fmt::Debug;

pub mod polars;

#[derive(Clone, Debug, PartialEq, derive_more::From)]
pub enum Data {
    Polars(::polars::frame::DataFrame),
}

impl Data {
    pub fn column_f64(&self, column: &str) -> Vec<f64> {
        match self {
            Data::Polars(df) => DataMethod::column_f64(df, column),
        }
    }

    pub fn column_range_f64(&self, column: &str) -> (f64, f64) {
        match self {
            Data::Polars(df) => DataMethod::column_range_f64(df, column),
        }
    }

    pub fn column_len(&self, column: &str) -> usize {
        match self {
            Data::Polars(df) => DataMethod::column_len(df, column),
        }
    }
}

pub trait DataMethod {
    /// Get a column as a Vec of f64
    ///
    /// This is used to get the coordinates of the data points
    fn column_f64(&self, column: &str) -> Vec<f64>;

    /// Get the range of a column as a tuple of f64
    ///
    /// This is used to set the range of the axis
    fn column_range_f64(&self, column: &str) -> (f64, f64);

    /// Get the length of a column
    ///
    /// This is used to get the number of data points
    fn column_len(&self, column: &str) -> usize;
}
