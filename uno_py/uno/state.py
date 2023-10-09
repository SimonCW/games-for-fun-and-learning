from dataclasses import dataclass, field
from itertools import product
from random import shuffle, choice
from typing import Iterable, Optional, Self
from collections import deque

N_INITAL_CARDS = 7

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
        return f"""{color_code}Card({
        self.value if self.value is not None else self.face
        })\033[0m"""


Hand = list[Card]


@dataclass
class CommunityCards:
    # TODO: Define direction (left, right). Where do new cards go? From where to draw?
    _deck: deque[Card]
    _pile: deque[Card]  # The discard pile

    def flip_first_card(self) -> None:
        self._pile.append(self.draw())

    def draw(self) -> Card:
        try:
            card = self._deck.pop()
            return card
        except IndexError:
            self.shuffle_pile_as_deck()
            card = self._deck.pop()
            if card is None:
                raise AssertionError("Card shouldn't be None here")
            return card

    def shuffle_pile_as_deck(self) -> None:
        """Used when the deck is empty.

        The pile except for the top card gets shuffled and becomes the new deck.
        """

        print(f"Shuffle stack of {len(self._pile)} cards and use as new deck.")
        old_pile = self._pile
        self._pile = deque([old_pile.pop()])
        shuffle(old_pile)
        self._deck = old_pile

    @classmethod
    def new(cls) -> Self:
        d = list(product(numbers_str, colors))
        d.extend(list(product(actions, colors)))
        d.extend(list(product(actions, colors)))
        d = [Card(color=color, face=face) for face, color in d]
        wildcards = [Card(color="wild", face=w) for w in wilds * 4]
        d.extend(wildcards)
        shuffle(d)
        return cls(_deck=deque(d), _pile=deque())


@dataclass
class Player:
    hand: Hand
    name: str = "Jane"

    def get_playables(self, top_card: Card) -> list[Card]:
        playables = [
            c
            for c in self.hand
            if (c.color == top_card.color)
            or (c.face == top_card.face)
            or (c.color == "wild")
        ]
        if not playables:
            print(r"¯\_(ツ)_/¯. Nothing to play")
        print(f"Playable cards: {playables}")
        return playables

    def strategy_random(self, top_card: Card) -> Optional[Card]:
        # TODO: Strategies should be supplied to player aka strategy pattern
        try:
            card = choice(self.get_playables(top_card))
            self.hand.remove(card)
            return card
        except IndexError as _:
            return None


class PlayerCycle:
    """Gives the next Player. Can be reversed."""

    def __init__(self, players: Iterable[Player]) -> None:
        self._items: list[Player] = list(players)
        self._pos = None
        # 1 for normal direction, -1 for reversed
        self._direction = 1

    def __next__(self) -> Player:
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
    """Encapsulates the mutable state of the game.

    Helper methods are implemented if they cover basic game mechanics. Rules and
    strategies are implemented separately to stay flexible.
    """

    player_cycle: PlayerCycle
    community_cards: CommunityCards

    @classmethod
    def from_names(cls, names: list[str]) -> Self:
        c_cards = CommunityCards.new()
        initialized_players = []
        for name in names:
            cards = []
            for _ in range(N_INITAL_CARDS):
                cards.append(c_cards.draw())
            initialized_players.append(Player(name=name, hand=cards))
        return cls(
            player_cycle=PlayerCycle(initialized_players),
            community_cards=c_cards,
        )

    def check_win_condition(self):
        # Copy to avoid mutating the player cycle
        for player in self.player_cycle._items.copy():
            if len(player.hand) == 0:
                print(f"Player {player.name} has won!")
                return True
        return False


if __name__ == "__main__":
    print("Hi")
