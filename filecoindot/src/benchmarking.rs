// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Benchmarking setup for filecoindot

use crate::*;
use filecoindot_io::benchmarking::{amt_proof_generation, hamt_proof_generation};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, vec};
use frame_support::traits::EnsureOrigin;
use frame_system::RawOrigin;

benchmarks! {
    add_relayer {
        let caller = T::ManagerOrigin::successful_origin();
        let relayer: T::AccountId = account("relayer", 0, 0);
    }: {
        Pallet::<T>::add_relayer(caller, relayer.clone())?;
    } verify {
        assert!(Relayers::<T>::contains_key(&relayer));
    }

    remove_relayer {
        let caller = T::ManagerOrigin::successful_origin();
        let relayer: T::AccountId = account("relayer", 0, 0);

        Pallet::<T>::add_relayer(caller.clone(), relayer.clone())?;
    }: {
        Pallet::<T>::remove_relayer(caller, relayer.clone())?;
    } verify {
        assert!(!Relayers::<T>::contains_key(&relayer));
    }

    set_vote_threshold {
    }: {
        Pallet::<T>::set_vote_threshold(T::ManagerOrigin::successful_origin(), 2)?;
    } verify {
        assert_eq!(VoteThreshold::<T>::get(), 2);
    }

    submit_block_vote {
        let caller = T::ManagerOrigin::successful_origin();
        let relayer: T::AccountId = account("relayer", 0, 0);

        Pallet::<T>::add_relayer(caller.clone(), relayer.clone())?;
        Pallet::<T>::set_vote_threshold(caller, 1)?;
    }: {
        Pallet::<T>::submit_block_vote(RawOrigin::Signed(relayer).into(), vec![0], vec![0])?;
    } verify {
        assert!(!BlockSubmissionProposals::<T>::contains_key(&vec![0]));
    }

    close_block_proposal {
        let caller = T::ManagerOrigin::successful_origin();
        let relayer: T::AccountId = account("relayer", 0, 0);

        Pallet::<T>::add_relayer(caller.clone(), relayer.clone())?;
        Pallet::<T>::submit_block_vote(RawOrigin::Signed(relayer).into(), vec![0], vec![0])?;

        let now = frame_system::Pallet::<T>::block_number();
        frame_system::Pallet::<T>::set_block_number(now + VotingPeriod::<T>::get() + 1u32.into());
    }: {
        Pallet::<T>::close_block_proposal(caller, vec![0])?;
    } verify {
        assert_eq!(
            BlockSubmissionProposals::<T>::get(vec![0]),
            None,
        );
    }

    verify_receipt {
        let caller = T::ManagerOrigin::successful_origin();
        let alice: T::AccountId = account("alice", 0, 0);
        let bob: T::AccountId = account("bob", 1, 1);
        let charlie: T::AccountId = account("charlie", 2, 2);

        Pallet::<T>::add_relayer(caller.clone(), alice.clone())?;
        Pallet::<T>::add_relayer(caller.clone(), bob.clone())?;
        Pallet::<T>::add_relayer(caller, charlie.clone())?;

        let block_cid = vec![0, 1];
        let message_cid = vec![0, 1];

        Pallet::<T>::submit_block_vote(
            RawOrigin::Signed(alice.clone()).into(),
            block_cid.clone(),
            message_cid.clone()
        ).unwrap();
        Pallet::<T>::submit_block_vote(
            RawOrigin::Signed(bob).into(),
            block_cid.clone(),
            message_cid.clone()
        ).unwrap();
        Pallet::<T>::submit_block_vote(
            RawOrigin::Signed(charlie).into(),
            block_cid.clone(),
            message_cid
        ).unwrap();
        let (proof, cid) = amt_proof_generation(100);
    }: {
        Pallet::<T>::verify_receipt(RawOrigin::Signed(alice).into(), proof, block_cid, cid)?;
    }

    verify_state {
        let caller = T::ManagerOrigin::successful_origin();
        let alice: T::AccountId = account("alice", 0, 0);
        let bob: T::AccountId = account("bob", 1, 1);
        let charlie: T::AccountId = account("charlie", 2, 2);

        Pallet::<T>::add_relayer(caller.clone(), alice.clone())?;
        Pallet::<T>::add_relayer(caller.clone(), bob.clone())?;
        Pallet::<T>::add_relayer(caller, charlie.clone())?;

        let block_cid = vec![0, 1];
        let message_cid = vec![0, 1];

        Pallet::<T>::submit_block_vote(
            RawOrigin::Signed(alice.clone()).into(),
            block_cid.clone(),
            message_cid.clone()
        ).unwrap();
        Pallet::<T>::submit_block_vote(
            RawOrigin::Signed(bob).into(),
            block_cid.clone(),
            message_cid.clone()
        ).unwrap();
        Pallet::<T>::submit_block_vote(
            RawOrigin::Signed(charlie).into(),
            block_cid.clone(),
            message_cid
        ).unwrap();

        let (proof, cid) = hamt_proof_generation();
    }: {
        Pallet::<T>::verify_state(RawOrigin::Signed(alice).into(), proof, block_cid, cid)?;
    }

    verify_message {
        let caller = T::ManagerOrigin::successful_origin();
        let alice: T::AccountId = account("alice", 0, 0);
        let bob: T::AccountId = account("bob", 1, 1);
        let charlie: T::AccountId = account("charlie", 2, 2);

        Pallet::<T>::add_relayer(caller.clone(), alice.clone())?;
        Pallet::<T>::add_relayer(caller.clone(), bob.clone())?;
        Pallet::<T>::add_relayer(caller, charlie.clone())?;

        let block_cid = vec![0, 1];
        let message_cid = vec![0, 1];

        Pallet::<T>::submit_block_vote(
            RawOrigin::Signed(alice.clone()).into(),
            block_cid.clone(),
            message_cid.clone()
        ).unwrap();
        Pallet::<T>::submit_block_vote(
            RawOrigin::Signed(bob).into(),
            block_cid.clone(),
            message_cid.clone()
        ).unwrap();
        Pallet::<T>::submit_block_vote(
            RawOrigin::Signed(charlie).into(),
            block_cid.clone(),
            message_cid
        ).unwrap();

        let (proof, cid) = hamt_proof_generation();
    }: {
        Pallet::<T>::verify_message(RawOrigin::Signed(alice).into(), proof, block_cid, cid)?;
    }
}

impl_benchmark_test_suite!(
    Pallet,
    crate::tests::mock::ExtBuilder::default().build(),
    crate::tests::mock::Test
);
