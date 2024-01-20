#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::must_use_candidate)]
#![allow(unused_variables)]
#![allow(clippy::missing_panics_doc)]
use crate::cards::{Card, Color, ColoredCard, CommunityCards};
use crate::player::{Player, Players};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub mod cards;
pub mod player;

pub fn main() {
    // Game Setup
    let mut ccards = CommunityCards::new();
    let player_list: Vec<Player> = ["Jane", "Walther", "Jojo"]
        .iter()
        .map(|name| Player {
            name: (*name).to_string(),
            hand: ccards.draw(7),
        })
        .collect();
    let mut players = Players::new(player_list);
    let mut round = 1;
    let top_card = ccards.draw(1).pop().unwrap();
    ccards.add_to_top_of_pile(top_card);
    loop {
        if round != 1 {
            let top_card = ccards.draw(1).pop().unwrap();
            ccards.add_to_top_of_pile(top_card);
        }
        println!("Round {round}");
        let top_card = ccards
            .top_of_pile()
            .expect("There should be a top card at this point");
        println!("Top card: {top_card:?}");
        let up = whose_turn(top_card, &mut players);
        if round == 7 {
            players.reverse();
        }
        if round == 10 {
            break;
        }
        round += 1;
    }
}

fn whose_turn<'a>(top_card: &Card, players: &'a mut Players) -> &'a mut Player {
    match &top_card {
        // TODO: Return "up" from the match statement?
        Card::Colored(_, ColoredCard::Skip) => {
            let skipped_name = players.next_player().name.clone();
            let up = players.next_player();
            println!("Skipping player {}, it's {}'s turn", skipped_name, up.name);
            up
        }
        Card::Colored(_, ColoredCard::Reverse) => {
            players.reverse();
            let up = players.next_player();
            println!("Reversing play direction, it's {}'s turn", up.name);
            up
        }
        _ => {
            let up = players.next_player();
            println!("It's {}'s turn", up.name);
            up
        }
    }
}

fn random_color() -> Color {
    let mut rng = thread_rng();
    cards::COLORS
        .choose(&mut rng)
        .expect("Constant shouldn't be empty")
        .clone()
}

// Trying to implement the game logic in a more functional style. This would allow to test scenarios more
// easily, i.e., have one function that takes necessary inputs. But should it be a pure func and just return
// a new deck? Or should it mutate the deck and return just the topmost card or sth. like this?
// I'd like to do without mutation but that seems kinda hard with Games where basically the whole
// thing revolves around one shared mutable state.
// Still, containing the muatation in one function would be better than having it spread out all over the place.
fn play_turn(up: &mut Player, top_card: Card, ccards: &mut CommunityCards) -> Card {
    match top_card {
        Card::Colored(_, ColoredCard::DrawTwo) => {
            println!("{} draws two cards.", up.name);
            println!("Cards before drawing {}", up.hand.len());
            up.hand.extend(ccards.draw(2));
            println!("Cards after drawing {}", up.hand.len());
            // up.play(top_card);
            // Check if player has won
        }
        Card::WildWishColor => {
            let color_wish = random_color();
            // up.play(top_card);
            // Check if player has won
        }
        Card::WildWishColorDrawFour => {
            let color_wish = random_color();
            up.hand.extend(ccards.draw(4));
            // up.play(top_card);
            // Check if player has won
        }
        Card::Colored(..) => println!("No action to take"),
    }
}
// Do card action (if any), e.g. skip, reverse, draw cards, new color
// Next player (depending on skip and reverse)
// play_turn()
//

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{Card, Color, ColoredCard, CommunityCards};

    // TODO: Is there something like pytest.fixture with yield to run setup and teardown only once?
    fn create_players() -> Players {
        let mut ccards = CommunityCards::new();
        let player_list: Vec<Player> = ["Jane", "Walther", "Jojo", "Alex"]
            .iter()
            .map(|name| Player {
                name: (*name).to_string(),
                hand: ccards.draw(7),
            })
            .collect();
        Players::new(player_list)
    }

    #[test]
    fn test_should_skip() {
        let mut players = create_players();
        let top_card = Card::Colored(Color::Yellow, ColoredCard::Skip);
        let up = whose_turn(&top_card, &mut players);
        assert_eq!(up.name, "Walther".to_string());
    }

    #[test]
    fn test_should_reverse() {
        let mut players = create_players();
        let top_card = Card::Colored(Color::Yellow, ColoredCard::DrawTwo);
        let up = whose_turn(&top_card, &mut players);
        assert_eq!(up.name, "Jane".to_string());
        let top_card = Card::Colored(Color::Yellow, ColoredCard::Reverse);
        let up = whose_turn(&top_card, &mut players);
        assert_eq!(up.name, "Alex".to_string());
    }

    #[test]
    fn test_should_reverse_on_first_player() {
        let mut players = create_players();
        let top_card = Card::Colored(Color::Yellow, ColoredCard::Reverse);
        let up = whose_turn(&top_card, &mut players);
        assert_eq!(up.name, "Alex".to_string());
    }

    #[test]
    fn test_should_extend_hand() {}
}
