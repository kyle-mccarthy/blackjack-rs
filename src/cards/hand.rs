use crate::cards::card::Card;

#[derive(Clone)]
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

    pub fn with_cards(cards: Vec<&'h Card>) -> Hand<'h> {
        Hand { cards }
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

    pub fn add_cards(&mut self, cards: Vec<&'h Card>) {
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
        let card = Card::from(Suit::Club, Rank::Ace);

        let mut hand = Hand::with_cards(vec![&card]);

        assert_eq!(hand.get_card_count(), 1);

        hand.reset_cards();

        assert_eq!(hand.get_card_count(), 0);
    }
}
