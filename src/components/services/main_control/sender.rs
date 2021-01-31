use std::collections::HashSet;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use std::io::Write;
use std::sync::mpsc::Receiver;

use super::messages::Message;
pub fn start(stream_handle: Arc<Mutex<TcpStream>>, sender_event_receiver: Receiver<SenderEvent>) {
    let mut pending_queue = HashSet::new();
    loop {
        match sender_event_receiver.recv() {
            Ok(event) => match event {
                SenderEvent::Ack(uuid) => {
                    let mut stream = stream_handle.lock().unwrap();
                    let msg = Message::Ack(uuid);
                    let msg_b = Message::serialize(msg);

                    stream.write(&msg_b);
                },
                SenderEvent::Msg(msg) => {
                    let mut stream = stream_handle.lock().unwrap();
                    let msg = Message::new(msg);

                    let uuid = msg.get_uuid();
                    pending_queue.insert(*uuid);

                    let msg_b = Message::serialize(msg);

                    stream.write(&msg_b);
                },
                SenderEvent::Acked(uuid) => {
                    pending_queue.remove(&uuid);
                    println!("Message acknowledged! {}", uuid);
                },
            },
            _ => {}
        }
    }
}

pub enum SenderEvent {
    Ack(Uuid),
    Msg(Vec<u8>),
    Acked(Uuid),
}
