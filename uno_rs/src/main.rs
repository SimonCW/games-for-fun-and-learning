#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
use crate::state::Deck;

pub mod state;

pub fn main() {
    let deck = Deck::new();
    let player_names = vec!["Jane", "Walther", "Jojo"];
    let last_index = player_names.len() - 1;

    println!("{}", player_names[last_index]);

    let mut i = 0;
    loop {
        println!("Hi");
        i += 1;
        if i > 5 {
            break;
        }
    }
}

// This is supid. There probably is a better way in Rust but I don't have internet atm
pub struct PlayerCycle {
    items: Vec<String>,
    last_index: usize,
    pos: usize,
    direction: i8,
}

impl PlayerCycle {
    pub fn new(names: Vec<String>) -> PlayerCycle {
        PlayerCycle {
            last_index: &names.len() - 1,
            items: names,
            pos: 0,
            direction: 1,
        }
    }
}
//     def __init__(self, players: list[T]) -> None:
//         self._items = list(players)
//         self._pos = None
//         # 1 for normal direction, -1 for reversed
//         self._direction = 1
//
//     def __iter__(self) -> "PlayerCycle[T]":
//         return self
//
//     def __next__(self) -> T:
//         # First play in the game
//         if self._pos is None:
//             self._pos = 0 if self._direction == 1 else -1
//             return self._items[self._pos]
//
//         # Modulo avoids positions that are "out of index".
//         self._pos = (self._direction + self._pos) % len(self._items)
//         element = self._items[self._pos]
//         return element
//
//     def reverse(self):
//         self._direction *= -1
//
//
//
// ["A", "B", "C", "D"];
//        x
// ["D", "C", "B", "A"];
