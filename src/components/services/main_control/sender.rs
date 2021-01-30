use std::sync::{Arc, Mutex};
use uuid::Uuid;
use std::net::TcpStream;

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
    stream_handle: Arc<Mutex<TcpStream>>
}

impl SenderSrv {
    pub fn new(stream_handle: Arc<Mutex<TcpStream>>) -> Self {
        Self { stream_handle }
    }

    pub fn dispatch(&self, event: SenderEvent) {
        match event {
            SenderEvent::Ack(uuid) => {},
            SenderEvent::Msg(msg) => {},
            SenderEvent::Acked(Uuid) => {},
        }
    }
}

pub enum SenderEvent {
    Ack(Uuid),
    Msg(Vec<u8>),
    Acked(Uuid),
}
