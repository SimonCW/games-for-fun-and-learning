from random import choice
from typing import Optional
from uno.state import GameState, Hand, Card


def main():
    player_names = ["Jane", "Walther", "Jojo"]
    game_state = GameState.from_names(player_names)
    # Put first open card on the pile
    game_state.community_cards.flip_first_card()
    top_of_pile = game_state.community_cards._pile[0]
    print(f"Top of pile: {top_of_pile}")
    do_card_action(gs=game_state)
    # Todo: Maybe have a next method on player_cycle or next_player on state
    up = next(game_state.player_cycle)
    print(f"Next Player: {up}")
    card_played = strategy_random(
        top_card=game_state.community_cards._pile[0], hand=up.hand  # type: ignore
    )
    print(f"Card played: {card_played}")


def do_card_action(
    gs: GameState,
) -> None | tuple[Card, Card] | tuple[Card, Card, Card, Card]:
    # Todo: Argh, this is mutating game_state. Maybe I could hand a new state back
    match gs.community_cards._pile[0].face:
        case "reverse":
            print("Reversing play direction")
            gs.player_cycle.reverse()
            return
        case "skip":
            skipped = next(gs.player_cycle)
            print(f"Skipping player: {skipped.name}")  # type: ignore
            return
        case "draw2":
            print("Düdum, you have to draw 2")
            return (gs.community_cards.draw(), gs.community_cards.draw())
        case "draw4":
            print("Puh, you have to draw 4")
            return (
                gs.community_cards.draw(),
                gs.community_cards.draw(),
                gs.community_cards.draw(),
                gs.community_cards.draw(),
            )
        case _:
            print("No specific action to take")


def get_playables(top_card: Card, hand: Hand) -> list[Card]:
    playables = [
        c
        for c in hand
        if (c.color == top_card.color)
        or (c.face == top_card.face)
        or (c.color == "wild")
    ]
    if not playables:
        print(r"¯\_(ツ)_/¯. Nothing to play")
    print(f"Playable cards: {playables}")
    return playables


def strategy_random(top_card: Card, hand: Hand) -> Optional[Card]:
    try:
        return choice(get_playables(top_card, hand))
    except IndexError as _:
        return None


if __name__ == "__main__":
    main()
