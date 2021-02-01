use tokio::io::AsyncReadExt;
use tokio::net::tcp::OwnedReadHalf;
use tokio::sync::mpsc::Sender;

use super::messages::Message;
use crate::services::main_control::Event;

pub async fn start(
    event_sender: Sender<Event>,
    mut socket_read: OwnedReadHalf
) -> Result<(), Box<dyn std::error::Error>> {
    /* EVENT LOOP */
    loop {
        let mut pkt_len: [u8;1] = [0; 1];
        match socket_read.read(&mut pkt_len).await {
            // Client disconnect
            Ok(0) => {
                event_sender.send(Event::ClientDc).await?
            },
            // Received packet
            Ok(1) => {
                let mut msg = vec![0; pkt_len[0] as usize];
                if let Ok(n) = socket_read.read(&mut msg).await {
                    if n > 0 {
                        if let Some(msg) = Message::deserialize(msg) {
                            match msg {
                                Message::Ack(uuid) =>
                                    event_sender.send(Event::AckMsg(uuid)).await?,
                                Message::Msg(uuid, msg_contents) =>
                                    event_sender.send(Event::RecvMsg(uuid, msg_contents)).await?,
                            }
                        }
                    }
                }
            },
            _ => {},
        }
    }
}
