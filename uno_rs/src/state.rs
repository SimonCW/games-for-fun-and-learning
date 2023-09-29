const COLOR_SPECIALS: [&str; 3] = ["Draw Two", "Reverse", "Skip"];
const WILD_SPECIALS: [&str; 2] = ["Draw Four", "Wish Color"];
const COLORS: [Color; 4] = [Color::Blue, Color::Green, Color::Red, Color::Yellow];
const VALUES: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

#[derive(Debug)]
pub struct Card {
    color: Color,
    face: String,
    value: Option<u8>,
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::<Card>::new();
        for color in COLORS {
            for value in VALUES {
                let card = Card {
                    color: color.clone(),
                    value: Some(value),
                    face: value.to_string(),
                };
                cards.push(card);
            }
        }
        Deck { cards }
    }
}

#[derive(Debug, Clone)]
enum Color {
    Blue,
    Green,
    Red,
    Yellow,
}
