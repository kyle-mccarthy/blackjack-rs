use std::borrow::Cow;
use std::rc::Rc;

use crossbeam_channel::{unbounded, Receiver, Sender};

#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub enum Phase {
    Initial,
    Betting,
    Dealing,
    Playing,
    Results,
}

#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct LifecyclePhase {
    prev: Option<Phase>,
    curr: Phase,
    index: usize,
}

impl Default for LifecyclePhase {
    fn default() -> LifecyclePhase {
        LifecyclePhase {
            prev: None,
            curr: Phase::Initial,
            index: 0,
        }
    }
}

impl LifecyclePhase {
    pub fn new() -> LifecyclePhase {
        Default::default()
    }

    pub fn phases() -> [Phase; 5] {
        [
            Phase::Initial,
            Phase::Betting,
            Phase::Dealing,
            Phase::Playing,
            Phase::Results,
        ]
    }

    pub fn from_index(index: usize) -> Option<Phase> {
        if let Some(phase) = Self::phases().get(index).take() {
            return Some(phase.to_owned());
        }
        None
    }

    pub fn curr(&self) -> &Phase {
        &self.curr
    }

    pub fn prev(&self) -> &Option<Phase> {
        &self.prev
    }

    pub fn reset(&mut self) {
        self.prev = None;
        self.curr = Phase::Initial;
        self.index = 0;
    }
}

impl Iterator for LifecyclePhase {
    type Item = Phase;

    fn next(&mut self) -> Option<Phase> {
        let next_index = self.index + 1;
        match Self::from_index(next_index) {
            Some(phase) => {
                self.prev = Some(self.curr.to_owned());
                self.curr = phase;
                self.index += 1;
                Some(self.curr.to_owned())
            }
            None => None,
        }
    }
}

pub struct Lifecycle {
    phase: LifecyclePhase,
}

impl Default for Lifecycle {
    fn default() -> Lifecycle {
        Lifecycle {
            phase: LifecyclePhase::new(),
        }
    }
}

impl Lifecycle {
    pub fn new() -> Lifecycle {
        Default::default()
    }

    pub fn get_phase(&self) -> &LifecyclePhase {
        &self.phase
    }

    pub fn get_phase_mut(&mut self) -> &mut LifecyclePhase {
        &mut self.phase
    }

    pub fn next_phase(&mut self) -> Option<Phase> {
        self.phase.next()
    }
}

#[cfg(test)]
mod tests {
    use crate::blackjack::game::lifecycle::{Lifecycle, Phase};

    #[test]
    fn does_start_with_initial_phase() {
        let lifecycle = Lifecycle::new();

        assert_eq!(lifecycle.get_phase().curr(), &Phase::Initial);
    }

    #[test]
    fn does_get_next_phase() {
        let mut lifecycle = Lifecycle::new();

        let next = lifecycle.get_phase_mut().next();

        assert!(next.is_some());
        assert_eq!(lifecycle.get_phase().curr(), &next.unwrap());
        assert_eq!(lifecycle.get_phase().curr(), &Phase::Betting);
        assert_eq!(
            lifecycle.get_phase().prev().to_owned().unwrap(),
            Phase::Initial
        );
    }
}
