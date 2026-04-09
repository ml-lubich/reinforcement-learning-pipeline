from __future__ import annotations

from rl_pipeline.bus.protocol import Message
from rl_pipeline.envs.tiny_grid import Transition


class TransitionLog:
    def __init__(self, topic: str) -> None:
        self._topic = topic
        self.items: list[Transition] = []

    def topics(self) -> frozenset[str]:
        return frozenset({self._topic})

    def on_message(self, message: Message[object]) -> None:
        payload = message.payload
        if not isinstance(payload, Transition):
            raise TypeError(f"expected Transition, got {type(payload).__name__}")
        self.items.append(payload)
