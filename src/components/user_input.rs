use actix::prelude::Address;

pub fn start(addr: Address, stream: TcpStream) {
    // On incoming message
    let message = RecvMessage { msg: vec![] };
    let recv_result = addr.send(message).await;

    match recv_result {
        Ok(res) => println!("Got result: {}", res.unwrap()),
        Err(err) => println!("Got error: {}", err),
    }
}
