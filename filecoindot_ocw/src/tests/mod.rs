// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
mod mock;
mod pallet;

use pallet::pallet::*;

use sp_core::offchain::{testing, OffchainDbExt, OffchainWorkerExt};

#[test]
fn test_http_request() {
    let (offchain, _state) = testing::TestOffchainExt::new();
    let mut t = sp_io::TestExternalities::default();
    t.register_extension(OffchainWorkerExt::new(offchain.clone()));
    t.register_extension(OffchainDbExt::new(offchain));

    t.execute_with(|| {
        crate::Offchain::chain_get_tip_set_by_height(1199840).unwrap();
    })
}
