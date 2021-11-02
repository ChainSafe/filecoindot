#![feature(in_band_lifetimes)]

mod errors;
// #[cfg(feature="forest")]
mod forest_adaptor;

mod generate;
mod hamt;
mod traits;
mod verify;

pub use forest_adaptor::*;