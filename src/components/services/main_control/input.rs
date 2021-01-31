use std::io;
use std::io::Read;
use std::sync::mpsc::Sender;

use super::messages::Message;
use super::Event;

pub fn start(event_queue: Sender<Event>) {
    loop {
        let mut buffer = vec![];
        let mut stdin = io::stdin();
        stdin.read(&mut buffer);
        event_queue.send(Event::SendMsg(buffer));
    }
}
