// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{
    api::Api,
    types::{Block, Cid},
};
use frame_support::sp_std::vec::Vec;
use serde::{Deserialize, Serialize};

/// Method `Filecoin.ChainGetTipSetByHeight`
pub const CHAIN_GET_TIP_SET_BY_HEIGHT: &str = "Filecoin.ChainGetTipSetByHeight";

/// Response of `Filecoin.ChainGetTipSetByHeight`
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChainGetTipSetByHeightResult {
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

/// `Filecoin.ChainGetTipSetByHeight`
pub struct ChainGetTipSetByHeight;

impl Api for ChainGetTipSetByHeight {
    const METHOD: &'static str = CHAIN_GET_TIP_SET_BY_HEIGHT;
    type Params = Vec<Option<u64>>;
    type Result = ChainGetTipSetByHeightResult;
}
