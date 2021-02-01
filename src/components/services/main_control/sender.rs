use std::collections::HashSet;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;

use super::messages::Message;
pub async fn start(
    mut socket_write: OwnedWriteHalf,
    mut sender_event_receiver: Receiver<SenderEvent>
) -> Result<(), Box<dyn std::error::Error>> {
    let mut pending_queue = HashSet::new();
    loop {
        match sender_event_receiver.recv().await {
            Some(event) => match event {
                SenderEvent::Ack(uuid) => {
                    let msg = Message::Ack(uuid);
                    let msg_b = Message::serialize(msg);

                    socket_write.write_all(&msg_b).await?;
                },
                SenderEvent::Msg(msg) => {
                    let msg = Message::new(msg);
                    let msg_b = Message::serialize(msg.clone());

                    socket_write.write_all(&msg_b).await?;

                    let uuid = msg.get_uuid();
                    pending_queue.insert(*uuid);
                },
                SenderEvent::Acked(uuid) => {
                    pending_queue.remove(&uuid);
                },
            },
            _ => {}
        }
    }
}

#[derive(Debug)]
pub enum SenderEvent {
    Ack(Uuid),
    Msg(Vec<u8>),
    Acked(Uuid),
}
