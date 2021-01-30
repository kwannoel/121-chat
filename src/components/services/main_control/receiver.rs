use std::thread;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::io::Read;

use super::messages::Message;

use crate::services::main_control::Event;

pub fn start(event_queue: Arc<Mutex<Vec<Event>>>, stream: Arc<Mutex<TcpStream>>) {
    /* EVENT LOOP */
    loop {
        let mut msg = vec![];
        let stream_handle = stream.lock().unwrap();
        match stream_handle.read_to_end(&mut msg) {
            Ok(n) => {
                // Message received
                if n > 0 {
                    match Message::deserialize(msg) {
                        Message::Ack(uuid) => {
                            println!("time"); // TODO
                        },
                        Message::Msg(uuid, msg) => {
                            println!("message received");
                            // Send ack
                        }
                    }
                }
            },
            _ => { continue; }
        }
    }
}
