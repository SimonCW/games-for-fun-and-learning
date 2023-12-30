#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
use crate::state::{Deck, DiscardPile, PlayerCycle};

pub mod state;

pub fn main() {
    let (mut deck, mut pile) = (Deck::new(), DiscardPile::new());
    let player_names = vec!["Jane", "Walther", "Jojo"]
        .iter()
        .map(std::string::ToString::to_string)
        .collect();
    let mut player_cycle = PlayerCycle::new(player_names);

    let mut round = 1;
    // TODO: need to interact with the pile here
    let card = deck.draw();
    loop {
        println!("Round {round}");
        let up = player_cycle
            .next()
            .expect("Infinite Iterator will always be Some");
        println!("Player: {up}");

        if round == 7 {
            player_cycle.reverse();
        }
        if round == 10 {
            break;
        }
        round += 1;
    }
}

/*
// Trying to implement the game logic in a more functional style. This would allow to test scenarios more
// easily, i.e., have one function that takes necessary inputs. But should it be a pure func and just return
// a new deck? Or should it mutate the deck and return just the topmost card or sth. like this?
// I'd like to do without mutation but that seems kinda hard with Games where basically the whole
// thing revolves around one shared mutable state.
// Still, containing the muatation in one function would be better than having it spread out all over the place.
fn play_turn(up: Player, topmost_card: Card, deck: &mut Deck) -> Card {
    todo!()
    // 1. Draw cards if "draw 2"
    // 2. Play card (or draw 1)
    // 3. Check if player has won
}
// Do card action (if any), e.g. skip, reverse, draw cards, new color
// Next player (depending on skip and reverse)
// play_turn()
//
*/

#[cfg(test)]
mod tests {
    use super::*;
}
