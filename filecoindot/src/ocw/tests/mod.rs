// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
mod data;
mod ext;
mod get_tip_set_by_height;

use crate::ocw::api::Api;
use ext::{Env, OffchainExt};
use frame_support::sp_runtime::offchain::Timestamp;
use get_tip_set_by_height::ChainGetTipSetByHeight;
use sp_core::offchain::{OffchainDbExt, OffchainWorkerExt};

#[test]
fn test_http_request() {
    let offchain = OffchainExt::new();
    let mut t = sp_io::TestExternalities::default();
    t.register_extension(OffchainWorkerExt::new(offchain.clone()));
    t.register_extension(OffchainDbExt::new(offchain));

    t.execute_with(|| {
        assert_eq!(
            ChainGetTipSetByHeight
                .req(
                    &Env::rpc().unwrap(),
                    vec![Some(1199840), None],
                    Timestamp::from_unix_millis(1_000_000)
                )
                .unwrap(),
            data::get_tip_set_by_height_1199840()
        );
    })
}
