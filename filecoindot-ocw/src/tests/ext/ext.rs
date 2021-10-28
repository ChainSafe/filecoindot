// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Filecoindot offchain Externalities

use super::state::{OffchainState, Response};
use frame_support::sp_runtime::offchain::{
    Externalities, HttpRequestStatus, OpaqueNetworkState, Timestamp,
};
use futures::future::join_all;
use parking_lot::RwLock;
use reqwest::Method;
use sp_core::{
    offchain::{HttpError, HttpRequestId},
    OpaquePeerId,
};
use std::{collections::BTreeMap, str::FromStr, sync::Arc};

/// Filecoindot offchain Externalities
#[derive(Clone)]
pub struct OffchainExt(pub Arc<RwLock<OffchainState>>);

impl OffchainExt {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(OffchainState::default())))
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
                .unwrap_or_default()
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
        let mut state = self.0.write();
        let req = state.client.request(
            Method::from_str(&method.to_uppercase()).map_err(|_| ())?,
            uri,
        );

        let id = state.counter;
        state.requests.insert(id, req.into());
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
        req.req = req.req.try_clone().ok_or(())?.header(name, value);
        Ok(())
    }

    fn http_request_write_body(
        &mut self,
        request_id: HttpRequestId,
        chunk: &[u8],
        _deadline: Option<Timestamp>,
    ) -> Result<(), HttpError> {
        if chunk.is_empty() {
            return Ok(());
        }

        let mut state = self.0.write();
        let req = state
            .requests
            .get_mut(&request_id.0)
            .ok_or(HttpError::Invalid)?;

        req.req = req
            .req
            .try_clone()
            .ok_or(HttpError::IoError)?
            .body(chunk.to_vec());
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
            if let Some(req) = state.requests.get(id) {
                if let Some(cloned_req) = req.req.try_clone() {
                    queue_ids.push(id);
                    queue.push(async {
                        if let Ok(resp) = cloned_req.send().await {
                            Ok((
                                resp.status().as_u16(),
                                resp.headers()
                                    .iter()
                                    .map(|(key, value)| {
                                        (
                                            key.as_str().as_bytes().to_vec(),
                                            value.as_bytes().to_vec(),
                                        )
                                    })
                                    .collect(),
                                resp.bytes().await,
                            ))
                        } else {
                            Err(())
                        }
                    });
                }
            }
        });

        // wait all futures
        if let Ok(rt) = tokio::runtime::Runtime::new() {
            let mut resps = rt.block_on(join_all(queue));
            for (idx, id) in queue_ids.iter().enumerate() {
                if let Ok((status, headers, maybe_resp_body)) = resps.remove(idx) {
                    res.insert(*id, HttpRequestStatus::Finished(status));
                    if let Some(req) = state.requests.get_mut(id) {
                        req.resp = Response {
                            status,
                            headers,
                            body: maybe_resp_body.unwrap_or_default().to_vec(),
                        };
                    }
                }
            }
        }

        res.into_values().collect::<Vec<_>>()
    }

    fn http_response_headers(&mut self, request_id: HttpRequestId) -> Vec<(Vec<u8>, Vec<u8>)> {
        let state = self.0.read();
        if let Some(req) = state.requests.get(&request_id.0) {
            req.resp.headers.clone()
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
        let mut state = self.0.write();
        let req = state
            .requests
            .get_mut(&request_id.0)
            .ok_or(HttpError::IoError)?;

        let response = &req.resp.body;
        if req.read >= response.len() {
            state.requests.remove(&request_id.0);
            Ok(0)
        } else {
            let read = std::cmp::min(buffer.len(), response[req.read..].len());
            buffer[0..read].copy_from_slice(&response[req.read..req.read + read]);
            req.read += read;
            Ok(read)
        }
    }

    fn set_authorized_nodes(&mut self, _nodes: Vec<OpaquePeerId>, _authorized_only: bool) {}
}
