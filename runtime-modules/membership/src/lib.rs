//! Joystream membership module.
//!
//! Memberships are the stable identifier under which actors occupy roles,
//! submit proposals and communicate on the platform.
//!
//! ### Overview
//! A membership is a representation of an actor on the platform,
//! and it exist to serve the profile and reputation purposes.
//!
//! #### Profile
//! A membership has an associated rich profile that includes information that support presenting
//! the actor in a human friendly way in applications, much more so than raw accounts in isolation.
//!
//! #### Reputation
//!
//! Facilitates the consolidation of all activity under one stable identifier,
//! allowing an actor to invest in the reputation of a membership through prolonged participation
//! with good conduct. This gives honest and competent actors a practical way to signal quality,
//! and this quality signal is a key screening parameter allowing entry into more important and
//! sensitive activities. While nothing technically prevents an actor from registering for multiple
//! memberships, the value of doing a range of activities under one membership should be greater
//! than having it fragmented, since reputation, in essence, increases with the length and scope of
//! the history of consistent good conduct.
//!
//! It's important to be aware that a membership is not an account, but a higher level concept that
//! involves accounts for authentication. The membership subsystem is responsible for storing and
//! managing all memberships on the platform, as well as enabling the creation of new memberships,
//! and the terms under which this may happen.
//!
//! Supported extrinsics:
//! - [update_profile](./struct.Module.html#method.update_profile) - updates profile parameters.
//! - [buy_membership](./struct.Module.html#method.buy_membership) - allows to buy membership
//! for non-members.
//! - [update_accounts](./struct.Module.html#method.update_accounts) - updates member accounts.
//! - [update_profile_verification](./struct.Module.html#method.update_profile_verification) -
//! updates member profile verification status.
//! - [set_referral_cut](./struct.Module.html#method.set_referral_cut) - updates the referral cut.
//! - [transfer_invites](./struct.Module.html#method.transfer_invites) - transfers the invites
//! from one member to another.
//!
//! [Joystream handbook description](https://joystream.gitbook.io/joystream-handbook/subsystems/membership)

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub mod genesis;
mod tests;

use codec::{Decode, Encode};
use frame_support::traits::{Currency, Get};
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure};
use frame_system::ensure_root;
use frame_system::ensure_signed;
use sp_arithmetic::traits::{One, Zero};
use sp_runtime::traits::Hash;
use sp_std::vec::Vec;

use common::working_group::WorkingGroupIntegration;
use sp_std::collections::btree_map::BTreeMap;

// Balance type alias
type BalanceOf<T> = <T as balances::Trait>::Balance;

pub trait Trait:
    frame_system::Trait + balances::Trait + pallet_timestamp::Trait + common::Trait
{
    /// Membership module event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// Defines the default membership fee.
    type DefaultMembershipPrice: Get<BalanceOf<Self>>;

    /// Working group pallet integration.
    type WorkingGroup: common::working_group::WorkingGroupIntegration<Self>;

    /// Defines the default balance for the invited member.
    type DefaultInitialInvitationBalance: Get<BalanceOf<Self>>;

    /// Defines the maximum staking account number.
    type StakingAccountNumberLimit: Get<u32>;
}

pub(crate) const DEFAULT_MEMBER_INVITES_COUNT: u32 = 5;

/// Public membership profile alias.
pub type Membership<T> = MembershipObject<<T as frame_system::Trait>::AccountId>;

#[derive(Encode, Decode, Default)]
/// Stored information about a registered user.
pub struct MembershipObject<AccountId: Ord> {
    /// The hash of the handle chosen by member.
    pub handle_hash: Vec<u8>,

    /// Member's root account id. Only the root account is permitted to set a new root account
    /// and update the controller account. Other modules may only allow certain actions if
    /// signed with root account. It is intended to be an account that can remain offline and
    /// potentially hold a member's funds, and be a source for staking roles.
    pub root_account: AccountId,

    /// Member's controller account id. This account is intended to be used by
    /// a member to act under their identity in other modules. It will usually be used more
    /// online and will have less funds in its balance.
    pub controller_account: AccountId,

    /// An indicator that reflects whether the implied real world identity in the profile
    /// corresponds to the true actor behind the membership.
    pub verified: bool,

    /// Defines how many invitations this member has
    pub invites: u32,

    /// Staking account IDs bound to the membership. Each account must be confirmed.
    /// A map consists from staking account IDs and their confirmation flags.
    pub staking_account_ids: BTreeMap<AccountId, bool>,
}

impl<AccountId: Ord> MembershipObject<AccountId> {
    /// Add staking account id to the membership. Set its confirmation flag to false by default.
    pub fn add_staking_account_candidate(&mut self, staking_account_id: AccountId) {
        self.staking_account_ids.insert(staking_account_id, false);
    }

    /// Remove staking account id to the membership.
    pub fn remove_staking_account(&mut self, staking_account_id: AccountId) {
        self.staking_account_ids.remove(&staking_account_id);
    }

    /// Sets staking account confirmation flag as True. No effect on non-existing account.
    pub fn confirm_staking_account(&mut self, staking_account_id: AccountId) {
        if self.staking_account_exists(&staking_account_id) {
            self.staking_account_ids.insert(staking_account_id, true);
        }
    }

    /// Verifies existence of the staking account.
    pub fn staking_account_exists(&self, staking_account_id: &AccountId) -> bool {
        self.staking_account_ids.contains_key(&staking_account_id)
    }

    /// Verifies confirmation of the staking account.
    pub fn staking_account_confirmed(&self, staking_account_id: &AccountId) -> bool {
        let staking_account_confirmation = self.staking_account_ids.get(staking_account_id);

        staking_account_confirmation.copied().unwrap_or(false)
    }

    /// Returns current staking account number.
    pub fn staking_account_count(&self) -> u32 {
        self.staking_account_ids.len() as u32
    }
}

/// Parameters for the buy_membership extrinsic.
#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct BuyMembershipParameters<AccountId, MemberId> {
    /// New member root account.
    pub root_account: AccountId,

    /// New member controller account.
    pub controller_account: AccountId,

    /// New member user name.
    pub name: Option<Vec<u8>>,

    /// New member handle.
    pub handle: Option<Vec<u8>>,

    /// New member avatar URI.
    pub avatar_uri: Option<Vec<u8>>,

    /// New member 'about' text.
    pub about: Option<Vec<u8>>,

    /// Referrer member id.
    pub referrer_id: Option<MemberId>,
}

/// Parameters for the invite_member extrinsic.
#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct InviteMembershipParameters<AccountId, MemberId> {
    /// Inviting member id.
    pub inviting_member_id: MemberId,

    /// New member root account.
    pub root_account: AccountId,

    /// New member controller account.
    pub controller_account: AccountId,

    /// New member user name.
    pub name: Option<Vec<u8>>,

    /// New member handle.
    pub handle: Option<Vec<u8>>,

    /// New member avatar URI.
    pub avatar_uri: Option<Vec<u8>>,

    /// New member 'about' text.
    pub about: Option<Vec<u8>>,
}

decl_error! {
    /// Membership module predefined errors
    pub enum Error for Module<T: Trait> {
        /// Not enough balance to buy membership.
        NotEnoughBalanceToBuyMembership,

        /// Controller account required.
        ControllerAccountRequired,

        /// Root account required.
        RootAccountRequired,

        /// Invalid origin.
        UnsignedOrigin,

        /// Member profile not found (invalid member id).
        MemberProfileNotFound,

        /// Handle already registered.
        HandleAlreadyRegistered,

        /// Handle must be provided during registration.
        HandleMustBeProvidedDuringRegistration,

        /// Cannot find a membership for a provided referrer id.
        ReferrerIsNotMember,

        /// Should be a member to receive invites.
        CannotTransferInvitesForNotMember,

        /// Not enough invites to perform an operation.
        NotEnoughInvites,

        /// Membership working group leader is not set.
        WorkingGroupLeaderNotSet,

        /// Staking account for membership exists.
        StakingAccountExists,

        /// Staking account for membership doesn't exist.
        StakingAccountDoesntExist,

        /// Cannot add more staking account id.
        MaximumStakingAccountNumberExceeded,
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as Membership {
        /// MemberId to assign to next member that is added to the registry, and is also the
        /// total number of members created. MemberIds start at Zero.
        pub NextMemberId get(fn members_created) : T::MemberId;

        /// Mapping of member's id to their membership profile.
        pub MembershipById get(fn membership) : map hasher(blake2_128_concat)
            T::MemberId => Membership<T>;

        /// Mapping of a root account id to vector of member ids it controls.
        pub(crate) MemberIdsByRootAccountId : map hasher(blake2_128_concat)
            T::AccountId => Vec<T::MemberId>;

        /// Mapping of a controller account id to vector of member ids it controls.
        pub(crate) MemberIdsByControllerAccountId : map hasher(blake2_128_concat)
            T::AccountId => Vec<T::MemberId>;

        /// Registered unique handles hash and their mapping to their owner.
        pub MemberIdByHandleHash get(fn handles) : map hasher(blake2_128_concat)
            Vec<u8> => T::MemberId;

        /// Referral cut to receive during on buying the membership.
        pub ReferralCut get(fn referral_cut) : BalanceOf<T>;

        /// Current membership price.
        pub MembershipPrice get(fn membership_price) : BalanceOf<T> =
            T::DefaultMembershipPrice::get();

        /// Initial invitation count for the newly bought membership.
        pub InitialInvitationCount get(fn initial_invitation_count) : u32  =
            DEFAULT_MEMBER_INVITES_COUNT;

        /// Initial invitation balance for the invited member.
        pub InitialInvitationBalance get(fn initial_invitation_balance) : BalanceOf<T> =
            T::DefaultInitialInvitationBalance::get();
    }
    add_extra_genesis {
        config(members) : Vec<genesis::Member<T::MemberId, T::AccountId>>;
        build(|config: &GenesisConfig<T>| {
            for member in &config.members {
                let handle_hash = <Module<T>>::get_handle_hash(
                    Some(member.handle.clone().into_bytes()),
                ).expect("Importing Member Failed");

                let member_id = <Module<T>>::insert_member(
                    &member.root_account,
                    &member.controller_account,
                    handle_hash,
                    Zero::zero(),
                ).expect("Importing Member Failed");

                // ensure imported member id matches assigned id
                assert_eq!(member_id, member.member_id, "Import Member Failed: MemberId Incorrect");
            }
        });
    }
}

decl_event! {
    pub enum Event<T> where
      <T as common::Trait>::MemberId,
      Balance = BalanceOf<T>,
      <T as frame_system::Trait>::AccountId,
    {
        MemberRegistered(MemberId),
        MemberProfileUpdated(MemberId),
        MemberAccountsUpdated(MemberId),
        MemberVerificationStatusUpdated(MemberId, bool),
        ReferralCutUpdated(Balance),
        InvitesTransferred(MemberId, MemberId, u32),
        MembershipPriceUpdated(Balance),
        InitialInvitationBalanceUpdated(Balance),
        LeaderInvitationQuotaUpdated(u32),
        InitialInvitationCountUpdated(u32),
        StakingAccountAdded(MemberId, AccountId),
        StakingAccountRemoved(MemberId, AccountId),
        StakingAccountConfirmed(MemberId, AccountId),
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Non-members can buy membership.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn buy_membership(
            origin,
            params: BuyMembershipParameters<T::AccountId, T::MemberId>
        ) {
            let who = ensure_signed(origin)?;

            let fee = Self::membership_price();

            // Ensure enough free balance to cover membership fee.
            ensure!(
                balances::Module::<T>::usable_balance(&who) >= fee,
                Error::<T>::NotEnoughBalanceToBuyMembership
            );

            let handle_hash = Self::get_handle_hash(
                params.handle,
            )?;

            let referrer = params
                .referrer_id
                .map(|referrer_id| {
                    Self::ensure_membership_with_error(referrer_id, Error::<T>::ReferrerIsNotMember)
                })
                .transpose()?;

            //
            // == MUTATION SAFE ==
            //

            let member_id = Self::insert_member(
                &params.root_account,
                &params.controller_account,
                handle_hash,
                Self::initial_invitation_count(),
            )?;

            // Collect membership fee (just burn it).
            let _ = balances::Module::<T>::slash(&who, fee);

            // Reward the referring member.
            if let Some(referrer) = referrer{
                let referral_cut: BalanceOf<T> = Self::get_referral_bonus();

                if referral_cut > Zero::zero() {
                    let _ = balances::Module::<T>::deposit_creating(
                        &referrer.controller_account,
                        referral_cut
                    );
                }
            }

            // Fire the event.
            Self::deposit_event(RawEvent::MemberRegistered(member_id));
        }

        /// Update member's all or some of name, handle, avatar and about text.
        /// No effect if no changed fields.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_profile(
            origin,
            member_id: T::MemberId,
            name: Option<Vec<u8>>,
            handle: Option<Vec<u8>>,
            avatar_uri: Option<Vec<u8>>,
            about: Option<Vec<u8>>
        ) {
            // No effect if no changes.
            if name.is_none() && handle.is_none() && avatar_uri.is_none() && about.is_none() {
                return Ok(())
            }

            Self::ensure_member_controller_account_signed(origin, &member_id)?;

            let membership = Self::ensure_membership(member_id)?;

            let new_handle_hash = handle
                .map(|handle| Self::get_handle_hash(Some(handle)))
                .transpose()?;

            //
            // == MUTATION SAFE ==
            //

            if let Some(new_handle_hash) = new_handle_hash{
                // remove old handle hash
                <MemberIdByHandleHash<T>>::remove(&membership.handle_hash);

                <MembershipById<T>>::mutate(&member_id, |membership| {
                    membership.handle_hash = new_handle_hash.clone();
                });

                <MemberIdByHandleHash<T>>::insert(new_handle_hash, member_id);

                Self::deposit_event(RawEvent::MemberProfileUpdated(member_id));
            }
        }

        /// Updates member root or controller accounts. No effect if both new accounts are empty.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_accounts(
            origin,
            member_id: T::MemberId,
            new_root_account: Option<T::AccountId>,
            new_controller_account: Option<T::AccountId>,
        ) {
            // No effect if no changes.
            if new_root_account.is_none() && new_controller_account.is_none() {
                return Ok(())
            }

            let sender = ensure_signed(origin)?;
            let mut membership = Self::ensure_membership(member_id)?;

            ensure!(membership.root_account == sender, Error::<T>::RootAccountRequired);

            //
            // == MUTATION SAFE ==
            //

            if let Some(root_account) = new_root_account {
                <MemberIdsByRootAccountId<T>>::mutate(&membership.root_account, |ids| {
                    ids.retain(|id| *id != member_id);
                });

                <MemberIdsByRootAccountId<T>>::mutate(&root_account, |ids| {
                    ids.push(member_id);
                });

                membership.root_account = root_account;
            }

            if let Some(controller_account) = new_controller_account {
                <MemberIdsByControllerAccountId<T>>::mutate(&membership.controller_account, |ids| {
                    ids.retain(|id| *id != member_id);
                });

                <MemberIdsByControllerAccountId<T>>::mutate(&controller_account, |ids| {
                    ids.push(member_id);
                });

                membership.controller_account = controller_account;
            }

            <MembershipById<T>>::insert(member_id, membership);
            Self::deposit_event(RawEvent::MemberAccountsUpdated(member_id));
        }

        /// Updates member profile verification status. Requires working group member origin.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn update_profile_verification(
            origin,
            worker_id: T::ActorId,
            target_member_id: T::MemberId,
            is_verified: bool
        ) {
            T::WorkingGroup::ensure_worker_origin(origin, &worker_id)?;

            Self::ensure_membership(target_member_id)?;

            //
            // == MUTATION SAFE ==
            //

            <MembershipById<T>>::mutate(&target_member_id, |membership| {
                    membership.verified = is_verified;
            });

            Self::deposit_event(
                RawEvent::MemberVerificationStatusUpdated(target_member_id, is_verified)
            );
        }

        /// Updates membership referral cut. Requires root origin.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn set_referral_cut(origin, value: BalanceOf<T>) {
            ensure_root(origin)?;

            //
            // == MUTATION SAFE ==
            //

            <ReferralCut<T>>::put(value);

            Self::deposit_event(RawEvent::ReferralCutUpdated(value));
        }

        /// Transfers invites from one member to another.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn transfer_invites(
            origin,
            source_member_id: T::MemberId,
            target_member_id: T::MemberId,
            number_of_invites: u32
        ) {
            Self::ensure_member_controller_account_signed(origin, &source_member_id)?;

            let source_membership = Self::ensure_membership(source_member_id)?;
            Self::ensure_membership_with_error(
                target_member_id,
                Error::<T>::CannotTransferInvitesForNotMember
            )?;

            ensure!(source_membership.invites >= number_of_invites, Error::<T>::NotEnoughInvites);

            //
            // == MUTATION SAFE ==
            //

            // Decrease source member invite number.
            <MembershipById<T>>::mutate(&source_member_id, |membership| {
                membership.invites = membership.invites.saturating_sub(number_of_invites);
            });

            // Increase target member invite number.
            <MembershipById<T>>::mutate(&target_member_id, |membership| {
                membership.invites = membership.invites.saturating_add(number_of_invites);
            });

            Self::deposit_event(RawEvent::InvitesTransferred(
                source_member_id,
                target_member_id,
                number_of_invites
            ));
        }

        /// Invite a new member.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn invite_member(
            origin,
            params: InviteMembershipParameters<T::AccountId, T::MemberId>
        ) {
            let membership = Self::ensure_member_controller_account_signed(
                origin,
                &params.inviting_member_id
            )?;

            ensure!(membership.invites > Zero::zero(), Error::<T>::NotEnoughInvites);

            let handle_hash = Self::get_handle_hash(
                params.handle,
            )?;

            //
            // == MUTATION SAFE ==
            //

            let member_id = Self::insert_member(
                &params.root_account,
                &params.controller_account,
                handle_hash,
                Zero::zero(),
            )?;

            // Save the updated profile.
            <MembershipById<T>>::mutate(&member_id, |membership| {
                membership.invites = membership.invites.saturating_sub(1);
            });

            // Fire the event.
            Self::deposit_event(RawEvent::MemberRegistered(member_id));
        }

        /// Updates membership price. Requires root origin.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn set_membership_price(origin, new_price: BalanceOf<T>) {
            ensure_root(origin)?;

            //
            // == MUTATION SAFE ==
            //

            <MembershipPrice<T>>::put(new_price);

            Self::deposit_event(RawEvent::MembershipPriceUpdated(new_price));
        }

        /// Updates leader invitation quota. Requires root origin.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn set_leader_invitation_quota(origin, invitation_quota: u32) {
            ensure_root(origin)?;

            let leader_member_id = T::WorkingGroup::get_leader_member_id();

            if let Some(member_id) = leader_member_id{
                Self::ensure_membership(member_id)?;
            }

            ensure!(leader_member_id.is_some(), Error::<T>::WorkingGroupLeaderNotSet);

            //
            // == MUTATION SAFE ==
            //

            if let Some(member_id) = leader_member_id{
                <MembershipById<T>>::mutate(&member_id, |membership| {
                        membership.invites = invitation_quota;
                });

                Self::deposit_event(RawEvent::LeaderInvitationQuotaUpdated(invitation_quota));
            }
        }

        /// Updates initial invitation balance for a invited member. Requires root origin.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn set_initial_invitation_balance(origin, new_initial_balance: BalanceOf<T>) {
            ensure_root(origin)?;

            //
            // == MUTATION SAFE ==
            //

            <InitialInvitationBalance<T>>::put(new_initial_balance);

            Self::deposit_event(RawEvent::InitialInvitationBalanceUpdated(new_initial_balance));
        }

        /// Updates initial invitation count for a member. Requires root origin.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn set_initial_invitation_count(origin, new_invitation_count: u32) {
            ensure_root(origin)?;

            //
            // == MUTATION SAFE ==
            //

            InitialInvitationCount::put(new_invitation_count);

            Self::deposit_event(RawEvent::InitialInvitationCountUpdated(new_invitation_count));
        }

        /// Add staking account candidate for a member.
        /// The staking account candidate must be confirmed by its owner before usage.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn add_staking_account_candidate(
            origin,
            member_id: T::MemberId,
            staking_account_id: T::AccountId
        ) {
            let membership = Self::ensure_member_controller_account_signed(origin, &member_id)?;

            ensure!(
                !membership.staking_account_exists(&staking_account_id),
                Error::<T>::StakingAccountExists
            );

            ensure!(
                membership.staking_account_count() < T::StakingAccountNumberLimit::get(),
                Error::<T>::MaximumStakingAccountNumberExceeded
            );

            //
            // == MUTATION SAFE ==
            //

            <MembershipById<T>>::mutate(&member_id, |membership| {
                membership.add_staking_account_candidate(staking_account_id.clone());
            });

            Self::deposit_event(RawEvent::StakingAccountAdded(member_id, staking_account_id));
        }

        /// Remove staking account for a member.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn remove_staking_account(
            origin,
            member_id: T::MemberId,
            staking_account_id: T::AccountId
        ) {
            let membership = Self::ensure_member_controller_account_signed(origin, &member_id)?;

            ensure!(
                membership.staking_account_exists(&staking_account_id),
                Error::<T>::StakingAccountDoesntExist
            );

            //
            // == MUTATION SAFE ==
            //

            <MembershipById<T>>::mutate(&member_id, |membership| {
                membership.remove_staking_account(staking_account_id.clone());
            });

            Self::deposit_event(RawEvent::StakingAccountRemoved(member_id, staking_account_id));
        }

        /// Confirm staking account candidate for a member.
        #[weight = 10_000_000] // TODO: adjust weight
        pub fn confirm_staking_account(
            origin,
            member_id: T::MemberId,
        ) {
            let staking_account_id = ensure_signed(origin)?;

            let membership = Self::ensure_membership(member_id)?;

            ensure!(
                membership.staking_account_exists(&staking_account_id),
                Error::<T>::StakingAccountDoesntExist
            );

            //
            // == MUTATION SAFE ==
            //

            <MembershipById<T>>::mutate(&member_id, |membership| {
                membership.confirm_staking_account(staking_account_id.clone());
            });

            Self::deposit_event(RawEvent::StakingAccountConfirmed(member_id, staking_account_id));
        }
    }
}

impl<T: Trait> Module<T> {
    /// Provided that the member_id exists return its membership. Returns error otherwise.
    pub fn ensure_membership(member_id: T::MemberId) -> Result<Membership<T>, Error<T>> {
        Self::ensure_membership_with_error(member_id, Error::<T>::MemberProfileNotFound)
    }

    /// Provided that the member_id exists return its membership. Returns provided error otherwise.
    fn ensure_membership_with_error(
        id: T::MemberId,
        error: Error<T>,
    ) -> Result<Membership<T>, Error<T>> {
        if <MembershipById<T>>::contains_key(&id) {
            Ok(Self::membership(&id))
        } else {
            Err(error)
        }
    }

    /// Returns true if account is either a member's root or controller account
    pub fn is_member_account(who: &T::AccountId) -> bool {
        <MemberIdsByRootAccountId<T>>::contains_key(who)
            || <MemberIdsByControllerAccountId<T>>::contains_key(who)
    }

    // Ensure possible member handle hash is unique.
    fn ensure_unique_handle_hash(handle_hash: Vec<u8>) -> Result<(), Error<T>> {
        ensure!(
            !<MemberIdByHandleHash<T>>::contains_key(handle_hash),
            Error::<T>::HandleAlreadyRegistered
        );
        Ok(())
    }

    // Validate handle and return its hash.
    fn get_handle_hash(handle: Option<Vec<u8>>) -> Result<Vec<u8>, Error<T>> {
        // Handle is required during registration
        let handle = handle.ok_or(Error::<T>::HandleMustBeProvidedDuringRegistration)?;

        if handle.is_empty() {
            return Err(Error::<T>::HandleMustBeProvidedDuringRegistration);
        }

        let hashed = T::Hashing::hash(&handle);
        let handle_hash = hashed.as_ref().to_vec();

        Self::ensure_unique_handle_hash(handle_hash.clone())?;

        Ok(handle_hash)
    }

    // Inserts a member using a validated information. Sets handle, accounts caches, etc..
    fn insert_member(
        root_account: &T::AccountId,
        controller_account: &T::AccountId,
        handle_hash: Vec<u8>,
        allowed_invites: u32,
    ) -> Result<T::MemberId, Error<T>> {
        let new_member_id = Self::members_created();

        let membership: Membership<T> = MembershipObject {
            handle_hash: handle_hash.clone(),
            root_account: root_account.clone(),
            controller_account: controller_account.clone(),
            verified: false,
            invites: allowed_invites,
            staking_account_ids: BTreeMap::new(),
        };

        <MemberIdsByRootAccountId<T>>::mutate(root_account, |ids| {
            ids.push(new_member_id);
        });
        <MemberIdsByControllerAccountId<T>>::mutate(controller_account, |ids| {
            ids.push(new_member_id);
        });

        <MembershipById<T>>::insert(new_member_id, membership);
        <MemberIdByHandleHash<T>>::insert(handle_hash, new_member_id);

        <NextMemberId<T>>::put(new_member_id + One::one());
        Ok(new_member_id)
    }

    // Ensure origin corresponds to the controller account of the member.
    fn ensure_member_controller_account_signed(
        origin: T::Origin,
        member_id: &T::MemberId,
    ) -> Result<Membership<T>, Error<T>> {
        // Ensure transaction is signed.
        let signer_account_id = ensure_signed(origin).map_err(|_| Error::<T>::UnsignedOrigin)?;

        Self::ensure_is_controller_account_for_member(member_id, &signer_account_id)
    }

    /// Ensure that given member has given account as the controller account
    pub fn ensure_is_controller_account_for_member(
        member_id: &T::MemberId,
        account: &T::AccountId,
    ) -> Result<Membership<T>, Error<T>> {
        ensure!(
            MembershipById::<T>::contains_key(member_id),
            Error::<T>::MemberProfileNotFound
        );

        let membership = MembershipById::<T>::get(member_id);

        ensure!(
            membership.controller_account == *account,
            Error::<T>::ControllerAccountRequired
        );

        Ok(membership)
    }

    // Calculate current referral bonus. It minimum between membership fee and referral cut.
    pub(crate) fn get_referral_bonus() -> BalanceOf<T> {
        let membership_fee = Self::membership_price();
        let referral_cut = Self::referral_cut();

        membership_fee.min(referral_cut)
    }
}

impl<T: Trait> common::StakingAccountValidator<T> for Module<T> {
    fn is_member_staking_account(
        member_id: &common::MemberId<T>,
        account_id: &T::AccountId,
    ) -> bool {
        Self::ensure_membership(*member_id)
            .ok()
            .map(|membership| membership.staking_account_confirmed(account_id))
            .unwrap_or(false)
    }
}
