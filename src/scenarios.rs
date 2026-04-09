//! Shared flows for the CLI, example binary, and integration tests.

use crate::env::BanditEnv;
use crate::train::{run_greedy_session, run_random_episode};

/// Deterministic narrative report comparing random vs ε-greedy collectors.
#[must_use]
pub fn run_demo_report() -> String {
    let env = BanditEnv::new(6, 4, 1.0, 0.15).expect("env");
    let rnd = run_random_episode(&env, 200, 42).expect("random");
    let smart = run_greedy_session(&env, 200, 0.12, 42).expect("greedy");
    format!(
        "rl pipeline sample\n  arms={}\n  random_mean={:.4}\n  greedy_mean={:.4}\n",
        env.num_arms(),
        rnd.mean_reward,
        smart.mean_reward,
    )
}
