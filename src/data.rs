use ::polars::prelude as pl;
use num_traits::NumCast;

pub mod polars;

#[derive(Clone, Debug, derive_more::From)]
pub enum Data {
    Polars(pl::DataFrame),
}

impl Data {
    pub fn column<T: NumCast>(&self, column: &str) -> Vec<T> {
        match self {
            Data::Polars(df) => DataMethod::column(df, column),
        }
    }

    pub fn column_range<T: NumCast>(&self, column: &str) -> (T, T) {
        match self {
            Data::Polars(df) => df.column_range(column),
        }
    }
}

pub trait DataMethod {
    fn column<T: NumCast>(&self, column: &str) -> Vec<T>;

    fn column_range<T: NumCast>(&self, column: &str) -> (T, T);
}
