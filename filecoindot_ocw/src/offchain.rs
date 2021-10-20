// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Filecoindot offchain Externalities

use crate::state::OffchainState;
use frame_support::sp_runtime::offchain::{
    Externalities, HttpRequestStatus, OpaqueMultiaddr, OpaqueNetworkState, Timestamp,
};
use parking_lot::RwLock;
use reqwest::{
    header::HeaderName,
    Method, Url, {Body, Client, Request, Response},
};
use sp_core::{
    offchain::{HttpError, HttpRequestId},
    OpaquePeerId,
};
use std::{collections::HashMap, str::FromStr, sync::Arc};

/// Filecoindot offchain Externalities
#[derive(Default)]
pub struct OffchainExt(pub Arc<RwLock<OffchainState>>);

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
        let req = Request::new(
            Method::from_str(&method.to_uppercase()).map_err(|_| ())?,
            Url::parse(uri).map_err(|_| ())?,
        );
        let mut state = self.0.write();
        let id = state.counter;
        state.requests.insert(id.into(), (req, None));
        state.counter += 1;
        Ok(HttpRequestId(state.counter))
    }

    fn http_request_add_header(
        &mut self,
        request_id: HttpRequestId,
        name: &str,
        value: &str,
    ) -> Result<(), ()> {
        let mut state = self.0.write();
        let mut req = state.requests.get_mut(&request_id.0).ok_or(())?;
        req.0.headers_mut().insert(
            HeaderName::from_bytes(name.to_uppercase().as_bytes()).map_err(|_| ())?,
            value.parse().unwrap(),
        );
        Ok(())
    }

    fn http_request_write_body(
        &mut self,
        request_id: HttpRequestId,
        chunk: &[u8],
        deadline: Option<Timestamp>,
    ) -> Result<(), HttpError> {
        // let mut req = self.requests.get_mut(&request_id.0).unwrap();
        // req.0 = req.0.try_clone().unwrap().body(chunk.to_vec());
        Ok(())
    }

    fn http_response_wait(
        &mut self,
        ids: &[HttpRequestId],
        deadline: Option<Timestamp>,
    ) -> Vec<HttpRequestStatus> {
        // let mut ret = vec![];
        // for id in ids {
        //     let mut req = self.requests.get_mut(&id.0).unwrap();
        //     req.1 = Some(req.0.try_clone().unwrap().send().unwrap());
        //     ret.push(HttpRequestStatus::Finished(0));
        // }
        //
        // ret
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
        // let mut req = self.requests.remove(&request_id.0).unwrap();
        // buffer.copy_from_slice(&mut req.1.unwrap().text().unwrap().as_bytes());
        Ok(0)
    }

    fn set_authorized_nodes(&mut self, nodes: Vec<OpaquePeerId>, authorized_only: bool) {}
}
