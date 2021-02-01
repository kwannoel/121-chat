use log::{debug, error, info};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio;
use uuid::Uuid;

pub mod sender;
pub mod receiver;
pub mod input;
pub mod messages;

/* MAIN CONTROL */

pub async fn start_chat_service(socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {

    /* INIT */
    let (event_sender, event_receiver) = channel(10);
    let (sender_srv_dispatch, sender_event_receiver) = channel(10);
    let (socket_read, socket_write) = socket.into_split();

    let sender_child = tokio::spawn(async move {
        if let Err(e) = sender::start(socket_write, sender_event_receiver).await {
            error!(target: "Message sender", "{:?}", e);
        };
    });
    debug!("Started message sending service");

    let event_sender_2 = event_sender.clone();
    let receiver_child = tokio::spawn(async move {
        if let Err(e) = receiver::start(event_sender_2, socket_read).await {
            error!(target: "Message receiver", "{:?}", e);
        }
    });
    debug!("Started message receiving service");

    let event_sender_3 = event_sender.clone();
    let input_child = tokio::spawn(async move {
        if let Err(e) = input::start(event_sender_3).await {
            error!(target: "Input", "{:?}", e);
        }
    });

    /* EVENT LOOP */
    start_event_loop(event_receiver, sender_srv_dispatch).await?;
    debug!("Event loop ended");

    tokio::try_join!(
        sender_child,
        receiver_child,
        input_child
    )?;

    return Ok(());
}


async fn start_event_loop(
    mut event_receiver: Receiver<Event>,
    sender_srv_dispatch: Sender<sender::SenderEvent>)
-> Result<(), Box<dyn std::error::Error>> {
    loop {
        if let Some(event) = event_receiver.recv().await {
            match event {
                Event::RecvMsg(uuid, msg) => {
                    info!("received message: {:?}", msg);
                    sender_srv_dispatch.send(sender::SenderEvent::Ack(uuid.clone())).await?;
                },

                Event::SendMsg(msg) => {
                    info!("Sending message {:?}", msg);
                    sender_srv_dispatch.send(sender::SenderEvent::Msg(msg.to_vec())).await?;
                },

                Event::AckMsg(uuid) => {
                    sender_srv_dispatch.send(sender::SenderEvent::Acked(uuid.clone())).await?;
                },
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Event
{
    RecvMsg(Uuid, Vec<u8>),
    SendMsg(Vec<u8>),
    AckMsg(Uuid),
}
