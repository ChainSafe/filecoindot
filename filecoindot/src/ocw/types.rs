// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Filecoin api types
#![allow(missing_docs)]
use crate::ocw::{Error, Result};
use frame_support::sp_std::vec::Vec;
use serde::{Deserialize, Serialize};

/// Response of the [`ChainHead`](https://docs.filecoin.io/reference/lotus-api/#chainhead) RPC call
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct GetChainHead {
    pub result: TipSet,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct TipSet {
    #[serde(rename = "Cids")]
    pub cids: Vec<Cid>,
    #[serde(rename = "Blocks")]
    pub blocks: Vec<Block>,
    #[serde(rename = "Height")]
    pub height: i64,
}

impl TipSet {
    /// get (cid, message_root) pairs
    pub fn pairs(self) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
        if self.cids.len() != self.blocks.len() {
            return Err(Error::InvalidTipSet);
        }

        Ok(self
            .cids
            .into_iter()
            .zip(self.blocks.into_iter())
            .map(|(cid, block)| (cid.inner, block.messages.inner))
            .collect())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Block {
    #[serde(rename = "Messages")]
    pub messages: Cid,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Cid {
    #[serde(deserialize_with = "crate::ocw::de::bytes")]
    #[serde(rename = "/")]
    pub inner: Vec<u8>,
}
