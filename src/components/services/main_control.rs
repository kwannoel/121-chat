use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, channel};
use uuid::Uuid;

pub mod sender;
pub mod receiver;
pub mod input;
pub mod messages;

/* MAIN CONTROL */

pub fn start_chat_service(stream_handle: Arc<Mutex<TcpStream>>) -> std::io::Result<()> {

    /* INIT */
    let (event_sender, event_receiver) = channel();

    let sender_srv = sender::start(stream_handle.clone());
    let receiver_srv = receiver::start(event_sender.clone(), stream_handle.clone());
    let input_srv = input::start(event_sender.clone());

    /* EVENT LOOP */
    start_event_loop(event_receiver, sender_srv);

    return Ok(());
}


fn start_event_loop(event_receiver: Receiver<Event>, mut sender_srv: sender::SenderSrv) {
    loop {
        match event_receiver.recv() {
            Ok(event) => match event {
                Event::RecvMsg(uuid, msg) => {
                    println!("received message: {:?}", msg);
                    return sender_srv.dispatch(sender::SenderEvent::Ack(uuid.clone()));
                },

                Event::SendMsg(msg) =>
                    sender_srv.dispatch(sender::SenderEvent::Msg(msg.to_vec())),

                Event::AckMsg(uuid) =>
                    sender_srv.dispatch(sender::SenderEvent::Acked(uuid.clone())),
            }
            _ => { continue; }
        }
    }
}


pub enum Event
{
    RecvMsg(Uuid, Vec<u8>),
    SendMsg(Vec<u8>),
    AckMsg(Uuid),
}
