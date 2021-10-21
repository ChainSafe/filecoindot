// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
mod mock;
mod pallet;

use pallet::pallet::*;

use crate::{
    api::{Api, ChainGetTipSetByHeight},
    OffchainExt,
};
use sp_core::offchain::{OffchainDbExt, OffchainWorkerExt};

#[test]
fn test_http_request() {
    let offchain = OffchainExt::new().unwrap();
    let mut t = sp_io::TestExternalities::default();
    t.register_extension(OffchainWorkerExt::new(offchain.clone()));
    t.register_extension(OffchainDbExt::new(offchain));

    t.execute_with(|| {
        // crate::Offchain::chain_get_tip_set_by_height(1199840).unwrap();
        ChainGetTipSetByHeight
            .req(vec![Some(1199840), None])
            .unwrap();
        // .unwrap();
    })
}
