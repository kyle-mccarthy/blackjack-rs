use crossbeam_channel::Sender;
use cursive::Cursive;

use crate::blackjack::game::Action;

use strum_macros::Display;

pub fn try_remove_layer(s: &mut Cursive, id: ViewId) {
    let stack = s.screen_mut();
    let id_str: &str = id.into();

    if let Some(layer_position) = stack.find_layer_from_id(id_str) {
        stack.remove_layer(layer_position);
    }
}

pub fn cb(
    tx: Sender<Action>,
    g: impl Fn(Sender<Action>, &mut Cursive),
) -> impl Fn(&mut Cursive) {
    move |s: &mut Cursive| {
        let t = tx.clone();
        g(t, s);
    }
}

pub fn cb2<T: ?Sized>(
    tx: Sender<Action>,
    g: impl Fn(Sender<Action>, &mut Cursive, &T),
) -> impl Fn(&mut Cursive, &T) {
    move |s: &mut Cursive, v: &T| {
        let t = tx.clone();
        g(t, s, v);
    }
}

// in an attempt to organize things + prevent duplicate ids, use ViewId
#[derive(Display, IntoStaticStr)]
pub enum ViewId {
    AddPlayer,
    AddPlayerName,
    PlayerContainer,
    PlayerContainerLeftColumn,
    PlayerContainerRightColumn,
    PlayerBankroll,
    HitButton,
    StayButton,
}
