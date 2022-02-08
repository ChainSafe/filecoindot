// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use crate as pallet;
use frame_support::construct_runtime;
use frame_support::pallet_prelude::EnsureOrigin;
#[cfg(test)]
use frame_support::pallet_prelude::GenesisBuild;
use frame_support::{parameter_types, sp_runtime, sp_std};
use frame_system::ensure_signed;
use sp_core::{
    sr25519::{Public, Signature},
    H256,
};
use sp_runtime::{testing::Header, traits::IdentityLookup};
use sp_runtime::{
    testing::TestXt,
    traits::{Extrinsic as ExtrinsicT, Verify},
};
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type AccountId = Public;
pub const ALICE: AccountId = Public([1u8; 32]);
pub const RELAYER1: AccountId = Public([2u8; 32]);
pub const RELAYER2: AccountId = Public([3u8; 32]);
pub const RELAYER3: AccountId = Public([4u8; 32]);
pub const RELAYER4: AccountId = Public([5u8; 32]);

// Configure a mock runtime to test the pallet.
construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        FileCoinModule: pallet::{Pallet, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
    pub const OffchainWorkerTimeout: u64 = 1_000_000;
}

/// An implementation of EnsureOrigin
//  This is for the extrinsics only can be called after the
/// approval of the committee
pub struct MockedRelayerAdmin<T>(sp_std::marker::PhantomData<T>);

impl EnsureOrigin<<Test as frame_system::Config>::Origin> for MockedRelayerAdmin<Test> {
    type Success = AccountId;
    fn try_origin(
        o: <Test as frame_system::Config>::Origin,
    ) -> Result<Self::Success, <Test as frame_system::Config>::Origin> {
        let account = ensure_signed(o.clone()).unwrap();
        if account == ALICE {
            Ok(account)
        } else {
            Err(o)
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn successful_origin() -> <Test as frame_system::Config>::Origin {
        Origin::signed(ALICE)
    }
}

impl frame_system::Config for Test {
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = sp_runtime::traits::BlakeTwo256;
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

pub type Extrinsic = TestXt<Call, ()>;

impl frame_system::offchain::SigningTypes for Test {
    type Public = <Signature as Verify>::Signer;
    type Signature = Signature;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
    Call: From<LocalCall>,
{
    type OverarchingCall = Call;
    type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test
where
    Call: From<LocalCall>,
{
    fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
        call: Call,
        _public: <Signature as Verify>::Signer,
        _account: AccountId,
        nonce: u64,
    ) -> Option<(Call, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
        Some((call, (nonce, ())))
    }
}

impl pallet::Config for Test {
    type ManagerOrigin = MockedRelayerAdmin<Self>;
    type Event = Event;
    type WeightInfo = ();
    type AuthorityId = pallet::FilecoindotId;
    type OffchainWorkerTimeout = OffchainWorkerTimeout;
}

pub struct ExtBuilder {
    pub vote_threshold: u32,
    pub relayers: Vec<AccountId>,
}

const RELAYERS: [AccountId; 3] = [RELAYER1, RELAYER2, RELAYER3];

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            vote_threshold: 3,
            relayers: Vec::from(RELAYERS.clone()),
        }
    }
}

#[cfg(test)]
impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        pallet::GenesisConfig::<Test> {
            vote_threshold: self.vote_threshold,
            vote_period: 1,
            relayers: Vec::from(RELAYERS.clone()),
        }
        .assimilate_storage(&mut t)
        .unwrap();
        t.into()
    }
}
