//! Proposals integration tests - with stake, membership, governance modules.

#![cfg(test)]

mod working_group_proposals;

use crate::tests::run_to_block;
use crate::{ProposalCancellationFee, Runtime};
use codec::Encode;
use proposals_codex::{GeneralProposalParameters, ProposalDetails};
use proposals_engine::{
    ApprovedProposalDecision, Proposal, ProposalCreationParameters, ProposalParameters,
    ProposalStatus, VoteKind, VotersParameters, VotingResults,
};

use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::traits::Currency;
use frame_support::{StorageMap, StorageValue};
use frame_system::RawOrigin;
use sp_runtime::AccountId32;

use super::{
    increase_total_balance_issuance_using_account_id, initial_test_ext, insert_member,
    set_staking_account,
};

use crate::tests::elect_council;
use crate::CouncilManager;

pub type Balances = pallet_balances::Module<Runtime>;
pub type System = frame_system::Module<Runtime>;
pub type ProposalsEngine = proposals_engine::Module<Runtime>;
pub type ProposalCodex = proposals_codex::Module<Runtime>;

fn setup_members(count: u8) {
    for i in 0..count {
        let account_id: [u8; 32] = [i; 32];
        let account_id_converted: AccountId32 = account_id.clone().into();
        insert_member(account_id_converted.clone());
        set_staking_account(account_id_converted, i as u64);
    }
}

// Max Council size is 3
fn setup_council(cycle_id: u64) {
    let councilor0 = AccountId32::default();
    let councilor1: [u8; 32] = [1; 32];
    let councilor2: [u8; 32] = [2; 32];
    elect_council(
        vec![councilor0, councilor1.into(), councilor2.into()],
        cycle_id,
    );
}

struct VoteGenerator {
    proposal_id: u32,
    current_account_id: AccountId32,
    current_account_id_seed: u8,
    current_voter_id: u64,
    pub auto_increment_voter_id: bool,
}

impl VoteGenerator {
    fn new(proposal_id: u32) -> Self {
        VoteGenerator {
            proposal_id,
            current_voter_id: 0,
            current_account_id_seed: 0,
            current_account_id: AccountId32::default(),
            auto_increment_voter_id: true,
        }
    }
    fn vote_and_assert_ok(&mut self, vote_kind: VoteKind) {
        self.vote_and_assert(vote_kind, Ok(()));
    }

    fn vote_and_assert(&mut self, vote_kind: VoteKind, expected_result: DispatchResult) {
        assert_eq!(self.vote(vote_kind.clone()), expected_result);
    }

    fn vote(&mut self, vote_kind: VoteKind) -> DispatchResult {
        let vote_result = ProposalsEngine::vote(
            frame_system::RawOrigin::Signed(self.current_account_id.clone()).into(),
            self.current_voter_id,
            self.proposal_id,
            vote_kind,
            Vec::new(),
        );

        if self.auto_increment_voter_id {
            self.current_account_id_seed += 1;
            self.current_voter_id += 1;
            let account_id: [u8; 32] = [self.current_account_id_seed; 32];
            self.current_account_id = account_id.into();
        }

        vote_result
    }
}

#[derive(Clone)]
struct DummyProposalFixture {
    parameters: ProposalParameters<u32, u128>,
    account_id: AccountId32,
    proposer_id: u64,
    proposal_code: Vec<u8>,
    title: Vec<u8>,
    description: Vec<u8>,
    staking_account_id: Option<AccountId32>,
    exact_execution_block: Option<u32>,
}

impl Default for DummyProposalFixture {
    fn default() -> Self {
        let title = b"title".to_vec();
        let description = b"description".to_vec();
        let dummy_proposal =
            proposals_codex::Call::<Runtime>::execute_signal_proposal(b"signal".to_vec());

        DummyProposalFixture {
            parameters: ProposalParameters {
                voting_period: 3,
                approval_quorum_percentage: 60,
                approval_threshold_percentage: 60,
                slashing_quorum_percentage: 60,
                slashing_threshold_percentage: 60,
                grace_period: 0,
                required_stake: None,
                constitutionality: 1,
            },
            account_id: <Runtime as frame_system::Trait>::AccountId::default(),
            proposer_id: 0,
            proposal_code: dummy_proposal.encode(),
            title,
            description,
            staking_account_id: None,
            exact_execution_block: None,
        }
    }
}

impl DummyProposalFixture {
    fn with_parameters(self, parameters: ProposalParameters<u32, u128>) -> Self {
        DummyProposalFixture { parameters, ..self }
    }

    fn with_constitutionality(&self, constitutionality: u32) -> Self {
        DummyProposalFixture {
            parameters: ProposalParameters {
                constitutionality,
                ..self.parameters
            },
            ..self.clone()
        }
    }

    fn with_account_id(self, account_id: AccountId32) -> Self {
        DummyProposalFixture { account_id, ..self }
    }

    fn with_voting_period(self, voting_period: u32) -> Self {
        DummyProposalFixture {
            parameters: ProposalParameters {
                voting_period,
                ..self.parameters
            },
            ..self
        }
    }

    fn with_stake(self, account_id: AccountId32) -> Self {
        DummyProposalFixture {
            staking_account_id: Some(account_id),
            ..self
        }
    }

    fn with_proposer(self, proposer_id: u64) -> Self {
        DummyProposalFixture {
            proposer_id,
            ..self
        }
    }

    fn create_proposal_and_assert(self, result: Result<u32, DispatchError>) -> Option<u32> {
        let proposal_id_result = ProposalsEngine::create_proposal(ProposalCreationParameters {
            account_id: self.account_id,
            proposer_id: self.proposer_id,
            proposal_parameters: self.parameters,
            title: self.title,
            description: self.description,
            staking_account_id: self.staking_account_id,
            encoded_dispatchable_call_code: self.proposal_code,
            exact_execution_block: self.exact_execution_block,
        });

        assert_eq!(proposal_id_result, result);

        proposal_id_result.ok()
    }
}

struct CancelProposalFixture {
    origin: RawOrigin<AccountId32>,
    proposal_id: u32,
    proposer_id: u64,
}

impl CancelProposalFixture {
    fn new(proposal_id: u32) -> Self {
        let account_id = <Runtime as frame_system::Trait>::AccountId::default();
        CancelProposalFixture {
            proposal_id,
            origin: RawOrigin::Signed(account_id),
            proposer_id: 0,
        }
    }

    fn with_proposer(self, proposer_id: u64) -> Self {
        CancelProposalFixture {
            proposer_id,
            ..self
        }
    }

    fn cancel_and_assert(self, expected_result: DispatchResult) {
        assert_eq!(
            ProposalsEngine::cancel_proposal(
                self.origin.into(),
                self.proposer_id,
                self.proposal_id
            ),
            expected_result
        );
    }
}

/// Main purpose of this integration test: check balance of the member on proposal finalization (cancellation)
/// It tests StakingEventsHandler integration. Also, membership module is tested during the proposal creation (MemberOriginValidator).
#[test]
fn proposal_cancellation_with_slashes_with_balance_checks_succeeds() {
    initial_test_ext().execute_with(|| {
        let account_id = <Runtime as frame_system::Trait>::AccountId::default();

        setup_members(2);
        let member_id = 0; // newly created member_id

        let stake_amount = 20000u128;
        let parameters = ProposalParameters {
            voting_period: 3,
            approval_quorum_percentage: 50,
            approval_threshold_percentage: 60,
            slashing_quorum_percentage: 60,
            slashing_threshold_percentage: 60,
            grace_period: 5,
            required_stake: Some(stake_amount),
            constitutionality: 1,
        };
        let dummy_proposal = DummyProposalFixture::default()
            .with_parameters(parameters)
            .with_account_id(account_id.clone())
            .with_stake(account_id.clone())
            .with_proposer(member_id);

        let account_balance = 500000;
        let _imbalance = Balances::deposit_creating(&account_id, account_balance);

        assert_eq!(Balances::usable_balance(&account_id), account_balance);

        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();
        assert_eq!(
            Balances::usable_balance(&account_id),
            account_balance - stake_amount
        );

        let proposal = ProposalsEngine::proposals(proposal_id);

        let expected_proposal = Proposal {
            parameters,
            proposer_id: member_id,
            activated_at: 0,
            status: ProposalStatus::Active,
            voting_results: VotingResults::default(),
            exact_execution_block: None,
            nr_of_council_confirmations: 0,
            staking_account_id: Some(account_id.clone()),
        };

        assert_eq!(proposal, expected_proposal);

        let cancel_proposal_fixture =
            CancelProposalFixture::new(proposal_id).with_proposer(member_id);

        cancel_proposal_fixture.cancel_and_assert(Ok(()));

        let cancellation_fee = ProposalCancellationFee::get() as u128;
        assert_eq!(
            Balances::usable_balance(&account_id),
            account_balance - cancellation_fee
        );
    });
}

#[test]
fn proposal_reset_succeeds() {
    initial_test_ext().execute_with(|| {
        setup_members(4);
        setup_council(0);
        // create proposal
        let dummy_proposal = DummyProposalFixture::default().with_voting_period(100);
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        // create some votes
        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);
        vote_generator.vote_and_assert_ok(VoteKind::Slash);

        // check
        let proposal = ProposalsEngine::proposals(proposal_id);
        assert_eq!(
            proposal.voting_results,
            VotingResults {
                abstentions: 1,
                approvals: 0,
                rejections: 1,
                slashes: 1,
            }
        );

        // Ensure council was elected
        assert_eq!(CouncilManager::<Runtime>::total_voters_count(), 3);

        // Check for votes.
        assert_eq!(
            ProposalsEngine::vote_by_proposal_by_voter(proposal_id, 0),
            VoteKind::Reject
        );
        assert_eq!(
            ProposalsEngine::vote_by_proposal_by_voter(proposal_id, 1),
            VoteKind::Abstain
        );
        assert_eq!(
            ProposalsEngine::vote_by_proposal_by_voter(proposal_id, 2),
            VoteKind::Slash
        );

        // Check proposals CouncilElected hook just trigger the election hook (empty council).
        //<Runtime as governance::election::Trait>::CouncilElected::council_elected(Vec::new(), 10);

        end_idle_period();
        setup_council(1);

        let updated_proposal = ProposalsEngine::proposals(proposal_id);

        assert_eq!(
            updated_proposal.voting_results,
            VotingResults {
                abstentions: 0,
                approvals: 0,
                rejections: 0,
                slashes: 0,
            }
        );

        // No votes could survive cleaning: should be default value.
        assert_eq!(
            ProposalsEngine::vote_by_proposal_by_voter(proposal_id, 2),
            VoteKind::default()
        );

        // Check council CouncilElected hook. It should set current council.
        assert_eq!(CouncilManager::<Runtime>::total_voters_count(), 3);
    });
}

// Ends council idle period
// Preconditions: currently in idle period, idle period started in currnet block
fn end_idle_period() {
    let current_block = System::block_number();
    let idle_period_duration = <Runtime as council::Trait>::IdlePeriodDuration::get();
    run_to_block(current_block + idle_period_duration);
}

struct CodexProposalTestFixture<SuccessfulCall>
where
    SuccessfulCall: Fn() -> DispatchResult,
{
    successful_call: SuccessfulCall,
    member_id: u64,
    setup_environment: bool,
    proposal_id: u32,
}

impl<SuccessfulCall> CodexProposalTestFixture<SuccessfulCall>
where
    SuccessfulCall: Fn() -> DispatchResult,
{
    fn default_for_call(call: SuccessfulCall) -> Self {
        Self {
            successful_call: call,
            member_id: 1,
            setup_environment: true,
            proposal_id: 1,
        }
    }

    fn disable_setup_enviroment(self) -> Self {
        Self {
            setup_environment: false,
            ..self
        }
    }
    fn with_setup_enviroment(self, setup_environment: bool) -> Self {
        Self {
            setup_environment,
            ..self
        }
    }

    fn with_member_id(self, member_id: u64) -> Self {
        Self { member_id, ..self }
    }

    fn with_expected_proposal_id(self, expected_proposal_id: u32) -> Self {
        Self {
            proposal_id: expected_proposal_id,
            ..self
        }
    }
}

impl<SuccessfulCall> CodexProposalTestFixture<SuccessfulCall>
where
    SuccessfulCall: Fn() -> DispatchResult,
{
    fn call_extrinsic_and_assert(&self) {
        let account_id: [u8; 32] = [self.member_id as u8; 32];

        if self.setup_environment {
            setup_members(15);
            setup_council(0);

            increase_total_balance_issuance_using_account_id(account_id.clone().into(), 1_500_000);
        }

        assert_eq!((self.successful_call)(), Ok(()));

        // Council size is 3
        let mut vote_generator = VoteGenerator::new(self.proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);

        run_to_block(System::block_number() + 1);
    }
}

#[test]
fn text_proposal_execution_succeeds() {
    initial_test_ext().execute_with(|| {
        let member_id = 10;
        let account_id: [u8; 32] = [member_id; 32];

        let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
            let general_proposal_parameters = GeneralProposalParameters::<Runtime> {
                member_id: member_id.into(),
                title: b"title".to_vec(),
                description: b"body".to_vec(),
                staking_account_id: Some(account_id.into()),
                exact_execution_block: None,
            };

            ProposalCodex::create_proposal(
                RawOrigin::Signed(account_id.into()).into(),
                general_proposal_parameters,
                ProposalDetails::Signal(b"signal".to_vec()),
            )
        })
        .with_member_id(member_id as u64);

        codex_extrinsic_test_fixture.call_extrinsic_and_assert();
    });
}

/* TODO: Test will be commented out until Spending proposal is changed with the new
 * FundingRequest
#[test]
fn spending_proposal_execution_succeeds() {
    initial_test_ext().execute_with(|| {
        let member_id = 10;
        let account_id: [u8; 32] = [member_id; 32];
        let new_balance = pallet_council::Balance::<Runtime>::from(5555u32);

        let target_account_id: [u8; 32] = [12; 32];

        assert!(Council::set_budget(RawOrigin::Root.into(), new_balance).is_ok());

        let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
            let general_proposal_parameters = GeneralProposalParameters::<Runtime> {
                member_id: member_id.into(),
                title: b"title".to_vec(),
                description: b"body".to_vec(),
                staking_account_id: Some(account_id.into()),
                exact_execution_block: None,
            };

            ProposalCodex::create_proposal(
                RawOrigin::Signed(account_id.clone().into()).into(),
                general_proposal_parameters,
                ProposalDetails::FundingRequest(new_balance, target_account_id.clone().into()),
            )
        })
        .with_member_id(member_id as u64);

        let converted_account_id: AccountId32 = target_account_id.clone().into();
        assert_eq!(Balances::free_balance(converted_account_id.clone()), 0);

        codex_extrinsic_test_fixture.call_extrinsic_and_assert();

        run_to_block(14410);

        assert_eq!(Balances::free_balance(converted_account_id), new_balance);
    });
}
*/

#[test]
fn set_validator_count_proposal_execution_succeeds() {
    initial_test_ext().execute_with(|| {
        let member_id = 1;
        let account_id: [u8; 32] = [member_id; 32];

        let new_validator_count = 8;
        assert_eq!(<pallet_staking::ValidatorCount>::get(), 0);

        let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
            let general_proposal_parameters = GeneralProposalParameters::<Runtime> {
                member_id: member_id.into(),
                title: b"title".to_vec(),
                description: b"body".to_vec(),
                staking_account_id: Some(account_id.into()),
                exact_execution_block: None,
            };

            ProposalCodex::create_proposal(
                RawOrigin::Signed(account_id.clone().into()).into(),
                general_proposal_parameters,
                ProposalDetails::SetMaxValidatorCount(new_validator_count),
            )
        });
        codex_extrinsic_test_fixture.call_extrinsic_and_assert();

        run_to_block(14410);

        assert_eq!(<pallet_staking::ValidatorCount>::get(), new_validator_count);
    });
}

#[test]
fn amend_constitution_proposal_execution_succeeds() {
    initial_test_ext().execute_with(|| {
        let member_id = 10;
        let account_id: [u8; 32] = [member_id; 32];

        let codex_extrinsic_test_fixture = CodexProposalTestFixture::default_for_call(|| {
            let general_proposal_parameters = GeneralProposalParameters::<Runtime> {
                member_id: member_id.into(),
                title: b"title".to_vec(),
                description: b"body".to_vec(),
                staking_account_id: Some(account_id.into()),
                exact_execution_block: None,
            };

            ProposalCodex::create_proposal(
                RawOrigin::Signed(account_id.into()).into(),
                general_proposal_parameters,
                ProposalDetails::AmendConstitution(b"Constitution text".to_vec()),
            )
        })
        .with_member_id(member_id as u64);

        codex_extrinsic_test_fixture.call_extrinsic_and_assert();
    });
}

#[test]
fn proposal_reactivation_succeeds() {
    initial_test_ext().execute_with(|| {
        setup_members(5);
        setup_council(0);

        let starting_block = System::block_number();
        // create proposal
        let dummy_proposal = DummyProposalFixture::default()
            .with_voting_period(100)
            .with_constitutionality(2);
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(1)).unwrap();

        // create some votes
        // Council size is 3
        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);

        run_to_block(starting_block + 2);

        // check
        let proposal = ProposalsEngine::proposals(proposal_id);
        assert_eq!(
            proposal.status,
            ProposalStatus::approved(
                ApprovedProposalDecision::PendingConstitutionality,
                starting_block
            )
        );

        // Ensure council was elected
        assert_eq!(CouncilManager::<Runtime>::total_voters_count(), 3);

        end_idle_period();
        setup_council(1);

        run_to_block(10);

        let updated_proposal = ProposalsEngine::proposals(proposal_id);

        assert_eq!(updated_proposal.status, ProposalStatus::Active);

        // Check council CouncilElected hook. It should set current council. And we elected single councilor.
        assert_eq!(CouncilManager::<Runtime>::total_voters_count(), 3);
    });
}
