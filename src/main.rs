#[macro_use]
extern crate log;
extern crate strum;
#[macro_use]
extern crate strum_macros;

use blackjack::ui::start_ui;

mod blackjack;
mod cards;

fn main() {
    start_ui();
}
