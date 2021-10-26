#![feature(in_band_lifetimes)]

mod errors;
mod forest_adaptor;
mod generate;
mod hamt;
mod hash;
mod node;
mod traits;

use serde::{Deserialize, Serialize};

const MAX_ARRAY_WIDTH: usize = 3;

/// Default bit width for indexing a hash at each depth level
const DEFAULT_BIT_WIDTH: u32 = 8;
