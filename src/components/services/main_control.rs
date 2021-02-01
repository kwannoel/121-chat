use log::{debug, error, info};
use tokio;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use uuid::Uuid;

pub mod input;
pub mod messages;
pub mod receiver;
pub mod sender;

use messages::Message;

/* MAIN CONTROL */

pub async fn start_chat_service(socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    /* INIT */
    let (event_sender, event_receiver) = channel(10);
    let (sender_srv_dispatch, sender_event_receiver) = channel(10);
    let (socket_read, socket_write) = socket.into_split();

    tokio::spawn(async move {
        if let Err(e) = sender::start(socket_write, sender_event_receiver).await {
            error!("[MSG_SND] {:?}", e);
        };
    });
    debug!("Started message sending service");

    let event_sender_2 = event_sender.clone();
    let receiver_child = tokio::spawn(async move {
        if let Err(e) = receiver::start(event_sender_2, socket_read).await {
            error!("[MSG_RECV] {:?}", e);
        }
    });
    debug!("Started message receiving service");

    let event_sender_3 = event_sender.clone();
    tokio::spawn(async move {
        if let Err(e) = input::start(event_sender_3).await {
            error!("[INPUT] {:?}", e);
        }
    });

    /* EVENT LOOP */
    start_event_loop(event_receiver, sender_srv_dispatch).await?;

    // This thread terminates when client terminates
    tokio::try_join!(receiver_child)?;

    return Ok(());
}

async fn start_event_loop(
    mut event_receiver: Receiver<Event>,
    sender_srv_dispatch: Sender<sender::SenderEvent>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        if let Some(event) = event_receiver.recv().await {
            match event {
                Event::RecvMsg(uuid, msg) => {
                    info!("[RECV]\nUUID: {:?}\nMSG: {}", uuid, Message::pretty_print_contents(&msg));
                    sender_srv_dispatch
                        .send(sender::SenderEvent::Ack(uuid.clone()))
                        .await?;
                }

                Event::SendMsg(msg) => {
                    sender_srv_dispatch
                        .send(sender::SenderEvent::Msg(msg.to_vec()))
                        .await?;
                }

                Event::AckMsg(uuid) => {
                    sender_srv_dispatch
                        .send(sender::SenderEvent::Acked(uuid.clone()))
                        .await?;
                }
                Event::ClientDc => {
                    // Terminate the event loop
                    return Ok(());
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Event {
    RecvMsg(Uuid, Vec<u8>), // Handle a received message
    SendMsg(Vec<u8>),       // Send out a message
    AckMsg(Uuid),           // Message<uuid> acknowledged
    ClientDc,               // server/client disconnected
}
