//! Domain errors for environments and agents.

use thiserror::Error;

/// Invalid environment configuration.
#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum EnvError {
    /// No arms available to pull.
    #[error("bandit needs at least one arm")]
    NoArms,
    /// Best arm index out of range.
    #[error("best_arm must be < num_arms")]
    InvalidBestArm,
}

/// Agent action errors.
#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum AgentError {
    /// Action outside legal range.
    #[error("action {action} invalid for {num_arms} arms")]
    IllegalAction {
        /// Disallowed arm index that was requested.
        action: usize,
        /// Number of arms in the environment (legal indices are `0..num_arms`).
        num_arms: usize,
    },
}
