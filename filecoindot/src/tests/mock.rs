use crate as pallet;
use frame_support::construct_runtime;
use frame_support::pallet_prelude::{EnsureOrigin, GenesisBuild};
use frame_support::sp_runtime::AccountId32;
use frame_support::{parameter_types, sp_std};
use frame_system::ensure_signed;
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type AccountId = AccountId32;
pub const ALICE: AccountId = AccountId32::new([1u8; 32]);
pub const RELAYER1: AccountId = AccountId32::new([2u8; 32]);
pub const RELAYER2: AccountId = AccountId32::new([3u8; 32]);
pub const RELAYER3: AccountId = AccountId32::new([4u8; 32]);
pub const RELAYER4: AccountId = AccountId32::new([5u8; 32]);

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
}

impl frame_system::Config for Test {
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = ::sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
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
    type BaseCallFilter = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

impl pallet::Config for Test {
    type ManagerOrigin = MockedRelayerAdmin<Self>;
    type Event = Event;
    type WeightInfo = ();
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
