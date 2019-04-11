use std::rc::Rc;

use crate::cards::card::Card;
use std::sync::Arc;

#[derive(Clone)]
pub struct Hand {
    cards: Vec<Arc<Card>>,
}

impl Default for Hand {
    fn default() -> Hand {
        Hand {
            cards: vec![],
        }
    }
}

impl Hand {
    pub fn new() -> Hand {
        Default::default()
    }

    pub fn with_cards(cards: Vec<Arc<Card>>) -> Hand {
        Hand {
            cards,
        }
    }

    pub fn add_card(&mut self, card: Arc<Card>) {
        self.cards.push(card)
    }

    pub fn get_cards(&self) -> &Vec<Arc<Card>> {
        &self.cards
    }

    pub fn reset_cards(&mut self) {
        self.cards.clear()
    }

    pub fn get_card_count(&self) -> usize {
        self.cards.len()
    }

    pub fn add_cards(&mut self, cards: Vec<Arc<Card>>) {
        self.cards.extend(cards)
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::rank::Rank;
    use crate::cards::suit::Suit;

    use super::*;

    #[test]
    fn it_does_reset() {
        let card = Arc::new(Card::from(Suit::Club, Rank::Ace));

        let mut hand = Hand::with_cards(vec![card]);

        assert_eq!(hand.get_card_count(), 1);

        hand.reset_cards();

        assert_eq!(hand.get_card_count(), 0);
    }
}
