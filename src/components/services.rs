use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

mod main_control;

pub fn client(ip_addr: String) -> std::io::Result<()> {
    print!("Client initialized");
    let stream = TcpStream::connect(ip_addr)?;
    let stream_handle = Arc::new(Mutex::new(stream));
    main_control::start_chat_service(stream_handle.clone());

    Ok(())
}


pub fn server(port: String) -> std::io::Result<()> {
    print!("Server initialized");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    for stream in listener.incoming() {
        if let Ok(s) = stream {
            let stream_handle = Arc::new(Mutex::new(s));
            main_control::start_chat_service(stream_handle.clone());
        }
    }
    Ok(())
}
