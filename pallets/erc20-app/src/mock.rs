// Mock runtime

use crate::{Module, Trait};
use frame_support::{impl_outer_event, impl_outer_origin, parameter_types, weights::Weight};
use frame_system as system;
use mangata_primitives::{Amount, Balance, TokenId};
use orml_tokens::MultiTokenCurrencyAdapter;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify},
    MultiSignature, Perbill,
};
use sp_std::convert::From;

use artemis_asset as asset;

impl_outer_origin! {
    pub enum Origin for MockRuntime {}
}

mod test_events {
    pub use crate::Event;
}

impl_outer_event! {
    pub enum MockEvent for MockRuntime {
        system<T>,
        asset<T>,
        orml_tokens<T>,
        test_events<T>,
    }
}

pub type Signature = MultiSignature;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

#[derive(Clone, Eq, PartialEq)]
pub struct MockRuntime;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl system::Trait for MockRuntime {
    type BaseCallFilter = ();
    type Origin = Origin;
    type Call = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = MockEvent;
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type DbWeight = ();
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
    type PalletInfo = ();
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
}

impl orml_tokens::Trait for MockRuntime {
    type Event = MockEvent;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = TokenId;
    type OnReceived = ();
    type WeightInfo = ();
}

impl asset::Trait for MockRuntime {
    type Event = MockEvent;
    type Currency = MultiTokenCurrencyAdapter<MockRuntime>;
}

impl Trait for MockRuntime {
    type Event = MockEvent;
}

pub type System = system::Module<MockRuntime>;
pub type Tokens = <MockRuntime as asset::Trait>::Currency;
pub type ERC20 = Module<MockRuntime>;

pub fn new_tester() -> sp_io::TestExternalities {
    let storage = system::GenesisConfig::default()
        .build_storage::<MockRuntime>()
        .unwrap();
    let mut ext: sp_io::TestExternalities = storage.into();
    ext.execute_with(|| System::set_block_number(1));
    ext
}
