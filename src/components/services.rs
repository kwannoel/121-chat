use tokio::net::{TcpListener, TcpStream};

mod main_control;

pub async fn client(ip_addr: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Client initialized");

    let socket = TcpStream::connect(ip_addr.clone()).await?;
    println!("Connected to Server: {}", ip_addr);

    main_control::start_chat_service(socket).await?;
    println!("Chat service ended");

    Ok(())
}


pub async fn server(port: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Server initialized");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port.clone())).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        println!("Client connected: 127.0.0.1:{}", port);
        main_control::start_chat_service(socket).await?;
        println!("Chat service ended");
    }
}
