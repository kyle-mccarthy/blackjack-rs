use std::sync::{Arc, RwLock};

use cursive::Cursive;
use failure::Error;

use crate::blackjack::player::Player;
use crate::blackjack::ui::utils::try_remove_layer;
use crossbeam_channel::Sender;
use crate::blackjack::game::Action;
use crate::blackjack::ui::player_container::PlayerContainer;
use cursive::view::{Position, Offset};

pub fn handle_player_added(s: &mut Cursive, tx: Sender<Action>, results: Result<Arc<RwLock<Player>>, Error>) {
    try_remove_layer(s, "add_player");
}


fn add_player_container(s: &mut Cursive) {
    let size = s.screen_size();
    let screen = s.screen_mut();

    let container_width = size.x / 2;
    let container_height = size.y / 4;

    let player_container = PlayerContainer::build(container_width, container_height);

    let bounded_pos = screen
        .offset()
        .saturating_add((0, size.y - container_height - 5));

    screen.add_layer_at(
        Position::new(Offset::Center, Offset::Absolute(bounded_pos.y)),
        player_container,
    );
}