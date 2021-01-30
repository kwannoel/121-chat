use std::thread;

use messages::Message;

pub fn start(event_queue: Vec<Event>, stream: TcpStream) {
    /* EVENT LOOP */
    loop {
        let mut msg = vec![];
        match stream.read_to_end(&msg) {
            Ok(n) => {
                // Message received
                if n > 0 {
                    match Message::deserialize(msg) {
                        Message::Ack(uuid) => {
                            println!("time"); // TODO
                        },
                        Message::Msg(uuid, msg) => {
                            println!("message received");
                            // Send ack
                        }
                    }
                }
            },
            _ => { continue; }
        }
    }
}
