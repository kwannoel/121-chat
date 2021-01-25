use std::env;
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread;
use std::io::{self, Read};


//////////
/* MAIN */
//////////

fn main() -> std::io::Result<()> {
    // INITIALIZE
    let mode = Mode::new();
    match mode {
        Mode::Server => server(String::from("8000")),
        Mode::Client => client(String::from("127.0.0.1:8000")),
    }
}

////////////
/* CLIENT */
////////////

fn client(ip_addr: String) -> std::io::Result<()> {
    // Connect  to server
    let stream = TcpStream::connect(ip_addr)?;
    start_service(stream);

    Ok(())
}

////////////
/* SERVER */
////////////

fn server(port: String) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    // Connect to client
    for stream in listener.incoming() {
        start_service(stream?);
    }

    Ok(())
}

/////////////
/* SERVICE */
/////////////

fn start_service(stream: TcpStream){
    let mutex = Arc::new(Mutex::new(stream));

    // APPROACH
    // --------
    // We have 2 threads,
    // Thread 1: receives input from stdin, forwards it to server
    // Thread 2: listens for input coming from server

    let mutex_ref_1 = Arc::clone(&mutex);
    // THREAD 1: Receive input
    thread::spawn(move || {
        loop {
            let input = "hi from server".as_bytes();
            let mut stream = mutex_ref_1.lock().unwrap();
            stream.write(input); // TODO Handle error
        }
    });

    let mutex_ref_2 = Arc::clone(&mutex);
    // THREAD 2: Forward input
    thread::spawn(move || {
        loop {
            let mut b = [0; 1024];
            let mut stream = mutex_ref_2.lock().unwrap();
            print!("{:?}", stream.read(&mut b)); // TODO Handle error
        }
    });
}

////////////
/* CONFIG */
////////////

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
                Self::Client
            } else {
                Self::Server
            }
            _ => panic!("Mode not supplied")
        };
        return mode;
    }
}
