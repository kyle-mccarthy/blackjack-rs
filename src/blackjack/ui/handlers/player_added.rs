use std::sync::{Arc, RwLock};

use crossbeam_channel::Sender;
use cursive::direction::Direction;
use cursive::traits::View;
use cursive::view::{Offset, Position, Selector};
use cursive::views::IdView;
use cursive::Cursive;
use failure::Error;

use crate::blackjack::game::Action;
use crate::blackjack::player::Player;
use crate::blackjack::ui::containers::player_container::PlayerContainer;
use crate::blackjack::ui::utils::{try_remove_layer, ViewId};

pub fn handle_player_added(
    s: &mut Cursive,
    tx: Sender<Action>,
    results: Result<Arc<RwLock<Player>>, Error>,
) {
    match results {
        Ok(player) => {
            try_remove_layer(s, ViewId::AddPlayer);
            add_player_container(s, tx, player);
        }
        _ => {
            // @todo adding the player can't currently fail but needs some sort of error handling here for the future
        }
    }
}

fn add_player_container(
    s: &mut Cursive,
    tx: Sender<Action>,
    player: Arc<RwLock<Player>>,
) {
    let size = s.screen_size();
    let screen = s.screen_mut();

    let container_width = size.x / 2;
    let container_height = size.y / 4;

    let mut player_container =
        PlayerContainer::build(container_width, container_height, tx, player);

    let bounded_pos =
        screen.offset().saturating_add((0, size.y - container_height - 5));

    screen.add_layer_at(
        Position::new(Offset::Center, Offset::Absolute(bounded_pos.y)),
        player_container,
    );
}
