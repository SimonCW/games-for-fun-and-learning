use crate::state::Deck;

pub mod state;

pub fn main() {
    let deck = Deck::new();
    println!("{:?}", deck);
}
