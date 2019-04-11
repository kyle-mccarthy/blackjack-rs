use core::borrow::Borrow;
use std::sync::{Arc, RwLock};

use failure::{Error, format_err};
use uuid::Uuid;

use crate::blackjack::player::Player;
use crate::cards::shoe::Shoe;

pub enum Status {
    INITIAL,
}

struct Round<'r> {
    players: Vec<&'r Player>,
}

pub struct State<'r> {
    players: Vec<Arc<RwLock<Player>>>,
    dealer: Player,
    shoe: Shoe,
    status: Status,
    round: Option<Round<'r>>,
}

impl<'r> Default for State<'r> {
    fn default() -> State<'r> {
        State {
            players: vec![],
            dealer: Player::new_dealer(String::from("Dealer")),
            shoe: Shoe::with_decks(6),
            status: Status::INITIAL,
            round: None,
        }
    }
}

impl<'r> State<'r> {
    pub fn new() -> State<'r> {
        Default::default()
    }

    pub fn shoe_mut(&mut self) -> &mut Shoe {
        &mut self.shoe
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(Arc::new(RwLock::new(player)));
    }

    pub fn remove_player_by_id(
        &mut self,
        player_id: Uuid,
    ) -> Result<(), Error> {
        let player_id = self.get_player_position_from_id(player_id);
        match player_id {
            Some(pos) => {
                self.players.remove(pos);
                Ok(())
            }
            None => Err(format_err!(
                "Player with that id does not exists (id = {:?})",
                player_id
            )),
        }
    }

    pub fn get_player_by_id(&self, player_id: Uuid) -> Option<Arc<RwLock<Player>>> {
        if let Some(position) = self.get_player_position_from_id(player_id) {
            if let Some(player) = self.players.get(position) {
                return Some(player.to_owned().clone());
            }
        }
        None
    }

    fn get_player_position_from_id(&self, player_id: Uuid) -> Option<usize> {
        self.players.iter().position(|p|
            p.read().unwrap().get_id() == player_id
        )
    }
}
