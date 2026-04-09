//! Minimal multi-armed bandit environment (stateless single-step “episode”).

use crate::error::EnvError;

/// Outcome of one stochastic interaction step.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Step {
    /// Reward returned by the environment.
    pub reward: f32,
    /// Terminal flag (always `true` for the toy bandit).
    pub done: bool,
}

/// Multi-armed bandit: pulling the `best_arm` yields `high_reward`, others `low_reward`.
#[derive(Debug, Clone, PartialEq)]
pub struct BanditEnv {
    num_arms: usize,
    best_arm: usize,
    high_reward: f32,
    low_reward: f32,
}

impl BanditEnv {
    /// Build a stationary bandit.
    ///
    /// # Errors
    ///
    /// Returns [`EnvError::NoArms`] when `num_arms == 0`, or [`EnvError::InvalidBestArm`] when
    /// `best_arm >= num_arms`.
    pub fn new(
        num_arms: usize,
        best_arm: usize,
        high_reward: f32,
        low_reward: f32,
    ) -> Result<Self, EnvError> {
        if num_arms == 0 {
            return Err(EnvError::NoArms);
        }
        if best_arm >= num_arms {
            return Err(EnvError::InvalidBestArm);
        }
        Ok(Self {
            num_arms,
            best_arm,
            high_reward,
            low_reward,
        })
    }

    /// Number of legal actions `0 .. num_arms`.
    #[must_use]
    pub fn num_arms(&self) -> usize {
        self.num_arms
    }

    /// Pull an arm and observe reward.
    ///
    /// # Errors
    ///
    /// Returns [`crate::error::AgentError::IllegalAction`] when `arm` is out of range.
    pub fn step(&self, arm: usize) -> Result<Step, crate::error::AgentError> {
        if arm >= self.num_arms {
            return Err(crate::error::AgentError::IllegalAction {
                action: arm,
                num_arms: self.num_arms,
            });
        }
        let reward = if arm == self.best_arm {
            self.high_reward
        } else {
            self.low_reward
        };
        Ok(Step { reward, done: true })
    }
}

/// Summary statistics after running multiple pulls.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EpisodeOutcome {
    /// Sum of rewards.
    pub return_sum: f32,
    /// Number of pulls.
    pub steps: usize,
    /// Average reward per pull.
    pub mean_reward: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best_arm_pays_more() {
        let env = BanditEnv::new(3, 1, 1.0, 0.0).expect("env");
        assert_eq!(env.step(1).expect("step").reward, 1.0);
        assert_eq!(env.step(0).expect("step").reward, 0.0);
    }

    #[test]
    fn illegal_action_errors() {
        let env = BanditEnv::new(2, 0, 1.0, 0.0).expect("env");
        assert!(env.step(5).is_err());
    }
}
