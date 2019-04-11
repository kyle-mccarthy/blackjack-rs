use std::sync::{Arc, RwLock};

use crossbeam_channel::{Receiver, Sender, unbounded};
use failure::{Error, format_err};

use crate::blackjack::game::lifecycle::{Lifecycle, Phase};
use crate::blackjack::game::state::State;
use crate::blackjack::player::Player;
use crate::cards::shuffleable::Shuffleable;

pub mod lifecycle;
pub mod state;

pub struct Channel<M> {
    tx: Sender<M>,
    rx: Receiver<M>,
}

impl<M> Channel<M> {
    fn new() -> Channel<M> {
        let (tx, rx): (Sender<M>, Receiver<M>) = unbounded();
        Channel {
            tx,
            rx,
        }
    }
}

//
//  |->start->shuffle->add_players->announce(BETTING)->PHASE_END(BETTING - TO or all bet)
//  |
//  |
//  |
//

pub enum LifecycleMessage {
    StartPhase(Phase),
    EndPhase(Phase),
}

// the incoming actions and outgoing events basically have a 1-1 correspondence, but this lets us
// validate any actions before pushing them to the ui
pub enum Action {
    AddPlayer(Player),
    CreateAndAddPlayer(String),
}


pub enum Event {
    PlayerAdded(Result<Arc<RwLock<Player>>, Error>),
}

pub struct Channels {
    lifecycle: Channel<LifecycleMessage>,
    actions: Channel<Action>,
    events: Channel<Event>,
}

impl<'g> Default for Channels {
    fn default() -> Channels {
        Channels {
            lifecycle: Channel::new(),
            actions: Channel::new(),
            events: Channel::new(),
        }
    }
}

pub struct Game<'s> {
    lifecycle: Lifecycle,
    state: State<'s>,
    pub channels: Channels,
}

impl<'s> Default for Game<'s> {
    fn default() -> Game<'s> {
        Game {
            lifecycle: Lifecycle::new(),
            state: State::new(),
            channels: Channels::default(),
        }
    }
}

impl<'s> Game<'s> {
    pub fn new() -> Game<'s> {
        Default::default()
    }

    pub fn setup(&mut self) {
        self.state.shoe_mut().shuffle();
    }

    pub fn step(&mut self) {
        let incoming_messages = self.channels.actions.rx.clone();

        incoming_messages
            .try_iter()
            .for_each(|action| self.handle_incoming_action(action));
    }

    pub fn get_channels(&self) -> &Channels {
        &self.channels
    }

    pub fn get_events_receiver(&self) -> Receiver<Event> {
        self.channels.events.rx.clone()
    }

    pub fn get_actions_sender(&self) -> Sender<Action> {
        self.channels.actions.tx.clone()
    }

    pub fn handle_incoming_action(&mut self, action: Action) {
        match action {
            Action::AddPlayer(player) => {}
            Action::CreateAndAddPlayer(name) => {
                let player = Player::new(name);
                self.add_player(player);
            }
        }
    }

    fn add_player(&mut self, player: Player) {
        let id = player.get_id();

        self.state.add_player(player);

        if let Some(player) = self.state.get_player_by_id(id).take() {
            self.channels
                .events
                .tx
                .send(Event::PlayerAdded(Ok(player))).unwrap();
        }
    }

    fn emit(&self, event: Event) {
        self.channels.events.tx.send(event).unwrap();
    }
}
