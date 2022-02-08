// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
#![allow(unused_imports)]
mod data;
mod ext;
mod get_tip_set_by_height;

use crate::ocw::api::{Api, ChainHead};
use ext::OffchainExt;
use frame_support::sp_runtime::offchain::Timestamp;
use get_tip_set_by_height::ChainGetTipSetByHeight;
use sp_core::offchain::{OffchainDbExt, OffchainWorkerExt};

// infura rpc for testing
const FILECOIN_RPC: &str = "https://api.node.glif.io";

// #[test]
// fn test_http_request() {
//     let offchain = OffchainExt::new();
//     let mut t = sp_io::TestExternalities::default();
//     t.register_extension(OffchainWorkerExt::new(offchain.clone()));
//     t.register_extension(OffchainDbExt::new(offchain));
//
//     t.execute_with(|| {
//         assert_eq!(
//             ChainGetTipSetByHeight
//                 .req(
//                     FILECOIN_RPC,
//                     vec![Some(1199840), None],
//                     Timestamp::from_unix_millis(1_000_000)
//                 )
//                 .unwrap(),
//             data::get_tip_set_by_height_1199840()
//         );
//     })
// }

#[test]
fn test_can_parse_chain_head() {
    let offchain = OffchainExt::new();
    let mut t = sp_io::TestExternalities::default();
    t.register_extension(OffchainWorkerExt::new(offchain.clone()));
    t.register_extension(OffchainDbExt::new(offchain));

    t.execute_with(|| {
        assert!(ChainHead
            .req(FILECOIN_RPC, vec![], Timestamp::from_unix_millis(1_000_000))
            .is_ok());
    })
}
