from rl_pipeline.pipeline.runner import run_random_episode_pipeline


def test_episode_terminates_and_logs_transitions() -> None:
    result = run_random_episode_pipeline(grid_size=3, max_steps=100, random_seed=42)
    assert result.steps >= 1
    assert len(result.transitions) == result.steps
    assert result.transitions[-1].done is True
    assert result.total_reward > 0
