use env_logger;
use tokio;

mod components;

use crate::components::config::Config;
use crate::components::services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // INITIALIZE
    let config = Config::new();
    return services::start(config).await;
}
