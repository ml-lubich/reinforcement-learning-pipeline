from __future__ import annotations

from collections.abc import Callable
from dataclasses import dataclass
from typing import Generic, Protocol, TypeVar

T = TypeVar("T")


@dataclass(frozen=True, slots=True)
class Message(Generic[T]):
    topic: str
    payload: T


class Subscriber(Protocol[T]):
    def topics(self) -> frozenset[str]:
        """Topics this subscriber listens to."""

    def on_message(self, message: Message[T]) -> None:
        """Handle a single message."""


Callback = Callable[[Message[T]], None]
