from uno.state import GameState, Player, Card
from random import choice
from typing import Optional


def main():
    player_names = ["Jane", "Walther", "Jojo"]
    gs = GameState.from_names(player_names)
    # Put first open card on the pile
    gs.community_cards.flip_first_card()
    # TODO: Shouldn't have to access private attribute _pile here.
    print(
        f"""--- Starting Game ---
    Players: {player_names}
    Top of pile: {gs.community_cards.top_card}
    """
    )
    round = 1
    while True:
        print(f"--- Round {round} ---")
        # TODO: Sth. with the state of the pile is wrong. The top card
        # doesn't update properly.
        print(f"Top card: {gs.community_cards.top_card}")
        match gs.community_cards.top_card.face:
            case "reverse":
                print("🔄, Reversing play direction")
                gs.player_cycle.reverse()
            case "skip":
                skipped: Player = next(gs.player_cycle)  # type: ignore
                print(f"🛑, Skipping player: {skipped.name}")
        up: Player = next(gs.player_cycle)  # type: ignore
        match gs.community_cards.top_card.face:
            case "draw2":
                to_draw = [
                    gs.community_cards.draw(),
                    gs.community_cards.draw(),
                ]
                print(f"🤷, you have to draw 2: {to_draw}")
                up.hand.extend(to_draw)
            case "draw4":
                to_draw = [
                    gs.community_cards.draw(),
                    gs.community_cards.draw(),
                    gs.community_cards.draw(),
                    gs.community_cards.draw(),
                ]
                print(f"🤯, you have to draw 4: {to_draw}")
                up.hand.extend(to_draw)
            case _:
                print("No action to take")
        print(f"{up} is up")
        card_played = strategy_random(player=up, top_card=gs.community_cards.top_card)
        if card_played:
            gs.community_cards.put_to_pile(card_played)
        else:
            to_draw = gs.community_cards.draw()
            print(f"☝️, You cannot play, you have to draw 1: {to_draw}")
            up.hand.extend([])
            card_played = strategy_random(
                player=up, top_card=gs.community_cards.top_card
            )
            if card_played:
                gs.community_cards.put_to_pile(card_played)
        print(f"Card played: {card_played}")
        if gs.check_win_condition():
            break
        round += 1


def get_playables(hand, top_card: Card) -> list[Card]:
    playables = [
        c
        for c in hand
        if (c.color == top_card.color)
        or (c.face == top_card.face)
        or (c.color == "wild")
        or (top_card.color == "wild")
    ]
    if not playables:
        print(r"¯\_(ツ)_/¯. Nothing to play")
    print(f"Playable cards: {playables}")
    return playables


def strategy_random(player: Player, top_card: Card) -> Optional[Card]:
    # TODO: Strategies could be supplied to player aka strategy pattern
    #   to enable different strategies for different players.
    try:
        card = choice(get_playables(player.hand, top_card))
        player.hand.remove(card)
        return card
    except IndexError as _:
        return None


if __name__ == "__main__":
    main()
