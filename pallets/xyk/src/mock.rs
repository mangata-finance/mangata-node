// Copyright (C) 2020 Mangata team

use super::*;

use sp_core::H256;

use sp_runtime::{
	testing::Header,
	traits::{AccountIdConversion, BlakeTwo256, IdentityLookup},
};

use crate as xyk;
use frame_support::{
	construct_runtime, parameter_types,
	traits::{ConstU128, ConstU32, Contains, Everything},
	PalletId,
};
use frame_system as system;
use mangata_primitives::{Amount, Balance, TokenId};
use orml_tokens::{MultiTokenCurrency, MultiTokenCurrencyAdapter, MultiTokenCurrencyExtended};
use orml_traits::parameter_type_with_key;
use pallet_assets_info as assets_info;
use pallet_issuance::PoolPromoteApi;
use std::{collections::HashMap, sync::Mutex};

pub const NATIVE_CURRENCY_ID: u32 = 0;

pub(crate) type AccountId = u128;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		Tokens: orml_tokens::{Pallet, Storage, Call, Event<T>, Config<T>},
		AssetsInfoModule: assets_info::{Pallet, Call, Config, Storage, Event<T>},
		XykStorage: xyk::{Pallet, Call, Storage, Event<T>, Config<T>},
	}
);

lazy_static::lazy_static! {
	static ref PROMOTED_POOLS: Mutex<HashMap<TokenId, Balance>> = {
		let m = HashMap::new();
		Mutex::new(m)
	};
}

pub struct MockPromotedPoolApi;

impl MockPromotedPoolApi {
	pub fn instance() -> &'static Mutex<HashMap<TokenId, Balance>> {
		&PROMOTED_POOLS
	}
}

impl PoolPromoteApi for MockPromotedPoolApi {
	fn promote_pool(liquidity_token_id: TokenId) -> bool {
		let mut pools = PROMOTED_POOLS.lock().unwrap();
		if pools.contains_key(&liquidity_token_id) {
			false
		} else {
			pools.insert(liquidity_token_id, 0);
			true
		}
	}

	fn get_pool_rewards(liquidity_token_id: TokenId) -> Option<Balance> {
		let mut pools = PROMOTED_POOLS.lock().unwrap();

		pools.get(&liquidity_token_id).map(|x| *x)
	}

	fn claim_pool_rewards(liquidity_token_id: TokenId, claimed_amount: Balance) -> bool {
		let mut pools = PROMOTED_POOLS.lock().unwrap();

		if let Some(reward) = pools.get_mut(&liquidity_token_id) {
			*reward = *reward - claimed_amount;
			true
		} else {
			false
		}
	}

	fn len() -> usize {
		PROMOTED_POOLS.lock().unwrap().len()
	}
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}
impl system::Config for Test {
	type BaseCallFilter = Everything;
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type PalletInfo = PalletInfo;
	type BlockWeights = ();
	type BlockLength = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

parameter_type_with_key! {
	pub ExistentialDeposits: |currency_id: TokenId| -> Balance {
		match currency_id {
			_ => 0,
		}
	};
}

pub struct DustRemovalWhitelist;
impl Contains<AccountId> for DustRemovalWhitelist {
	fn contains(a: &AccountId) -> bool {
		*a == TreasuryAccount::get()
	}
}

parameter_types! {
	pub TreasuryAccount: AccountId = TreasuryPalletId::get().into_account();
	pub const MaxLocks: u32 = 50;
}

impl orml_tokens::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = TokenId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = MaxLocks;
	type DustRemovalWhitelist = DustRemovalWhitelist;
}

parameter_types! {
	pub const MinLengthName: usize = 1;
	pub const MaxLengthName: usize = 255;
	pub const MinLengthSymbol: usize = 1;
	pub const MaxLengthSymbol: usize = 255;
	pub const MinLengthDescription: usize = 1;
	pub const MaxLengthDescription: usize = 255;
	pub const MaxDecimals: u32 = 255;
}

impl assets_info::Config for Test {
	type Event = Event;
	type MinLengthName = MinLengthName;
	type MaxLengthName = MaxLengthName;
	type MinLengthSymbol = MinLengthSymbol;
	type MaxLengthSymbol = MaxLengthSymbol;
	type MinLengthDescription = MinLengthDescription;
	type MaxLengthDescription = MaxLengthDescription;
	type MaxDecimals = MaxDecimals;
	type Currency = orml_tokens::MultiTokenCurrencyAdapter<Test>;
}

parameter_types! {
	pub const NativeCurrencyId: u32 = NATIVE_CURRENCY_ID;
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
	pub const BnbTreasurySubAccDerive: [u8; 4] = *b"bnbt";
}

parameter_types! {
	pub const LiquidityMiningIssuanceVaultId: PalletId = PalletId(*b"py/lqmiv");
	pub FakeLiquidityMiningIssuanceVault: AccountId = LiquidityMiningIssuanceVaultId::get().into_account();
}

impl Config for Test {
	type Event = Event;
	type Currency = MultiTokenCurrencyAdapter<Test>;
	type NativeCurrencyId = NativeCurrencyId;
	type TreasuryPalletId = TreasuryPalletId;
	type BnbTreasurySubAccDerive = BnbTreasurySubAccDerive;
	type LiquidityMiningIssuanceVault = FakeLiquidityMiningIssuanceVault;
	type PoolPromoteApi = MockPromotedPoolApi;
	type PoolFeePercentage = ConstU128<20>;
	type TreasuryFeePercentage = ConstU128<5>;
	type BuyAndBurnFeePercentage = ConstU128<5>;
	type RewardsDistributionPeriod = ConstU32<10000>;
	type WeightInfo = ();
}

impl<T: Config> Pallet<T> {
	pub fn balance(id: TokenId, who: T::AccountId) -> Balance {
		<T as Config>::Currency::free_balance(id.into(), &who).into()
	}
	pub fn total_supply(id: TokenId) -> Balance {
		<T as Config>::Currency::total_issuance(id.into()).into()
	}
	pub fn transfer(
		currency_id: TokenId,
		source: T::AccountId,
		dest: T::AccountId,
		value: Balance,
	) -> DispatchResult {
		<T as Config>::Currency::transfer(
			currency_id.into(),
			&source,
			&dest,
			value.into(),
			ExistenceRequirement::KeepAlive,
		)
		.into()
	}
	pub fn create_new_token(who: &T::AccountId, amount: Balance) -> TokenId {
		<T as Config>::Currency::create(who, amount.into())
			.expect("Token creation failed")
			.into()
	}
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
