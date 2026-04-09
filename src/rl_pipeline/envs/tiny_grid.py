from __future__ import annotations

import random
from dataclasses import dataclass


@dataclass(frozen=True, slots=True)
class Transition:
    state: int
    action: int
    reward: float
    next_state: int
    done: bool


class TinyGridEnv:
    """Minimal 1-D grid: states 0..n-1, goal at right, -1 step penalty."""

    def __init__(self, *, size: int, random_seed: int | None = 0) -> None:
        if size < 2:
            raise ValueError("size must be at least 2")
        self._size = size
        self._rng = random.Random(random_seed)
        self.state = 0

    def reset(self) -> int:
        self.state = 0
        return self.state

    def step(self, action: int) -> Transition:
        if action not in (0, 1):
            raise ValueError("action must be 0 (left) or 1 (right)")
        next_state = self.state + (1 if action == 1 else -1)
        next_state = max(0, min(self._size - 1, next_state))
        goal = self._size - 1
        reward = 1.0 if next_state == goal else -0.01
        done = next_state == goal
        transition = Transition(
            state=self.state,
            action=action,
            reward=reward,
            next_state=next_state,
            done=done,
        )
        self.state = next_state
        return transition

    def random_action(self) -> int:
        return self._rng.randint(0, 1)
