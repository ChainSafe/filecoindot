use frame_support::pallet_prelude::*;
use frame_support::sp_std::collections::btree_map::BTreeMap;
use frame_support::sp_std::collections::btree_set::BTreeSet;

use crate::{Config, Error};

/// The filecoin block submission proposal
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub(crate) struct BlockSubmissionProposal<T: Config> {
    proposer: T::AccountId,
    /// The accounts with voted
    voted: BTreeSet<T::AccountId>,
    /// The status of the proposal
    status: ProposalStatus,
    /// Message root tracker
    message_root_counter: BTreeMap<Vec<u8>, u32>,
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
            message_root_counter: BTreeMap::new(),
            start_block,
            end_block,
        }
    }

    /// Get the status of the proposal
    pub fn get_status(&self) -> &ProposalStatus {
        &self.status
    }

    /// Vote for the proposal. Will reject the operation is status is invalid
    /// The content of the vote is actually the message root of the block
    pub fn vote_with_resolve(
        &mut self,
        who: T::AccountId,
        message_root: Vec<u8>,
        when: &T::BlockNumber,
        threshold: u32,
    ) -> DispatchResult {
        ensure!(!self.is_voted(&who), Error::<T>::AlreadyVoted);
        ensure!(
            self.status == ProposalStatus::Active,
            Error::<T>::ProposalCompleted
        );

        // when expired, we set the status to be rejected
        if self.is_expired(when) {
            self.status = ProposalStatus::Rejected;
            return Err(Error::<T>::ProposalExpired.into());
        }

        // add `clone` to make clippy happy
        let mut count = *self
            .message_root_counter
            .get(&message_root)
            .unwrap_or(&0);
        count = count.saturating_add(1);

        if count > threshold {
            self.status = ProposalStatus::Approved;
        }

        self.message_root_counter.insert(message_root, count);
        self.voted.insert(who);

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
