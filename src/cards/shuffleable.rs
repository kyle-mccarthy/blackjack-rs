use rand::seq::SliceRandom;

use crate::cards::card::Card;
use std::sync::Arc;

pub trait Shuffleable {
    fn get_cards(&mut self) -> &mut Vec<Arc<Card>>;

    fn shuffle(&mut self) {
        self.shuffle_rounds(2);
    }

    fn shuffle_rounds(&mut self, rounds: i32) {
        let mut rng = rand::thread_rng();
        for _round in 0..rounds {
            self.get_cards().shuffle(&mut rng)
        }
    }
}
