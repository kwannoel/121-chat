use std::net::{TcpListener, TcpStream};

pub async fn client(ip_addr: String) -> std::io::Result<()> {
    print!("Client initialized");
    let stream = TcpStream::connect(ip_addr)?;
    main_control::start_chat_service(stream).await;

    Ok(())
}


pub async fn server(port: String) -> std::io::Result<()> {
    print!("Server initialized");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    for stream in listener.incoming() {
        if let Ok(s) = stream {
            main_control::start_chat_service(s).await;
        }
    }
    Ok(())
}
