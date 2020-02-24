//! Proposals codex module for the Joystream platform. Version 2.
//! Contains preset proposal types
//!
//! Supported extrinsics (proposal type):
//! - create_text_proposal
//!

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Do not delete! Cannot be uncommented by default, because of Parity decl_module! issue.
//#![warn(missing_docs)]

#[cfg(test)]
mod tests;

use codec::Encode;
use proposal_engine::*;
use rstd::clone::Clone;
use rstd::prelude::*;
use rstd::str::from_utf8;
use rstd::vec::Vec;
use srml_support::{decl_module, decl_storage, ensure, print};
use system::ensure_root;

/// 'Proposals codex' substrate module Trait
pub trait Trait: system::Trait + proposal_engine::Trait {}

use srml_support::traits::Currency;

/// Balance alias
pub type BalanceOf<T> =
    <<T as stake::Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

/// Balance alias for staking
pub type NegativeImbalance<T> =
    <<T as stake::Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::NegativeImbalance;

// Defines max allowed text proposal text length. Can be override in the config.
const DEFAULT_TEXT_PROPOSAL_MAX_LEN: u32 = 20_000;
// Defines max allowed text proposal text length. Can be override in the config.
const DEFAULT_RUNTIME_PROPOSAL_WASM_MAX_LEN: u32 = 20_000;

// Storage for the proposals codex module
decl_storage! {
    pub trait Store for Module<T: Trait> as ProposalCodex{
        /// Defines max allowed text proposal text length.
        pub TextProposalMaxLen get(text_max_len) config(): u32 = DEFAULT_TEXT_PROPOSAL_MAX_LEN;

        /// Defines max allowed runtime upgrade proposal wasm code length.
        pub RuntimeUpgradeMaxLen get(wasm_max_len) config(): u32 = DEFAULT_RUNTIME_PROPOSAL_WASM_MAX_LEN;
    }
}

decl_module! {
    /// 'Proposal codex' substrate module
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Predefined errors
 //       type Error = Error;

        /// Create text (signal) proposal type. On approval prints its content.
        pub fn create_text_proposal(
            origin,
            title: Vec<u8>,
            body: Vec<u8>,
            text: Vec<u8>,
            stake_balance: Option<BalanceOf<T>>,
        ) {
            let parameters = crate::ProposalParameters {
                voting_period: T::BlockNumber::from(50000u32),
                grace_period: T::BlockNumber::from(10000u32),
                approval_quorum_percentage: 40,
                approval_threshold_percentage: 51,
                slashing_quorum_percentage: 80,
                slashing_threshold_percentage: 80,
                required_stake: Some(<BalanceOf<T>>::from(500u32))
            };

            ensure!(!text.is_empty(), "TextProposalIsEmpty");
            ensure!(text.len() as u32 <=  Self::text_max_len(),
                "TextProposalSizeExceeded");

            let proposal_code = <Call<T>>::text_proposal(title.clone(), body.clone(), text);

            <proposal_engine::Module<T>>::create_proposal(
                origin,
                parameters,
                title,
                body,
                stake_balance,
                proposal_code.encode()
            )?;
        }

        /// Text proposal extrinsic. Should be used as callable object to pass to the engine module
        fn text_proposal(
            origin,
            title: Vec<u8>,
            body: Vec<u8>,
            text: Vec<u8>,
        ) {
            ensure_root(origin)?;
            print("Proposal: ");
            print(from_utf8(title.as_slice()).unwrap());
            print("Description:");
            print(from_utf8(body.as_slice()).unwrap());
            print("Text:");
            print(from_utf8(text.as_slice()).unwrap());
        }

        // /// Create runtime upgrade proposal type. On approval prints its content.
        // pub fn create_runtime_upgrade_proposal(
        //     origin,
        //     title: Vec<u8>,
        //     body: Vec<u8>,
        //     wasm: Vec<u8>,
        //     stake_balance: Option<BalanceOf<T>>,
        // ) {
        //     let parameters = crate::ProposalParameters {
        //         voting_period: T::BlockNumber::from(50000u32),
        //         grace_period: T::BlockNumber::from(10000u32),
        //         approval_quorum_percentage: 80,
        //         approval_threshold_percentage: 80,
        //         slashing_quorum_percentage: 80,
        //         slashing_threshold_percentage: 80,
        //         required_stake: Some(<BalanceOf<T>>::from(50000u32))
        //     };
        //
        //     ensure!(!wasm.is_empty(), Error::RuntimeProposalIsEmpty);
        //     ensure!(wasm.len() as u32 <= Self::wasm_max_len(),
        //         Error::RuntimeProposalSizeExceeded);
        //
        //     let proposal = RuntimeUpgradeProposalExecutable{
        //         title: title.clone(),
        //         body: body.clone(),
        //         wasm,
        //         marker : PhantomData::<T>
        //        };
        //     let proposal_code = proposal.encode();
        //
        //     <proposal_engine::Module<T>>::create_proposal(
        //         origin,
        //         parameters,
        //         title,
        //         body,
        //         stake_balance,
        //         proposal.proposal_type(),
        //         proposal_code
        //     )?;
        // }
    }
}
