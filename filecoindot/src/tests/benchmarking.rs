// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Benchmarking setup for filecoindot

use crate::tests::mock::{Origin, Test, ALICE, RELAYER1, RELAYER2, RELAYER3, RELAYER4};
use crate::tests::verify::{amt_proof_generation, hamt_proof_generation};
use crate::*;

#[allow(unused)]
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};

benchmarks! {
    add_relayer {
    }: {
        Pallet::<Test>::add_relayer(Origin::signed(ALICE), RELAYER4)?;
    } verify {
        assert!(Relayers::<Test>::contains_key(&RELAYER4));
    }

    remove_relayer {
        Pallet::<Test>::add_relayer(Origin::signed(ALICE), RELAYER4)?;
    }: {
        Pallet::<Test>::remove_relayer(Origin::signed(ALICE), RELAYER4)?;
    } verify {
        assert!(!Relayers::<Test>::contains_key(&RELAYER4));
    }

    set_vote_threshold {
    }: {
        Pallet::<Test>::set_vote_threshold(Origin::signed(ALICE), 2)?;
    } verify {
        assert_eq!(VoteThreshold::<Test>::get(), 2);
    }

    submit_block_vote_approve {
        Pallet::<Test>::add_relayer(Origin::signed(ALICE), ALICE)?;
        Pallet::<Test>::set_vote_threshold(Origin::signed(ALICE), 1)?;
    }: {
        Pallet::<Test>::submit_block_vote(Origin::signed(ALICE), vec![0], vec![0])?;
    } verify {
        assert!(!BlockSubmissionProposals::<Test>::contains_key(&vec![0]));
    }

    verify_receipt {
        let block_cid = vec![0, 1];
        let message_cid = vec![0, 1];
        Pallet::<Test>::submit_block_vote(
            Origin::signed(RELAYER1),
            block_cid.clone(),
            message_cid.clone()
        ).unwrap();
        Pallet::<Test>::submit_block_vote(
            Origin::signed(RELAYER2),
            block_cid.clone(),
            message_cid.clone()
        ).unwrap();
        Pallet::<Test>::submit_block_vote(
            Origin::signed(RELAYER3),
            block_cid.clone(),
            message_cid.clone()
        ).unwrap();
        let (proof, cid) = amt_proof_generation(100);
    }: {
        Pallet::<Test>::verify_receipt(Origin::signed(ALICE), proof, block_cid, cid.to_bytes())?;
    }

    verify_state {
        let block_cid = vec![0, 1];
        let message_cid = vec![0, 1];
        Pallet::<Test>::submit_block_vote(
            Origin::signed(RELAYER1),
            block_cid.clone(),
            message_cid.clone()
        ).unwrap();
        Pallet::<Test>::submit_block_vote(
            Origin::signed(RELAYER2),
            block_cid.clone(),
            message_cid.clone()
        ).unwrap();
        Pallet::<Test>::submit_block_vote(
            Origin::signed(RELAYER3),
            block_cid.clone(),
            message_cid.clone()
        ).unwrap();
        let (proof, cid) = hamt_proof_generation();
    }: {
        Pallet::<Test>::verify_state(Origin::signed(ALICE), proof, block_cid, cid.to_bytes())?;
    }
}

impl_benchmark_test_suite!(
    Pallet,
    crate::tests::mock::ExtBuilder::default().build(),
    crate::tests::mock::Test
);
