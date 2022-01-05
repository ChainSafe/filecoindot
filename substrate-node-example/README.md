## filecoin-ocw-example

This is an example for configuring filecointdot to your substrate node


### 0. add `filecoindot` to your runtime

```toml
# /runtime/Cargo.toml

filecoindot = { git = "https://github.com/chainSafe/filecoindot",  default-features = false }
```


### 1. configure `filecoindot` into your runtime

```rust
//! runtime/src/lib.rs

/// ManagerOrigin which manages the approved relayer set, in this case root
type ManagerOrigin = frame_system::EnsureRoot<AccountId>;

impl filecoindot::Config for Runtime {
    /// Origin used to administer the pallet
    type ManagerOrigin = ManagerOrigin;
    type Event = Event;
    type WeightInfo = ();
    /// The identifier type for the offchain worker.
    type AuthorityId = filecoindot::FilecoindotId;
    /// Timeout for the http requests of the offchain worker
    type OffchainWorkerTimeout = OffchainWorkerTimeout;
}
```


### 2. implement `CreateSignedTransaction` for your runtime

```rust
//! runtime/src/lib.rs

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
	Call: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		public: <Signature as sp_runtime::traits::Verify>::Signer,
		account: AccountId,
		index: Index,
	) -> Option<(Call, <UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload)> {
		let period = BlockHashCount::get() as u64;
		let current_block = System::block_number()
			.saturated_into::<u64>()
			.saturating_sub(1);
		let tip = 0;
		let extra: SignedExtra = (
			frame_system::CheckSpecVersion::<Runtime>::new(),
			frame_system::CheckTxVersion::<Runtime>::new(),
			frame_system::CheckGenesis::<Runtime>::new(),
			frame_system::CheckEra::<Runtime>::from(generic::Era::mortal(period, current_block)),
			frame_system::CheckNonce::<Runtime>::from(index),
			frame_system::CheckWeight::<Runtime>::new(),
			pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
		);

		let raw_payload = SignedPayload::new(call, extra)
			.map_err(|e| {
				log::warn!("Unable to create signed payload: {:?}", e);
			})
			.ok()?;
		let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
		let address = account;
		let (call, extra, _) = raw_payload.deconstruct();
		Some((call, (sp_runtime::MultiAddress::Id(address), signature.into(), extra)))
	}
}

impl frame_system::offchain::SigningTypes for Runtime {
	type Public = <Signature as sp_runtime::traits::Verify>::Signer;
	type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
	Call: From<C>,
{
	type OverarchingCall = Call;
	type Extrinsic = UncheckedExtrinsic;
}
```

This trait is required by the offchain worker, see [CreateSignedTransaction][0] for more detail.


### 3. extend rpc handler with `filecoindot-rpc`

```toml
# node/Cargo.toml

filecoindot = { git = "https://github.com/chainSafe/filecoindot" }
```

```rust
//! node/src/rpc.rs

/// Full client dependencies.
pub struct FullDeps<C, P, S> {
    /// The offchain storage instance to use.
    pub storage: Option<Arc<RwLock<S>>>,
    
    // ...
}


/// Instantiate all full RPC extensions.
pub fn create_full<C, P, S>(deps: FullDeps<C, P, S>) -> jsonrpc_core::IoHandler<sc_rpc::Metadata> {
    
    // ...
    
    // filecoindot rpc
    if let Some(storage) = storage {
        io.extend_with(FilecoindotApi::to_delegate(Filecoindot::new(storage)));
    }
}
```

### 4. extend runtime interfaces with filecoindot's host functions

```toml
# node/Cargo.toml

filecoindot-io = { git = "https://github.com/chainSafe/filecoindot" }
```

```rust
//! node/src/service.rs
// Our native executor instance.
pub struct ExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
    /// Only enable the benchmarking host functions when we actually want to benchmark.
    #[cfg(feature = "runtime-benchmarks")]
    type ExtendHostFunctions = (
        frame_benchmarking::benchmarking::HostFunctions,
        filecoindot_io::forest_proof_verify::HostFunctions,
    );
    /// Otherwise we only use the default Substrate host functions.
    #[cfg(not(feature = "runtime-benchmarks"))]
    type ExtendHostFunctions = (
        filecoindot_io::forest_proof_verify::HostFunctions,
    );

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        node_template_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        node_template_runtime::native_version()
    }
}
```

### 5. integrate NFT pallet
We need to configure both ORML nft pallet and filecoindot-nft pallet.
```rust
// config orml_ft
parameter_types! {
    pub MaxClassMetadata: u32 = 1024;
    pub MaxTokenMetadata: u32 = 1024;
}

impl orml_nft::Config for Runtime {
    type ClassId = u32;
    type TokenId = u32;
    type ClassData = ClassData;
    type TokenData = TokenData;
    type MaxClassMetadata = MaxClassMetadata;
    type MaxTokenMetadata = MaxTokenMetadata;
}

// config filecoindot_nft
parameter_types! {
    pub DefaultClassId: u32 = 0;
}

impl filecoindot_nft::Config for Runtime {
    type Event = Event;
    type DefaultClassId = DefaultClassId;
    type WeightInfo = ();
}

// integrate with Runtime
construct_runtime!(
    pub enum Runtime where
        Block = Block,
    NodeBlock = opaque::Block,
    UncheckedExtrinsic = UncheckedExtrinsic
    {
        NFT: orml_nft::{Pallet, Config<T>, Storage},
        FilecoindotNFT: filecoindot_nft::{Pallet, Call, Config<T>, Event<T>},
    }
);

// config chain spec for genesis build
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        nft: NFTConfig { tokens: vec![] },
        filecoindot_nft: FilecoindotNFTConfig {
            default_class: (get_account_id_from_seed::<sr25519::Public>("Alice"), vec![]),
        },
    }
}
```

[0]: https://docs.substrate.io/rustdocs/latest/frame_system/offchain/trait.CreateSignedTransaction.html
