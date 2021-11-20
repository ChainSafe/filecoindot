// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use frame_support::pallet_prelude::EnsureOrigin;
use frame_support::{assert_err, assert_ok};
use sp_runtime::DispatchError::BadOrigin;

use crate::types::{EnsureRelayer, ProposalStatus};
use crate::{
    tests::mock::*, BlockSubmissionProposals, Error, MessageRootCidCounter, VerifiedBlocks,
};

#[test]
fn submit_block_vote_works() {
    let block_cid = vec![0, 1];
    let message_cid = vec![0, 1];
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER1),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_eq!(
            MessageRootCidCounter::<Test>::get(&block_cid, &message_cid).unwrap(),
            1
        );

        let p = BlockSubmissionProposals::<Test>::get(&block_cid).unwrap();
        assert_eq!(*p.get_status(), ProposalStatus::Active);
    });
}

#[test]
fn submit_block_vote_fails_not_relayer() {
    let block_cid = vec![0, 1];
    let message_cid = vec![0, 1];
    ExtBuilder::default().build().execute_with(|| {
        assert_err!(
            FileCoinModule::submit_block_vote(
                Origin::signed(ALICE),
                block_cid.clone(),
                message_cid.clone()
            ),
            Error::<Test>::NotRelayer
        );
        assert!(MessageRootCidCounter::<Test>::get(&block_cid, &message_cid).is_none(),);
    });
}

#[test]
fn submit_block_vote_fails_already_voted() {
    let block_cid = vec![0, 1];
    let message_cid = vec![0, 1];
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER1),
            block_cid.clone(),
            message_cid.clone()
        ));
        assert_err!(
            FileCoinModule::submit_block_vote(
                Origin::signed(RELAYER1),
                block_cid.clone(),
                message_cid.clone()
            ),
            Error::<Test>::AlreadyVoted
        );
        assert_eq!(
            MessageRootCidCounter::<Test>::get(&block_cid, &message_cid).unwrap(),
            1
        );
    });
}

#[test]
fn submit_block_vote_resolve_rejected() {
    let block_cid = vec![0, 1];
    let message_cid = vec![0, 1];
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER1),
            block_cid.clone(),
            message_cid.clone()
        ));
        System::set_block_number(100);
        assert_err!(
            FileCoinModule::submit_block_vote(
                Origin::signed(RELAYER2),
                block_cid.clone(),
                message_cid.clone()
            ),
            Error::<Test>::ProposalExpired
        );
        assert!(!VerifiedBlocks::<Test>::contains_key(&block_cid));
        assert!(BlockSubmissionProposals::<Test>::get(&block_cid).is_none(),);
        assert!(MessageRootCidCounter::<Test>::get(&block_cid, &message_cid).is_none(),);
    });
}

#[test]
fn submit_block_vote_resolve_approved() {
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
        // assert_eq!(*p.get_status(), ProposalStatus::Approved);
        assert!(VerifiedBlocks::<Test>::contains_key(&block_cid));
        // assert_eq!(BlockSubmissionProposals::<Test>::get(&block_cid).is_none(), true);
        assert!(MessageRootCidCounter::<Test>::get(&block_cid, &message_cid).is_none(),);
    });
}

#[test]
fn submit_block_vote_resolve_completed() {
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
        System::set_block_number(100);
        assert_ok!(FileCoinModule::add_relayer(Origin::signed(ALICE), RELAYER4));
        assert_err!(
            FileCoinModule::submit_block_vote(
                Origin::signed(RELAYER4),
                block_cid.clone(),
                message_cid.clone()
            ),
            Error::<Test>::BlockAlreadyVerified
        );
        assert!(VerifiedBlocks::<Test>::contains_key(&block_cid));
        assert!(BlockSubmissionProposals::<Test>::get(&block_cid).is_none(),);
        assert!(MessageRootCidCounter::<Test>::get(&block_cid, &message_cid).is_none(),);
    });
}

#[test]
fn close_block_proposal_already_verified() {
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

        System::set_block_number(100);
        assert_err!(
            FileCoinModule::close_block_proposal(Origin::signed(ALICE), block_cid.clone()),
            Error::<Test>::BlockAlreadyVerified
        );
        assert!(VerifiedBlocks::<Test>::contains_key(&block_cid));
        assert!(BlockSubmissionProposals::<Test>::get(&block_cid).is_none(),);
        assert!(MessageRootCidCounter::<Test>::get(&block_cid, &message_cid).is_none(),);
    });
}

#[test]
fn close_block_proposal_not_allowed() {
    let block_cid = vec![0, 1];
    ExtBuilder::default().build().execute_with(|| {
        assert_err!(
            FileCoinModule::close_block_proposal(Origin::signed(RELAYER4), block_cid.clone()),
            BadOrigin
        );
    });
}

#[test]
fn close_block_proposal_works() {
    let block_cid = vec![0, 1];
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER1),
            block_cid.clone(),
            vec![0, 1]
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER2),
            block_cid.clone(),
            vec![0, 2]
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER3),
            block_cid.clone(),
            vec![0, 3]
        ));
        System::set_block_number(100);
        assert_ok!(FileCoinModule::close_block_proposal(
            Origin::signed(ALICE),
            block_cid.clone(),
        ));
        assert!(
            !VerifiedBlocks::<Test>::contains_key(&block_cid),
            "{}",
            false
        );
        assert!(BlockSubmissionProposals::<Test>::get(&block_cid).is_none(),);
        assert!(MessageRootCidCounter::<Test>::get(&block_cid, &vec![0, 1]).is_none(),);
        assert!(MessageRootCidCounter::<Test>::get(&block_cid, &vec![0, 2]).is_none(),);
        assert!(MessageRootCidCounter::<Test>::get(&block_cid, &vec![0, 3]).is_none(),);
    });
}

#[test]
fn close_block_proposal_no_effect() {
    let block_cid = vec![0, 1];
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER1),
            block_cid.clone(),
            vec![0, 1]
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER2),
            block_cid.clone(),
            vec![0, 2]
        ));
        assert_ok!(FileCoinModule::submit_block_vote(
            Origin::signed(RELAYER3),
            block_cid.clone(),
            vec![0, 3]
        ));
        assert_ok!(FileCoinModule::close_block_proposal(
            Origin::signed(ALICE),
            block_cid.clone(),
        ));
        assert!(VerifiedBlocks::<Test>::get(&block_cid).is_none());
        assert_eq!(
            BlockSubmissionProposals::<Test>::get(&block_cid)
                .unwrap()
                .get_status(),
            &ProposalStatus::Active
        );
        assert_eq!(
            MessageRootCidCounter::<Test>::get(&block_cid, &vec![0, 1]).unwrap(),
            1
        );
        assert_eq!(
            MessageRootCidCounter::<Test>::get(&block_cid, &vec![0, 2]).unwrap(),
            1
        );
        assert_eq!(
            MessageRootCidCounter::<Test>::get(&block_cid, &vec![0, 3]).unwrap(),
            1
        );
    });
}

#[test]
fn ensure_relayer_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert!(EnsureRelayer::<Test>::try_origin(Origin::signed(RELAYER1)).is_ok(),);
        assert!(EnsureRelayer::<Test>::try_origin(Origin::signed(ALICE)).is_err(),);
    });
}
