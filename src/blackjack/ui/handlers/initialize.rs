use crossbeam_channel::Sender;
use cursive::Cursive;

use crate::blackjack::game::Action;
use crate::blackjack::ui::utils::{cb, cb2, ViewId};
use cursive::traits::*;
use cursive::view::Selector;
use cursive::views::{Dialog, EditView, IdView};
use std::rc::Rc;

pub fn handle_initialization(s: &mut Cursive, tx: Sender<Action>) {
    show_add_player_dialog(s, tx);
}

fn show_add_player_dialog<'r>(s: &mut Cursive, tx: Sender<Action>) {
    let stack = s.screen_mut();

    let tx1 = tx.clone();
    let tx2 = tx.clone();

    let add_player = IdView::new(
        ViewId::AddPlayer.to_string(),
        Dialog::new()
            .title("Add Player")
            .padding((1, 1, 1, 0))
            .content(
                EditView::new()
                    .on_submit(cb2(tx1.clone(), |tx, s, name: &str| {
                        tx.send(Action::CreateAndAddPlayer(String::from(name)))
                            .unwrap();
                    }))
                    .with_id(ViewId::AddPlayerName.to_string())
                    .fixed_width(25),
            )
            .button(
                "Ok",
                cb(tx1.clone(), |tx, s: &mut Cursive| {
                    let name: Rc<String> = s
                        .call_on_id(
                            ViewId::AddPlayerName.into(),
                            |view: &mut EditView| view.get_content(),
                        )
                        .unwrap();

                    tx.send(Action::CreateAndAddPlayer(String::from(
                        name.as_str(),
                    )))
                    .unwrap();
                }),
            ),
    );

    stack.add_layer(add_player);
}
