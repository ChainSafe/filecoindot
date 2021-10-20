// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Filecoindot offchain Externalities

use crate::state::OffchainState;
use frame_support::sp_runtime::offchain::{
    Externalities, HttpRequestStatus, OpaqueNetworkState, Timestamp,
};
use futures::future::join_all;
use parking_lot::RwLock;
use reqwest::{
    header::HeaderName,
    Method, Url, {Body, Request},
};
use sp_core::{
    offchain::{HttpError, HttpRequestId},
    OpaquePeerId,
};
use std::{collections::BTreeMap, str::FromStr, sync::Arc};

/// Filecoindot offchain Externalities
#[derive(Clone)]
pub struct OffchainExt(pub Arc<RwLock<OffchainState>>);

impl OffchainExt {
    pub fn new() -> crate::Result<Self> {
        Ok(Self(Arc::new(RwLock::new(OffchainState::new()?))))
    }
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

    fn sleep_until(&mut self, _deadline: Timestamp) {}

    fn random_seed(&mut self) -> [u8; 32] {
        Default::default()
    }

    fn http_request_start(
        &mut self,
        method: &str,
        uri: &str,
        _meta: &[u8],
    ) -> Result<HttpRequestId, ()> {
        let req = Request::new(
            Method::from_str(&method.to_uppercase()).map_err(|_| ())?,
            Url::parse(uri).map_err(|_| ())?,
        );
        let mut state = self.0.write();
        let id = state.counter;
        state.requests.insert(id.into(), (req, None));
        state.counter += 1;
        Ok(HttpRequestId(id))
    }

    fn http_request_add_header(
        &mut self,
        request_id: HttpRequestId,
        name: &str,
        value: &str,
    ) -> Result<(), ()> {
        let mut state = self.0.write();
        let req = state.requests.get_mut(&request_id.0).ok_or(())?;
        req.0.headers_mut().insert(
            HeaderName::from_bytes(name.to_uppercase().as_bytes()).map_err(|_| ())?,
            value.parse().map_err(|_| ())?,
        );
        Ok(())
    }

    fn http_request_write_body(
        &mut self,
        request_id: HttpRequestId,
        chunk: &[u8],
        _deadline: Option<Timestamp>,
    ) -> Result<(), HttpError> {
        let mut state = self.0.write();
        let req = state
            .requests
            .get_mut(&request_id.0)
            .ok_or(HttpError::Invalid)?;
        *req.0.body_mut() = Some(Body::from(chunk.to_vec()));
        Ok(())
    }

    fn http_response_wait(
        &mut self,
        ids: &[HttpRequestId],
        _deadline: Option<Timestamp>,
    ) -> Vec<HttpRequestStatus> {
        let mut state = self.0.write();

        let mut res = BTreeMap::new();
        let mut queue_ids = vec![];
        let mut queue = vec![];

        ids.iter().for_each(|HttpRequestId(id)| {
            res.insert(id, HttpRequestStatus::Invalid);
            if let Some(req) = state.requests.get(&id) {
                if let Some(cloned_req) = req.0.try_clone() {
                    queue_ids.push(id);
                    queue.push(state.client.execute(cloned_req));
                }
            }
        });

        // construct runtime
        if let Ok(rt) = tokio::runtime::Runtime::new() {
            let mut resps = rt.block_on(join_all(queue));
            for (idx, id) in queue_ids.iter().enumerate() {
                let resp = resps.remove(idx);
                if let Some(req) = state.requests.get_mut(id) {
                    if let Ok(r) = resp {
                        res.insert(&*id, HttpRequestStatus::Finished(r.status().as_u16()));
                        req.1 = Some(r);
                    }
                }
            }
        }

        res.values().cloned().collect::<Vec<_>>()
    }

    fn http_response_headers(&mut self, request_id: HttpRequestId) -> Vec<(Vec<u8>, Vec<u8>)> {
        let state = self.0.read();
        if let Some(req) = state.requests.get(&request_id.0) {
            req.0
                .headers()
                .iter()
                .map(|(key, value)| (key.as_str().as_bytes().to_vec(), value.as_bytes().to_vec()))
                .collect()
        } else {
            Default::default()
        }
    }

    fn http_response_read_body(
        &mut self,
        request_id: HttpRequestId,
        buffer: &mut [u8],
        _deadline: Option<Timestamp>,
    ) -> Result<usize, HttpError> {
        if let Ok(rt) = tokio::runtime::Runtime::new() {
            let mut state = self.0.write();
            println!("got response");

            let req = state
                .requests
                .remove(&request_id.0)
                .ok_or(HttpError::Invalid)?;

            println!("got response");
            let body = rt
                .block_on(req.1.ok_or(HttpError::Invalid)?.text())
                .map_err(|_| HttpError::Invalid)?;

            println!("body: {:?}", &body);
            println!("body length: {:?}", body.as_bytes().len());
            // *buffer.copy_within(body, body.len());

            Ok(0)
        } else {
            Err(HttpError::Invalid)
        }
    }

    fn set_authorized_nodes(&mut self, _nodes: Vec<OpaquePeerId>, _authorized_only: bool) {}
}
