// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
mod data;
mod ext;
mod mock;
mod pallet;
use crate::api::{Api, ChainGetTipSetByHeight};
use ext::{Env, OffchainExt};
use pallet::pallet::*;
use sp_core::offchain::{OffchainDbExt, OffchainWorkerExt};

#[test]
fn test_http_request() {
    let offchain = OffchainExt::new().unwrap();
    let mut t = sp_io::TestExternalities::default();
    t.register_extension(OffchainWorkerExt::new(offchain.clone()));
    t.register_extension(OffchainDbExt::new(offchain));

    t.execute_with(|| {
        assert_eq!(
            ChainGetTipSetByHeight
                .req(&Env::rpc().unwrap(), vec![Some(1199840), None])
                .unwrap(),
            data::get_tip_set_by_height_1199840()
        );
    })
}
