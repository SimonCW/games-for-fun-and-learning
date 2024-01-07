#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
use crate::cards::{Card, ColoredCard, CommunityCards};
use crate::player::{Player, PlayerNameCycle};
use std::collections::HashMap;

pub mod cards;
pub mod player;

pub fn main() {
    // Game Setup
    let mut ccards = CommunityCards::new();
    let player_names: Vec<String> = vec!["Jane", "Walther", "Jojo"]
        .iter()
        .map(std::string::ToString::to_string)
        .collect();
    let mut player_cycle = PlayerNameCycle::new(player_names.clone());
    let mut players: HashMap<String, Player> = HashMap::new();
    for name in player_names {
        players.insert(
            name.clone(),
            Player {
                name,
                hand: ccards.draw(7),
            },
        );
    }

    let mut round = 1;
    let top_card = ccards.draw(1).pop().unwrap();
    ccards.add_to_top_of_pile(top_card);
    loop {
        println!("Round {round}");
        let up = players
            .get_mut(
                &player_cycle
                    .next()
                    .expect("Infinite Iterator will always be Some"),
            )
            .expect("Must be present or game setup is broken");

        if round == 7 {
            player_cycle.reverse();
        }
        if round == 10 {
            break;
        }
        round += 1;
    }
}

fn play_turn(up: String, top_card: Card, deck: &mut CommunityCards) -> Card {
    todo!()
}
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
