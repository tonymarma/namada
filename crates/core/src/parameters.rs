//! Protocol parameters types

use std::collections::BTreeMap;

use namada_macros::BorshDeserializer;
#[cfg(feature = "migrations")]
use namada_migrations::*;

use super::address::Address;
use super::chain::ProposalBytes;
use super::hash::Hash;
use super::time::DurationSecs;
use super::token;
use crate::borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// Protocol parameters
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    BorshSerialize,
    BorshDeserialize,
    BorshDeserializer,
    BorshSchema,
)]
pub struct Parameters {
    /// Max payload size, in bytes, for a mempool tx.
    pub max_tx_bytes: u32,
    /// Epoch duration (read only)
    pub epoch_duration: EpochDuration,
    /// Max payload size, in bytes, for a tx batch proposal.
    pub max_proposal_bytes: ProposalBytes,
    /// Max gas for block
    pub max_block_gas: u64,
    /// Allowed validity predicate hashes (read only)
    pub vp_allowlist: Vec<String>,
    /// Allowed tx hashes (read only)
    pub tx_allowlist: Vec<String>,
    /// Implicit accounts validity predicate WASM code hash
    pub implicit_vp_code_hash: Option<Hash>,
    /// Expected number of epochs per year (read only)
    pub epochs_per_year: u64,
    /// The multiplier for masp epochs (it requires this amount of epochs to
    /// transition to the next masp epoch)
    pub masp_epoch_multiplier: u64,
    /// The gas limit for a masp transaction paying fees
    pub masp_fee_payment_gas_limit: u64,
    /// Gas scale
    pub gas_scale: u64,
    /// Map of the cost per gas unit for every token allowed for fee payment
    pub minimum_gas_price: BTreeMap<Address, token::Amount>,
    /// Enable the native token transfer if it is true
    pub is_native_token_transferable: bool,
}

/// Epoch duration. A new epoch begins as soon as both the `min_num_of_blocks`
/// and `min_duration` have passed since the beginning of the current epoch.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    BorshSerialize,
    BorshDeserialize,
    BorshDeserializer,
    BorshSchema,
)]
pub struct EpochDuration {
    /// Minimum number of blocks in an epoch
    pub min_num_of_blocks: u64,
    /// Minimum duration of an epoch
    pub min_duration: DurationSecs,
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            max_tx_bytes: 1024 * 1024,
            epoch_duration: EpochDuration {
                min_num_of_blocks: 1,
                min_duration: DurationSecs(3600),
            },
            max_proposal_bytes: Default::default(),
            max_block_gas: 100,
            vp_allowlist: vec![],
            tx_allowlist: vec![],
            implicit_vp_code_hash: Default::default(),
            epochs_per_year: 365,
            masp_epoch_multiplier: 2,
            masp_fee_payment_gas_limit: 0,
            gas_scale: 100_000_000,
            minimum_gas_price: Default::default(),
            is_native_token_transferable: true,
        }
    }
}
