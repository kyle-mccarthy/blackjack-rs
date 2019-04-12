use std::sync::Arc;

use failure::{format_err, Error};
use uuid::Uuid;

use crate::blackjack::blackjack_hand::BlackjackHand;
use crate::cards::bankroll::Bankroll;
use crate::cards::card::Card;

#[derive(PartialEq, Clone)]
pub enum PlayerType {
    Player,
    Dealer,
}

pub struct Player {
    id: Uuid,
    bankroll: Bankroll,
    hands: Vec<BlackjackHand>,
    current_hand: usize,
    name: String,
    player_type: PlayerType,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            id: Uuid::new_v4(),
            bankroll: Bankroll::new(),
            hands: vec![BlackjackHand::new(PlayerType::Player)],
            current_hand: 0,
            player_type: PlayerType::Player,
            name,
        }
    }

    pub fn new_dealer(name: String) -> Player {
        Player {
            id: Uuid::new_v4(),
            bankroll: Bankroll::new(),
            hands: vec![BlackjackHand::new(PlayerType::Dealer)],
            current_hand: 0,
            player_type: PlayerType::Dealer,
            name,
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_current_hand(&self) -> Option<&BlackjackHand> {
        self.hands.get(self.current_hand)
    }

    pub fn push_card_to_current_hand(
        &mut self,
        card: Arc<Card>,
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

    pub fn add_funds(&mut self, funds: u32) {
        self.bankroll.add_funds(funds);
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
        let mut player = Player::new(String::from("Test"));

        let card = Arc::new(Card::from(Suit::Spade, Rank::King));

        assert!(player.push_card_to_current_hand(card.clone()).is_ok());

        let hand = player.get_current_hand();

        assert!(hand.is_some());

        let hand = hand.unwrap();

        let cards = hand.get_cards();

        assert_eq!(cards.len(), 1 as usize);

        let card_from_hand = cards.first();

        assert!(card_from_hand.is_some());

        let card_from_hand = card_from_hand.unwrap().clone();

        assert_eq!(card, card_from_hand);
    }
}
