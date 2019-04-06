use failure::{format_err, Error};

use crate::blackjack::blackjack_hand::BlackjackHand;
use crate::cards::bankroll::Bankroll;
use crate::cards::card::Card;

#[derive(PartialEq, Clone)]
pub enum PlayerType {
    Player,
    Dealer,
}

pub struct Player<'h, 'p: 'h> {
    bankroll: Bankroll,
    hands: Vec<BlackjackHand<'h>>,
    current_hand: usize,
    name: &'p str,
    player_type: PlayerType,
}

impl<'h, 'p: 'h> Player<'h, 'p> {
    pub fn new(name: &'p str) -> Player<'h, 'p> {
        Player {
            bankroll: Bankroll::new(),
            hands: vec![BlackjackHand::new(PlayerType::Player)],
            current_hand: 0,
            player_type: PlayerType::Player,
            name,
        }
    }

    pub fn new_dealer(name: &'p str) -> Player<'h, 'p> {
        Player {
            bankroll: Bankroll::new(),
            hands: vec![BlackjackHand::new(PlayerType::Dealer)],
            current_hand: 0,
            player_type: PlayerType::Dealer,
            name,
        }
    }

    pub fn get_name(&self) -> &'p str {
        self.name
    }

    pub fn get_current_hand(&self) -> Option<&'h BlackjackHand> {
        self.hands.get(self.current_hand)
    }

    pub fn push_card_to_current_hand(
        &mut self,
        card: &'h Card,
    ) -> Result<(), Error> {
        if let Some(hand) = self.hands.get_mut(self.current_hand) {
            hand.add_card(card);
            return Ok(());
        }
        Err(format_err!("Could not add card"))
    }

    pub fn get_available_funds(&self) -> u32 {
        self.bankroll.get_bankroll()
    }

    pub fn get_player_type(&self) -> &PlayerType {
        &self.player_type
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::card::Card;
    use crate::cards::rank::Rank;
    use crate::cards::suit::Suit;

    use super::*;

    #[test]
    fn it_does_push_cards_to_curr_hand() {
        let mut player = Player::new("Test");
        let card = Card {
            suit: Suit::Spade,
            rank: Rank::King,
        };

        assert!(player.push_card_to_current_hand(&card).is_ok());

        let hand = player.get_current_hand();

        assert!(hand.is_some());

        let hand = hand.unwrap();

        let cards = hand.get_cards();

        assert_eq!(cards.len(), 1 as usize);

        let card_from_hand = cards.first();

        assert!(card_from_hand.is_some());

        let card_from_hand = **card_from_hand.unwrap();

        assert_eq!(card, card_from_hand);
    }
}
