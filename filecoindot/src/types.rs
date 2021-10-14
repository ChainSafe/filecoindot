// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use frame_support::pallet_prelude::*;
use frame_support::sp_std;
use frame_support::sp_std::collections::btree_set::BTreeSet;
use frame_system::{Origin, RawOrigin};

use crate::{pallet, Admins, Config, Error, MessageRootCidCounter};

/// The filecoin block submission proposal
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub(crate) struct BlockSubmissionProposal<T: Config> {
    proposer: T::AccountId,
    /// The accounts with voted
    voted: BTreeSet<T::AccountId>,
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
            voted: BTreeSet::new(),
            status: ProposalStatus::Active,
            start_block,
            end_block,
        }
    }

    /// Get the status of the proposal
    pub fn get_status(&self) -> &ProposalStatus {
        &self.status
    }

    /// Vote for the proposal. Will reject the operation if its status is invalid
    /// The content of the vote is actually the message root of the block
    pub fn vote(
        &mut self,
        block_cid: pallet::BlockCid,
        message_root: pallet::MessageRootCid,
        who: T::AccountId,
        when: &T::BlockNumber,
        threshold: u32,
    ) -> Result<(), Error<T>> {
        ensure!(!self.is_voted(&who), Error::<T>::AlreadyVoted);
        ensure!(
            self.status == ProposalStatus::Active,
            Error::<T>::ProposalCompleted
        );

        // when expired, we set the status to be rejected
        if self.is_expired(when) {
            self.status = ProposalStatus::Rejected;
            return Err(Error::<T>::ProposalExpired);
        }

        // MessageRootCidCounter leaked into the struct, well not the best way for encapsulation
        // but works for now, come back later to fix this.
        let count = 1 + MessageRootCidCounter::<T>::get(&block_cid, &message_root).unwrap_or(0);
        if count >= threshold {
            self.status = ProposalStatus::Approved;
        }

        MessageRootCidCounter::<T>::insert(&block_cid, &message_root, count);
        self.voted.insert(who);

        Ok(())
    }

    /// Resolve the proposal status
    pub fn resolve(
        &mut self,
        block_cid: &[u8],
        when: &T::BlockNumber,
        threshold: u32,
    ) -> Result<(), Error<T>> {
        ensure!(
            self.status == ProposalStatus::Active,
            Error::<T>::ProposalCompleted
        );

        // when expired, we set the status to be rejected
        if self.is_expired(when) {
            self.status = ProposalStatus::Rejected;
        } else {
            // MessageRootCidCounter leaked into the struct, well not the best way for encapsulation
            // but works for now, come back later to fix this.
            for (_, count) in MessageRootCidCounter::<T>::iter_prefix(block_cid) {
                if count >= threshold {
                    self.status = ProposalStatus::Approved;
                    break;
                }
            }
        }

        Ok(())
    }

    /// Checks if the account has already voted
    fn is_voted(&self, who: &T::AccountId) -> bool {
        self.voted.contains(who)
    }

    /// Whether the proposal is still active, i.e. can vote
    fn is_expired(&self, now: &T::BlockNumber) -> bool {
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
pub struct EnsureRelayerAdmin<T: Config>(sp_std::marker::PhantomData<T>);

impl<O: Into<Result<Origin<T>, O>> + From<Origin<T>> + Clone, T: Config> EnsureOrigin<O>
    for EnsureRelayerAdmin<T>
{
    type Success = T::AccountId;

    fn try_origin(o: O) -> Result<Self::Success, O> {
        let origin = o.clone().into()?;
        match origin {
            RawOrigin::Signed(i) => {
                if Admins::<T>::contains_key(&i) {
                    Ok(i)
                } else {
                    Err(o)
                }
            }
            _ => Err(o),
        }
    }
}
