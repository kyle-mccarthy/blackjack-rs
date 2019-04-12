use std::sync::Arc;

use crate::cards::card::Card;
use crate::cards::deck::Deck;
use crate::cards::shuffleable::Shuffleable;

#[derive(Debug)]
pub struct Shoe {
    pub cards: Vec<Arc<Card>>,
    curr_index: usize,
    round_index: usize,
}

impl Shuffleable for Shoe {
    fn get_cards(&mut self) -> &mut Vec<Arc<Card>> {
        &mut self.cards
    }
}

impl Default for Shoe {
    fn default() -> Shoe {
        Shoe {
            cards: Vec::new(),
            curr_index: 0,
            round_index: 0,
        }
    }
}

impl Shoe {
    pub fn new() -> Shoe {
        Default::default()
    }

    pub fn with_decks(count: u32) -> Shoe {
        let mut cards = Vec::new();

        for _ in 0..count {
            let mut _deck = Deck::new();
            cards.append(_deck.get_cards());
        }

        Shoe {
            cards,
            curr_index: 0,
            round_index: 0,
        }
    }

    pub fn add_deck(&mut self) {
        let mut deck = Deck::new();
        self.cards.append(deck.get_cards())
    }

    pub fn start_round(&mut self) {
        self.round_index = self.curr_index;
    }

    pub fn take_card(&mut self) -> Option<Arc<Card>> {
        match self.cards.get(self.curr_index) {
            Some(card) => {
                self.curr_index += 1;
                Some(card.clone())
            }
            None => None,
        }
    }

    pub fn get_cards_in_play(&mut self) -> Option<&[Arc<Card>]> {
        if self.curr_index >= self.round_index {
            Some(&self.cards.as_slice()[self.round_index..self.curr_index])
        } else {
            None
        }
    }

    pub fn get_percent_undealt_cards(&self) -> f32 {
        self.curr_index as f32 / self.cards.len() as f32
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn create_shoe() {
        let shoe = Shoe::new();
        assert_eq!(shoe.cards.len(), 0);
    }

    #[test]
    fn add_deck_to_shoe() {
        let mut shoe = Shoe::new();
        shoe.add_deck();
        assert_eq!(shoe.cards.len(), 52);
    }

    #[test]
    fn can_add_multiple_decks_to_shoe() {
        let mut shoe = Shoe::new();
        shoe.add_deck();
        assert_eq!(shoe.cards.len(), 52);
        shoe.add_deck();
        assert_eq!(shoe.cards.len(), 104);
    }

    #[test]
    fn shoe_with_decks() {
        let shoe = Shoe::with_decks(2);
        assert_eq!(shoe.cards.len(), 52 * 2);
    }

    #[test]
    fn does_shuffle_cards() {
        let mut shoe = Shoe::with_decks(1);

        let cards_before_shuffle = shoe.cards.to_owned();
        assert_eq!(cards_before_shuffle, shoe.cards);

        shoe.shuffle();

        let cards_after_shuffle = shoe.cards.to_owned();
        assert_eq!(cards_after_shuffle, shoe.cards);
        assert_ne!(cards_before_shuffle, cards_after_shuffle);
    }

    #[test]
    fn can_take_card() {
        let mut shoe = Shoe::with_decks(1);

        let card = shoe.take_card();
        let next_card = shoe.take_card();

        assert!(card.is_some());
        assert!(next_card.is_some());
        assert_ne!(card.unwrap(), next_card.unwrap());
        assert!(shoe.curr_index.eq(&2));
    }

    #[test]
    fn can_start_round() {
        let mut shoe = Shoe::with_decks(1);

        assert!(shoe.round_index.eq(&0));

        shoe.start_round();

        assert!(shoe.round_index.eq(&0));
    }

    #[test]
    fn should_start_round_at_current_index() {
        let mut shoe = Shoe::with_decks(1);

        {
            shoe.take_card();
            shoe.take_card();
            shoe.take_card();
        }

        shoe.start_round();
        assert!(shoe.round_index.eq(&3));
        assert!(shoe.round_index.eq(&shoe.curr_index));
    }

    #[test]
    fn should_get_cards_in_play() {
        //        let mut shoe = Shoe::with_decks(1);
        //
        //        shoe.start_round();
        //
        //        let mut taken_cards: HashSet<Rc<Card>> = HashSet::new();
        //        let card1 = shoe.take_card().unwrap();
        //        let card2 = shoe.take_card().unwrap();
        //        let card3 = shoe.take_card().unwrap();
        //        taken_cards.insert(card1);
        //        taken_cards.insert(card2);
        //        taken_cards.insert(card3);
        //
        //        let in_play_cards = shoe.get_cards_in_play().unwrap();
        //
        //        assert!(taken_cards.len().eq(&in_play_cards.iter().len()));
        //
        //        for card in in_play_cards {
        //            assert!(taken_cards.contains(card));
        //        }
    }
}
