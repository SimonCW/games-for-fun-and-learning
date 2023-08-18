from itertools import product
from dataclasses import dataclass
from random import shuffle

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

def main ():
    deck = build_deck()
    player_names = ["Jane", "Walther", "Jojo"]
    players, deck = deal_cards(deck, player_names)
    print(players, len(deck))
    # Game Loop
    while True:
        pass

def build_deck() -> Deck:
    d = list(product(numbers, colors))
    d.extend(list(product(actions, colors)))
    d.extend(list(product(actions, colors)))
    d = [Card(face, color) for face, color in d]
    wildcards = [Card(w, "wild") for w in wilds*4]
    d.extend(wildcards)
    shuffle(d)
    return d

def deal_cards(deck: Deck, player_names: list[str]) -> tuple[list[Player], Deck]:
    initialized = []
    for name in player_names: 
        cards = []
        for _ in range(N_INITAL_CARDS):
            cards.append(deck.pop())
        initialized.append(Player(name=name, hand=cards))
    return initialized, deck




if __name__ == "__main__":
    main()
