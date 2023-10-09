from uno.state import GameState, Player


def main():
    player_names = ["Jane", "Walther", "Jojo"]
    game_state = GameState.from_names(player_names)
    # Put first open card on the pile
    game_state.community_cards.flip_first_card()
    # TODO: Shouldn't have to access private attribute _pile here.
    top_of_pile = game_state.community_cards._pile[0]
    print(
        f"""--- Starting Game ---
    Players: {player_names}
    Top of pile: {top_of_pile}
    """
    )
    round = 1
    while True:
        print(f"--- Round {round} ---")
        # TODO: Sth. with the state of the pile is wrong. The top card
        # doesn't update properly.
        print(f"Top card: {top_of_pile}")
        match top_of_pile.face:
            case "reverse":
                print("🔄, Reversing play direction")
                game_state.player_cycle.reverse()
            case "skip":
                skipped: Player = next(game_state.player_cycle)  # type: ignore
                print(f"🛑, Skipping player: {skipped.name}")
        up: Player = next(game_state.player_cycle)  # type: ignore
        print(f"Player {up} is up")
        match top_of_pile.face:
            case "draw2":
                to_draw = [
                    game_state.community_cards.draw(),
                    game_state.community_cards.draw(),
                ]
                print(f"🤷, you have to draw 2: {to_draw}")
                up.hand.extend(to_draw)
            case "draw4":
                to_draw = [
                    game_state.community_cards.draw(),
                    game_state.community_cards.draw(),
                    game_state.community_cards.draw(),
                    game_state.community_cards.draw(),
                ]
                print(f"🤯, you have to draw 4: {to_draw}")
                up.hand.extend(to_draw)
            case _:
                print("No action to take")
        card_played = up.strategy_random(
            top_card=game_state.community_cards._pile[0]  # type: ignore
        )
        if card_played:
            game_state.community_cards._pile.append(card_played)
        else:
            to_draw = game_state.community_cards.draw()
            print(f"☝️, You cannot play, you have to draw 1: {to_draw}")
            up.hand.extend([])
            card_played = up.strategy_random(
                top_card=game_state.community_cards._pile[0]  # type: ignore
            )
            if card_played:
                game_state.community_cards._pile.append(card_played)
        print(f"Card played: {card_played}")
        if game_state.check_win_condition():
            break
        top_of_pile = game_state.community_cards._pile[0]
        round += 1


if __name__ == "__main__":
    main()
