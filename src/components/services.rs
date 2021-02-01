use log::info;
use tokio::net::{TcpListener, TcpStream};

mod main_control;

use crate::components::config::Config;
use crate::components::services;

pub async fn start(conf: Config) -> Result<(), Box<dyn std::error::Error>> {
    return match conf {
        Config::Server(port) => services::server(&port).await,
        Config::Client(ip_addr) => services::client(&ip_addr).await,
    };
}

async fn client(ip_addr: &String) -> Result<(), Box<dyn std::error::Error>> {
    info!("Client initialized");

    let socket = TcpStream::connect(&ip_addr).await?;
    info!("Connected to Server: {}", &ip_addr);

    main_control::start_chat_service(socket).await?;
    info!("Server disconnected: {}", &ip_addr);

    Ok(())
}

async fn server(port: &String) -> Result<(), Box<dyn std::error::Error>> {
    info!("Server initialized");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", &port)).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        info!("Client connected: 127.0.0.1:{}", &port);
        main_control::start_chat_service(socket).await?;
        info!("Client disconnected: 127.0.0.1:{}", &port);
    }
}
