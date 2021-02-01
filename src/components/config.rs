use std::env;

pub enum Config {
    Server,
    Client(String),
}

impl Config {
    pub fn new() -> Self {
        let mode_key = "MODE";
        if let Some(s) = env::var_os(mode_key) {
            if s == "server" {
                return Self::Server;
            }
            if s == "client" {
                let ip_addr_key = "IP_ADDR";
                let ip_addr = match env::var_os(ip_addr_key) {
                    Some(ia) => String::from(ia.to_str().unwrap()),
                    _ => String::from("127.0.0.1"),
                };
                return Self::Client(format!("{}:8000", ip_addr));
            }
            panic!("Invalid mode")
        }
        panic!("Mode not supplied");
    }
}
