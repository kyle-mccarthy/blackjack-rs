use std::fmt::{Display, Formatter, Result};
use std::slice::Iter;

#[derive(Copy, Clone, PartialEq, Hash)]
#[allow(dead_code)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl Suit {
    pub fn as_str(&self) -> &str {
        match *self {
            Suit::Spade => "\u{2660}",
            Suit::Heart => "\u{2665}",
            Suit::Diamond => "\u{2666}",
            Suit::Club => "\u{2663}",
        }
    }

    pub fn iter() -> Iter<'static, Suit> {
        static SUITS: [Suit; 4] = [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club];
        SUITS.iter()
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.as_str())
    }
}
