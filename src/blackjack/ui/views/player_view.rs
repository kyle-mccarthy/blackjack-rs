use crate::cards::player::Player;
use cursive::view::{Identifiable, View};
use cursive::Printer;
use std::sync::{Arc, RwLock};

pub struct PlayerView {
    player: Arc<RwLock<Player>>,
}

impl PlayerView {}

impl View for PlayerView {
    fn draw(&self, printer: &Printer) {}
}
