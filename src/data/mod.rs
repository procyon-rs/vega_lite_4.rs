mod base_data;
pub use base_data::*;

#[cfg(feature = "ndarray")]
mod ndarray_data;
// #[cfg(feature = "ndarray")]
// use ndarray_data::*;

#[cfg(feature = "csv")]
mod csv_data;
// #[cfg(feature = "csv")]
// pub use csv_data::*;
