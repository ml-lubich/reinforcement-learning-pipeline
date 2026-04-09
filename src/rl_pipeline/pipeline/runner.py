from __future__ import annotations

from dataclasses import dataclass

from rl_pipeline.bus.in_memory import InMemoryBus
from rl_pipeline.envs.tiny_grid import TinyGridEnv, Transition
from rl_pipeline.publishers.episode import EpisodePublisher
from rl_pipeline.subscribers.transition_log import TransitionLog


@dataclass(frozen=True, slots=True)
class EpisodeRunResult:
    total_reward: float
    steps: int
    transitions: tuple[Transition, ...]


def run_random_episode_pipeline(
    *,
    grid_size: int = 5,
    max_steps: int = 50,
    topic_transitions: str = "rl/transitions",
    random_seed: int = 0,
) -> EpisodeRunResult:
    bus: InMemoryBus = InMemoryBus()
    log = TransitionLog(topic=topic_transitions)
    bus.subscribe(log)
    env = TinyGridEnv(size=grid_size, random_seed=random_seed)
    publisher = EpisodePublisher(
        bus=bus,
        env=env,
        topic_transitions=topic_transitions,
        max_steps=max_steps,
        policy="random",
    )
    total_reward, steps = publisher.run_episode()
    return EpisodeRunResult(
        total_reward=total_reward,
        steps=steps,
        transitions=tuple(log.items),
    )
