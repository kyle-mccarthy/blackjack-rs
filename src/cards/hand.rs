use crate::cards::card::Card;

pub struct Hand<'h> {
    cards: Vec<&'h Card>,
}

impl<'h> Default for Hand<'h> {
    fn default() -> Hand<'h> {
        Hand { cards: vec![] }
    }
}

impl<'h> Hand<'h> {
    pub fn new() -> Hand<'h> {
        Default::default()
    }

    pub fn add_card(&mut self, card: &'h Card) {
        self.cards.push(card)
    }

    pub fn get_cards(&self) -> &Vec<&'h Card> {
        &self.cards
    }

    pub fn reset_cards(&mut self) {
        self.cards.clear()
    }

    pub fn get_card_count(&self) -> usize {
        self.cards.len()
    }
}
