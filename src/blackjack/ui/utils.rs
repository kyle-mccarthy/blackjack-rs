use cursive::Cursive;
use crossbeam_channel::Sender;
use crate::blackjack::game::Action;

pub fn try_remove_layer(s: &mut Cursive, id: &str) {
    let stack = s.screen_mut();
    if let Some(layer_position) = stack.find_layer_from_id(id) {
        stack.remove_layer(layer_position);
    }
}

pub fn cb(tx: Sender<Action>, g: impl Fn(Sender<Action>, &mut Cursive)) -> impl Fn(&mut Cursive) {
    move |s: &mut Cursive| {
        let t = tx.clone();
        g(t, s);
    }
}

pub fn cb2<T: ?Sized>(tx: Sender<Action>, g: impl Fn(Sender<Action>, &mut Cursive, &T)) -> impl Fn(&mut Cursive, &T) {
    move |s: &mut Cursive, v: &T| {
        let t = tx.clone();
        g(t, s, v);
    }
}