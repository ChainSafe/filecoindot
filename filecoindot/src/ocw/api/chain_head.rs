// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::ocw::{api::Api, types::TipSet};
use frame_support::sp_std::vec::Vec;

/// Method `Filecoin.ChainHeight`
pub const CHAIN_HEAD: &str = "Filecoin.ChainHead";

/// `Filecoin.ChainHeight`
pub struct ChainHead;

impl Api for ChainHead {
    const METHOD: &'static str = CHAIN_HEAD;
    type Params = Vec<()>;
    type Result = TipSet;
}
