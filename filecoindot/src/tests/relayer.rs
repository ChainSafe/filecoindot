// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use crate::{tests::mock::*, Error, RelayerCount, VoteThreshold};
use frame_support::{assert_err, assert_ok};
use sp_runtime::DispatchError::BadOrigin;

#[test]
fn set_threshold_works() {
    let v = ExtBuilder::default();
    let relayers = v.relayers.len() as u32;
    assert!(relayers > 1, "invalid test config for relayers");
    v.build().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(FileCoinModule::set_vote_threshold(
            Origin::signed(ALICE),
            relayers - 1
        ));
        assert_eq!(VoteThreshold::<Test>::get(), relayers - 1);
    });
}

#[test]
fn set_threshold_fails_not_admin() {
    let v = ExtBuilder::default();
    let relayers = v.relayers.len() as u32;
    assert!(relayers > 1, "invalid test config for relayers");
    v.build().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_err!(
            FileCoinModule::set_vote_threshold(Origin::signed(RELAYER1), relayers),
            BadOrigin
        );
        assert_eq!(VoteThreshold::<Test>::get(), relayers);
    });
}

#[test]
fn set_threshold_fails_invalid_threshold_0() {
    let v = ExtBuilder::default();
    let relayers = v.relayers.len() as u32;
    assert!(relayers > 1, "invalid test config for relayers");
    v.build().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_err!(
            FileCoinModule::set_vote_threshold(Origin::signed(ALICE), 0),
            Error::<Test>::InvalidThreshold
        );
        assert_eq!(VoteThreshold::<Test>::get(), relayers);
    });
}

#[test]
fn set_threshold_fails_invalid_threshold_too_large() {
    let v = ExtBuilder::default();
    let relayers = v.relayers.len() as u32;
    assert!(relayers > 1, "invalid test config for relayers");
    v.build().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_err!(
            FileCoinModule::set_vote_threshold(Origin::signed(ALICE), relayers + 1),
            Error::<Test>::InvalidThreshold
        );
        assert_eq!(VoteThreshold::<Test>::get(), relayers);
    });
}

#[test]
fn add_relayer_works() {
    let v = ExtBuilder::default();
    let relayers = v.relayers.len() as u32 + 1;
    v.build().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(FileCoinModule::add_relayer(Origin::signed(ALICE), RELAYER4));
        assert_eq!(RelayerCount::<Test>::get(), relayers);
    });
}

#[test]
fn add_relayer_fails_already_relayer() {
    let v = ExtBuilder::default();
    v.build().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_err!(
            FileCoinModule::add_relayer(Origin::signed(ALICE), RELAYER1),
            Error::<Test>::RelayerAlreadyExists
        );
    });
}

#[test]
fn add_relayer_fails_not_admin() {
    ExtBuilder::default().build().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_err!(
            FileCoinModule::add_relayer(Origin::signed(RELAYER1), RELAYER1),
            BadOrigin
        );
    });
}

#[test]
fn remove_relayer_works() {
    let v = ExtBuilder::default();
    let relayers = v.relayers.len() as u32 + 1;
    v.build().execute_with(|| {
        assert_ok!(FileCoinModule::add_relayer(Origin::signed(ALICE), RELAYER4));
        assert_eq!(RelayerCount::<Test>::get(), relayers);
        assert_ok!(FileCoinModule::remove_relayer(
            Origin::signed(ALICE),
            RELAYER1
        ));
        assert_eq!(RelayerCount::<Test>::get(), relayers - 1);
    });
}

#[test]
fn remove_relayer_fails_not_enough() {
    let v = ExtBuilder::default();
    let relayers = v.relayers.len() as u32;
    v.build().execute_with(|| {
        assert_eq!(RelayerCount::<Test>::get(), relayers);
        assert_err!(
            FileCoinModule::remove_relayer(Origin::signed(ALICE), RELAYER2),
            Error::<Test>::NotEnoughRelayer
        );
        assert_eq!(RelayerCount::<Test>::get(), relayers);
    });
}

#[test]
fn remove_relayer_fails_not_relayer() {
    let v = ExtBuilder::default();
    let relayers = v.relayers.len() as u32;
    v.build().execute_with(|| {
        assert_eq!(RelayerCount::<Test>::get(), relayers);
        assert_err!(
            FileCoinModule::remove_relayer(Origin::signed(ALICE), ALICE),
            Error::<Test>::NotRelayer
        );
        assert_eq!(RelayerCount::<Test>::get(), relayers);
    });
}
