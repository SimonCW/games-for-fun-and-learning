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

#[cfg(test)]
mod tests {
    use super::*;
}
