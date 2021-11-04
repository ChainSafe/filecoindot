// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use frame_support::sp_std::vec::Vec;
use serde::de::{Deserialize, Deserializer, Error as _};
use serde_json::Value;

/// deserialize json `Value` to `Vec<u8>`
pub fn bytes<'de, D>(data: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(data)?;
    if let Value::String(s) = value {
        Ok(s.as_bytes().to_vec())
    } else {
        Err(D::Error::custom("field not string"))
    }
}
