use std::io::prelude::*;
use std::net::TcpStream;
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

    let receiver_srv = receiver::start(event_sender.clone(), stream_handle.clone());
    let input_srv = input::start(event_sender.clone());
    println!("Start message receiving service");

    /* EVENT LOOP */
    println!("Event loop started");
    start_event_loop(event_receiver, sender_srv_dispatch);

    println!("Event loop ended");

    sender_child.join();
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
                    println!("Sending message");
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


pub enum Event
{
    RecvMsg(Uuid, Vec<u8>),
    SendMsg(Vec<u8>),
    AckMsg(Uuid),
}
