use std::thread;
use tokio::net::TcpStream;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncReadExt;
use std::io::Read;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use super::messages::Message;
use crate::services::main_control::Event;

pub fn start(event_sender: Sender<Event>, stream: Arc<Mutex<TcpStream>>) {
    /* EVENT LOOP */
    loop {
        thread::sleep(Duration::from_millis(10));
        let mut msg;
        let mut pkt_len: [u8;1] = [0; 1];
        println!("lock taken");
        match stream.try_lock() {
            Ok(mut stream_handle) => {
                println!("trying to read");
                match stream_handle.read(&mut pkt_len) {
                    Ok(n) => {
                        if n == 1 {
                            println!("{}", pkt_len[0]);
                            msg = vec![0; pkt_len[0] as usize];
                            match stream_handle.read(&mut msg) {
                                Ok(n) => {
                                    drop(stream_handle);
                                    println!("lock released");
                                    // Message received
                                    println!("message {:?}", msg.clone());
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
                                _ => {
                                    println!("Failed to get msg");
                                    drop(stream_handle);
                                }
                            }
                        } else {
                            drop(stream_handle);
                            println!("lock released");
                        }
                    },
                    Err(e) => {
                        println!("{:?}", e);
                        println!("lock released");
                        drop(stream_handle);
                    }
                }
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }

    }
}
