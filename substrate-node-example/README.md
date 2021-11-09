## filecoin-ocw-example

This is an example for configuring filecointdot to your substrate node

### 0. add `filecoindot` to your runtime's Cargo.toml

```
//! /runtime/Cargo.toml

filecoindot = { git = "https://github.com/chainSafe/filecoindot",  default-features = false }
```


### 1. configure `filecoindot` to your runtime

```
// ManagerOrigin which manages the approved relayer set, in this case root
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


### 2. implement `CreateSignedTransaction` to your runtime

```rust
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


[0]: https://docs.substrate.io/rustdocs/latest/frame_system/offchain/trait.CreateSignedTransaction.html
