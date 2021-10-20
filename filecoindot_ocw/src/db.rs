// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Offchain db implementation
use crate::OffchainExt;
use sp_core::offchain::{DbExternalities, StorageKind};

impl DbExternalities for OffchainExt {
    fn local_storage_set(&mut self, _kind: StorageKind, key: &[u8], value: &[u8]) {
        let state = self.0.write();
        let _ = state.db.put(key, value);
    }

    fn local_storage_clear(&mut self, _kind: StorageKind, key: &[u8]) {
        let state = self.0.read();
        let _ = state.db.delete(key);
    }

    fn local_storage_compare_and_set(
        &mut self,
        _kind: StorageKind,
        key: &[u8],
        _old_value: Option<&[u8]>,
        new_value: &[u8],
    ) -> bool {
        let state = self.0.write();
        state.db.put(key, new_value).is_ok()
    }

    fn local_storage_get(&mut self, _kind: StorageKind, key: &[u8]) -> Option<Vec<u8>> {
        let state = self.0.read();
        state.db.get(key).unwrap_or_default()
    }
}
