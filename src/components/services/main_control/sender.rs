use log::info;
use std::collections::HashSet;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;

use super::messages::Message;
pub async fn start(
    mut socket_write: OwnedWriteHalf,
    mut sender_event_receiver: Receiver<SenderEvent>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut pending_queue = HashSet::new();
    loop {
        match sender_event_receiver.recv().await {
            Some(event) => match event {
                SenderEvent::Ack(id) => {
                    let msg = Message::Ack(id);
                    let msg_b = Message::serialize(&msg);
                    socket_write.write_all(&msg_b).await?;
                    info!("[ACKING] {:?}", id);
                }
                SenderEvent::Msg(msg) => {
                    let msg = Message::new(msg);
                    let msg_b = Message::serialize(&msg);

                    socket_write.write_all(&msg_b).await?;

                    let id = msg.get_uuid();
                    pending_queue.insert(*id);

                    info!("[SENT] {:?}", &id);
                }
                SenderEvent::Acked(id) => {
                    pending_queue.remove(&id);
                    info!("[ACKED] {:?}", &id);
                }
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
