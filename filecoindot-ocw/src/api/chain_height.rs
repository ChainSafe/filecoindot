// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{
    api::Api,
    types::{Block, Cid},
};
use frame_support::sp_std::vec::Vec;
use serde::{Deserialize, Serialize};

/// Method `Filecoin.ChainHeight`
pub const CHAIN_HEIGHT: &str = "Filecoin.ChainHeight";

/// Response of `Filecoin.ChainHeight`
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChainHeightResult {
    /// TipSet Cids
    #[serde(rename = "Cids")]
    pub cids: Vec<Cid>,
    /// TipSet Blocks
    #[serde(rename = "Blocks")]
    pub blocks: Vec<Block>,
    /// TipSet Height
    #[serde(rename = "Height")]
    pub height: u64,
}

/// `Filecoin.ChainHeight`
pub struct ChainGetTipSetByHeight;

impl Api for ChainGetTipSetByHeight {
    const METHOD: &'static str = CHAIN_HEIGHT;
    type Params = Vec<()>;
    type Result = ChainHeightResult;
}
