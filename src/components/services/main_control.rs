use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub mod sender;
pub mod receiver;
pub mod input;
pub mod messages;

/* MAIN CONTROL */

pub fn start_chat_service(stream_handle: Arc<Mutex<TcpStream>>) -> std::io::Result<()> {

    /* INIT */
    let mut event_queue = Arc::new(Mutex::new(vec![]));

    let sender_srv = sender::start(stream_handle.clone());
    let receiver_srv = receiver::start(event_queue.clone(), stream_handle.clone());
    let input_srv = input::start(event_queue.clone());

    /* EVENT LOOP */
    start_event_loop(event_queue.clone(), sender_srv);

    return Ok(());
}

fn start_event_loop(event_queue: Arc<Mutex<Vec<Event>>>, sender_srv: sender::SenderSrv) {
    let events = event_queue.iter_mut();
    loop {
        match events.next() {
            Some(event) => match event {
                Event::RecvMsg(uuid, msg) => sender_srv.dispatch(sender::SenderEvent::Ack(uuid.clone())), // Send msg to stdout
                Event::SendMsg(msg) => sender_srv.dispatch(sender::SenderEvent::Msg(msg.to_vec())),
                Event::AckMsg(uuid) => sender_srv.dispatch(sender::SenderEvent::Acked(uuid.clone())),
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
