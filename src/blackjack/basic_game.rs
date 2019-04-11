use core::borrow::Borrow;

use failure::{format_err, Error, Fail};

use crate::blackjack::player::Player;
use crate::cards::shoe::Shoe;
use crate::cards::shuffleable::Shuffleable;

#[derive(Debug, Fail)]
pub enum DealingError {
    #[fail(display = "Failed to take card from shoe")]
    TakingCardFromShoe,
    #[fail(display = "Failed to push card to player's current hand")]
    PushCardToPlayer,
}

pub struct BasicGame {
    players: Vec<Player>,
    dealer: Player,
    shoe: Shoe,
}

impl BasicGame {
    pub fn new() -> BasicGame {
        BasicGame {
            players: Vec::with_capacity(7),
            dealer: Player::new_dealer(String::from("Dealer")),
            shoe: Shoe::with_decks(6),
        }
    }

    pub fn with_n_decks(n: u32) -> BasicGame {
        BasicGame {
            players: Vec::with_capacity(7),
            dealer: Player::new_dealer(String::from("Dealer")),
            shoe: Shoe::with_decks(n),
        }
    }

    pub fn setup(&mut self) {
        self.shoe.shuffle();
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn can_add_player(&self) -> bool {
        self.players.len() < self.players.capacity()
    }

    pub fn get_players(&self) -> &Vec<Player> {
        self.players.borrow()
    }

    pub fn get_dealer(&self) -> &Player {
        &self.dealer
    }

    pub fn deal_cards(&mut self) -> Result<(), DealingError> {
        for _ in 0..2 {
            match (
                self.deal_single_card_to_all_players(),
                self.deal_card_to_dealer(),
            ) {
                (Err(e), _) | (_, Err(e)) => {
                    return Err(e);
                }
                _ => {}
            }
        }
        return Ok(());
    }

    fn deal_single_card_to_all_players(&mut self) -> Result<(), DealingError> {
        let shoe = &mut self.shoe;
        let mut result: Result<(), DealingError> = Ok(());

        self.players.iter_mut().for_each(|p| {
            let card = shoe.take_card();

            if card.is_none() {
                result = Err(DealingError::TakingCardFromShoe);
                return;
            }

            let card = card.unwrap();

            if p.push_card_to_current_hand(card.clone()).is_err() {
                result = Err(DealingError::PushCardToPlayer);
                return;
            }
        });

        result
    }

    fn deal_card_to_dealer(&mut self) -> Result<(), DealingError> {
        let shoe = &mut self.shoe;
        let mut result: Result<(), DealingError> = Ok(());

        if let Some(card) = shoe.take_card() {
            if self.dealer.push_card_to_current_hand(card.clone()).is_err() {
                result = Err(DealingError::PushCardToPlayer);
            }
        } else {
            result = Err(DealingError::TakingCardFromShoe);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_player() {
        let mut game = BasicGame::new();

        let player = Player::new(String::from("Tester"));

        assert!(game.can_add_player());

        let expected_id = player.get_id();

        game.add_player(player);

        game.get_players().iter().find(|p| p.get_id() == expected_id).is_some();
    }

    #[test]
    fn it_deals_cards() {
        let mut game = BasicGame::new();

        let player = Player::new(String::from("Tester"));
        let id = player.get_id();

        game.add_player(player);
        assert!(game.deal_cards().is_ok());

        let player = game.get_players().iter().find(|p| p.get_id() == id);
        assert!(player.is_some());

        let player = player.unwrap();
        assert!(player.get_current_hand().is_some());

        let current_hand = player.get_current_hand().unwrap();
        assert_eq!(current_hand.get_card_count(), 2);

        let dealer = game.get_dealer();
        assert!(dealer.get_current_hand().is_some());

        let current_hand = dealer.get_current_hand().unwrap();
        assert_eq!(current_hand.get_card_count(), 2);
    }

    #[test]
    fn it_does_not_deal_with_no_decks() {
        let mut game = BasicGame::with_n_decks(0);

        let player = Player::new(String::from("Tester"));

        game.add_player(player);

        let deal_cards_result = game.deal_cards();

        assert!(deal_cards_result.is_err());
    }
}
