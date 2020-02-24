mod mock;

use crate::*;
use mock::*;

use codec::Encode;
use rstd::rc::Rc;
use runtime_primitives::traits::{OnFinalize, OnInitialize};
use srml_support::{dispatch, StorageMap, StorageValue};
use system::RawOrigin;
use system::{EventRecord, Phase};

use srml_support::traits::Currency;

struct ProposalParametersFixture {
    parameters: ProposalParameters<u64, u64>,
}

impl ProposalParametersFixture {
    fn with_required_stake(&self, required_stake: BalanceOf<Test>) -> Self {
        ProposalParametersFixture {
            parameters: ProposalParameters {
                required_stake: Some(required_stake),
                ..self.parameters
            },
        }
    }
    fn with_grace_period(&self, grace_period: u64) -> Self {
        ProposalParametersFixture {
            parameters: ProposalParameters {
                grace_period,
                ..self.parameters
            },
        }
    }

    fn params(&self) -> ProposalParameters<u64, u64> {
        self.parameters.clone()
    }
}

impl Default for ProposalParametersFixture {
    fn default() -> Self {
        ProposalParametersFixture {
            parameters: ProposalParameters {
                voting_period: 3,
                approval_quorum_percentage: 60,
                approval_threshold_percentage: 60,
                slashing_quorum_percentage: 60,
                slashing_threshold_percentage: 60,
                grace_period: 0,
                required_stake: None,
            },
        }
    }
}

#[derive(Clone)]
struct DummyProposalFixture {
    parameters: ProposalParameters<u64, u64>,
    origin: RawOrigin<u64>,
    proposal_code: Vec<u8>,
    title: Vec<u8>,
    body: Vec<u8>,
    stake_balance: Option<BalanceOf<Test>>,
}

impl Default for DummyProposalFixture {
    fn default() -> Self {
        let dummy_proposal = crate::Call::<Test>::text_proposal(
            b"title".to_vec(),
            b"body".to_vec(),
            b"text".to_vec(),
        );

        DummyProposalFixture {
            parameters: ProposalParameters {
                voting_period: 3,
                approval_quorum_percentage: 60,
                approval_threshold_percentage: 60,
                slashing_quorum_percentage: 60,
                slashing_threshold_percentage: 60,
                grace_period: 0,
                required_stake: None,
            },
            origin: RawOrigin::Signed(1),
            proposal_code: dummy_proposal.encode(),
            title: b"title".to_vec(),
            body: b"body".to_vec(),
            stake_balance: None,
        }
    }
}

impl DummyProposalFixture {
    fn with_title_and_body(self, title: Vec<u8>, body: Vec<u8>) -> Self {
        DummyProposalFixture {
            title,
            body,
            ..self
        }
    }

    fn with_parameters(self, parameters: ProposalParameters<u64, u64>) -> Self {
        DummyProposalFixture { parameters, ..self }
    }

    fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        DummyProposalFixture { origin, ..self }
    }

    fn with_stake(self, stake_balance: BalanceOf<Test>) -> Self {
        DummyProposalFixture {
            stake_balance: Some(stake_balance),
            ..self
        }
    }

    fn with_proposal_type_and_code(self, proposal_code: Vec<u8>) -> Self {
        DummyProposalFixture {
            proposal_code,
            ..self
        }
    }

    fn create_proposal_and_assert(self, result: dispatch::Result) -> Option<u32> {
        assert_eq!(
            ProposalsEngine::create_proposal(
                self.origin.into(),
                self.parameters,
                self.title,
                self.body,
                self.stake_balance,
                self.proposal_code,
            ),
            result
        );

        if result.is_ok() {
            // last created proposal id equals current proposal count
            let proposal_id = <ProposalCount>::get();

            Some(proposal_id)
        } else {
            None
        }
    }
}

struct CancelProposalFixture {
    origin: RawOrigin<u64>,
    proposal_id: u32,
}

impl CancelProposalFixture {
    fn new(proposal_id: u32) -> Self {
        CancelProposalFixture {
            proposal_id,
            origin: RawOrigin::Signed(1),
        }
    }

    fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        CancelProposalFixture { origin, ..self }
    }

    fn cancel_and_assert(self, expected_result: dispatch::Result) {
        assert_eq!(
            ProposalsEngine::cancel_proposal(self.origin.into(), self.proposal_id,),
            expected_result
        );
    }
}
struct VetoProposalFixture {
    origin: RawOrigin<u64>,
    proposal_id: u32,
}

impl VetoProposalFixture {
    fn new(proposal_id: u32) -> Self {
        VetoProposalFixture {
            proposal_id,
            origin: RawOrigin::Root,
        }
    }

    fn with_origin(self, origin: RawOrigin<u64>) -> Self {
        VetoProposalFixture { origin, ..self }
    }

    fn veto_and_assert(self, expected_result: dispatch::Result) {
        assert_eq!(
            ProposalsEngine::veto_proposal(self.origin.into(), self.proposal_id,),
            expected_result
        );
    }
}

struct VoteGenerator {
    proposal_id: u32,
    current_account_id: u64,
    pub auto_increment_voter_id: bool,
}

impl VoteGenerator {
    fn new(proposal_id: u32) -> Self {
        VoteGenerator {
            proposal_id,
            current_account_id: 0,
            auto_increment_voter_id: true,
        }
    }
    fn vote_and_assert_ok(&mut self, vote_kind: VoteKind) {
        self.vote_and_assert(vote_kind, Ok(()));
    }

    fn vote_and_assert(&mut self, vote_kind: VoteKind, expected_result: dispatch::Result) {
        assert_eq!(self.vote(vote_kind.clone()), expected_result);
    }

    fn vote(&mut self, vote_kind: VoteKind) -> dispatch::Result {
        if self.auto_increment_voter_id {
            self.current_account_id += 1;
        }

        ProposalsEngine::vote(
            system::RawOrigin::Signed(self.current_account_id).into(),
            self.proposal_id,
            vote_kind,
        )
    }
}

struct EventFixture;
impl EventFixture {
    fn assert_events(expected_raw_events: Vec<RawEvent<u32, u64, u64>>) {
        let expected_events = expected_raw_events
            .iter()
            .map(|ev| EventRecord {
                phase: Phase::ApplyExtrinsic(0),
                event: TestEvent::engine(ev.clone()),
                topics: vec![],
            })
            .collect::<Vec<EventRecord<_, _>>>();

        assert_eq!(System::events(), expected_events);
    }
}

// Recommendation from Parity on testing on_finalize
// https://substrate.dev/docs/en/next/development/module/tests
fn run_to_block(n: u64) {
    while System::block_number() < n {
        <System as OnFinalize<u64>>::on_finalize(System::block_number());
        <ProposalsEngine as OnFinalize<u64>>::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        <System as OnInitialize<u64>>::on_initialize(System::block_number());
        <ProposalsEngine as OnInitialize<u64>>::on_initialize(System::block_number());
    }
}

fn run_to_block_and_finalize(n: u64) {
    run_to_block(n);
    <ProposalsEngine as OnFinalize<u64>>::on_finalize(n);
}

#[test]
fn create_dummy_proposal_succeeds() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();

        dummy_proposal.create_proposal_and_assert(Ok(()));
    });
}

#[test]
fn create_dummy_proposal_fails_with_insufficient_rights() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default().with_origin(RawOrigin::None);

        dummy_proposal.create_proposal_and_assert(Err("Invalid origin"));
    });
}

#[test]
fn vote_succeeds() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
    });
}

#[test]
fn vote_fails_with_insufficient_rights() {
    initial_test_ext().execute_with(|| {
        assert_eq!(
            ProposalsEngine::vote(system::RawOrigin::None.into(), 1, VoteKind::Approve),
            Err("Invalid origin")
        );
    });
}

#[test]
fn proposal_execution_succeeds() {
    initial_test_ext().execute_with(|| {
        let parameters_fixture = ProposalParametersFixture::default();
        let dummy_proposal =
            DummyProposalFixture::default().with_parameters(parameters_fixture.params());
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        // internal active proposal counter check
        assert_eq!(<ActiveProposalCount>::get(), 1);

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);

        run_to_block_and_finalize(1);

        let proposal = <crate::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal,
            Proposal {
                parameters: parameters_fixture.params(),
                proposer_id: 1,
                created_at: 1,
                status: ProposalStatus::approved(ApprovedProposalStatus::Executed),
                title: b"title".to_vec(),
                body: b"body".to_vec(),
                approved_at: Some(1),
                voting_results: VotingResults {
                    abstentions: 0,
                    approvals: 4,
                    rejections: 0,
                    slashes: 0,
                },
                finalized_at: Some(1),
                stake_id: None,
            }
        );

        // internal active proposal counter check
        assert_eq!(<ActiveProposalCount>::get(), 0);
    });
}

#[test]
fn proposal_execution_failed() {
    initial_test_ext().execute_with(|| {
        let parameters_fixture = ProposalParametersFixture::default();
        let faulty_proposal = FaultyExecutable;

        let dummy_proposal = DummyProposalFixture::default()
            .with_parameters(parameters_fixture.params())
            .with_proposal_type_and_code(faulty_proposal.encode());

        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);

        run_to_block_and_finalize(2);

        let proposal = <crate::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal,
            Proposal {
                parameters: parameters_fixture.params(),
                proposer_id: 1,
                created_at: 1,
                status: ProposalStatus::approved(ApprovedProposalStatus::failed_execution(
                    "ExecutionFailed"
                )),
                title: b"title".to_vec(),
                body: b"body".to_vec(),
                approved_at: Some(1),
                voting_results: VotingResults {
                    abstentions: 0,
                    approvals: 4,
                    rejections: 0,
                    slashes: 0,
                },
                finalized_at: Some(1),
                stake_id: None,
            }
        )
    });
}

#[test]
fn voting_results_calculation_succeeds() {
    initial_test_ext().execute_with(|| {
        let parameters = ProposalParameters {
            voting_period: 3,
            approval_quorum_percentage: 50,
            approval_threshold_percentage: 50,
            slashing_quorum_percentage: 60,
            slashing_threshold_percentage: 60,
            grace_period: 0,
            required_stake: None,
        };
        let dummy_proposal = DummyProposalFixture::default().with_parameters(parameters);
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);

        run_to_block_and_finalize(2);

        let proposal = <crate::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal.voting_results,
            VotingResults {
                abstentions: 1,
                approvals: 2,
                rejections: 1,
                slashes: 0,
            }
        )
    });
}

#[test]
fn rejected_voting_results_and_remove_proposal_id_from_active_succeeds() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);

        assert!(<ActiveProposalIds<Test>>::exists(proposal_id));

        run_to_block_and_finalize(2);

        let proposal = <Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal.voting_results,
            VotingResults {
                abstentions: 2,
                approvals: 0,
                rejections: 2,
                slashes: 0,
            }
        );

        assert_eq!(
            proposal.status,
            ProposalStatus::finalized(ProposalDecisionStatus::Rejected),
        );
        assert!(!<ActiveProposalIds<Test>>::exists(proposal_id));
    });
}

#[test]
fn create_proposal_fails_with_invalid_body_or_title() {
    initial_test_ext().execute_with(|| {
        let mut dummy_proposal =
            DummyProposalFixture::default().with_title_and_body(Vec::new(), b"body".to_vec());
        dummy_proposal.create_proposal_and_assert(Err("Proposal cannot have an empty title"));

        dummy_proposal =
            DummyProposalFixture::default().with_title_and_body(b"title".to_vec(), Vec::new());
        dummy_proposal.create_proposal_and_assert(Err("Proposal cannot have an empty body"));

        let too_long_title = vec![0; 200];
        dummy_proposal =
            DummyProposalFixture::default().with_title_and_body(too_long_title, b"body".to_vec());
        dummy_proposal.create_proposal_and_assert(Err("Title is too long"));

        let too_long_body = vec![0; 11000];
        dummy_proposal =
            DummyProposalFixture::default().with_title_and_body(b"title".to_vec(), too_long_body);
        dummy_proposal.create_proposal_and_assert(Err("Body is too long"));
    });
}

#[test]
fn vote_fails_with_expired_voting_period() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        run_to_block_and_finalize(6);

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert(VoteKind::Approve, Err("Proposal is finalized already"));
    });
}

#[test]
fn vote_fails_with_not_active_proposal() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);

        run_to_block_and_finalize(2);

        let mut vote_generator_to_fail = VoteGenerator::new(proposal_id);
        vote_generator_to_fail
            .vote_and_assert(VoteKind::Approve, Err("Proposal is finalized already"));
    });
}

#[test]
fn vote_fails_with_absent_proposal() {
    initial_test_ext().execute_with(|| {
        let mut vote_generator = VoteGenerator::new(2);
        vote_generator.vote_and_assert(VoteKind::Approve, Err("This proposal does not exist"));
    });
}

#[test]
fn vote_fails_on_double_voting() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.auto_increment_voter_id = false;

        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert(
            VoteKind::Approve,
            Err("You have already voted on this proposal"),
        );
    });
}

#[test]
fn cancel_proposal_succeeds() {
    initial_test_ext().execute_with(|| {
        let parameters_fixture = ProposalParametersFixture::default();
        let dummy_proposal =
            DummyProposalFixture::default().with_parameters(parameters_fixture.params());
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        let cancel_proposal = CancelProposalFixture::new(proposal_id);
        cancel_proposal.cancel_and_assert(Ok(()));

        let proposal = <crate::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal,
            Proposal {
                parameters: parameters_fixture.params(),
                proposer_id: 1,
                created_at: 1,
                status: ProposalStatus::finalized(ProposalDecisionStatus::Canceled),
                title: b"title".to_vec(),
                body: b"body".to_vec(),
                approved_at: None,
                voting_results: VotingResults::default(),
                finalized_at: Some(1),
                stake_id: None,
            }
        )
    });
}

#[test]
fn cancel_proposal_fails_with_not_active_proposal() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        run_to_block_and_finalize(6);

        let cancel_proposal = CancelProposalFixture::new(proposal_id);
        cancel_proposal.cancel_and_assert(Err("Proposal is finalized already"));
    });
}

#[test]
fn cancel_proposal_fails_with_not_existing_proposal() {
    initial_test_ext().execute_with(|| {
        let cancel_proposal = CancelProposalFixture::new(2);
        cancel_proposal.cancel_and_assert(Err("This proposal does not exist"));
    });
}

#[test]
fn cancel_proposal_fails_with_insufficient_rights() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        let cancel_proposal =
            CancelProposalFixture::new(proposal_id).with_origin(RawOrigin::Signed(2));
        cancel_proposal.cancel_and_assert(Err("You do not own this proposal"));
    });
}

#[test]
fn veto_proposal_succeeds() {
    initial_test_ext().execute_with(|| {
        // internal active proposal counter check
        assert_eq!(<ActiveProposalCount>::get(), 0);

        let parameters_fixture = ProposalParametersFixture::default();
        let dummy_proposal =
            DummyProposalFixture::default().with_parameters(parameters_fixture.params());
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        // internal active proposal counter check
        assert_eq!(<ActiveProposalCount>::get(), 1);

        let veto_proposal = VetoProposalFixture::new(proposal_id);
        veto_proposal.veto_and_assert(Ok(()));

        let proposal = <crate::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal,
            Proposal {
                parameters: parameters_fixture.params(),
                proposer_id: 1,
                created_at: 1,
                status: ProposalStatus::finalized(ProposalDecisionStatus::Vetoed),
                title: b"title".to_vec(),
                body: b"body".to_vec(),
                approved_at: None,
                voting_results: VotingResults::default(),
                finalized_at: Some(1),
                stake_id: None,
            }
        );

        // internal active proposal counter check
        assert_eq!(<ActiveProposalCount>::get(), 0);
    });
}

#[test]
fn veto_proposal_fails_with_not_active_proposal() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        run_to_block_and_finalize(6);

        let veto_proposal = VetoProposalFixture::new(proposal_id);
        veto_proposal.veto_and_assert(Err("Proposal is finalized already"));
    });
}

#[test]
fn veto_proposal_fails_with_not_existing_proposal() {
    initial_test_ext().execute_with(|| {
        let veto_proposal = VetoProposalFixture::new(2);
        veto_proposal.veto_and_assert(Err("This proposal does not exist"));
    });
}

#[test]
fn veto_proposal_fails_with_insufficient_rights() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        let veto_proposal = VetoProposalFixture::new(proposal_id).with_origin(RawOrigin::Signed(2));
        veto_proposal.veto_and_assert(Err("RequireRootOrigin"));
    });
}

#[test]
fn create_proposal_event_emitted() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        dummy_proposal.create_proposal_and_assert(Ok(()));

        EventFixture::assert_events(vec![RawEvent::ProposalCreated(1, 1)]);
    });
}

#[test]
fn veto_proposal_event_emitted() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        let veto_proposal = VetoProposalFixture::new(proposal_id);
        veto_proposal.veto_and_assert(Ok(()));

        EventFixture::assert_events(vec![
            RawEvent::ProposalCreated(1, 1),
            RawEvent::ProposalStatusUpdated(
                1,
                ProposalStatus::finalized(ProposalDecisionStatus::Vetoed),
            ),
        ]);
    });
}

#[test]
fn cancel_proposal_event_emitted() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        let cancel_proposal = CancelProposalFixture::new(proposal_id);
        cancel_proposal.cancel_and_assert(Ok(()));

        EventFixture::assert_events(vec![
            RawEvent::ProposalCreated(1, 1),
            RawEvent::ProposalStatusUpdated(
                1,
                ProposalStatus::Finalized(FinalizationStatus {
                    proposal_status: ProposalDecisionStatus::Canceled,
                    finalization_error: None,
                }),
            ),
        ]);
    });
}

#[test]
fn vote_proposal_event_emitted() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);

        EventFixture::assert_events(vec![
            RawEvent::ProposalCreated(1, 1),
            RawEvent::Voted(1, 1, VoteKind::Approve),
        ]);
    });
}

#[test]
fn create_proposal_and_expire_it() {
    initial_test_ext().execute_with(|| {
        let parameters_fixture = ProposalParametersFixture::default();
        let dummy_proposal =
            DummyProposalFixture::default().with_parameters(parameters_fixture.params());
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        run_to_block_and_finalize(8);

        let proposal = <crate::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal,
            Proposal {
                parameters: parameters_fixture.params(),
                proposer_id: 1,
                created_at: 1,
                status: ProposalStatus::finalized(ProposalDecisionStatus::Expired),
                title: b"title".to_vec(),
                body: b"body".to_vec(),
                approved_at: None,
                voting_results: VotingResults::default(),
                finalized_at: Some(4),
                stake_id: None,
            }
        )
    });
}

#[test]
fn proposal_execution_postponed_because_of_grace_period() {
    initial_test_ext().execute_with(|| {
        let parameters_fixture = ProposalParametersFixture::default().with_grace_period(2);
        let dummy_proposal =
            DummyProposalFixture::default().with_parameters(parameters_fixture.params());

        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);

        run_to_block_and_finalize(1);
        run_to_block_and_finalize(2);

        // check internal cache for proposal_id presense
        assert!(<PendingExecutionProposalIds<Test>>::enumerate()
            .find(|(x, _)| *x == proposal_id)
            .is_some());

        let proposal = <crate::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal,
            Proposal {
                parameters: parameters_fixture.params(),
                proposer_id: 1,
                created_at: 1,
                status: ProposalStatus::approved(ApprovedProposalStatus::PendingExecution),
                title: b"title".to_vec(),
                body: b"body".to_vec(),
                approved_at: Some(1),
                finalized_at: Some(1),
                voting_results: VotingResults {
                    abstentions: 0,
                    approvals: 4,
                    rejections: 0,
                    slashes: 0,
                },
                stake_id: None,
            }
        );
    });
}

#[test]
fn proposal_execution_succeeds_after_the_grace_period() {
    initial_test_ext().execute_with(|| {
        let parameters_fixture = ProposalParametersFixture::default().with_grace_period(1);
        let dummy_proposal =
            DummyProposalFixture::default().with_parameters(parameters_fixture.params());
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);
        vote_generator.vote_and_assert_ok(VoteKind::Approve);

        run_to_block_and_finalize(1);

        // check internal cache for proposal_id presence
        assert!(<PendingExecutionProposalIds<Test>>::enumerate()
            .find(|(x, _)| *x == proposal_id)
            .is_some());

        let mut proposal = <crate::Proposals<Test>>::get(proposal_id);

        let mut expected_proposal = Proposal {
            parameters: parameters_fixture.params(),
            proposer_id: 1,
            created_at: 1,
            status: ProposalStatus::approved(ApprovedProposalStatus::PendingExecution),
            title: b"title".to_vec(),
            body: b"body".to_vec(),
            approved_at: Some(1),
            finalized_at: Some(1),
            voting_results: VotingResults {
                abstentions: 0,
                approvals: 4,
                rejections: 0,
                slashes: 0,
            },
            stake_id: None,
        };

        assert_eq!(proposal, expected_proposal);

        run_to_block_and_finalize(2);

        proposal = <crate::Proposals<Test>>::get(proposal_id);

        expected_proposal.status = ProposalStatus::approved(ApprovedProposalStatus::Executed);

        assert_eq!(proposal, expected_proposal);

        // check internal cache for proposal_id absense
        assert!(<PendingExecutionProposalIds<Test>>::enumerate()
            .find(|(x, _)| *x == proposal_id)
            .is_none());
    });
}

#[test]
fn create_proposal_fails_on_exceeding_max_active_proposals_count() {
    initial_test_ext().execute_with(|| {
        for idx in 0..100 {
            let dummy_proposal = DummyProposalFixture::default();
            dummy_proposal.create_proposal_and_assert(Ok(()));
            // internal active proposal counter check
            assert_eq!(<ActiveProposalCount>::get(), idx + 1);
        }

        let dummy_proposal = DummyProposalFixture::default();
        dummy_proposal.create_proposal_and_assert(Err("Max active proposals number exceeded"));
        // internal active proposal counter check
        assert_eq!(<ActiveProposalCount>::get(), 100);
    });
}

#[test]
fn voting_internal_cache_exists_after_proposal_finalization() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        dummy_proposal.create_proposal_and_assert(Ok(()));

        // last created proposal id equals current proposal count
        let proposal_id = <ProposalCount>::get();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);
        vote_generator.vote_and_assert_ok(VoteKind::Abstain);

        // cache exists
        assert!(<crate::VoteExistsByProposalByVoter<Test>>::exists(
            proposal_id,
            1
        ));

        run_to_block_and_finalize(2);

        // cache still exists and is not cleared
        assert!(<crate::VoteExistsByProposalByVoter<Test>>::exists(
            proposal_id,
            1
        ));
    });
}

#[test]
fn create_dummy_proposal_succeeds_with_stake() {
    initial_test_ext().execute_with(|| {
        let account_id = 1;

        let required_stake = 200;
        let parameters_fixture =
            ProposalParametersFixture::default().with_required_stake(required_stake);

        let dummy_proposal = DummyProposalFixture::default()
            .with_parameters(parameters_fixture.params())
            .with_origin(RawOrigin::Signed(account_id))
            .with_stake(200);

        let _imbalance = <Test as stake::Trait>::Currency::deposit_creating(&account_id, 500);

        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        let proposal = <crate::Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal,
            Proposal {
                parameters: parameters_fixture.params(),
                proposer_id: 1,
                created_at: 1,
                status: ProposalStatus::Active,
                title: b"title".to_vec(),
                body: b"body".to_vec(),
                approved_at: None,
                voting_results: VotingResults::default(),
                finalized_at: None,
                stake_id: Some(0), // valid stake_id
            }
        )
    });
}

#[test]
fn create_dummy_proposal_fail_with_stake_on_empty_account() {
    initial_test_ext().execute_with(|| {
        let account_id = 1;

        let required_stake = 200;
        let parameters_fixture =
            ProposalParametersFixture::default().with_required_stake(required_stake);
        let dummy_proposal = DummyProposalFixture::default()
            .with_parameters(parameters_fixture.params())
            .with_origin(RawOrigin::Signed(account_id))
            .with_stake(required_stake);

        dummy_proposal.create_proposal_and_assert(Err("too few free funds in account"));
    });
}

#[test]
fn create_proposal_fais_with_invalid_stake_parameters() {
    initial_test_ext().execute_with(|| {
        let parameters_fixture = ProposalParametersFixture::default();

        let mut dummy_proposal = DummyProposalFixture::default()
            .with_parameters(parameters_fixture.params())
            .with_stake(200);

        dummy_proposal.create_proposal_and_assert(Err("Stake should be empty for this proposal"));

        let parameters_fixture_stake_200 = parameters_fixture.with_required_stake(200);
        dummy_proposal =
            DummyProposalFixture::default().with_parameters(parameters_fixture_stake_200.params());

        dummy_proposal.create_proposal_and_assert(Err("Stake cannot be empty with this proposal"));

        let parameters_fixture_stake_300 = parameters_fixture.with_required_stake(300);
        dummy_proposal = DummyProposalFixture::default()
            .with_parameters(parameters_fixture_stake_300.params())
            .with_stake(200);

        dummy_proposal
            .create_proposal_and_assert(Err("Stake differs from the proposal requirements"));
    });
}
/* TODO: restore after the https://github.com/Joystream/substrate-runtime-joystream/issues/161
// issue will be fixed: "Fix stake module and allow slashing and unstaking in the same block."
#[test]
fn finalize_expired_proposal_and_check_stake_removing_with_balance_checks_succeeds() {
    initial_test_ext().execute_with(|| {
        let account_id = 1;

        let stake_amount = 200;
        let parameters = ProposalParameters {
            voting_period: 3,
            approval_quorum_percentage: 50,
            approval_threshold_percentage: 60,
            slashing_quorum_percentage: 60,
            slashing_threshold_percentage: 60,
            grace_period: 5,
            required_stake: Some(stake_amount),
        };
        let dummy_proposal = DummyProposalFixture::default()
            .with_parameters(parameters)
            .with_origin(RawOrigin::Signed(account_id))
            .with_stake(stake_amount);

        let account_balance = 500;
        let _imbalance =
            <Test as stake::Trait>::Currency::deposit_creating(&account_id, account_balance);

        assert_eq!(
            <Test as stake::Trait>::Currency::total_balance(&account_id),
            account_balance
        );

        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();
        assert_eq!(
            <Test as stake::Trait>::Currency::total_balance(&account_id),
            account_balance - stake_amount
        );

        let mut proposal = <crate::Proposals<Test>>::get(proposal_id);

        let mut expected_proposal = Proposal {
            proposal_type: 1,
            parameters,
            proposer_id: 1,
            created_at: 1,
            status: ProposalStatus::Active,
            title: b"title".to_vec(),
            body: b"body".to_vec(),
            approved_at: None,
            voting_results: VotingResults::default(),
            finalized_at: None,
            stake_id: Some(0), // valid stake_id
        };

        assert_eq!(proposal, expected_proposal);

        run_to_block_and_finalize(5);

        proposal = <crate::Proposals<Test>>::get(proposal_id);

        expected_proposal.stake_id = None;
        expected_proposal.finalized_at = Some(4);
        expected_proposal.status = ProposalStatus::Finalized(FinalizationStatus {
            proposal_status: ProposalDecisionStatus::Expired,
            finalization_error: None,
        });

        assert_eq!(proposal, expected_proposal);

        let rejection_fee = <RejectionFee<Test>>::get();
        assert_eq!(
            <Test as stake::Trait>::Currency::total_balance(&account_id),
            account_balance - rejection_fee
        );
    });
}
*/
#[test]
fn finalize_proposal_using_stake_mocks() {
    handle_mock(|| {
        initial_test_ext().execute_with(|| {
            let mock = {
                let mut mock = crate::types::MockStakeHandler::<Test>::new();
                mock.expect_create_stake().times(1).returning(|_, _| Ok(1));

                mock.expect_remove_stake().times(1).returning(|_| Ok(()));

                mock.expect_slash().times(1).returning(|_, _| Ok(()));

                Rc::new(mock)
            };
            set_stake_handler_impl(mock.clone());

            let account_id = 1;

            let stake_amount = 200;
            let parameters_fixture =
                ProposalParametersFixture::default().with_required_stake(stake_amount);
            let dummy_proposal = DummyProposalFixture::default()
                .with_parameters(parameters_fixture.params())
                .with_origin(RawOrigin::Signed(account_id))
                .with_stake(stake_amount);

            let _proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

            run_to_block_and_finalize(5);
        });
    });
}

#[test]
fn proposal_slashing_succeeds() {
    initial_test_ext().execute_with(|| {
        let dummy_proposal = DummyProposalFixture::default();
        let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

        let mut vote_generator = VoteGenerator::new(proposal_id);
        vote_generator.vote_and_assert_ok(VoteKind::Reject);
        vote_generator.vote_and_assert_ok(VoteKind::Slash);
        vote_generator.vote_and_assert_ok(VoteKind::Slash);
        vote_generator.vote_and_assert_ok(VoteKind::Slash);

        assert!(<ActiveProposalIds<Test>>::exists(proposal_id));

        run_to_block_and_finalize(2);

        let proposal = <Proposals<Test>>::get(proposal_id);

        assert_eq!(
            proposal.voting_results,
            VotingResults {
                abstentions: 0,
                approvals: 0,
                rejections: 1,
                slashes: 3,
            }
        );

        assert_eq!(
            proposal.status,
            ProposalStatus::Finalized(FinalizationStatus {
                proposal_status: ProposalDecisionStatus::Slashed,
                finalization_error: None,
            }),
        );
        assert!(!<ActiveProposalIds<Test>>::exists(proposal_id));
    });
}

#[test]
fn finalize_proposal_failed_using_stake_mocks() {
    handle_mock(|| {
        initial_test_ext().execute_with(|| {
            let mock = {
                let mut mock = crate::types::MockStakeHandler::<Test>::new();
                mock.expect_create_stake().times(1).returning(|_, _| Ok(1));

                mock.expect_remove_stake()
                    .times(1)
                    .returning(|_| Err("Cannot remove stake"));

                mock.expect_slash().times(1).returning(|_, _| Ok(()));

                Rc::new(mock)
            };
            set_stake_handler_impl(mock.clone());

            let account_id = 1;

            let stake_amount = 200;
            let parameters_fixture =
                ProposalParametersFixture::default().with_required_stake(stake_amount);
            let dummy_proposal = DummyProposalFixture::default()
                .with_parameters(parameters_fixture.params())
                .with_origin(RawOrigin::Signed(account_id))
                .with_stake(stake_amount);

            let proposal_id = dummy_proposal.create_proposal_and_assert(Ok(())).unwrap();

            run_to_block_and_finalize(5);

            let proposal = <Proposals<Test>>::get(proposal_id);
            assert_eq!(
                proposal,
                Proposal {
                    parameters: parameters_fixture.params(),
                    proposer_id: 1,
                    created_at: 1,
                    status: ProposalStatus::finalized_with_error(
                        ProposalDecisionStatus::Expired,
                        Some("Cannot remove stake")
                    ),
                    title: b"title".to_vec(),
                    body: b"body".to_vec(),
                    approved_at: None,
                    finalized_at: Some(4),
                    voting_results: VotingResults::default(),
                    stake_id: Some(1),
                }
            );
        });
    });
}
