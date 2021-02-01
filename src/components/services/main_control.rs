use std::io::prelude::*;
use tokio::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use uuid::Uuid;

pub mod sender;
pub mod receiver;
pub mod input;
pub mod messages;

/* MAIN CONTROL */

pub fn start_chat_service(stream_handle: Arc<Mutex<TcpStream>>) -> std::io::Result<()> {

    /* INIT */
    let (event_sender, event_receiver) = channel();

    let (sender_srv_dispatch, sender_event_receiver) = channel();

    let stream_handle_1 = Arc::clone(&stream_handle);
    let sender_child = thread::spawn(move || {
        sender::start(stream_handle_1, sender_event_receiver);
    });
    println!("Started message sending service");

    let stream_handle_2 = Arc::clone(&stream_handle);
    let event_sender_2 = event_sender.clone();
    let receiver_child = thread::spawn(move || {
        receiver::start(event_sender_2, stream_handle_2);
    });
    println!("Started message receiving service");

    let event_sender_3 = event_sender.clone();
    let input_child = thread::spawn(move || {
        input::start(event_sender_3);
    });

    /* EVENT LOOP */
    println!("Event loop started");
    start_event_loop(event_receiver, sender_srv_dispatch);

    println!("Event loop ended");

    sender_child.join();
    receiver_child.join();
    input_child.join();
    return Ok(());
}


fn start_event_loop(event_receiver: Receiver<Event>, sender_srv_dispatch: Sender<sender::SenderEvent>) {
    loop {
        match event_receiver.recv() {
            Ok(event) => match event {
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
            Err(e) => {
                println!("{:?}", e);
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
