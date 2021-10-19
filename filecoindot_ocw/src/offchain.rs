// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Filecoindot offchain Externalities

use frame_support::sp_runtime::offchain::{
    Externalities, HttpRequestStatus, OpaqueMultiaddr, OpaqueNetworkState, Timestamp,
};
use reqwest::{
    blocking::{Body, Client, RequestBuilder, Response},
    header::HeaderName,
    Method, Url,
};
use sp_core::{
    offchain::{HttpError, HttpRequestId},
    OpaquePeerId,
};
use std::{collections::HashMap, str::FromStr};

/// Filecoindot offchain Externalities
#[derive(Default)]
pub struct OffchainExt {
    counter: u16,
    client: Client,
    requests: HashMap<u16, (RequestBuilder, Option<Response>)>,
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
        let req = self.client.request(
            Method::from_str(&method.to_uppercase()).unwrap(),
            Url::parse(uri).unwrap(),
        );
        let id = self.counter;
        self.requests.insert(id.into(), (req, None));
        self.counter += 1;
        Ok(HttpRequestId(self.counter))
    }

    fn http_request_add_header(
        &mut self,
        request_id: HttpRequestId,
        name: &str,
        value: &str,
    ) -> Result<(), ()> {
        let mut req = self.requests.get_mut(&request_id.0).unwrap();
        req.0 = req.0.try_clone().unwrap().header(name, value);
        Ok(())
    }

    fn http_request_write_body(
        &mut self,
        request_id: HttpRequestId,
        chunk: &[u8],
        deadline: Option<Timestamp>,
    ) -> Result<(), HttpError> {
        let mut req = self.requests.get_mut(&request_id.0).unwrap();
        req.0 = req.0.try_clone().unwrap().body(chunk.to_vec());
        Ok(())
    }

    fn http_response_wait(
        &mut self,
        ids: &[HttpRequestId],
        deadline: Option<Timestamp>,
    ) -> Vec<HttpRequestStatus> {
        let mut ret = vec![];
        for id in ids {
            let mut req = self.requests.get_mut(&id.0).unwrap();
            req.1 = Some(req.0.try_clone().unwrap().send().unwrap());
            ret.push(HttpRequestStatus::Finished(0));
        }

        ret
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
        let mut req = self.requests.remove(&request_id.0).unwrap();
        buffer.copy_from_slice(&mut req.1.unwrap().text().unwrap().as_bytes());
        Ok(0)
    }

    fn set_authorized_nodes(&mut self, nodes: Vec<OpaquePeerId>, authorized_only: bool) {}
}
