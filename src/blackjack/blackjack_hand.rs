use crate::blackjack::hand_value::{HandValue, WithHandValue};
use crate::blackjack::player::PlayerType;
use crate::cards::{Card, Hand};

pub enum HandStatus {
    Default,
    Played,
    DoubledDown,
    Won,
    Lost,
    Push,
    Natural, // blackjack by ace - 10-card
}

pub struct BlackjackHand<'h> {
    hand: Hand<'h>,
    player_type: PlayerType,
    status: HandStatus,
}

impl<'h> BlackjackHand<'h> {
    pub fn new(player_type: PlayerType) -> BlackjackHand<'h> {
        BlackjackHand {
            player_type,
            hand: Hand::new(),
            status: HandStatus::Default,
        }
    }

    pub fn with_cards(
        player_type: PlayerType,
        cards: Vec<&'h Card>,
    ) -> BlackjackHand<'h> {
        BlackjackHand {
            player_type,
            hand: Hand::with_cards(cards),
            status: HandStatus::Default,
        }
    }

    pub fn get_status(&self) -> &HandStatus {
        &self.status
    }

    pub fn add_card(&mut self, card: &'h Card) {
        self.hand.add_card(card)
    }

    pub fn add_cards(&mut self, cards: Vec<&'h Card>) {
        self.hand.add_cards(cards);
    }

    pub fn get_cards(&self) -> &Vec<&'h Card> {
        self.hand.get_cards()
    }

    pub fn get_card_count(&self) -> usize {
        self.hand.get_card_count()
    }

    pub fn can_split(&self) -> bool {
        match self.player_type {
            PlayerType::Dealer => false,
            PlayerType::Player => {
                self.hand.get_card_count() == 2
                    && self.hand.get_cards().get(0)
                        == self.hand.get_cards().get(1)
            }
        }
    }

    pub fn split(&mut self) -> Option<[BlackjackHand<'h>; 2]> {
        if self.can_split() && !self.is_dealer() {
            let player_type = self.player_type.clone();
            let cards = self.hand.get_cards();

            return Some([
                BlackjackHand::with_cards(
                    player_type.clone(),
                    vec![*cards.get(0).unwrap()],
                ),
                BlackjackHand::with_cards(
                    player_type,
                    vec![*cards.get(1).unwrap()],
                ),
            ]);
        }
        None
    }

    pub fn can_double_down(&self) -> bool {
        match &self.player_type {
            PlayerType::Dealer => false,
            PlayerType::Player => self.hand.get_card_count() == 2,
        }
    }

    pub fn can_hit(&self) -> bool {
        match self.player_type {
            PlayerType::Player => match self.get_value() {
                Some(HandValue::V(n)) => n < 21,
                Some(HandValue::Ace(_, high)) => high < 21,
                _ => false,
            },
            PlayerType::Dealer => match self.get_value() {
                Some(HandValue::V(n)) => n < 17,
                Some(HandValue::Ace(low, high)) => low <= 17 || high < 17, // hit on soft 17
                _ => false,
            },
        }
    }

    pub fn is_dealer(&self) -> bool {
        self.player_type == PlayerType::Dealer
    }

    pub fn reset(&mut self) {
        self.hand.reset_cards();
    }
}

impl<'h> WithHandValue for BlackjackHand<'h> {
    fn get_cards(&self) -> &Vec<&Card> {
        self.hand.get_cards()
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

        let mut player = BlackjackHand::new(PlayerType::Player);
        let mut dealer = BlackjackHand::new(PlayerType::Dealer);

        player.add_card(&card);
        player.add_card(&card2);

        dealer.add_card(&card);
        dealer.add_card(&card2);

        assert!(player.can_split());
        assert!(!dealer.can_split());
    }

    #[test]
    fn it_does_split() {
        let card1 = Card::from(Suit::Club, Rank::Eight);
        let card2 = Card::from(Suit::Club, Rank::Eight);

        let mut hand = BlackjackHand::new(PlayerType::Player);

        hand.add_card(&card1);
        hand.add_card(&card2);

        let hands = hand.split();

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

        let player =
            BlackjackHand::with_cards(PlayerType::Player, vec![&card1, &card2]);
        let dealer =
            BlackjackHand::with_cards(PlayerType::Dealer, vec![&card1, &card2]);

        assert!(player.can_double_down());
        assert!(!dealer.can_double_down());
    }

    #[test]
    fn can_hit_player() {
        let card1 = Card::from(Suit::Club, Rank::Five);
        let card2 = Card::from(Suit::Club, Rank::Seven);

        let mut player =
            BlackjackHand::with_cards(PlayerType::Player, vec![&card1, &card2]);

        assert!(player.can_hit());

        let card3 = Card::from(Suit::Club, Rank::King);
        player.add_card(&card3);

        assert!(!player.can_hit());
    }

    #[test]
    fn can_hit_dealer() {
        let card1 = Card::from(Suit::Club, Rank::Three);
        let card2 = Card::from(Suit::Club, Rank::Four);

        let mut dealer =
            BlackjackHand::with_cards(PlayerType::Dealer, vec![&card1, &card2]);

        assert!(dealer.can_hit()); // 7

        let card3 = Card::from(Suit::Club, Rank::King);
        dealer.add_card(&card3);

        assert!(!dealer.can_hit()); // 17

        dealer.reset();

        let card1 = Card::from(Suit::Club, Rank::Ace);
        let card2 = Card::from(Suit::Club, Rank::Six);

        dealer.add_cards(vec![&card1, &card2]);

        assert!(dealer.can_hit()); // soft 17

        dealer.add_card(&card3);

        assert!(!dealer.can_hit()); // hard 17
    }
}
