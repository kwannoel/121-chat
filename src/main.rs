mod components;

use crate::components::config;
use crate::components::services;

async fn main() -> std::io::Result<()> {
    // INITIALIZE
    let mode = config::Mode::new();
    match mode {
        Mode::Server => services::server(String::from("8000")),
        Mode::Client => services::client(String::from("127.0.0.1:8000")),
    }
}
