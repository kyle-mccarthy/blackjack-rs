use std::fmt::{Debug, Display, Formatter, Result};

use crate::cards::rank::Rank;
use crate::cards::suit::Suit;

#[derive(Copy, Clone, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let rank = self.rank.as_str();
        let suit = self.suit.as_str();

        write!(f, "{}{}", suit, rank)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.rank == other.rank && self.suit == other.suit
    }
}

impl Eq for Card {}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{{ suit: {}, rank: {} }}",
            self.suit.as_str(),
            self.rank.as_str()
        )
    }
}

impl Card {
    pub fn from(suit: Suit, rank: Rank) -> Card {
        Card {
            suit,
            rank,
        }
    }
}
