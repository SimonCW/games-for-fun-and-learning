from itertools import product
from dataclasses import dataclass

@dataclass
class Card:
    face: int | str
    color: str

numbers = [int(x) for x in "0 1 2 3 4 5 6 7 8 9".split()]
colors = "blue red yellow green".split()
actions = "draw2 skip reverse".split()
wildcards = "draw4 wishcolor".split()

def main ():
    deck = build_deck()
    print(deck)

def build_deck() -> list[Card]:
    d = list(product(numbers, colors))
    d.extend(list(product(actions, colors)))
    d.extend(list(product(actions, colors)))
    d = [Card(face, color) for face, color in d]

    return d




if __name__ == "__main__":
    main()
