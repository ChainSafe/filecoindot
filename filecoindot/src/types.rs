use crate::{Config, Error};
use frame_support::pallet_prelude::*;

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
    /// Proposal is cancelled
    Canceled,
    /// Proposal is expired
    Expired,
    /// Proposal is executed
    Executed,
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug)]
pub(crate) struct ProposalDetail<T: Config> {
    /// The cid of the filecoin block.
    block_cid: Vec<u8>,
    /// The message root cid
    message_root_cid: Vec<u8>,
    // TODO: how many relayers will there be again? Save some bits?
    /// The accounts with `for` votes
    for_votes: Vec<T::AccountId>,
    /// The accounts with `against` votes
    against_votes: Vec<T::AccountId>,
    /// The block number that the proposal started
    start_block: T::BlockNumber,
    /// The block number that the proposal ended
    end_block: T::BlockNumber,
    /// Whether the proposal is executed
    is_executed: bool,
    /// Whether the proposal is canceled
    is_canceled: bool,
}

impl<T: Config> ProposalDetail<T> {
    pub fn new(
        block_cid: Vec<u8>,
        message_root_cid: Vec<u8>,
        start_block: T::BlockNumber,
        end_block: T::BlockNumber,
    ) -> Self {
        ProposalDetail {
            block_cid,
            message_root_cid,
            for_votes: Vec::new(),
            against_votes: Vec::new(),
            start_block,
            end_block,
            is_executed: false,
            is_canceled: false,
        }
    }

    /// Derive the current proposal status.
    /// Need to pass in the current block number as the status is very time sensitive
    pub fn status(&self, _now: T::BlockNumber) -> ProposalStatus {
        // TODO: the logic here needs to be clarified
        todo!()
    }

    pub fn try_resolve(
        &mut self,
        threshold: u32,
        total_relayers: u32,
        now: T::BlockNumber,
    ) -> Result<ProposalStatus, Error<T>> {
        let status = self.status(now);

        // TODO: we are not handling Canceled?
        // TODO: this is also very strange as the status can be Approved first then Rejected again?
        // TODO: should we consider once not active then not allowing any operations?
        ensure!(
            status != ProposalStatus::Executed,
            Error::<T>::ProposalAlreadyExecuted
        );
        ensure!(
            status != ProposalStatus::Expired,
            Error::<T>::ProposalExpired
        );

        if total_relayers >= threshold
            && self.against_votes.len() as u32 + threshold > total_relayers
        {
            return Ok(ProposalStatus::Rejected);
        }

        if self.for_votes.len() >= threshold as usize {
            return Ok(ProposalStatus::Approved);
        }

        Err(Error::MustBeRelayer)
    }
}
