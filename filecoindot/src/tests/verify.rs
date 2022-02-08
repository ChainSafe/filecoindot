// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use cid::Cid;
use frame_support::{assert_err, assert_ok};

use crate::{tests::mock::*, Error};

pub fn hamt_proof_generation() -> (Vec<Vec<u8>>, Cid) {
    let (p, cid) = filecoindot_io::benchmarking::hamt_proof_generation();
    (p, Cid::read_bytes(&*cid).unwrap())
}

pub fn amt_proof_generation(n: usize) -> (Vec<Vec<u8>>, Cid) {
    let (p, cid) = filecoindot_io::benchmarking::amt_proof_generation(n as u64);
    (p, Cid::read_bytes(&*cid).unwrap())
}

#[test]
fn verify_state_works() {
    let (proof, cid) = hamt_proof_generation();

    let block_cid = vec![0, 1];
    let message_cid = vec![0, 1];
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER1),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER2),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER3),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_ok!(FileCoinModule::verify_state_inner(
            proof,
            block_cid,
            cid.to_bytes()
        ));
    });
}

#[test]
fn verify_state_fails_invalid_block_cid() {
    let (proof, cid) = hamt_proof_generation();

    let block_cid = vec![0, 1];
    let message_cid = vec![0, 1];
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER1),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER2),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER3),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_err!(
            FileCoinModule::verify_state_inner(proof, vec![0, 2], cid.to_bytes()),
            Error::<Test>::VerificationError
        );
    });
}

#[test]
fn verify_receipt_works() {
    let (proof, cid) = amt_proof_generation(100);

    let block_cid = vec![0, 1];
    let message_cid = vec![0, 1];
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER1),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER2),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER3),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_ok!(FileCoinModule::verify_receipt_inner(
            proof,
            block_cid,
            cid.to_bytes()
        ));
    });
}

#[test]
fn verify_receipt_fails_invalid_block_cid() {
    let (proof, cid) = amt_proof_generation(100);

    let block_cid = vec![0, 1];
    let message_cid = vec![0, 1];
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER1),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER2),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER3),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_err!(
            FileCoinModule::verify_receipt_inner(proof, vec![0, 2], cid.to_bytes()),
            Error::<Test>::VerificationError
        );
    });
}

#[test]
fn verify_message_works() {
    let (proof, cid) = hamt_proof_generation();

    let block_cid = vec![0, 1];
    let message_cid = vec![0, 1];
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER1),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER2),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER3),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_ok!(FileCoinModule::verify_message_inner(
            proof,
            block_cid,
            cid.to_bytes()
        ));
    });
}

#[test]
fn verify_message_fails() {
    let (proof, cid) = hamt_proof_generation();

    let block_cid = vec![0, 1];
    let message_cid = vec![0, 1];
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER1),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER2),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER3),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_err!(
            FileCoinModule::verify_message_inner(proof, vec![0, 2], cid.to_bytes()),
            Error::<Test>::VerificationError
        );
    });
}
