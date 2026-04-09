from __future__ import annotations

from rl_pipeline.bus.in_memory import InMemoryBus
from rl_pipeline.bus.protocol import Message
from rl_pipeline.envs.tiny_grid import TinyGridEnv, Transition


class EpisodePublisher:
    """Runs the environment and publishes each transition on a bus topic."""

    def __init__(
        self,
        *,
        bus: InMemoryBus,
        env: TinyGridEnv,
        topic_transitions: str,
        max_steps: int,
        policy: str = "random",
    ) -> None:
        if max_steps < 1:
            raise ValueError("max_steps must be at least 1")
        if policy != "random":
            raise ValueError("only policy='random' is implemented in this skeleton")
        self._bus = bus
        self._env = env
        self._topic = topic_transitions
        self._max_steps = max_steps

    def run_episode(self) -> tuple[float, int]:
        self._env.reset()
        total_reward = 0.0
        steps = 0
        for _ in range(self._max_steps):
            action = self._env.random_action()
            transition: Transition = self._env.step(action)
            self._bus.publish(Message(topic=self._topic, payload=transition))
            total_reward += transition.reward
            steps += 1
            if transition.done:
                break
        return total_reward, steps
