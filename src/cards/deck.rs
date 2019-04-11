use crate::cards::card::Card;
use crate::cards::rank::Rank;
use crate::cards::shuffleable::Shuffleable;
use crate::cards::suit::Suit;
use std::sync::Arc;

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Arc<Card>>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Arc<Card>> = Vec::with_capacity(52);

        Suit::iter().for_each(|&suit| {
            Rank::iter().for_each(|&rank| {
                cards.push(Arc::new(Card {
                    suit,
                    rank,
                }));
            })
        });

        Deck {
            cards,
        }
    }
}

impl Shuffleable for Deck {
    fn get_cards(&mut self) -> &mut Vec<Arc<Card>> {
        &mut self.cards
    }
}
