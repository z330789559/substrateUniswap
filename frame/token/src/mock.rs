use crate::{ Module, Trait };
use sp_core::H256;
use frame_support::{
    impl_outer_origin, parameter_types, weights::Weight
};
use sp_runtime::{
    traits::{ BlakeTwo256, IdentityLookup },
    testing::Header,
    Perbill,
};

impl_outer_origin! {
    pub enum Origin for Test {}
}

#[derive(Clone, Eq, PartialEq)]
pub struct Test;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    pub const ExistentialDeposit: u64 = 1;
}

impl system::Trait for Test {
    type Origin = Origin;
    type Call = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
    type ModuleToIndex = ();
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
}

impl Trait for Test {
    type Event = ();
    // type Currency = pallet_balances::Module<Test>;
    type TokenBalance = u64;
    type TokenId = u64;
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
    system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}