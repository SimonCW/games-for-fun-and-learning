from uno.state import GameState, Card, Player


def main():
    player_names = ["Jane", "Walther", "Jojo"]
    game_state = GameState.from_names(player_names)
    # Put first open card on the pile
    game_state.community_cards.flip_first_card()
    # TODO: Shouldn't have to access private attribute _pile here.
    top_of_pile = game_state.community_cards._pile[0]
    print(f"Top of pile: {top_of_pile}")
    round = 1
    while True:
        match top_of_pile.face:
            case "reverse":
                print("Reversing play direction")
                game_state.player_cycle.reverse()
            case "skip":
                skipped: Player = next(game_state.player_cycle)  # type: ignore
                print(f"Skipping player: {skipped.name}")
        up: Player = next(game_state.player_cycle)  # type: ignore
        print(f"Player {up} is up")
        match top_of_pile.face:
            case "draw2":
                print("Düdum, you have to draw 2")
                up.hand.extend(
                    [
                        game_state.community_cards.draw(),
                        game_state.community_cards.draw(),
                    ]
                )
            case "draw4":
                print("Puh, you have to draw 4")
                up.hand.extend(
                    [
                        game_state.community_cards.draw(),
                        game_state.community_cards.draw(),
                        game_state.community_cards.draw(),
                        game_state.community_cards.draw(),
                    ]
                )
            case _:
                print("No specific action to take")
        card_played = up.strategy_random(
            top_card=game_state.community_cards._pile[0]  # type: ignore
        )
        if card_played:
            game_state.community_cards._pile.appendleft(card_played)
        else:
            up.hand.extend([game_state.community_cards.draw()])
            card_played = up.strategy_random(
                top_card=game_state.community_cards._pile[0]  # type: ignore
            )
            if card_played:
                game_state.community_cards._pile.appendleft(card_played)
        round += 1
        print(f"Card played: {card_played}")
        # TODO: Add win condition. Maybe as method on player?


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


if __name__ == "__main__":
    main()
