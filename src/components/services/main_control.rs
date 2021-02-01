use std::io::prelude::*;
use tokio::net::TcpStream;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio;
use uuid::Uuid;

pub mod sender;
pub mod receiver;
pub mod input;
pub mod messages;

/* MAIN CONTROL */

pub async fn start_chat_service(socket: TcpStream) -> std::io::Result<()> {

    /* INIT */
    let (event_sender, event_receiver) = channel(100);
    let (sender_srv_dispatch, sender_event_receiver) = channel(100);
    let (socket_read, socket_write) = socket.into_split();

    let sender_child = tokio::spawn(async move {
        sender::start(socket_write, sender_event_receiver).await;
    });
    println!("Started message sending service");

    let event_sender_2 = event_sender.clone();
    let receiver_child = tokio::spawn(async move {
        receiver::start(event_sender_2, socket_read).await;
    });
    println!("Started message receiving service");

    let event_sender_3 = event_sender.clone();
    let input_child = tokio::spawn(async move {
        input::start(event_sender_3);
    });

    /* EVENT LOOP */
    println!("Event loop started");
    start_event_loop(event_receiver, sender_srv_dispatch).await;

    println!("Event loop ended");

    tokio::join!(
        sender_child,
        receiver_child,
        input_child
    );
    return Ok(());
}


async fn start_event_loop(mut event_receiver: Receiver<Event>, sender_srv_dispatch: Sender<sender::SenderEvent>) {
    loop {
        match event_receiver.recv().await {
            Some(event) => match event {
                Event::RecvMsg(uuid, msg) => {
                    println!("received message: {:?}", msg);
                    sender_srv_dispatch.send(sender::SenderEvent::Ack(uuid.clone()));
                },

                Event::SendMsg(msg) => {
                    println!("Sending message {:?}", msg);
                    sender_srv_dispatch.send(sender::SenderEvent::Msg(msg.to_vec()));
                },

                Event::AckMsg(uuid) => {
                    sender_srv_dispatch.send(sender::SenderEvent::Acked(uuid.clone()));
                },
            }
            None => {
                println!("No Events");
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
