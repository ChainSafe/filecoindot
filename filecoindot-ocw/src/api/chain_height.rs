// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{api::Api, types::TipSet};
use frame_support::sp_std::vec::Vec;

/// Method `Filecoin.ChainHeight`
pub const CHAIN_HEIGHT: &str = "Filecoin.ChainHeight";

/// `Filecoin.ChainHeight`
pub struct ChainHeight;

impl Api for ChainHeight {
    const METHOD: &'static str = CHAIN_HEIGHT;
    type Params = Vec<()>;
    type Result = TipSet;
}
