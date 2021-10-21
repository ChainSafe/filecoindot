// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Filecoindot offchain Externalities

use crate::state::{OffchainState, Request};
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
        let mut state = self.0.write();
        let req = state.client.request(
            Method::from_str(&method.to_uppercase()).map_err(|_| ())?,
            uri,
        );

        let id = state.counter;
        state.requests.insert(id.into(), req.into());
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

        // ids.iter().for_each(|HttpRequestId(id)| {
        //     res.insert(id, HttpRequestStatus::Invalid);
        //     if let Some(req) = state.requests.get(&id) {
        //         if let Some(cloned_req) = req.req.try_clone() {
        //             // queue_ids.push(id);
        //             // queue.push(cloned_req.send());
        //             let resp = cloned_req.send().map_err(|_| HttpError::IoError)?;
        //             res.insert(*id, HttpRequestStatus::Finished(resp.status().as_u16()));
        //             req.resp = Some(resp);
        //         }
        //     }
        // });

        ids.iter().for_each(|HttpRequestId(id)| {
            res.insert(id, HttpRequestStatus::Invalid);
            if let Some(req) = state.requests.get(&id) {
                if let Some(cloned_req) = req.req.try_clone() {
                    queue_ids.push(id);
                    queue.push(cloned_req.send());
                }
            }
        });

        // wait all futures
        if let Ok(rt) = tokio::runtime::Runtime::new() {
            let mut resps = rt.block_on(join_all(queue));
            for (idx, id) in queue_ids.iter().enumerate() {
                if let Ok(resp) = resps.remove(idx) {
                    let status = resp.status().as_u16();
                    res.insert(*id, HttpRequestStatus::Finished(status));
                    if let Some(req) = state.requests.get_mut(id) {
                        req.resp = Some(resp);
                    }
                }
            }
        }

        res.values().cloned().collect::<Vec<_>>()
    }

    fn http_response_headers(&mut self, request_id: HttpRequestId) -> Vec<(Vec<u8>, Vec<u8>)> {
        let state = self.0.read();
        if let Some(Request {
            resp: Some(resp), ..
        }) = state.requests.get(&request_id.0)
        {
            resp.headers()
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
        let mut state = self.0.write();
        let req = state
            .requests
            .get_mut(&request_id.0)
            .ok_or(HttpError::IoError)?;

        if let Some(resp) = req.resp.take() {
            if let Ok(rt) = tokio::runtime::Runtime::new() {
                req.resp_body = Some(
                    rt.block_on(resp.bytes())
                        .map_err(|_| HttpError::IoError)?
                        .to_vec(),
                );
            }
            //req.resp_body = Some(resp.bytes().map_err(|_| HttpError::IoError)).to_vec();
        }

        let response = req.resp_body.as_ref().ok_or(HttpError::IoError)?;
        if req.read >= response.len() {
            state.requests.remove(&request_id.0);
            Ok(0)
        } else {
            let read = std::cmp::min(buffer.len(), response[req.read..].len());
            buffer[0..read].copy_from_slice(&response[req.read..read]);
            req.read += read;
            Ok(read)
        }
    }

    fn set_authorized_nodes(&mut self, _nodes: Vec<OpaquePeerId>, _authorized_only: bool) {}
}
