// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use crate as pallet;
use crate::{ClassData, TokenData};
use frame_support::construct_runtime;
use frame_support::pallet_prelude::GenesisBuild;
use frame_support::parameter_types;
use sp_core::{sr25519::Public, H256};
use sp_runtime::{testing::Header, traits::IdentityLookup};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type AccountId = Public;
pub const ALICE: AccountId = Public([1u8; 32]);

// Configure a mock runtime to test the pallet.
construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        FilecoinNFT: pallet::{Pallet, Call, Storage, Event<T>},
        NFT: orml_nft::{Pallet, Storage},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
    pub const OffchainWorkerTimeout: u64 = 1_000_000;
}

impl frame_system::Config for Test {
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = ::sp_runtime::traits::BlakeTwo256;
    type AccountId = sp_core::sr25519::Public;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type BlockWeights = ();
    type BlockLength = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BaseCallFilter = frame_support::traits::Everything;
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

parameter_types! {
    pub MaxClassMetadata: u32 = 1024;
    pub MaxTokenMetadata: u32 = 1024;
}

impl orml_nft::Config for Test {
    type ClassId = u32;
    type TokenId = u32;
    type ClassData = ClassData;
    type TokenData = TokenData;
    type MaxClassMetadata = MaxClassMetadata;
    type MaxTokenMetadata = MaxTokenMetadata;
}

parameter_types! {
    pub DefaultClassId: u32 = 0;
}

impl pallet::Config for Test {
    type Event = Event;
    type DefaultClassId = DefaultClassId;
    type WeightInfo = ();
}

pub struct ExtBuilder {
    pub default_class: (AccountId, Vec<u8>),
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            default_class: (ALICE, vec![0]),
        }
    }
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        pallet::GenesisConfig::<Test> {
            default_class: (ALICE, vec![0]),
        }
        .assimilate_storage(&mut t)
        .unwrap();
        t.into()
    }
}
