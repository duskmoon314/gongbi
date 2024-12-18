use std::fmt::Debug;

use dyn_clone::DynClone;

pub mod polars;

pub trait Data: Debug + DynClone {
    fn column_f64(&self, column_name: &str) -> Vec<f64>;

    fn column_range_f64(&self, column_name: &str) -> (f64, f64);

    fn column_len(&self, column_name: &str) -> usize;
}

dyn_clone::clone_trait_object!(Data);
