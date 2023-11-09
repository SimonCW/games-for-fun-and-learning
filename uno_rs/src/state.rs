// Using these consts b/c it's the easiest way to iterate over the enums ...
const COLOR_SPECIALS: [ColoredCard; 3] =
    [ColoredCard::Skip, ColoredCard::Reverse, ColoredCard::Draw];
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
    Draw,
}

#[derive(Debug, Clone)]
pub enum Card {
    Colored(CardColor, ColoredCard),
    WildWishColor,
    WildWishColorDrawFour,
}

#[derive(Debug)]
pub struct Deck {
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
        Deck { cards }
    }
}
impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_deck_has_108() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 108);
    }
}
