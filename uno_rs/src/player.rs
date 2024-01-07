use crate::cards::Card;

// TODO: Do I want clone?
#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
}

#[derive(Debug, Clone)]
// This is supid. There probably is a better way in Rust but I don't have internet atm
pub struct PlayerNameCycle {
    items: Vec<String>,
    pos: Option<usize>,
    direction: isize,
}

impl PlayerNameCycle {
    pub fn new(player_names: Vec<String>) -> PlayerNameCycle {
        PlayerNameCycle {
            items: player_names,
            pos: None,
            direction: 1,
        }
    }
    pub fn reverse(&mut self) {
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
