use crossterm::{
    event::{self, Event, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::thread;
use tokio::sync::mpsc::{self, error::TryRecvError};

use crate::Result;

#[derive(Debug)]
pub struct KeyListener {
    rx: mpsc::Receiver<KeyEvent>,
    _handle: thread::JoinHandle<()>,
}

impl KeyListener {
    pub fn new() -> Result<Self> {
        Self::bounded(1)
    }

    pub fn bounded(buffer_size: usize) -> Result<Self> {
        let (tx, rx) = mpsc::channel(buffer_size);

        let handle = thread::spawn(move || {
            if enable_raw_mode().is_err() {
                return;
            }

            loop {
                if let Ok(Event::Key(key)) = event::read() {
                    if !key.is_press() {
                        continue;
                    }

                    if tx.blocking_send(key).is_err() {
                        break;
                    }
                }
            }

            let _ = disable_raw_mode();
        });

        Ok(KeyListener {
            rx,
            _handle: handle,
        })
    }

    pub fn receiver(&self) -> &mpsc::Receiver<KeyEvent> {
        &self.rx
    }

    pub async fn recv(&mut self) -> Option<KeyEvent> {
        self.rx.recv().await
    }

    pub fn try_recv(&mut self) -> std::result::Result<KeyEvent, TryRecvError> {
        self.rx.try_recv()
    }
}

impl Drop for KeyListener {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}
