#![cfg(test)]

pub(crate) mod fixtures;
pub(crate) mod mock;

use crate::{Error, Event};
use fixtures::*;
use mock::*;

use common::StakingAccountValidator;
use frame_support::traits::{LockIdentifier, LockableCurrency, WithdrawReasons};
use frame_support::{assert_ok, StorageMap, StorageValue};
use frame_system::RawOrigin;
use sp_runtime::DispatchError;

#[test]
fn buy_membership_succeeds() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_balance = DefaultMembershipPrice::get();
        set_alice_free_balance(initial_balance);

        let next_member_id = Membership::members_created();

        assert_ok!(buy_default_membership_as_alice());

        let member_ids = vec![0];
        assert_eq!(member_ids, vec![next_member_id]);

        let profile = get_membership_by_id(next_member_id);

        assert_eq!(Some(profile.handle_hash), get_alice_info().handle_hash);
        assert_eq!(profile.invites, crate::DEFAULT_MEMBER_INVITES_COUNT);

        // controller account initially set to primary account
        assert_eq!(profile.controller_account, ALICE_ACCOUNT_ID);
        assert_eq!(
            <crate::MemberIdsByControllerAccountId<Test>>::get(ALICE_ACCOUNT_ID),
            vec![next_member_id]
        );

        EventFixture::assert_last_crate_event(Event::<Test>::MemberRegistered(next_member_id));
    });
}

#[test]
fn buy_membership_fails_without_enough_balance() {
    build_test_externalities().execute_with(|| {
        let initial_balance = DefaultMembershipPrice::get() - 1;
        set_alice_free_balance(initial_balance);

        assert_dispatch_error_message(
            buy_default_membership_as_alice(),
            Err(Error::<Test>::NotEnoughBalanceToBuyMembership.into()),
        );
    });
}

#[test]
fn buy_membership_fails_without_enough_balance_with_locked_balance() {
    build_test_externalities().execute_with(|| {
        let initial_balance = DefaultMembershipPrice::get();
        let lock_id = LockIdentifier::default();
        Balances::set_lock(lock_id, &ALICE_ACCOUNT_ID, 1, WithdrawReasons::all());
        set_alice_free_balance(initial_balance);

        assert_dispatch_error_message(
            buy_default_membership_as_alice(),
            Err(Error::<Test>::NotEnoughBalanceToBuyMembership.into()),
        );
    });
}

#[test]
fn buy_membership_fails_with_non_unique_handle() {
    build_test_externalities().execute_with(|| {
        let initial_balance = DefaultMembershipPrice::get();
        set_alice_free_balance(initial_balance);

        // alice's handle already taken
        <crate::MemberIdByHandleHash<Test>>::insert(get_alice_info().handle_hash.unwrap(), 1);

        // should not be allowed to buy membership with that handle
        assert_dispatch_error_message(
            buy_default_membership_as_alice(),
            Err(Error::<Test>::HandleAlreadyRegistered.into()),
        );
    });
}

#[test]
fn update_profile_succeeds() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_balance = DefaultMembershipPrice::get();
        set_alice_free_balance(initial_balance);

        let next_member_id = Membership::members_created();

        assert_ok!(buy_default_membership_as_alice());
        let info = get_bob_info();
        assert_ok!(Membership::update_profile(
            Origin::signed(ALICE_ACCOUNT_ID),
            next_member_id,
            info.name,
            info.handle,
            info.avatar_uri,
            info.about,
        ));

        let profile = get_membership_by_id(next_member_id);

        assert_eq!(Some(profile.handle_hash), get_bob_info().handle_hash);

        assert!(<crate::MemberIdByHandleHash<Test>>::contains_key(
            get_bob_info().handle_hash.unwrap()
        ));

        EventFixture::assert_last_crate_event(Event::<Test>::MemberProfileUpdated(next_member_id));
    });
}

#[test]
fn update_profile_has_no_effect_on_empty_parameters() {
    build_test_externalities().execute_with(|| {
        let initial_balance = DefaultMembershipPrice::get();
        set_alice_free_balance(initial_balance);

        let next_member_id = Membership::members_created();

        assert_ok!(buy_default_membership_as_alice());
        assert_ok!(Membership::update_profile(
            Origin::signed(ALICE_ACCOUNT_ID),
            next_member_id,
            None,
            None,
            None,
            None,
        ));

        let profile = get_membership_by_id(next_member_id);

        assert_eq!(Some(profile.handle_hash), get_alice_info().handle_hash);

        assert!(<crate::MemberIdByHandleHash<Test>>::contains_key(
            get_alice_info().handle_hash.unwrap()
        ));
    });
}

#[test]
fn update_profile_accounts_succeeds() {
    let initial_members = [(ALICE_MEMBER_ID, ALICE_ACCOUNT_ID)];

    build_test_externalities_with_initial_members(initial_members.to_vec()).execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        const ALICE_NEW_ACCOUNT_ID: u64 = 2;

        assert_ok!(Membership::update_accounts(
            Origin::signed(ALICE_ACCOUNT_ID),
            ALICE_MEMBER_ID,
            Some(ALICE_NEW_ACCOUNT_ID),
            Some(ALICE_NEW_ACCOUNT_ID),
        ));

        let profile = get_membership_by_id(ALICE_MEMBER_ID);

        assert_eq!(profile.controller_account, ALICE_NEW_ACCOUNT_ID);
        assert_eq!(
            <crate::MemberIdsByControllerAccountId<Test>>::get(&ALICE_NEW_ACCOUNT_ID),
            vec![ALICE_MEMBER_ID]
        );
        assert!(<crate::MemberIdsByControllerAccountId<Test>>::get(&ALICE_ACCOUNT_ID).is_empty());

        assert_eq!(profile.root_account, ALICE_NEW_ACCOUNT_ID);
        assert_eq!(
            <crate::MemberIdsByRootAccountId<Test>>::get(&ALICE_NEW_ACCOUNT_ID),
            vec![ALICE_MEMBER_ID]
        );
        assert!(<crate::MemberIdsByRootAccountId<Test>>::get(&ALICE_ACCOUNT_ID).is_empty());

        EventFixture::assert_last_crate_event(Event::<Test>::MemberAccountsUpdated(
            ALICE_MEMBER_ID,
        ));
    });
}

#[test]
fn update_accounts_has_effect_on_empty_account_parameters() {
    let initial_members = [(ALICE_MEMBER_ID, ALICE_ACCOUNT_ID)];

    build_test_externalities_with_initial_members(initial_members.to_vec()).execute_with(|| {
        assert_ok!(Membership::update_accounts(
            Origin::signed(ALICE_ACCOUNT_ID),
            ALICE_MEMBER_ID,
            None,
            None,
        ));

        let profile = get_membership_by_id(ALICE_MEMBER_ID);

        assert_eq!(profile.controller_account, ALICE_ACCOUNT_ID);
        assert_eq!(
            <crate::MemberIdsByControllerAccountId<Test>>::get(&ALICE_ACCOUNT_ID),
            vec![ALICE_MEMBER_ID]
        );

        assert_eq!(profile.root_account, ALICE_ACCOUNT_ID);
        assert_eq!(
            <crate::MemberIdsByRootAccountId<Test>>::get(&ALICE_ACCOUNT_ID),
            vec![ALICE_MEMBER_ID]
        );
    });
}

#[test]
fn update_verification_status_succeeds() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_balance = DefaultMembershipPrice::get();
        set_alice_free_balance(initial_balance);

        let next_member_id = Membership::members_created();
        assert_ok!(buy_default_membership_as_alice());

        UpdateMembershipVerificationFixture::default()
            .with_member_id(next_member_id)
            .call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(Event::<Test>::MemberVerificationStatusUpdated(
            next_member_id,
            true,
        ));
    });
}

#[test]
fn update_verification_status_fails_with_bad_origin() {
    build_test_externalities().execute_with(|| {
        let next_member_id = Membership::members_created();

        UpdateMembershipVerificationFixture::default()
            .with_member_id(next_member_id)
            .with_origin(RawOrigin::None)
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn update_verification_status_fails_with_invalid_member_id() {
    build_test_externalities().execute_with(|| {
        let invalid_member_id = 44;

        UpdateMembershipVerificationFixture::default()
            .with_member_id(invalid_member_id)
            .call_and_assert(Err(Error::<Test>::MemberProfileNotFound.into()));
    });
}

#[test]
fn update_verification_status_fails_with_invalid_worker_id() {
    build_test_externalities().execute_with(|| {
        let initial_balance = DefaultMembershipPrice::get();
        set_alice_free_balance(initial_balance);

        let next_member_id = Membership::members_created();
        assert_ok!(buy_default_membership_as_alice());

        let invalid_worker_id = 44;

        UpdateMembershipVerificationFixture::default()
            .with_member_id(next_member_id)
            .with_worker_id(invalid_worker_id)
            .call_and_assert(Err(working_group::Error::<
                Test,
                MembershipWorkingGroupInstance,
            >::WorkerDoesNotExist
                .into()));
    });
}

#[test]
fn buy_membership_fails_with_non_member_referrer_id() {
    build_test_externalities().execute_with(|| {
        let initial_balance = DefaultMembershipPrice::get();
        set_alice_free_balance(initial_balance);

        let invalid_member_id = 111;

        let buy_membership_fixture =
            BuyMembershipFixture::default().with_referrer_id(invalid_member_id);

        buy_membership_fixture.call_and_assert(Err(Error::<Test>::ReferrerIsNotMember.into()));
    });
}

#[test]
fn buy_membership_with_referral_cut_succeeds() {
    let initial_members = [(ALICE_MEMBER_ID, ALICE_ACCOUNT_ID)];

    build_test_externalities_with_initial_members(initial_members.to_vec()).execute_with(|| {
        let initial_balance = 10000;
        increase_total_balance_issuance_using_account_id(BOB_ACCOUNT_ID, initial_balance);

        let buy_membership_fixture = BuyMembershipFixture::default()
            .with_handle(b"bobs_handle".to_vec())
            .with_accounts(BOB_ACCOUNT_ID)
            .with_origin(RawOrigin::Signed(BOB_ACCOUNT_ID))
            .with_referrer_id(ALICE_MEMBER_ID);

        buy_membership_fixture.call_and_assert(Ok(()));

        let referral_cut = Membership::get_referral_bonus();

        assert_eq!(Balances::usable_balance(&ALICE_ACCOUNT_ID), referral_cut);
        assert_eq!(
            Balances::usable_balance(&BOB_ACCOUNT_ID),
            initial_balance - DefaultMembershipPrice::get()
        );
    });
}

#[test]
fn referral_bonus_calculated_successfully() {
    build_test_externalities().execute_with(|| {
        // it should take minimum of the referral cut and membership fee
        let membership_fee = DefaultMembershipPrice::get();
        let diff = 10;

        let referral_cut = membership_fee.saturating_sub(diff);
        <crate::ReferralCut<Test>>::put(referral_cut);
        assert_eq!(Membership::get_referral_bonus(), referral_cut);

        let referral_cut = membership_fee.saturating_add(diff);
        <crate::ReferralCut<Test>>::put(referral_cut);
        assert_eq!(Membership::get_referral_bonus(), membership_fee);
    });
}

#[test]
fn set_referral_cut_succeeds() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        SetReferralCutFixture::default().call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(Event::<Test>::ReferralCutUpdated(
            DEFAULT_REFERRAL_CUT_VALUE,
        ));
    });
}

#[test]
fn set_referral_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        SetReferralCutFixture::default()
            .with_origin(RawOrigin::Signed(ALICE_ACCOUNT_ID))
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn transfer_invites_succeeds() {
    let initial_members = [(ALICE_MEMBER_ID, ALICE_ACCOUNT_ID)];

    build_test_externalities_with_initial_members(initial_members.to_vec()).execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_balance = 10000;
        increase_total_balance_issuance_using_account_id(BOB_ACCOUNT_ID, initial_balance);

        BuyMembershipFixture::default()
            .with_handle(b"bobs_handle".to_vec())
            .with_accounts(BOB_ACCOUNT_ID)
            .with_origin(RawOrigin::Signed(BOB_ACCOUNT_ID))
            .call_and_assert(Ok(()));

        let bob_member_id = 1;

        let tranfer_invites_fixture = TransferInvitesFixture::default()
            .with_origin(RawOrigin::Signed(BOB_ACCOUNT_ID))
            .with_source_member_id(bob_member_id)
            .with_target_member_id(ALICE_MEMBER_ID);

        tranfer_invites_fixture.call_and_assert(Ok(()));

        let alice = Membership::membership(ALICE_MEMBER_ID);
        let bob = Membership::membership(bob_member_id);

        assert_eq!(alice.invites, tranfer_invites_fixture.invites);
        assert_eq!(
            bob.invites,
            crate::DEFAULT_MEMBER_INVITES_COUNT - tranfer_invites_fixture.invites
        );

        EventFixture::assert_last_crate_event(Event::<Test>::InvitesTransferred(
            bob_member_id,
            ALICE_MEMBER_ID,
            tranfer_invites_fixture.invites,
        ));
    });
}

#[test]
fn transfer_invites_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        TransferInvitesFixture::default()
            .with_origin(RawOrigin::None)
            .call_and_assert(Err(Error::<Test>::UnsignedOrigin.into()));
    });
}

#[test]
fn transfer_invites_fails_with_source_member_id() {
    build_test_externalities().execute_with(|| {
        TransferInvitesFixture::default()
            .call_and_assert(Err(Error::<Test>::MemberProfileNotFound.into()));
    });
}

#[test]
fn transfer_invites_fails_with_target_member_id() {
    let initial_members = [(ALICE_MEMBER_ID, ALICE_ACCOUNT_ID)];

    build_test_externalities_with_initial_members(initial_members.to_vec()).execute_with(|| {
        TransferInvitesFixture::default()
            .call_and_assert(Err(Error::<Test>::CannotTransferInvitesForNotMember.into()));
    });
}

#[test]
fn transfer_invites_fails_when_not_enough_invites() {
    let initial_members = [(ALICE_MEMBER_ID, ALICE_ACCOUNT_ID)];

    build_test_externalities_with_initial_members(initial_members.to_vec()).execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_balance = 10000;
        increase_total_balance_issuance_using_account_id(BOB_ACCOUNT_ID, initial_balance);

        BuyMembershipFixture::default()
            .with_handle(b"bobs_handle".to_vec())
            .with_accounts(BOB_ACCOUNT_ID)
            .with_origin(RawOrigin::Signed(BOB_ACCOUNT_ID))
            .call_and_assert(Ok(()));

        let bob_member_id = 1;

        let invalid_invites_number = 100;
        let tranfer_invites_fixture = TransferInvitesFixture::default()
            .with_origin(RawOrigin::Signed(BOB_ACCOUNT_ID))
            .with_source_member_id(bob_member_id)
            .with_target_member_id(ALICE_MEMBER_ID)
            .with_invites_number(invalid_invites_number);

        tranfer_invites_fixture.call_and_assert(Err(Error::<Test>::NotEnoughInvites.into()));
    });
}

#[test]
fn invite_member_succeeds() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        let initial_balance = DefaultMembershipPrice::get();
        set_alice_free_balance(initial_balance);

        assert_ok!(buy_default_membership_as_alice());
        let bob_member_id = Membership::members_created();

        InviteMembershipFixture::default().call_and_assert(Ok(()));

        let profile = get_membership_by_id(bob_member_id);

        let bob = get_bob_info();
        assert_eq!(Some(profile.handle_hash), bob.handle_hash);
        assert_eq!(profile.invites, 0);

        // controller account initially set to primary account
        assert_eq!(profile.controller_account, BOB_ACCOUNT_ID);
        assert_eq!(
            <crate::MemberIdsByControllerAccountId<Test>>::get(BOB_ACCOUNT_ID),
            vec![bob_member_id]
        );

        EventFixture::assert_last_crate_event(Event::<Test>::MemberRegistered(bob_member_id));
    });
}

#[test]
fn invite_member_fails_with_bad_origin() {
    build_test_externalities().execute_with(|| {
        InviteMembershipFixture::default()
            .with_origin(RawOrigin::None)
            .call_and_assert(Err(Error::<Test>::UnsignedOrigin.into()));
    });
}

#[test]
fn invite_member_fails_with_invalid_member_id() {
    build_test_externalities().execute_with(|| {
        let initial_balance = DefaultMembershipPrice::get();
        set_alice_free_balance(initial_balance);

        assert_ok!(buy_default_membership_as_alice());
        let invalid_member_id = 222;

        InviteMembershipFixture::default()
            .with_member_id(invalid_member_id)
            .call_and_assert(Err(Error::<Test>::MemberProfileNotFound.into()));
    });
}

#[test]
fn invite_member_fails_with_not_controller_account() {
    build_test_externalities().execute_with(|| {
        let initial_balance = DefaultMembershipPrice::get();
        set_alice_free_balance(initial_balance);

        assert_ok!(buy_default_membership_as_alice());
        let invalid_account_id = 222;

        InviteMembershipFixture::default()
            .with_accounts(invalid_account_id)
            .with_origin(RawOrigin::Signed(invalid_account_id))
            .call_and_assert(Err(Error::<Test>::ControllerAccountRequired.into()));
    });
}

#[test]
fn invite_member_fails_with_not_enough_invites() {
    let initial_members = [(ALICE_MEMBER_ID, ALICE_ACCOUNT_ID)];

    build_test_externalities_with_initial_members(initial_members.to_vec()).execute_with(|| {
        InviteMembershipFixture::default()
            .call_and_assert(Err(Error::<Test>::NotEnoughInvites.into()));
    });
}

#[test]
fn invite_member_fails_with_bad_user_information() {
    build_test_externalities().execute_with(|| {
        let initial_balance = DefaultMembershipPrice::get();
        set_alice_free_balance(initial_balance);

        assert_ok!(buy_default_membership_as_alice());

        InviteMembershipFixture::default()
            .with_handle(Vec::new())
            .call_and_assert(Err(
                Error::<Test>::HandleMustBeProvidedDuringRegistration.into()
            ));

        InviteMembershipFixture::default()
            .with_empty_handle()
            .call_and_assert(Err(
                Error::<Test>::HandleMustBeProvidedDuringRegistration.into()
            ));

        let handle = b"Non-unique handle".to_vec();
        InviteMembershipFixture::default()
            .with_handle(handle.clone())
            .call_and_assert(Ok(()));

        InviteMembershipFixture::default()
            .with_handle(handle)
            .call_and_assert(Err(Error::<Test>::HandleAlreadyRegistered.into()));
    });
}

#[test]
fn set_membership_price_succeeds() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        SetMembershipPriceFixture::default().call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(Event::<Test>::MembershipPriceUpdated(
            DEFAULT_MEMBERSHIP_PRICE,
        ));
    });
}

#[test]
fn set_membership_price_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        SetMembershipPriceFixture::default()
            .with_origin(RawOrigin::Signed(ALICE_ACCOUNT_ID))
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn buy_membership_with_zero_membership_price_succeeds() {
    build_test_externalities().execute_with(|| {
        let initial_balance = 10000;
        increase_total_balance_issuance_using_account_id(ALICE_ACCOUNT_ID, initial_balance);

        // Set zero membership price
        SetMembershipPriceFixture::default()
            .with_price(0)
            .call_and_assert(Ok(()));

        // Try to buy membership.
        BuyMembershipFixture::default().call_and_assert(Ok(()));

        assert_eq!(Balances::usable_balance(&ALICE_ACCOUNT_ID), initial_balance);
    });
}

#[test]
fn set_leader_invitation_quota_succeeds() {
    let initial_members = [(ALICE_MEMBER_ID, ALICE_ACCOUNT_ID)];

    build_test_externalities_with_initial_members(initial_members.to_vec()).execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        SetLeaderInvitationQuotaFixture::default().call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(Event::<Test>::LeaderInvitationQuotaUpdated(
            DEFAULT_LEADER_INVITATION_QUOTA,
        ));
    });
}

#[test]
fn set_leader_invitation_quota_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        SetLeaderInvitationQuotaFixture::default()
            .with_origin(RawOrigin::Signed(ALICE_ACCOUNT_ID))
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn set_leader_invitation_quota_fails_with_not_found_leader_membership() {
    build_test_externalities().execute_with(|| {
        SetLeaderInvitationQuotaFixture::default()
            .call_and_assert(Err(Error::<Test>::MemberProfileNotFound.into()));
    });
}

#[test]
fn set_initial_invitation_balance_succeeds() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        SetInitialInvitationBalanceFixture::default().call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(Event::<Test>::InitialInvitationBalanceUpdated(
            DEFAULT_INITIAL_INVITATION_BALANCE,
        ));
    });
}

#[test]
fn set_initial_invitation_balance_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        SetInitialInvitationBalanceFixture::default()
            .with_origin(RawOrigin::Signed(ALICE_ACCOUNT_ID))
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn set_initial_invitation_count_succeeds() {
    build_test_externalities().execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        SetInitialInvitationCountFixture::default().call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(Event::<Test>::InitialInvitationCountUpdated(
            DEFAULT_INITIAL_INVITATION_COUNT,
        ));
    });
}

#[test]
fn set_initial_invitation_count_fails_with_invalid_origin() {
    build_test_externalities().execute_with(|| {
        SetInitialInvitationCountFixture::default()
            .with_origin(RawOrigin::Signed(ALICE_ACCOUNT_ID))
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn add_staking_account_candidate_succeeds() {
    let initial_members = [(ALICE_MEMBER_ID, ALICE_ACCOUNT_ID)];

    build_test_externalities_with_initial_members(initial_members.to_vec()).execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        AddStakingAccountFixture::default().call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(Event::<Test>::StakingAccountAdded(
            ALICE_MEMBER_ID,
            BOB_ACCOUNT_ID,
        ));
    });
}

#[test]
fn add_staking_account_candidate_fails_with_bad_origin() {
    build_test_externalities().execute_with(|| {
        AddStakingAccountFixture::default()
            .with_origin(RawOrigin::None)
            .call_and_assert(Err(Error::<Test>::UnsignedOrigin.into()));
    });
}

#[test]
fn add_staking_account_candidate_fails_with_invalid_member_id() {
    build_test_externalities().execute_with(|| {
        let initial_balance = DefaultMembershipPrice::get();
        set_alice_free_balance(initial_balance);

        assert_ok!(buy_default_membership_as_alice());
        let invalid_member_id = 222;

        AddStakingAccountFixture::default()
            .with_member_id(invalid_member_id)
            .call_and_assert(Err(Error::<Test>::MemberProfileNotFound.into()));
    });
}

#[test]
fn add_staking_account_candidate_fails_with_duplicated_staking_account_id() {
    let initial_members = [(ALICE_MEMBER_ID, ALICE_ACCOUNT_ID)];

    build_test_externalities_with_initial_members(initial_members.to_vec()).execute_with(|| {
        AddStakingAccountFixture::default().call_and_assert(Ok(()));

        AddStakingAccountFixture::default()
            .call_and_assert(Err(Error::<Test>::StakingAccountExists.into()));
    });
}

#[test]
fn add_staking_account_candidate_fails_with_exceeding_account_limit() {
    let initial_members = [(ALICE_MEMBER_ID, ALICE_ACCOUNT_ID)];

    build_test_externalities_with_initial_members(initial_members.to_vec()).execute_with(|| {
        AddStakingAccountFixture::default().call_and_assert(Ok(()));

        AddStakingAccountFixture::default()
            .with_staking_account_id(ALICE_ACCOUNT_ID)
            .call_and_assert(Err(
                Error::<Test>::MaximumStakingAccountNumberExceeded.into()
            ));
    });
}

#[test]
fn remove_staking_account_succeeds() {
    let initial_members = [(ALICE_MEMBER_ID, ALICE_ACCOUNT_ID)];

    build_test_externalities_with_initial_members(initial_members.to_vec()).execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        AddStakingAccountFixture::default().call_and_assert(Ok(()));

        RemoveStakingAccountFixture::default().call_and_assert(Ok(()));

        EventFixture::assert_last_crate_event(Event::<Test>::StakingAccountRemoved(
            ALICE_MEMBER_ID,
            BOB_ACCOUNT_ID,
        ));
    });
}

#[test]
fn remove_staking_account_fails_with_bad_origin() {
    build_test_externalities().execute_with(|| {
        RemoveStakingAccountFixture::default()
            .with_origin(RawOrigin::None)
            .call_and_assert(Err(Error::<Test>::UnsignedOrigin.into()));
    });
}

#[test]
fn remove_staking_account_fails_with_invalid_member_id() {
    build_test_externalities().execute_with(|| {
        let initial_balance = DefaultMembershipPrice::get();
        set_alice_free_balance(initial_balance);

        assert_ok!(buy_default_membership_as_alice());
        let invalid_member_id = 222;

        RemoveStakingAccountFixture::default()
            .with_member_id(invalid_member_id)
            .call_and_assert(Err(Error::<Test>::MemberProfileNotFound.into()));
    });
}

#[test]
fn remove_staking_account_candidate_fails_with_missing_staking_account_id() {
    let initial_members = [(ALICE_MEMBER_ID, ALICE_ACCOUNT_ID)];

    build_test_externalities_with_initial_members(initial_members.to_vec()).execute_with(|| {
        RemoveStakingAccountFixture::default()
            .call_and_assert(Err(Error::<Test>::StakingAccountDoesntExist.into()));
    });
}

#[test]
fn confirm_staking_account_succeeds() {
    let initial_members = [(ALICE_MEMBER_ID, ALICE_ACCOUNT_ID)];

    build_test_externalities_with_initial_members(initial_members.to_vec()).execute_with(|| {
        let starting_block = 1;
        run_to_block(starting_block);

        AddStakingAccountFixture::default().call_and_assert(Ok(()));

        ConfirmStakingAccountFixture::default().call_and_assert(Ok(()));

        let membership = Membership::membership(ALICE_MEMBER_ID);
        assert!(membership.staking_account_ids.get(&BOB_ACCOUNT_ID).unwrap());

        EventFixture::assert_last_crate_event(Event::<Test>::StakingAccountConfirmed(
            ALICE_MEMBER_ID,
            BOB_ACCOUNT_ID,
        ));
    });
}

#[test]
fn confirm_staking_account_fails_with_bad_origin() {
    build_test_externalities().execute_with(|| {
        ConfirmStakingAccountFixture::default()
            .with_origin(RawOrigin::None)
            .call_and_assert(Err(DispatchError::BadOrigin));
    });
}

#[test]
fn confirm_staking_account_fails_with_invalid_member_id() {
    build_test_externalities().execute_with(|| {
        let initial_balance = DefaultMembershipPrice::get();
        set_alice_free_balance(initial_balance);

        assert_ok!(buy_default_membership_as_alice());
        let invalid_member_id = 222;

        ConfirmStakingAccountFixture::default()
            .with_member_id(invalid_member_id)
            .call_and_assert(Err(Error::<Test>::MemberProfileNotFound.into()));
    });
}

#[test]
fn confirm_staking_account_candidate_fails_with_missing_staking_account_id() {
    let initial_members = [(ALICE_MEMBER_ID, ALICE_ACCOUNT_ID)];

    build_test_externalities_with_initial_members(initial_members.to_vec()).execute_with(|| {
        ConfirmStakingAccountFixture::default()
            .call_and_assert(Err(Error::<Test>::StakingAccountDoesntExist.into()));
    });
}

#[test]
fn is_member_staking_account_works() {
    let initial_members = [(ALICE_MEMBER_ID, ALICE_ACCOUNT_ID)];

    build_test_externalities_with_initial_members(initial_members.to_vec()).execute_with(|| {
        // Before adding candidate should be false.
        assert_eq!(
            Membership::is_member_staking_account(&ALICE_MEMBER_ID, &BOB_ACCOUNT_ID),
            false
        );
        AddStakingAccountFixture::default().call_and_assert(Ok(()));

        // After adding but before confirmation of the candidate should be false.
        assert_eq!(
            Membership::is_member_staking_account(&ALICE_MEMBER_ID, &BOB_ACCOUNT_ID),
            false
        );
        ConfirmStakingAccountFixture::default().call_and_assert(Ok(()));

        // After confirmation of the candidate should be true.
        assert_eq!(
            Membership::is_member_staking_account(&ALICE_MEMBER_ID, &BOB_ACCOUNT_ID),
            true
        );

        // After removing of the staking account should be false.
        RemoveStakingAccountFixture::default().call_and_assert(Ok(()));
        assert_eq!(
            Membership::is_member_staking_account(&ALICE_MEMBER_ID, &BOB_ACCOUNT_ID),
            false
        );
    });
}
