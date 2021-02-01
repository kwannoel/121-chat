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
        println!("asd2");
        match sender_event_receiver.recv() {
            Ok(event) => match event {
                SenderEvent::Ack(uuid) => {
                    loop {
                        match stream_handle.try_lock() {
                            Ok(mut stream) => {
                                println!("Sending ack");
                                let msg = Message::Ack(uuid);
                                let msg_b = Message::serialize(msg);

                                stream.write(&msg_b);
                                drop(stream);
                                break;
                            },
                            Err(e) => {
                                println!("trying to ack");
                                println!("{:?}", e);
                            }
                        }
                    }
                },
                SenderEvent::Msg(msg) => {
                    println!("dispatched message {:?}", msg);
                    loop {
                        match stream_handle.try_lock() {
                            Ok(mut stream) => {
                                let msg = Message::new(msg);
                                let msg_b = Message::serialize(msg.clone());
                                println!("serialized message");

                                stream.write(&msg_b);
                                drop(stream);

                                let uuid = msg.get_uuid();
                                pending_queue.insert(*uuid);
                                println!("inserted message");
                                println!("sent message {:?}", msg_b);
                                break;
                            },
                            Err(e) => {
                                println!("Cannot acquire lock");
                                println!("{:?}", e);
                            }
                        }
                    }
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
