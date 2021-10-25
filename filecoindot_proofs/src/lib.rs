mod blockstore;
mod errors;
mod generate;
mod hamt;
mod hash;
mod hash_algorithm;
mod hash_bits;
mod node;
mod bitmap;

use serde::{Deserialize, Serialize};

const MAX_ARRAY_WIDTH: usize = 3;

/// Default bit width for indexing a hash at each depth level
const DEFAULT_BIT_WIDTH: u32 = 8;

type HashedKey = [u8; 32];

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
