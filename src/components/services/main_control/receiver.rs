use std::thread;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::io::Read;
use std::sync::mpsc::Sender;

use super::messages::Message;
use crate::services::main_control::Event;

pub fn start(event_sender: Sender<Event>, stream: Arc<Mutex<TcpStream>>) {
    /* EVENT LOOP */
    loop {
        let mut msg = vec![];
        let mut stream_handle = stream.lock().unwrap();
        match stream_handle.read_to_end(&mut msg) {
            Ok(n) => {
                // Message received
                if n > 0 {
                    match Message::deserialize(msg) {
                        Message::Ack(uuid) => {
                            event_sender.send(Event::AckMsg(uuid));
                        },
                        Message::Msg(uuid, msg_contents) => {
                            event_sender.send(Event::RecvMsg(uuid, msg_contents));
                        }
                    }
                }
            },
            _ => { continue; }
        }
    }
}
