use crate::{ Module, Trait };
use sp_core::H256;
use frame_support::{
    impl_outer_origin, parameter_types, weights::Weight,impl_outer_event
};

use sp_runtime::{
    traits::{ BlakeTwo256, IdentityLookup },
    testing::Header,
    Perbill,
};

mod token {
	pub use crate::Event;
}

impl_outer_origin! {
    pub enum Origin for Test {}
}
impl_outer_event! {
	pub enum Event for Test {
		frame_system<T>,
		token<T>,
	}
}


#[derive(Clone, Eq, PartialEq)]
pub struct Test;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}
impl frame_system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Call = ();
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
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
impl Trait for Test {
    type Event = Event;
    // type Currency = pallet_balances::Module<Test>;
    type Balance = u128;
    type AssetId = u64;
}

// impl pallet_balances::Trait for Test {
//     type Balance = u64;
//     type Event = ();
//     type DustRemoval = ();
//     type ExistentialDeposit = ExistentialDeposit;
//     type AccountStore = System;
// }

// type System = system::Module<Test>;
// type Balances = pallet_balances::Module<Test>;
pub type FungiblePallet = Module<Test>;

pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}