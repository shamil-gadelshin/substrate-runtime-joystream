#![cfg(test)]

pub use crate::{GenesisConfig, Trait};

pub use frame_support::traits::{Currency, LockIdentifier};
use frame_support::weights::Weight;
use frame_support::{impl_outer_origin, parameter_types};
pub use frame_system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};

pub use common::currency::GovernanceCurrency;

impl_outer_origin! {
    pub enum Origin for Test {}
}

// Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Test;
parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: u32 = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::one();
    pub const MinimumPeriod: u64 = 5;
}

impl frame_system::Trait for Test {
    type BaseCallFilter = ();
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
    type DbWeight = ();
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = ();
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
    type PalletInfo = ();
    type AccountData = balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
}

impl pallet_timestamp::Trait for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: u32 = 0;
    pub const MembershipFee: u64 = 100;
}

impl balances::Trait for Test {
    type Balance = u64;
    type DustRemoval = ();
    type Event = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
}

impl common::Trait for Test {
    type MemberId = u64;
    type ActorId = u32;
}

parameter_types! {
    pub const MaxWorkerNumberLimit: u32 = 3;
    pub const LockId: LockIdentifier = [9; 8];
}

pub struct WorkingGroupWeightInfo;
impl working_group::Trait<crate::MembershipWorkingGroupInstance> for Test {
    type Event = ();
    type MaxWorkerNumberLimit = MaxWorkerNumberLimit;
    type StakingHandler = staking_handler::StakingManager<Self, LockId>;
    type MemberOriginValidator = ();
    type MinUnstakingPeriodLimit = ();
    type RewardPeriod = ();
    type WeightInfo = WorkingGroupWeightInfo;
}

impl working_group::WeightInfo for WorkingGroupWeightInfo {
    fn on_initialize_leaving(_: u32) -> Weight {
        0
    }
    fn on_initialize_rewarding_with_missing_reward(_: u32) -> Weight {
        0
    }
    fn on_initialize_rewarding_with_missing_reward_cant_pay(_: u32) -> Weight {
        0
    }
    fn on_initialize_rewarding_without_missing_reward(_: u32) -> Weight {
        0
    }
    fn apply_on_opening(_: u32) -> Weight {
        0
    }
    fn fill_opening_lead() -> Weight {
        0
    }
    fn fill_opening_worker(_: u32) -> Weight {
        0
    }
    fn update_role_account() -> Weight {
        0
    }
    fn cancel_opening() -> Weight {
        0
    }
    fn withdraw_application() -> Weight {
        0
    }
    fn slash_stake(_: u32) -> Weight {
        0
    }
    fn terminate_role_worker(_: u32) -> Weight {
        0
    }
    fn terminate_role_lead(_: u32) -> Weight {
        0
    }
    fn increase_stake() -> Weight {
        0
    }
    fn decrease_stake() -> Weight {
        0
    }
    fn spend_from_budget() -> Weight {
        0
    }
    fn update_reward_amount() -> Weight {
        0
    }
    fn set_status_text(_: u32) -> Weight {
        0
    }
    fn update_reward_account() -> Weight {
        0
    }
    fn set_budget() -> Weight {
        0
    }
    fn add_opening(_: u32) -> Weight {
        0
    }
    fn leave_role_immediatly() -> Weight {
        0
    }
    fn leave_role_later() -> Weight {
        0
    }
}

impl common::origin::ActorOriginValidator<Origin, u64, u64> for () {
    fn ensure_actor_origin(origin: Origin, _: u64) -> Result<u64, &'static str> {
        let account_id = frame_system::ensure_signed(origin)?;

        Ok(account_id)
    }
}

impl Trait for Test {
    type Event = ();
    type MembershipFee = MembershipFee;
}

pub struct TestExternalitiesBuilder<T: Trait> {
    system_config: Option<frame_system::GenesisConfig>,
    membership_config: Option<GenesisConfig<T>>,
}

impl<T: Trait> Default for TestExternalitiesBuilder<T> {
    fn default() -> Self {
        Self {
            system_config: None,
            membership_config: None,
        }
    }
}

impl<T: Trait> TestExternalitiesBuilder<T> {
    pub fn set_membership_config(mut self, membership_config: GenesisConfig<T>) -> Self {
        self.membership_config = Some(membership_config);
        self
    }
    pub fn build(self) -> sp_io::TestExternalities {
        // Add system
        let mut t = self
            .system_config
            .unwrap_or(frame_system::GenesisConfig::default())
            .build_storage::<T>()
            .unwrap();

        // Add membership
        self.membership_config
            .unwrap_or(GenesisConfig::default())
            .assimilate_storage(&mut t)
            .unwrap();

        t.into()
    }
}

pub type Balances = balances::Module<Test>;
pub type Members = crate::Module<Test>;
pub type System = frame_system::Module<Test>;
