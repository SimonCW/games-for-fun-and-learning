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
    pos: Option<usize>,
    direction: isize,
}

impl PlayerCycle {
    pub fn new(names: Vec<String>) -> PlayerCycle {
        PlayerCycle {
            last_index: &names.len() - 1,
            items: names,
            pos: None,
            direction: 1,
        }
    }
    pub fn reverse(&mut self) {
        self.direction *= -1;
    }
}

impl Iterator for PlayerCycle {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.items.len() as isize;
        // Initialize or update the position
        self.pos = Some(match self.pos {
            // rem_euclid is basically modulo but always returning positive numbers
            Some(pos) => (pos as isize + self.direction).rem_euclid(len) as usize,
            None => 0, // Default to the start if it's the first call
        });
        Some(self.items[self.pos.expect("Shouldn't be None at this point")].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_cycle_works() {
        let names: Vec<String> = vec!["Jane", "Walther", "Jojo", "Alex"]
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        let mut cycle = PlayerCycle::new(names);
        assert_eq!(cycle.next(), Some("Jane".to_string()));
        assert_eq!(cycle.next(), Some("Walther".to_string()));
        assert_eq!(cycle.next(), Some("Jojo".to_string()));
        assert_eq!(cycle.next(), Some("Alex".to_string()));
        assert_eq!(cycle.next(), Some("Jane".to_string()));
        cycle.reverse();
        assert_eq!(cycle.next(), Some("Alex".to_string()));
        assert_eq!(cycle.next(), Some("Jojo".to_string()));
    }
}
