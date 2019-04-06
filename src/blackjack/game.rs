use crate::cards::hand::Hand;
use crate::cards::player::Player;

pub struct Game {
    players: Vec<Player>,
    dealer: Player,
    shoe_size: u8,
}
