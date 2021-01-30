use std::env;
use std::io::{self, Read};

enum Mode {
    Server,
    Client,
}

impl Mode
{
    fn new() -> Self {
        let mode_key = "MODE";
        let mode = match env::var_os(mode_key) {
            Some(val) => if val == "server" {
                Self::Server
            } else {
                Self::Client
            }
            _ => panic!("Mode not supplied")
        };
        return mode;
    }
}
