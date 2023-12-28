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
    pos: Option<isize>,
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
        // Modulo avoids positions that are out of bounds and negative values
        // Increment position
        if let Some(pos) = self.pos {
            // TODO: Avoid conversion and possible wrapping error
            let result = (pos + self.direction) % self.items.len() as isize;
            let non_negative_result = if result < 0 {
                result + self.items.len() as isize
            } else {
                result
            };
            self.pos = Some(non_negative_result);
        } else {
            self.pos = Some(0);
        }
        Some(
            self.items[usize::try_from(
                dbg!(self.pos).expect("Position should be set after the previous if else"),
            )
            .expect("Position shouldn't be negative or so big as to truncate")]
            .clone(),
        )
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
