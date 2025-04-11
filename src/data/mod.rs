mod base_data;
pub use base_data::*;

#[cfg(feature = "nalgebra")]
mod nalgebra_data;

#[cfg(feature = "rulinalg")]
mod rulinalg_data;

#[cfg(feature = "ndarray")]
mod ndarray_data;

#[cfg(feature = "csv")]
mod csv_data;

#[cfg(feature = "polars")]
mod polars_data;
