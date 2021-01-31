use std::collections::HashSet;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use std::io::Write;

use super::messages::Message;

pub fn start(stream_handle: Arc<Mutex<TcpStream>>) -> SenderSrv {

    /* SENDER CONTROL */
    return SenderSrv::new(stream_handle);

    // // Use this to indicate current message has been sent and we can continue
    // let (sender, receiver) = mpsc::channel();

    // // THREAD 1: Send msg
    // let mutex_ref_1 = Arc::clone(&mutex);
    // let send_handle = thread::spawn(move || {
    //     loop {
    //         let input = [1, 1, 1, 1]; // TODO read from stdin
    //         let mut stream = mutex_ref_1.lock().unwrap();
    //         stream.write(input); // TODO Handle error
    //         loop {

    //         }
    //     }
    // });

    // // THREAD 2: Receive msg
    // let mutex_ref_2 = Arc::clone(&mutex);
    // let receive_handle = thread::spawn(move || {
    //     loop {
    //         let mut b = [0; 4];
    //         let mut stream = mutex_ref_2.lock().unwrap();
    //         match stream.read(&mut b) {

    //             // Fail to receive message
    //             Err(_) => {
    //                 continue;
    //             }
    //         }
    //     }
    // });

    // send_handle.join();
    // receive_handle.join();

}

pub struct SenderSrv {
    stream_handle: Arc<Mutex<TcpStream>>,
    pending_queue: HashSet<Uuid>,
}

impl SenderSrv {
    pub fn new(stream_handle: Arc<Mutex<TcpStream>>) -> Self {
        Self { stream_handle, pending_queue: HashSet::new() }
    }

    pub fn dispatch(&mut self, event: SenderEvent) {
        match event {
            SenderEvent::Ack(uuid) => {
                let mut stream = self.stream_handle.lock().unwrap();
                let msg = Message::Ack(uuid);
                let msg_b = Message::serialize(msg);

                stream.write(&msg_b);
            },
            SenderEvent::Msg(msg) => {
                let mut stream = self.stream_handle.lock().unwrap();
                let msg = Message::new(msg);

                let uuid = msg.get_uuid();
                self.pending_queue.insert(*uuid);

                let msg_b = Message::serialize(msg);

                stream.write(&msg_b);
            },
            SenderEvent::Acked(uuid) => {
                &self.pending_queue.remove(&uuid);
                println!("Message acknowledged! {}", uuid);
            },
        }
    }
}

pub enum SenderEvent {
    Ack(Uuid),
    Msg(Vec<u8>),
    Acked(Uuid),
}
