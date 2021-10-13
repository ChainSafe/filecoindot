// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Benchmarking setup for filecoindot

#![cfg(feature = "runtime-benchmarks")]

#[allow(unused)]
use crate::*;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    add_relayer {
        let relayer: T::AccountId = account("relayer", 0, 0);
    }: _(RawOrigin::Root, relayer)

    remove_relayer {
        let relayer: T::AccountId = account("relayer", 0, 0);
        <Call<T>>::add_relayer(RawOrigin::Root, relayer.clone());
    }: _(RawOrigin::Root, relayer)

    set_vote_threshold {
        let _ in 0..3 {
            let relayer: T::AccountId = account("relayer", 0, 0);
            <Call<T>>::add_relayer(RawOrigin::Root, relayer);
        }
    }: _(RawOrigin::Root, 2)

    submit_block_vote {
        let caller: T::AccountId = whitelisted_caller();
        <Call<T>>::add_relayer(RawOrigin::Root, caller);
        <Call<T>>::set_vote_threshold(RawOrigin::Root, 1);
    }: _(RawOrigin::Root, vec![1,2], vec![2,3])
}
