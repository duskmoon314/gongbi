//! # Geom module
//!
//! This module contains multiple geometric layers implementations and utility functions to implement them.

use std::rc::Rc;

use crate::{aes::Aes, data::Data};

pub mod line;
pub mod point;

/// Get the range of the x and y columns of the data.
///
/// If the mapping does not have a x or y column, this function will panic.
pub fn range_2d_xy(data: &Rc<dyn Data>, mapping: &Aes) -> (f64, f64, f64, f64) {
    let x = mapping
        .x
        .unwrap_or_else(|| panic!("Layer does not have x mapping"));

    let y = mapping
        .y
        .unwrap_or_else(|| panic!("Layer does not have y mapping"));

    let x_range = data.column_range_f64(x);
    let y_range = data.column_range_f64(y);

    (x_range.0, x_range.1, y_range.0, y_range.1)
}

/// Get the range of the x and y columns of the data.
///
/// If the mapping does not have a x column, this function will panic.
///
/// If the mapping does not have a y column, the y range will be the range of the x column, and the x range will be the range of the length of the x column.
pub fn range_2d_x_xy(data: &Rc<dyn Data>, mapping: &Aes) -> (f64, f64, f64, f64) {
    let x = mapping
        .x
        .unwrap_or_else(|| panic!("Layer does not have x mapping"));

    match mapping.y {
        Some(y) => {
            let x_range = data.column_range_f64(x);
            let y_range = data.column_range_f64(y);

            (x_range.0, x_range.1, y_range.0, y_range.1)
        }
        None => {
            let x_len = data.column_len(x);
            let x_range = data.column_range_f64(x);

            (0.0, x_len as f64, x_range.0, x_range.1)
        }
    }
}
