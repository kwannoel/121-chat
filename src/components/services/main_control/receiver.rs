use std::thread;
use tokio::net::TcpStream;
use tokio::net::tcp::OwnedReadHalf;
use tokio::io::AsyncReadExt;
use std::io::Read;
use tokio::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use super::messages::Message;
use crate::services::main_control::Event;

pub async fn start(event_sender: Sender<Event>, mut socket_read: OwnedReadHalf) {
    /* EVENT LOOP */
    loop {
        let mut pkt_len: [u8;1] = [0; 1];
        match socket_read.read(&mut pkt_len).await {
            Ok(n) => {
                if n == 1 {
                    let mut msg = vec![0; pkt_len[0] as usize];
                    match socket_read.read(&mut msg).await {
                        Ok(n) => {
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
                        }
                    }
                }
            },
            _ => {
                println!("Failed to get msg");
            },
        }
    }
}
