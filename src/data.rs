//! Data module
//!
//! This module defines the `Data` trait and its implementations.

use std::fmt::Debug;

pub mod polars;

/// # Data trait
///
/// This trait abstracts the data source for the [`Plot`](crate::Plot) struct.
pub trait Data: Debug {
    /// Get a column as a vector of f64 values.
    fn column_f64(&self, column_name: &str) -> Vec<f64>;

    /// Get the minimum and maximum values of a column.
    fn column_range_f64(&self, column_name: &str) -> (f64, f64);

    /// Get the length of a column.
    fn column_len(&self, column_name: &str) -> usize;
}
