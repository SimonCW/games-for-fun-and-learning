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
        // TODO: There are 2 bugs
        // 1. Draw2 := Draw2 + Skip. RTFM dummy!
        // 2. If player B has to draw 2 and is then skipped, player C will again draw2, ...
        // Idea: A card creates an Action, e.g. Action::Draw2. This action is appeneded to a queue
        // and then popped. Hence, every action will only be excecuted once.
        println!("Round {round}");
        let top_card = ccards
            .top_of_pile()
            .expect("There should be a top card at this point")
            .to_owned();
        println!("Top card: {top_card:?}");

        // Todo: Seperate Deck and Pile. Lumping together the pile and the deck is biting me here. I cannot borrow mutably
        // here because of `top_of_pile?`. However, I'd just need the deck mutably here, not the
        // pile
        let up = whose_turn(top_card, &mut players, &mut ccards);

        println!("It's {}'s turn", up.name);
        if round == 10 {
            break;
        }
        round += 1;
    }
}

fn whose_turn<'a>(
    top_card: Card,
    players: &'a mut Players,
    ccards: &mut CommunityCards,
) -> &'a mut Player {
    let up = match &top_card {
        // TODO: Return "up" from the match statement?
        Card::Colored(_, ColoredCard::Skip) => {
            let skipped = players.next_player().name.clone();
            println!("Skipping player {}", skipped);
            let up = players.next_player();
            up
        }
        Card::Colored(_, ColoredCard::DrawTwo) => {
            let skipped = players.next_player();
            skipped.hand.extend(ccards.draw(2));
            let up = players.next_player();
            println!("{} draws two cards.", skipped.name);
            up
        }
        Card::WildWishColorDrawFour => {
            let skipped = players.next_player();
            skipped.hand.extend(ccards.draw(4));
            let up = players.next_player();
            println!("{} draws four cards.", skipped.name);
            up
        }
        Card::Colored(_, ColoredCard::Reverse) => {
            players.reverse();
            let up = players.next_player();
            println!("Reversing play direction");
            up
        }
        _ => {
            let up = players.next_player();
            up
        }
    };
    up
}

fn random_color() -> Color {
    let mut rng = thread_rng();
    cards::COLORS
        .choose(&mut rng)
        .expect("Constant shouldn't be empty")
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{Card, Color, ColoredCard, CommunityCards};

    // TODO: Is there something like pytest.fixture with yield to run setup and teardown only once?
    fn create_players(players: [&str; 4]) -> Players {
        let mut ccards = CommunityCards::new();
        let player_list: Vec<Player> = players
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
        let mut players = create_players(["Jane", "Walther", "Jojo", "Alex"]);
        let top_card = Card::Colored(Color::Yellow, ColoredCard::Skip);
        let up = whose_turn(&top_card, &mut players);
        assert_eq!(up.name, "Walther".to_string());
    }

    #[test]
    fn test_should_skip_on_draw2() {
        let mut players = create_players(["Jane", "Walther", "Jojo", "Alex"]);
        let top_card = Card::Colored(Color::Yellow, ColoredCard::DrawTwo);
        let up = whose_turn(&top_card, &mut players);
        assert_eq!(up.name, "Walther".to_string());
    }

    #[test]
    fn test_should_reverse() {
        let mut players = create_players(["Jane", "Walther", "Jojo", "Alex"]);
        let top_card = Card::Colored(Color::Yellow, ColoredCard::Number(2));
        let up = whose_turn(&top_card, &mut players);
        assert_eq!(up.name, "Jane".to_string());
        let top_card = Card::Colored(Color::Yellow, ColoredCard::Reverse);
        let up = whose_turn(&top_card, &mut players);
        assert_eq!(up.name, "Alex".to_string());
    }

    #[test]
    fn test_should_reverse_on_first_player() {
        let mut players = create_players(["Jane", "Walther", "Jojo", "Alex"]);
        let top_card = Card::Colored(Color::Yellow, ColoredCard::Reverse);
        let up = whose_turn(&top_card, &mut players);
        assert_eq!(up.name, "Alex".to_string());
    }

    #[test]
    fn test_should_extend_hand() {}
}
