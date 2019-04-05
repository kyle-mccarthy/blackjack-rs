use crate::cards::card::Card;
use crate::cards::rank::Rank;
use crate::cards::shuffleable::Shuffleable;
use crate::cards::suit::Suit;

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::with_capacity(52);

        Suit::iter().for_each(|&suit| {
            Rank::iter().for_each(|&rank| {
                cards.push(Card { suit, rank });
            })
        });

        Deck { cards }
    }
}

impl Shuffleable for Deck {
    fn get_cards(&mut self) -> &mut Vec<Card> {
        &mut self.cards
    }
}
