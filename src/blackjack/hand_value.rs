use crate::cards::card::Card;
use crate::cards::hand::Hand;
use crate::cards::rank::Rank;
use std::sync::Arc;

pub enum CardValue {
    Single(u8),
    Ace(u8, u8),
}

#[derive(PartialEq, Debug)]
pub enum HandValue {
    V(u8),
    Ace(u8, u8),
    Bust(u8),
}

pub trait WithHandValue {
    fn get_cards(&self) -> &Vec<Arc<Card>>;

    fn rank_value(rank: Rank) -> CardValue {
        match rank {
            Rank::Ace => CardValue::Ace(1, 11),
            Rank::Two => CardValue::Single(2),
            Rank::Three => CardValue::Single(3),
            Rank::Four => CardValue::Single(4),
            Rank::Five => CardValue::Single(5),
            Rank::Six => CardValue::Single(6),
            Rank::Seven => CardValue::Single(7),
            Rank::Eight => CardValue::Single(8),
            Rank::Nine => CardValue::Single(9),
            Rank::Ten => CardValue::Single(10),
            Rank::Jack => CardValue::Single(10),
            Rank::Queen => CardValue::Single(10),
            Rank::King => CardValue::Single(10),
        }
    }

    fn get_value(&self) -> Option<HandValue> {
        if self.get_cards().len() == 0 {
            return None;
        }

        Some(
            self.get_cards()
                .iter()
                .map(|card| Self::rank_value(card.rank))
                .fold(HandValue::V(0), |curr, val| match (curr, val) {
                    (HandValue::V(acc), CardValue::Single(v)) => {
                        self.derive_value(vec![acc + v])
                    }
                    (HandValue::V(acc), CardValue::Ace(v1, v2)) => {
                        self.derive_value(vec![acc + v1, acc + v2])
                    }
                    (HandValue::Ace(acc1, acc2), CardValue::Single(v)) => {
                        self.derive_value(vec![acc1 + v, acc2 + v])
                    }
                    (HandValue::Ace(acc1, acc2), CardValue::Ace(v1, v2)) => {
                        self.derive_value(vec![
                            acc1 + v1,
                            acc1 + v2,
                            acc2 + v1,
                            acc2 + v2,
                        ])
                    }
                    (HandValue::Bust(n), _) => HandValue::Bust(n),
                    _ => unreachable!(""),
                }),
        )
    }

    fn derive_value(&self, mut values: Vec<u8>) -> HandValue {
        values.sort_unstable();
        values.dedup();

        let less_than_21 = values.clone();
        let less_than_21: Vec<&u8> =
            less_than_21.iter().filter(|v| **v <= 21).collect();

        if let (Some(n1), Some(n2)) = (less_than_21.get(0), less_than_21.get(1))
        {
            return HandValue::Ace(**n1, **n2);
        }

        if let Some(n1) = less_than_21.get(0) {
            return HandValue::V(**n1);
        }

        if values.len() == 1 {
            if let Some(n) = values.get(0) {
                return HandValue::Bust(*n);
            }
        }

        unreachable!(format!(
            "Failed to get value from cards 0 >= n >= 3 {:?}",
            values
        ));
    }

    fn has_ace(&self) -> bool {
        let cards = self.get_cards();
        match cards.iter().find(|card| card.rank == Rank::Ace) {
            Some(_) => true,
            None => false,
        }
    }

    fn is_busted(&self) -> bool {
        match self.get_value() {
            Some(HandValue::Bust(_)) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::suit::Suit;

    use super::*;
    use std::sync::Arc;

    impl WithHandValue for Hand {
        fn get_cards(&self) -> &Vec<Arc<Card>> {
            self.get_cards()
        }
    }

    #[test]
    fn test_card_values_single_single() {
        let card1 = Arc::new(Card::from(Suit::Club, Rank::Three));
        let card2 = Arc::from(Card::from(Suit::Club, Rank::Two));

        let mut hand = Hand::new();
        hand.add_card(card1);
        hand.add_card(card2);

        assert_eq!(hand.get_value().unwrap(), HandValue::V(5));
    }

    #[test]
    fn test_card_values_single_ace() {
        let mut hand = Hand::new();

        let card1 = Arc::from(Card::from(Suit::Club, Rank::Four));
        let card2 = Arc::from(Card::from(Suit::Club, Rank::Ace));

        hand.add_card(card1);
        hand.add_card(card2);

        assert_eq!(hand.get_value().unwrap(), HandValue::Ace(5, 15));
    }

    #[test]
    fn test_card_values_ace_single() {
        let mut hand = Hand::new();

        let card1 = Arc::from(Card::from(Suit::Club, Rank::Six));
        let card2 = Arc::from(Card::from(Suit::Club, Rank::Ace));

        hand.add_card(card2);
        hand.add_card(card1);

        assert_eq!(hand.get_value().unwrap(), HandValue::Ace(7, 17));
    }

    #[test]
    fn test_card_values_ace_ace() {
        let mut hand = Hand::new();

        let card1 = Arc::from(Card::from(Suit::Club, Rank::Ace));
        let card2 = Arc::from(Card::from(Suit::Club, Rank::Ace));

        hand.add_card(card2);
        hand.add_card(card1);

        assert_eq!(hand.get_value().unwrap(), HandValue::Ace(2, 12));

        let card3 = Arc::from(Card::from(Suit::Club, Rank::Three));

        hand.add_card(card3);

        assert_eq!(hand.get_value().unwrap(), HandValue::Ace(5, 15));
    }

    #[test]
    fn test_reduces_bust_with_ace_to_single_card() {
        let mut hand = Hand::new();

        let card1 = Arc::from(Card::from(Suit::Club, Rank::Ace));
        let card2 = Arc::from(Card::from(Suit::Club, Rank::Eight));

        hand.add_card(card2);
        hand.add_card(card1);

        assert_eq!(hand.get_value().unwrap(), HandValue::Ace(9, 19));

        let card3 = Arc::from(Card::from(Suit::Club, Rank::Ace));

        hand.add_card(card3);
        assert_eq!(hand.get_value().unwrap(), HandValue::Ace(10, 20));

        let card4 = Arc::from(Card::from(Suit::Club, Rank::Four));
        hand.add_card(card4);

        assert_eq!(hand.get_value().unwrap(), HandValue::V(14));

        let card5 = Arc::from(Card::from(Suit::Club, Rank::Ace));
        hand.add_card(card5);

        assert_eq!(hand.get_value().unwrap(), HandValue::V(15));
    }

    #[test]
    fn test_can_bust() {
        let mut hand = Hand::new();

        let card1 = Arc::from(Card::from(Suit::Club, Rank::Ten));
        let card2 = Arc::from(Card::from(Suit::Club, Rank::Eight));
        let card3 = Arc::from(Card::from(Suit::Club, Rank::Ace));
        let card4 = Arc::from(Card::from(Suit::Club, Rank::Five));

        hand.add_card(card1);
        hand.add_card(card2);

        assert_eq!(hand.get_value().unwrap(), HandValue::V(18));

        hand.add_card(card3);

        assert_eq!(hand.get_value().unwrap(), HandValue::V(19));

        hand.add_card(card4);

        assert_eq!(hand.get_value().unwrap(), HandValue::Bust(24));
    }

    #[test]
    fn test_hand_value_ace_tuple_low_first_high_second() {
        let card1 = Arc::from(Card::from(Suit::Club, Rank::Eight));
        let card2 = Arc::from(Card::from(Suit::Club, Rank::Ace));

        let mut hand = Hand::with_cards(vec![card1.clone(), card2.clone()]);
        assert_eq!(hand.get_value().unwrap(), HandValue::Ace(9, 19));

        hand.reset_cards();

        let mut hand = Hand::with_cards(vec![card1.clone(), card2.clone()]);
        assert_eq!(hand.get_value().unwrap(), HandValue::Ace(9, 19));

        hand.reset_cards();

        let mut hand = Hand::new();
        hand.add_card(card1.clone());
        hand.add_card(card2.clone());
        assert_eq!(hand.get_value().unwrap(), HandValue::Ace(9, 19));

        hand.reset_cards();

        let mut hand = Hand::new();
        hand.add_card(card2.clone());
        hand.add_card(card1.clone());
        assert_eq!(hand.get_value().unwrap(), HandValue::Ace(9, 19));
    }

    #[test]
    fn test_empty_is_none() {
        let hand = Hand::new();

        assert!(hand.get_value().is_none());
    }
}
