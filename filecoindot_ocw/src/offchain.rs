// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Filecoindot offchain Externalities

use frame_support::sp_runtime::offchain::{
    Externalities, HttpRequestStatus, OpaqueMultiaddr, OpaqueNetworkState, Timestamp,
};
use reqwest::Request;
use sp_core::{
    offchain::{HttpError, HttpRequestId},
    OpaquePeerId,
};
use std::collections::HashMap;

/// Filecoindot offchain Externalities
pub struct OffchainExt {
    counter: u16,
    requests: HashMap<usize, Request>,
}

impl Externalities for OffchainExt {
    fn is_validator(&self) -> bool {
        false
    }

    fn network_state(&self) -> Result<OpaqueNetworkState, ()> {
        Ok(OpaqueNetworkState {
            peer_id: OpaquePeerId::new(Default::default()),
            external_addresses: vec![],
        })
    }

    fn timestamp(&mut self) -> Timestamp {
        Timestamp::from_unix_millis(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        )
    }

    fn sleep_until(&mut self, deadline: Timestamp) {}

    fn random_seed(&mut self) -> [u8; 32] {
        Default::default()
    }

    fn http_request_start(
        &mut self,
        method: &str,
        uri: &str,
        meta: &[u8],
    ) -> Result<HttpRequestId, ()> {
        Ok(HttpRequestId(self.counter))
    }

    fn http_request_add_header(
        &mut self,
        request_id: HttpRequestId,
        name: &str,
        value: &str,
    ) -> Result<(), ()> {
        Ok(())
    }

    fn http_request_write_body(
        &mut self,
        request_id: HttpRequestId,
        chunk: &[u8],
        deadline: Option<Timestamp>,
    ) -> Result<(), HttpError> {
        Ok(())
    }

    fn http_response_wait(
        &mut self,
        ids: &[HttpRequestId],
        deadline: Option<Timestamp>,
    ) -> Vec<HttpRequestStatus> {
        vec![]
    }

    fn http_response_headers(&mut self, request_id: HttpRequestId) -> Vec<(Vec<u8>, Vec<u8>)> {
        vec![]
    }

    fn http_response_read_body(
        &mut self,
        request_id: HttpRequestId,
        buffer: &mut [u8],
        deadline: Option<Timestamp>,
    ) -> Result<usize, HttpError> {
        Ok(0)
    }

    fn set_authorized_nodes(&mut self, nodes: Vec<OpaquePeerId>, authorized_only: bool) {}
}
