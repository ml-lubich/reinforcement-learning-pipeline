//! Reinforcement learning **pipeline** scaffold: [`env::BanditEnv`], [`agent`] policies, and
//! [`train`] rollouts, plus [`scenarios`] for scripted demos.
//!
//! Mirrors the layout of `pub-sub-pipeline`: library core + `rlpipe` binary + integration tests.

#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]

pub mod agent;
pub mod art;
pub mod env;
pub mod error;
pub mod scenarios;
pub mod train;

pub use agent::{EpsilonGreedyAgent, RandomAgent};
pub use env::{BanditEnv, EpisodeOutcome, Step};
pub use error::{AgentError, EnvError};
pub use train::{run_greedy_session, run_random_episode};
