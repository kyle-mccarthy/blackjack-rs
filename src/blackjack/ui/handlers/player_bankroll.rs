use crate::blackjack::ui::utils::ViewId;
use cursive::views::{TextView, ViewRef};
use cursive::Cursive;

pub fn set_player_bankroll(s: &mut Cursive, value: u32) {
    let mut view: Option<ViewRef<TextView>> =
        s.find_id(ViewId::PlayerBankroll.into());
    if let Some(mut view) = view {
        view.set_content(format!("Bankroll: {}", value));
    }
}
