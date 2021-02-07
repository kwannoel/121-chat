use log::{error, info};
use std::collections::HashMap;
use std::time::Instant;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;

use super::messages::Message;
pub async fn start(
    mut socket_write: OwnedWriteHalf,
    mut sender_event_receiver: Receiver<SenderEvent>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut pending_queue = HashMap::new();
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
                    let now = Instant::now();
                    pending_queue.insert(*id, now);

                    info!("[SENT] {:?}", &id);
                }
                SenderEvent::Acked(id) => {
                    match pending_queue.remove(&id) {
                        Some(sent_time) => {
                            let elapsed = sent_time.elapsed();
                            info!("[ACKED] {:?}\nTime elapsed (ns): {}", &id, elapsed.as_nanos());
                        },
                        _ => {
                            error!("[MISSING] {:?}", &id);
                        }
                    }
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
