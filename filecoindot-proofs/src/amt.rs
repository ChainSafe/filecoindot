// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::errors::Error;
use crate::traits::{AMTNode, BlockStore};
use cid::Cid;
use forest_encoding::de::Deserializer;
use serde::Deserialize;

pub fn nodes_for_height(bit_width: usize, height: usize) -> usize {
    let height_log_two = bit_width * height;
    if height_log_two >= 64 {
        return std::usize::MAX;
    }
    1 << height_log_two
}

const MAX_HEIGHT: usize = 8;
const MAX_INDEX: usize = (u64::MAX - 1) as usize;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Amt<'db, BS: BlockStore, N: AMTNode> {
    node: N,
    block_store: Option<&'db BS>,
    bit_width: usize,
    height: usize,
    count: usize,
}

impl<'db, 'de, BS: BlockStore, N: AMTNode + Deserialize<'de>> Deserialize<'de> for Amt<'db, BS, N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (bit_width, height, count, node): (_, _, _, N) =
            Deserialize::deserialize(deserializer)?;
        Ok(Self {
            bit_width,
            height,
            count,
            node,
            block_store: None,
        })
    }
}

impl<'db, BS, N> Amt<'db, BS, N>
where
    BS: BlockStore,
    N: AMTNode + for<'de> Deserialize<'de>,
{
    /// Constructs an AMT with a blockstore and a Cid of the root of the AMT
    pub fn load(cid: &Cid, block_store: &'db BS) -> Result<Self, Error> {
        // Load root bytes from database
        let mut root = block_store.get::<Self>(cid)?;

        // Sanity check, this should never be possible.
        if root.height > MAX_HEIGHT {
            return Err(Error::MaxHeightExceeded);
        }

        root.block_store = Some(block_store);
        Ok(root)
    }

    /// Get value at index of AMT
    pub fn generate_proof(&self, i: usize) -> Result<Vec<Vec<u8>>, Error> {
        if i > MAX_INDEX {
            return Err(Error::NotFound);
        }

        if i >= nodes_for_height(self.bit_width, self.height + 1) {
            return Err(Error::NotFound);
        }

        let mut path = Vec::new();
        if self.node.path_to_key(
            *self.block_store.as_ref().unwrap(),
            self.bit_width,
            self.height,
            i,
            &mut path,
        )? {
            Ok(path)
        } else {
            Err(Error::NotFound)
        }
    }
}
