use std::io;
use std::io::Read;
use tokio::sync::mpsc::Sender;

use super::messages::Message;
use super::Event;

pub async fn start(event_queue: Sender<Event>) {
    loop {
        let mut buffer = vec![];
        let mut stdin = io::stdin();
        stdin.read_to_end(&mut buffer);
        let msg = Event::SendMsg(buffer);
        event_queue.send(msg.clone()).await;
    }
}
