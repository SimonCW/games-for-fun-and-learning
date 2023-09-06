from dataclasses import dataclass, field
from itertools import product
from random import shuffle
from typing import Iterable, Optional, Self, TypeVar
from collections import deque

N_INITAL_CARDS = 7

T = TypeVar("T")


# from enum import Enum
# class Color(Enum):
#     BLUE = 1
#     GREEN = 2
#     RED = 3
#     YELLOW = 4
#     WILD = 5

numbers = [int(x) for x in "0 1 2 3 4 5 6 7 8 9".split()]
numbers_str = [x for x in "zero one two three four five six seven eight nine".split()]
number_str2int = dict(zip(numbers_str, numbers))
colors = "blue red yellow green".split()
actions = "draw2 skip reverse".split()
wilds = "draw4 wishcolor".split()


@dataclass
class Card:
    color: str
    face: str
    value: Optional[int] = field(init=False)

    def __post_init__(self):
        self.value = number_str2int.get(self.face, None)

    # Define a dictionary that maps each color to an ANSI escape sequence
    color_codes = {
        "blue": "\033[34m",
        "green": "\033[32m",
        "red": "\033[31m",
        "yellow": "\033[33m",
        "wild": "\033[35m",
    }

    # Define a custom __repr__ method that includes the right color for each attribute
    def __repr__(self):
        # Get the color code and value name for this card
        color_code = self.color_codes[self.color]
        return f"{color_code}Card({self.value if self.value is not None else self.face})\033[0m"


# Todo: Should deck also be a deque?
Deck = list[Card]
Stack = deque[Card]
Hand = list[Card]


@dataclass
class Player:
    hand: Hand
    name: str = "Jane"


class PlayerCycle:
    """Gives the next Player. Can be reversed."""

    def __init__(self, players: Iterable[T]) -> None:
        self._items: list[T] = list(players)
        self._pos = None
        # 1 for normal direction, -1 for reversed
        self._direction = 1

    def __next__(self) -> T:
        # First play in the game
        if self._pos is None:
            self._pos = 0 if self._direction == 1 else -1
            return self._items[self._pos]

        # Modulo avoids positions that are "out of index".
        self._pos = (self._direction + self._pos) % len(self._items)
        element = self._items[self._pos]
        return element

    def __iter__(self):
        return self

    def reverse(self):
        self._direction *= -1


@dataclass
class GameState:
    player_cycle: PlayerCycle
    deck: Deck
    stack: Stack

    @classmethod
    def from_names(cls, names: list[str]) -> Self:
        deck = build_deck()
        initialized_players = []
        for name in names:
            cards = []
            for _ in range(N_INITAL_CARDS):
                cards.append(deck.pop())
            initialized_players.append(Player(name=name, hand=cards))
        return cls(
            player_cycle=PlayerCycle(initialized_players), deck=deck, stack=deque()
        )


def build_deck() -> Deck:
    d = list(product(numbers_str, colors))
    d.extend(list(product(actions, colors)))
    d.extend(list(product(actions, colors)))
    d = [Card(color=color, face=face) for face, color in d]
    wildcards = [Card(color="wild", face=w) for w in wilds * 4]
    d.extend(wildcards)
    shuffle(d)
    return d


if __name__ == "__main__":
    print("Hi")