use log::info;
use tokio::net::{TcpListener, TcpStream};

mod main_control;

pub async fn client(ip_addr: String) -> Result<(), Box<dyn std::error::Error>> {
    info!("Client initialized");

    let socket = TcpStream::connect(ip_addr.clone()).await?;
    info!("Connected to Server: {}", ip_addr);

    main_control::start_chat_service(socket).await?;
    info!("Server disconnected: {}", ip_addr);

    Ok(())
}

pub async fn server(port: String) -> Result<(), Box<dyn std::error::Error>> {
    info!("Server initialized");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port.clone())).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        info!("Client connected: 127.0.0.1:{}", port);
        main_control::start_chat_service(socket).await?;
        info!("Client disconnected: 127.0.0.1:{}", port);
    }
}
