from uno.state import gs, Player


def main():
    player_names = ["Jane", "Walther", "Jojo"]
    game_state = gs.from_names(player_names)
    # Put first open card on the pile
    game_state.community_cards.flip_first_card()
    # TODO: Shouldn't have to access private attribute _pile here.
    print(
        f"""--- Starting Game ---
    Players: {player_names}
    Top of pile: {game_state.community_cards.top_card}
    """
    )
    round = 1
    while True:
        print(f"--- Round {round} ---")
        # TODO: Sth. with the state of the pile is wrong. The top card
        # doesn't update properly.
        print(f"Top card: {game_state.community_cards.top_card}")
        match game_state.community_cards.top_card.face:
            case "reverse":
                print("🔄, Reversing play direction")
                game_state.player_cycle.reverse()
            case "skip":
                skipped: Player = next(game_state.player_cycle)  # type: ignore
                print(f"🛑, Skipping player: {skipped.name}")
        up: Player = next(game_state.player_cycle)  # type: ignore
        print(f"Player {up} is up")
        match game_state.community_cards.top_card.face:
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
            game_state.community_cards.put_to_pile(card_played)
        else:
            to_draw = game_state.community_cards.draw()
            print(f"☝️, You cannot play, you have to draw 1: {to_draw}")
            up.hand.extend([])
            card_played = up.strategy_random(
                top_card=game_state.community_cards.top_card  # type: ignore
            )
            if card_played:
                game_state.community_cards.put_to_pile(card_played)
        print(f"Card played: {card_played}")
        if game_state.check_win_condition():
            break
        print(f"{game_state.community_cards._pile}")
        round += 1


if __name__ == "__main__":
    main()
