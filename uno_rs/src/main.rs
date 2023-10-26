#![warn(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
use crate::state::Deck;

pub mod state;

pub fn main() {
    let deck = Deck::new();
    println!("{deck:?}");
}
