// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! crypto identifier for filecoindot
use sp_core::crypto::KeyTypeId;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"fdot");

use frame_support::sp_runtime::{
    app_crypto::{app_crypto, sr25519},
    traits::Verify,
    MultiSignature, MultiSigner,
};
use sp_core::sr25519::Signature as Sr25519Signature;

app_crypto!(sr25519, KEY_TYPE);

/// filecoindot crypto type
pub struct FilecoindotId;

// implemented for runtime
impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for FilecoindotId {
    type RuntimeAppPublic = Public;
    type GenericSignature = sp_core::sr25519::Signature;
    type GenericPublic = sp_core::sr25519::Public;
}

impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
    for FilecoindotId
{
    type RuntimeAppPublic = Public;
    type GenericSignature = sp_core::sr25519::Signature;
    type GenericPublic = sp_core::sr25519::Public;
}
