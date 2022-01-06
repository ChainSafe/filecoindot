// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use cid::Cid;
use filecoindot_proofs::{
    deserialize_to_node, Amt, ForestAdaptedBlockStorage, ForestAdaptedHashAlgo,
    ForestAdaptedHashedBits, ForestAdaptedNode, ForestAmtAdaptedNode, GetCid, HAMTNodeType, Hamt,
};
use frame_support::{assert_err, assert_ok};

use crate::{tests::mock::*, Error};
use ipld_amt::Amt as ForestAmt;
use ipld_blockstore::MemoryDB;
use ipld_hamt::Hamt as ForestHamt;
use serde_cbor::from_slice;

pub fn hamt_proof_generation() -> (Vec<Vec<u8>>, Cid) {
    let bs = MemoryDB::default();
    let mut fhamt: ForestHamt<_, _, usize> = ForestHamt::new(&bs);

    let max = 1000;
    for i in 1..max {
        fhamt.set(i, i.to_string()).unwrap();
    }

    let cid = fhamt.flush().unwrap();
    let store = ForestAdaptedBlockStorage::new(bs);
    let hamt: Hamt<
        ForestAdaptedBlockStorage<MemoryDB>,
        usize,
        String,
        ForestAdaptedHashedBits,
        ForestAdaptedNode<usize, String, ForestAdaptedHashAlgo, _>,
        ForestAdaptedHashAlgo,
    > = Hamt::new(&cid, &store, 8).unwrap();
    let mut p = hamt.generate_proof(&(max / 2)).unwrap();
    p.reverse();
    let raw_node = p.get(0).unwrap();
    let node: HAMTNodeType = deserialize_to_node(None, raw_node).unwrap();
    (p, node.cid().unwrap())
}

pub fn amt_proof_generation(n: usize) -> (Vec<Vec<u8>>, Cid) {
    let bs = MemoryDB::default();
    let mut famt = ForestAmt::new(&bs);

    let max = 1000;
    for i in 1..max {
        famt.set(i, i.to_string()).unwrap();
    }

    let cid = famt.flush().unwrap();
    let store = ForestAdaptedBlockStorage::new(bs);
    let amt: Amt<ForestAdaptedBlockStorage<MemoryDB>, ForestAmtAdaptedNode<String>> =
        Amt::load(&cid, &store).unwrap();
    let p = amt.generate_proof(n).unwrap();
    let raw_node = p.get(0).unwrap();
    let node: ForestAmtAdaptedNode<String> = from_slice(raw_node).unwrap();
    (p, node.cid().unwrap())
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
