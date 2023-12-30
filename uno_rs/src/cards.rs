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
pub struct CommunityCards {
    // TODO: Do I need to push/pop from both sides? Maybe this should be a VecDeque
    deck: Vec<Card>,
    pile: Vec<Card>,
}

impl CommunityCards {
    /// Create a new deck of 108 cards. This is the standard Uno deck prior to 2018.
    pub fn new() -> CommunityCards {
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
        CommunityCards {
            deck: cards,
            pile: Vec::new(),
        }
    }

    pub fn draw(&mut self, n: usize) -> Vec<Card> {
        if self.deck.len() < n {
            self.reshuffle_pile_as_deck();
        }
        self.deck.drain(..n).collect()
    }
    pub fn reshuffle_pile_as_deck(&mut self) {
        let mut rng = thread_rng();
        self.pile.shuffle(&mut rng);
        self.deck = self.pile.drain(..).collect();
    }
}

impl Default for CommunityCards {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_deck_has_108_cards() {
        let deck = CommunityCards::new();
        assert_eq!(deck.deck.len(), 108);
    }

    #[test]
    fn test_drawing_last_card_reshuffles_pile() {
        let mut ccards = CommunityCards::new();
        ccards.pile = ccards.deck.drain(1..).collect();
        println!("{:?}", ccards.deck);
        println!("{:?}", ccards.pile);
    }
}
