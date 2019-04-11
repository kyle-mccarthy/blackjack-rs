use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use cursive::Cursive;
use cursive::event::Event as CursiveEvent;
use cursive::logger::init;

use crate::blackjack::game::{Action, Event, Game};

mod handlers;
mod utils;
mod player_container;

pub struct Ui {
    backend: Cursive,
    rx: Receiver<Event>,
    tx: Sender<Action>,
}


impl Ui {
    pub fn new(tx: Sender<Action>, rx: Receiver<Event>) -> Ui {
        Ui {
            backend: Cursive::default(),
            rx,
            tx,
        }
    }

    pub fn attach_handlers(&mut self) {
        self.backend
            .add_global_callback(CursiveEvent::Char('?'), Cursive::toggle_debug_console);
    }

    pub fn run(&mut self) {
        self.attach_handlers();

        handlers::handle_initialization(&mut self.backend, self.tx.clone());

        loop {
            if let Ok(event) = self.rx.try_recv() {
                match event {
                    Event::PlayerAdded(result) => {
                        handlers::handle_player_added(&mut self.backend, self.tx.clone(), result);
                    }
                }
                self.backend.refresh();
            }

            self.backend.step();

            if !self.backend.is_running() {
                return;
            }

            thread::sleep(Duration::from_millis(25));
        }
    }
}

pub fn start_ui() {
    let running = Arc::new(AtomicBool::new(true));
    init();

    thread::spawn(move || {
        let mut handles: Vec<JoinHandle<()>> = vec![];

        let mut game = Game::new();

        // start the UI in one thread
        let is_running = running.clone();

        let tx = game.get_actions_sender();
        let rx = game.get_events_receiver();

        handles.push(thread::spawn(move || {
            let mut ui = Ui::new(tx, rx);

            // cursive seems to capture the stop command itself
            // when ctrl+c is triggered cursive knows that it has stopped running
            // which exits the while loop, the setting is_running to false causes
            // the app to exit
            ui.run();

            // trigger the app to exit
            is_running.store(false, SeqCst);
        }));

        // start the app in another thread
        let is_running = running.clone();
        handles.push(thread::spawn(move || {
            while is_running.load(SeqCst) {
                game.step();
                thread::sleep(Duration::from_millis(25));
            }
        }));

        // on complete join
        for handle in handles {
            handle.join().unwrap();
        }
    })
        .join()
        .unwrap();
}
