//! Very small agents (random and ε-greedy) for bandit environments.

use crate::error::AgentError;
use crate::env::BanditEnv;

/// Sample legal actions uniformly.
#[derive(Debug, Clone, Copy, Default)]
pub struct RandomAgent {
    state: u64,
}

impl RandomAgent {
    /// Create with explicit RNG seed.
    #[must_use]
    pub const fn from_seed(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Pick `0 .. num_arms` using an LCG (demo quality).
    ///
    /// # Errors
    ///
    /// Returns [`AgentError::IllegalAction`] when `num_arms == 0`.
    pub fn act(&mut self, num_arms: usize) -> Result<usize, AgentError> {
        if num_arms == 0 {
            return Err(AgentError::IllegalAction {
                action: 0,
                num_arms: 0,
            });
        }
        self.state = self.state.wrapping_mul(636_413_623_846_793_005).wrapping_add(1);
        Ok((self.state as usize) % num_arms)
    }
}

/// Tracks empirical means and chooses greedily with probability `1 - ε`.
#[derive(Debug, Clone)]
pub struct EpsilonGreedyAgent {
    epsilon: f32,
    counts: Vec<u32>,
    values: Vec<f32>,
    rng: u64,
}

impl EpsilonGreedyAgent {
    /// Initialize value estimates to zero; `epsilon` in `[0,1]`.
    #[must_use]
    pub fn new(num_arms: usize, epsilon: f32, seed: u64) -> Self {
        Self {
            epsilon,
            counts: vec![0; num_arms],
            values: vec![0.0; num_arms],
            rng: seed,
        }
    }

    fn rand01(&mut self) -> f32 {
        self.rng = self.rng.wrapping_mul(636_413_623_846_793_005).wrapping_add(1);
        ((self.rng >> 33) as f32) / (u32::MAX as f32)
    }

    /// Select an arm, then observe `reward` from `env.step(arm)` to update estimates.
    ///
    /// # Errors
    ///
    /// Propagates environment and sizing errors.
    pub fn step_with_env(&mut self, env: &BanditEnv) -> Result<f32, AgentError> {
        let k = env.num_arms();
        debug_assert_eq!(
            k,
            self.counts.len(),
            "EpsilonGreedyAgent arm count must match BanditEnv"
        );
        let explore = self.rand01() < self.epsilon;
        let arm = if explore {
            let mut r = RandomAgent::from_seed(self.rng);
            r.act(k)?
        } else {
            self.argmax()
        };
        let r = env.step(arm)?.reward;
        let c = &mut self.counts[arm];
        *c = c.saturating_add(1);
        let n = *c as f32;
        self.values[arm] += (r - self.values[arm]) / n;
        Ok(r)
    }

    fn argmax(&self) -> usize {
        self.values
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Value estimates after learning (read-only).
    #[must_use]
    pub fn values(&self) -> &[f32] {
        &self.values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_agent_stays_in_bounds() {
        let mut a = RandomAgent::from_seed(4);
        for _ in 0..50 {
            let arm = a.act(5).expect("act");
            assert!(arm < 5);
        }
    }

    #[test]
    fn epsilon_greedy_learns_best_arm_value() {
        let env = BanditEnv::new(3, 2, 1.0, 0.0).expect("env");
        let mut agent = EpsilonGreedyAgent::new(3, 0.1, 99);
        for _ in 0..800 {
            let _ = agent.step_with_env(&env).expect("step");
        }
        assert!(agent.values()[2] > 0.85);
    }
}
