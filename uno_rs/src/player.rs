use std::collections::HashMap;

use crate::cards::Card;

// TODO: Do I want clone?
#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
}

pub struct Players {
    name_cycle: PlayerNameCycle,
    map: HashMap<String, Player>,
}

impl Players {
    pub fn new(players: Vec<Player>) -> Players {
        let name_cycle = PlayerNameCycle::new(players.iter().map(|p| p.name.clone()).collect());
        let mut map: HashMap<String, Player> = HashMap::new();
        for player in players {
            map.insert(player.name.clone(), player);
        }
        Players { name_cycle, map }
    }
    pub fn reverse(&mut self) {
        self.name_cycle.reverse();
    }
    pub fn next_player(&mut self) -> &mut Player {
        // I first tried to use the iterator here but it didn't work out because it would have
        // needed unsafe code to get the mutable reference out of the iterator
        let name = self
            .name_cycle
            .next()
            .expect("Infinite Iterator will always be Some");
        self.map
            .get_mut(&name)
            .expect("Must be present or new() is broken")
    }
}

#[derive(Debug, Clone)]
// This is supid. There probably is a better way in Rust but I don't have internet atm
struct PlayerNameCycle {
    items: Vec<String>,
    pos: Option<usize>,
    direction: isize,
}

impl PlayerNameCycle {
    fn new(player_names: Vec<String>) -> PlayerNameCycle {
        PlayerNameCycle {
            items: player_names,
            pos: None,
            direction: 1,
        }
    }
    fn reverse(&mut self) {
        self.direction *= -1;
    }
}

impl Iterator for PlayerNameCycle {
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
    // TODO: Try out property based testing in Rust (akin to hypothesis in Python)
    fn test_player_cycle_works() {
        let names: Vec<String> = vec!["Jane", "Walther", "Jojo", "Alex"]
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        let mut cycle = PlayerNameCycle::new(names);
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
