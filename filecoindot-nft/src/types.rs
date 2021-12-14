// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use frame_support::pallet_prelude::*;
use frame_support::sp_std;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_std::prelude::*;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ClassData {
    /// The data stored in the class
    pub data: Vec<u8>,
}

impl Default for ClassData {
    fn default() -> Self {
        ClassData {
            data: Default::default(),
        }
    }
}

impl ClassData {
    pub fn new(data: Vec<u8>) -> Self {
        ClassData { data }
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TokenData {
    /// The cid of the data in filecoin
    pub cid: Vec<u8>,
}

impl TokenData {
    pub fn new(cid: Vec<u8>) -> Self {
        TokenData { cid }
    }
}
