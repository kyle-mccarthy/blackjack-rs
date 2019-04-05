use failure::Error;

use crate::cards::hand::Hand;

pub enum HandType<'h> {
    Player(&'h Hand<'h>),
    Dealer(&'h Hand<'h>),
}

pub struct HandActions {}

impl<'h> HandActions {
    pub fn can_split(ht: &HandType) -> bool {
        match ht {
            HandType::Dealer(_) => false,
            HandType::Player(hand) => {
                hand.get_card_count() == 2
                    && hand.get_cards().get(0) == hand.get_cards().get(1)
            }
        }
    }

    pub fn split(ht: &'h HandType) -> Option<[Hand<'h>; 2]> {
        if Self::can_split(ht) {
            if let HandType::Player(hand) = ht {
                let cards = hand.get_cards();
                return Some([
                    Hand::with_cards(vec![*cards.get(0).unwrap()]),
                    Hand::with_cards(vec![*cards.get(1).unwrap()]),
                ]);
            }
        }
        None
    }

    pub fn can_double_down(ht: &HandType) -> bool {
        match ht {
            HandType::Dealer(_) => false,
            HandType::Player(hand) => hand.get_card_count() == 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::{Card, Hand, Rank, Suit};

    use super::*;

    #[test]
    fn it_can_split() {
        let card = Card::from(Suit::Club, Rank::Five);
        let card2 = Card::from(Suit::Club, Rank::Five);

        let mut hand = Hand::new();
        hand.add_card(&card);
        hand.add_card(&card2);

        assert!(HandActions::can_split(&HandType::Player(&hand)));
        assert!(!HandActions::can_split(&HandType::Dealer(&hand)));
    }

    #[test]
    fn it_does_split() {
        let card1 = Card::from(Suit::Club, Rank::Eight);
        let card2 = Card::from(Suit::Club, Rank::Eight);

        let mut hand = Hand::new();
        hand.add_card(&card1);
        hand.add_card(&card2);

        let hand_type = HandType::Player(&hand);

        let hands = HandActions::split(&hand_type);

        assert!(hands.is_some());

        let hands = hands.unwrap();
        let hand1 = hands.get(0).unwrap();
        let hand2 = hands.get(1).unwrap();

        assert_eq!(hand1.get_card_count(), 1);
        assert_eq!(hand2.get_card_count(), 1);

        let card1 = hand1.get_cards().first().unwrap();
        let card2 = hand2.get_cards().first().unwrap();

        assert_eq!(card1, card2);
    }

    #[test]
    fn can_double_down() {
        let card1 = Card::from(Suit::Club, Rank::Five);
        let card2 = Card::from(Suit::Club, Rank::Six);

        let hand = Hand::with_cards(vec![&card1, &card2]);
        assert!(HandActions::can_double_down(&HandType::Player(&hand)));
        assert!(!HandActions::can_double_down(&HandType::Dealer(&hand)));
    }
}
