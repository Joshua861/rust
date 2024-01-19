use crossterm::event::{ KeyEvent, MouseEvent };

#[derive(Clone, Copy, Debug)]
pub enum Event {
    pub enum Event {
        Tick,
        Key(KeyEvent),
        Mouse(MouseEvent),
        Resize(u16, u16),
    }
}

use std::{sync::mpsc, thread};

 /// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
    /// Event receiver channel.
    receiver: mpsc::Receiver<Event>,
    /// Event handler thread.
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}
