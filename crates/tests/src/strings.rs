//! Expected strings for integration and e2e tests.

/// Namada ledger started
pub const LEDGER_STARTED: &str = "Namada ledger node started";

/// Namada ledger has shut down
pub const LEDGER_SHUTDOWN: &str = "Namada ledger node has shut down";

/// Ledger is running as a validator
pub const VALIDATOR_NODE: &str = "This node is a validator";

/// Ledger is not running as a validator
pub const NON_VALIDATOR_NODE: &str = "This node is not a validator";

/// Inner tx applied and accepted by VPs.
pub const TX_APPLIED_SUCCESS: &str = "was successfully applied";

/// Inner transaction rejected by VP(s).
pub const TX_REJECTED: &str = "was rejected by VPs";

/// Inner transaction failed in execution of transfer with insufficient balance
pub const TX_INSUFFICIENT_BALANCE: &str = "insufficient balance";

/// Inner transaction failed in execution (no VPs ran).
pub const TX_FAILED: &str = "failed with error";

pub const WALLET_HD_PASSPHRASE_PROMPT: &str =
    "Enter BIP39 passphrase (empty for none): ";

pub const WALLET_HD_PASSPHRASE_CONFIRMATION_PROMPT: &str =
    "Enter same passphrase again: ";

pub const WALLET_FOUND_TRANSPARENT_KEYS: &str = "Found transparent keys:";
