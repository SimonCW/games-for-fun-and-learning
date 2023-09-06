from uno.state import GameState, Hand, Card


def main():
    player_names = ["Jane", "Walther", "Jojo"]
    game_state = GameState.from_names(player_names)
    # Put first open card on the stack
    game_state.stack.appendleft(game_state.deck.pop())
    top_of_stack = game_state.stack[0]
    print(f"Top of stack: {top_of_stack}")
    do_card_action(gs=game_state)
    # Todo: Maybe have a next method on player_cycle or next_player on state
    up = next(game_state.player_cycle)
    print(f"Next Player: {up}")
    playables(top_card=game_state.stack[0], hand=up.hand)


def do_card_action(
    gs: GameState,
) -> None | tuple[Card, Card] | tuple[Card, Card, Card, Card]:
    # Todo: Argh, this is mutating game_state. Maybe I could hand a new state back
    match gs.stack[0].face:
        case "reverse":
            print("Reversing play direction")
            gs.player_cycle.reverse()
            return
        case "skip":
            skipped = next(gs.player_cycle)
            print(f"Skipping player: {skipped.name}")
            return
        case "draw2":
            print("Düdum, you have to draw 2")
            return (gs.deck.pop(), gs.deck.pop())
        case "draw4":
            print("Puh, you have to draw 4")
            return (gs.deck.pop(), gs.deck.pop(), gs.deck.pop(), gs.deck.pop())
        case _:
            print("No specific action to take")


def playables(top_card: Card, hand: Hand):
    playables = [
        c
        for c in hand
        if (c.color == top_card.color)
        or (c.face == top_card.face)
        or (c.color == "wild")
    ]
    print(f"Playble: {playables}")
    return playables


if __name__ == "__main__":
    main()
