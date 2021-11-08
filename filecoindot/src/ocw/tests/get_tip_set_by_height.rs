// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::ocw::{api::Api, types::TipSet};
use frame_support::sp_std::vec::Vec;

/// Method `Filecoin.ChainGetTipSetByHeight`
pub const CHAIN_GET_TIP_SET_BY_HEIGHT: &str = "Filecoin.ChainGetTipSetByHeight";

/// `Filecoin.ChainGetTipSetByHeight`
pub struct ChainGetTipSetByHeight;

impl Api for ChainGetTipSetByHeight {
    const METHOD: &'static str = CHAIN_GET_TIP_SET_BY_HEIGHT;
    type Params = Vec<Option<u64>>;
    type Result = TipSet;
}
