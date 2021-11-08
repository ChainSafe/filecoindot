// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Offchain db implementation
use super::OffchainExt;
use sp_core::offchain::{DbExternalities, StorageKind};

impl DbExternalities for OffchainExt {
    fn local_storage_set(&mut self, _kind: StorageKind, key: &[u8], value: &[u8]) {
        let mut state = self.0.write();
        let _ = state.db.insert(key.to_vec(), value.to_vec());
    }

    fn local_storage_clear(&mut self, _kind: StorageKind, key: &[u8]) {
        let mut state = self.0.write();
        let _ = state.db.remove(key);
    }

    fn local_storage_compare_and_set(
        &mut self,
        _kind: StorageKind,
        key: &[u8],
        _old_value: Option<&[u8]>,
        new_value: &[u8],
    ) -> bool {
        let mut state = self.0.write();
        state.db.insert(key.to_vec(), new_value.to_vec()).is_some()
    }

    fn local_storage_get(&mut self, _kind: StorageKind, key: &[u8]) -> Option<Vec<u8>> {
        let state = self.0.read();
        state.db.get(key).map(|v| v.to_vec())
    }
}
