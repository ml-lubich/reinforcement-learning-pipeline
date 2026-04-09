//! Integration tests: bandit env, random and greedy rollouts.

use reinforcement_learning_pipeline::{
    BanditEnv, EnvError, run_greedy_session, run_random_episode,
};

#[test]
fn env_rejects_invalid_config() {
    assert_eq!(
        BanditEnv::new(0, 0, 1.0, 0.0).unwrap_err(),
        EnvError::NoArms
    );
    assert_eq!(
        BanditEnv::new(3, 5, 1.0, 0.0).unwrap_err(),
        EnvError::InvalidBestArm
    );
}

#[test]
fn greedy_beats_random_over_many_pulls() {
    let env = BanditEnv::new(7, 3, 1.0, 0.05).expect("env");
    let rnd = run_random_episode(&env, 500, 5)
        .expect("random")
        .mean_reward;
    let smart = run_greedy_session(&env, 500, 0.08, 5)
        .expect("greedy")
        .mean_reward;
    assert!(smart > rnd);
}
