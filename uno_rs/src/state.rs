use rand::seq::SliceRandom;
use rand::thread_rng;

// Using these consts b/c it's the easiest way to iterate over the enums ...
const COLOR_SPECIALS: [ColoredCard; 3] = [
    ColoredCard::Skip,
    ColoredCard::Reverse,
    ColoredCard::DrawTwo,
];
const COLORS: [CardColor; 4] = [
    CardColor::Blue,
    CardColor::Green,
    CardColor::Red,
    CardColor::Yellow,
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardColor {
    Blue,
    Green,
    Red,
    Yellow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColoredCard {
    Number(u8),
    Skip,
    Reverse,
    DrawTwo,
}

#[derive(Debug, Clone)]
pub enum Card {
    Colored(CardColor, ColoredCard),
    WildWishColor,
    WildWishColorDrawFour,
}

#[derive(Debug)]
pub struct Deck {
    // TODO: Do I need to push/pop from both sides? Maybe this should be a VecDeque
    cards: Vec<Card>,
}

impl Deck {
    /// Create a new deck of 108 cards. This is the standard Uno deck prior to 2018.
    pub fn new() -> Deck {
        let mut cards = Vec::<Card>::new();
        for color in &COLORS {
            for _ in 0..2 {
                // Numbers except 0
                for value in 1..10 {
                    cards.push(Card::Colored(color.clone(), ColoredCard::Number(value)));
                }
                // Specials
                for special in &COLOR_SPECIALS {
                    cards.push(Card::Colored(color.clone(), special.clone()));
                }
            }
            // Number 0, 1 per color
            cards.push(Card::Colored(color.clone(), ColoredCard::Number(0)));
        }
        // Wildcards, 4 of each
        for _ in 0..4 {
            cards.push(Card::WildWishColor);
            cards.push(Card::WildWishColorDrawFour);
        }
        //shuffle
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);
        Deck { cards }
    }
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default)]
pub struct DiscardPile {
    cards: Vec<Card>,
}

impl DiscardPile {
    pub fn new() -> DiscardPile {
        DiscardPile { cards: Vec::new() }
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
    fn test_new_deck_has_108_cards() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 108);
    }

    #[test]
    // TODO: Try out property based testing in Rust (akin to hypothesis in Python)
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
