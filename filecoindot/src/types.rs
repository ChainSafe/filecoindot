// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use frame_support::pallet_prelude::*;
use frame_support::sp_std;
use frame_system::{Origin, RawOrigin};

use crate::{Config, Relayers};

/// The filecoin block submission proposal
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub(crate) struct BlockSubmissionProposal<T: Config> {
    proposer: T::AccountId,
    /// The status of the proposal
    status: ProposalStatus,
    /// The block number that the proposal started
    start_block: T::BlockNumber,
    /// The block number that the proposal ended
    end_block: T::BlockNumber,
}

impl<T: Config> BlockSubmissionProposal<T> {
    pub fn new(
        proposer: T::AccountId,
        start_block: T::BlockNumber,
        end_block: T::BlockNumber,
    ) -> Self {
        BlockSubmissionProposal {
            proposer,
            //voted: BTreeSet::new(),
            status: ProposalStatus::Active,
            start_block,
            end_block,
        }
    }

    /// Get the status of the proposal
    pub fn get_status(&self) -> &ProposalStatus {
        &self.status
    }

    pub fn set_status(&mut self, new_status: ProposalStatus) {
        self.status = new_status;
    }

    /// Whether the proposal is still active, i.e. can vote
    pub fn is_expired(&self, now: &T::BlockNumber) -> bool {
        now.gt(&self.end_block)
    }
}

/// The status of the proposal
/// Expected status transition:
///     Active -> Approved -> Executed
///               Rejected    Canceled
///                           Expired
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub(crate) enum ProposalStatus {
    /// The proposal is active and relayers can start voting
    Active,
    /// Proposal is approved
    Approved,
    /// Proposal is rejected
    Rejected,
}

/// An implementation of EnsureOrigin that ensures an account is the admin to the pallet.
pub struct EnsureRelayer<T: Config>(sp_std::marker::PhantomData<T>);

impl<O: Into<Result<Origin<T>, O>> + From<Origin<T>> + Clone, T: Config> EnsureOrigin<O>
    for EnsureRelayer<T>
{
    type Success = T::AccountId;

    fn try_origin(o: O) -> Result<Self::Success, O> {
        let origin = o.clone().into()?;
        match origin {
            RawOrigin::Signed(i) => {
                if Relayers::<T>::contains_key(&i) {
                    Ok(i)
                } else {
                    Err(o)
                }
            }
            _ => Err(o),
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn successful_origin() -> O {
        RawOrigin::Signed(Default::default()).into()
    }
}
