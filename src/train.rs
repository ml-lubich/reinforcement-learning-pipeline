//! Episodic rollouts for bandit environments.

use crate::agent::{EpsilonGreedyAgent, RandomAgent};
use crate::env::{BanditEnv, EpisodeOutcome};
use crate::error::AgentError;

/// Pull arms with a random policy and accumulate rewards.
///
/// # Errors
///
/// Propagates [`AgentError`] from the agent when `env.num_arms() == 0`.
pub fn run_random_episode(
    env: &BanditEnv,
    pulls: usize,
    seed: u64,
) -> Result<EpisodeOutcome, AgentError> {
    let mut agent = RandomAgent::from_seed(seed);
    let mut return_sum = 0.0_f32;
    for _ in 0..pulls {
        let arm = agent.act(env.num_arms())?;
        let r = env.step(arm)?.reward;
        return_sum += r;
    }
    Ok(episode_outcome(return_sum, pulls))
}

/// Run many ε-greedy pulls against the same environment (single “meta-episode”).
///
/// # Errors
///
/// Propagates [`AgentError`] from the agent/environment interaction.
pub fn run_greedy_session(
    env: &BanditEnv,
    pulls: usize,
    epsilon: f32,
    seed: u64,
) -> Result<EpisodeOutcome, AgentError> {
    let mut agent = EpsilonGreedyAgent::new(env.num_arms(), epsilon, seed);
    let mut return_sum = 0.0_f32;
    for _ in 0..pulls {
        return_sum += agent.step_with_env(env)?;
    }
    Ok(episode_outcome(return_sum, pulls))
}

fn episode_outcome(return_sum: f32, pulls: usize) -> EpisodeOutcome {
    let steps = pulls;
    let mean_reward = if pulls == 0 {
        0.0
    } else {
        return_sum / pulls as f32
    };
    EpisodeOutcome {
        return_sum,
        steps,
        mean_reward,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_episode_finishes() {
        let env = BanditEnv::new(4, 2, 1.0, 0.25).expect("env");
        let out = run_random_episode(&env, 20, 7).expect("run");
        assert_eq!(out.steps, 20);
        assert!(out.return_sum.is_finite());
    }

    #[test]
    fn greedy_session_beats_random_on_average() {
        let env = BanditEnv::new(5, 3, 1.0, 0.1).expect("env");
        let random = run_random_episode(&env, 400, 1).expect("r").mean_reward;
        let smart = run_greedy_session(&env, 400, 0.05, 1)
            .expect("g")
            .mean_reward;
        assert!(smart > random);
    }
}
