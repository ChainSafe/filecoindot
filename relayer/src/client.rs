// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! RPC client for requesting data from filecoin RPC

use reqwest::Client as ReqwestClinet;

/// RPC Client of filecoindot relayers
pub struct Client(ReqwestClinet);
