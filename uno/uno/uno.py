from itertools import product
from dataclasses import dataclass
from random import shuffle
from typing import Iterable


N_INITAL_CARDS = 7


@dataclass
class Card:
    face: int | str
    color: str


Deck = list[Card]
Hand = list[Card]


@dataclass
class Player:
    hand: Hand
    name: str = "Jane"


numbers = [int(x) for x in "0 1 2 3 4 5 6 7 8 9".split()]
colors = "blue red yellow green".split()
actions = "draw2 skip reverse".split()
wilds = "draw4 wishcolor".split()


def main():
    deck = build_deck()
    player_names = ["Jane", "Walther", "Jojo"]
    players, deck = initialize_game(deck, player_names)

    pc = PlayerCycle(players)
    for _ in range(3):
        print(next(pc).name)
    print("reverse")
    pc.reverse()
    for _ in range(3):
        print(next(pc).name)

    # Game Loop
    while True:
        print("Game Loop")
        break


def build_deck() -> Deck:
    d = list(product(numbers, colors))
    d.extend(list(product(actions, colors)))
    d.extend(list(product(actions, colors)))
    d = [Card(face, color) for face, color in d]
    wildcards = [Card(w, "wild") for w in wilds * 4]
    d.extend(wildcards)
    shuffle(d)
    return d


def initialize_game(deck: Deck, player_names: list[str]) -> tuple[list[Player], Deck]:
    initialized = []
    for name in player_names:
        cards = []
        for _ in range(N_INITAL_CARDS):
            cards.append(deck.pop())
        initialized.append(Player(name=name, hand=cards))
    return initialized, deck


class PlayerCycle:
    """Gives the next Player. Can be reversed."""

    def __init__(self, player_names: Iterable) -> None:
        self._items = list(player_names)
        self._pos = None
        # 1 for normal direction, -1 for reversed
        self._direction = 1

    def __next__(self):
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


if __name__ == "__main__":
    main()
