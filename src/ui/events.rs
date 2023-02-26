use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use crossterm::event;
use log::error;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::ui::key::UiKey;

pub enum UiInputEvent {
    Input(UiKey),
    Tick,
}

pub struct UiEvents {
    rx: Receiver<UiInputEvent>,
    _tx: Sender<UiInputEvent>,
    stopper: Arc<AtomicBool>,
    _tui_event_reader: JoinHandle<()>,
}

impl UiEvents {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let stopper = Arc::new(AtomicBool::new(false));
        let event_tx = tx.clone();
        let event_stopper = stopper.clone();

        let t = thread::spawn(move || {
            loop {
                if event::poll(tick_rate).unwrap() {
                    if let event::Event::Key(key) = event::read().unwrap() {
                        let key = UiKey::from(key);
                        if let Err(err) = event_tx.blocking_send(UiInputEvent::Input(key)) {
                            error!("err {}", err);
                        }
                    }
                }
                else if let Err(err) = event_tx.blocking_send(UiInputEvent::Tick) {
                    error!("err {}", err);
                }

                if event_stopper.load(Ordering::Relaxed) {
                    break;
                }
            }
        });

        Self {
            rx,
            _tx: tx,
            stopper,
            _tui_event_reader: t,
        }
    }

    pub async fn next(&mut self) -> UiInputEvent {
        self.rx.recv().await.unwrap_or(UiInputEvent::Tick)
    }

    pub fn close(&mut self) {
        self.stopper.store(true, Ordering::Relaxed);
    }
}
