#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
use crate::cards::{Card, ColoredCard, CommunityCards};
use crate::player::{Player, Players};
use std::collections::HashMap;

pub mod cards;
pub mod player;

pub fn main() {
    // Game Setup
    let mut ccards = CommunityCards::new();
    let player_list: Vec<Player> = ["Jane", "Walther", "Jojo"]
        .iter()
        .map(|name| Player {
            name: name.to_string(),
            hand: ccards.draw(7),
        })
        .collect();
    let mut players = Players::new(player_list);
    let mut round = 1;
    let top_card = ccards.draw(1).pop().unwrap();
    ccards.add_to_top_of_pile(top_card);
    loop {
        println!("Round {round}");
        let up = players.next_player();
        let top_card = ccards
            .top_of_pile()
            .expect("There should be a top card at this point");
        println!("Top card: {top_card:?}");
        match top_card {
            Card::Colored(_, ColoredCard::Skip) => {
                println!("Skipping player {}", up.name);
                let up = players.next_player();
            }
            Card::Colored(_, ColoredCard::Reverse) => {}
            Card::Colored(_, ColoredCard::DrawTwo) => {}
            Card::WildWishColor => {}
            Card::WildWishColorDrawFour => {}
            Card::Colored(..) => {}
        }

        if round == 7 {
            player_cycle.reverse();
        }
        if round == 10 {
            break;
        }
        round += 1;
    }
}
// fn play_turn(up: &mut Player, ccards: &mut CommunityCards) {
//     let top_card = ccards
//         .top_of_pile()
//         .expect("There should be a top card at this point");
//     match top_card {
//         Card::Colored(_, ColoredCard::Skip) => {
//         todo!()
//     }
//
//         Card::Colored(_, ColoredCard::Reverse) => {
//         todo!()
//     }
//         _ => {
//             // do nothing
//             todo!()
//         }
//
// }
// //
// fn do_card_action(card: Card) {
//     todo!()
// }

/*
// Trying to implement the game logic in a more functional style. This would allow to test scenarios more
// easily, i.e., have one function that takes necessary inputs. But should it be a pure func and just return
// a new deck? Or should it mutate the deck and return just the topmost card or sth. like this?
// I'd like to do without mutation but that seems kinda hard with Games where basically the whole
// thing revolves around one shared mutable state.
// Still, containing the muatation in one function would be better than having it spread out all over the place.
fn play_turn(up: Player, top_card: Card, deck: &mut Deck) -> Card {
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
