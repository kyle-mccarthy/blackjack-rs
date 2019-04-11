mod blackjack;
mod cards;

#[macro_use]
extern crate log;

use blackjack::ui::start_ui;

fn main() {
    start_ui();
}
