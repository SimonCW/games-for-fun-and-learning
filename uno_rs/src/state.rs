// Using these consts b/c it's the easiest way to iterate over the colors
const COLOR_SPECIALS: [ColoredCard; 3] =
    [ColoredCard::Skip, ColoredCard::Reverse, ColoredCard::Draw];
const WILD_SPECIALS: [&str; 2] = ["Draw Four", "Wish Color"];
const COLORS: [CardColor; 4] = [
    CardColor::Blue,
    CardColor::Green,
    CardColor::Red,
    CardColor::Yellow,
];
const VALUES: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

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
    Wild,
    WildDrawFour,
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::<Card>::new();
        for color in &COLORS {
            for _ in 0..2 {
                for value in VALUES {
                    let card = Card::Colored(color.clone(), ColoredCard::Number(value));
                    cards.push(card);
                }
                for special in &COLOR_SPECIALS {
                    let card = Card::Colored(color.clone(), special.clone());
                    cards.push(card);
                }
            }
        }

        Deck { cards }
    }
}
impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}
