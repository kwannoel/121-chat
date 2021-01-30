use actix::prelude::*;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use uuid::Uuid;


/* MAIN CONTROL */

pub fn start_chat_service(stream: TcpStream) {

    /* INIT */
    let stream_handle = Arc::new(Mutex::new(stream));
    let mut event_queue = vec![];

    let sender_srv = sender::start(stream_handle);
    let output_srv = output::start();
    let receiver_srv = receiver::start(event_queue, stream_handle);
    let input_srv = input::start(event_queue);

    /* EVENT LOOP */
    start_event_loop(event_queue, sender_srv, output_srv);

    return Ok(());
}

fn start_event_loop(event_queue: Vec<Event>, sender_srv: SenderSrv, output_srv: OuputSrv) {
     loop {
        match event_queue.next() {
            Ok(event) => match event {
                RecvMsg(uuid, msg) => sender_srv.dispatch(SenderEvent::Ack(uuid)),
                SendMsg(msg) => sender_srv.dispatch(SenderEvent::Msg(msg)),
                AckMsg(uuid) => sender_srv.dispatch(SenderEvent::Acked(uuid)),
            }
            _ => { continue; return Ok(()); }
        }
    }
}

pub enum Event
{
    RecvMsg(Uuid, Vec<u8>),
    SendMsg(Vec<u8>),
    AckMsg(Uuid),
}
