# Task

Simple one on one chat.

Application should start in two modes:

    as a server, waiting for one client to connect or
    as a client, taking an IP address (or hostname) of server to connect to.

After connection is established, users on either side (server and client) can send messages to the other side. After connection is terminated by the client - server continues waiting for another client. The receiving side should acknowledge every incoming message (automatically send back "message received'' indication), the sending side should show the roundtrip time for acknowledgment. Wire protocol shouldn't make any assumptions on the message contents (e.g. allowed byte values, character encoding, etc).

# Usage

- Start server
``` sh
RUST_LOG=info MODE=server cargo run
```

- Start client
``` sh
RUST_LOG=info MODE=client cargo run
RUST_LOG=info MODE=client IP_ADDR=127.0.0.1 cargo run
```

- Chatting

1. Type your message
2. press `ENTER`
3. send `EOF` to terminal with `^D`

The text will then be forwarded to the other chat client.

- Disconnect

Send SIGINT with `^C`.

# Approach

Client and Server can share the same chat protocol, and utilize their own connection protocols, since their in-chat behaviour is the same.

## Architecture

We try to follow a message passing style, with `main_control` receiving event messages and handling them accordingly.

We maintain 3 threads throughout application lifetime, which communicate via `mpsc`s.

These are `receiver`, `sender` and `input`. 

The `receiver` thread listens to `TCPStream`, notifies main event loop via `Event` messsages, on incoming messages.

Likewise, `input` thread listens to `Stdin`, notifies main event loop via `Event` messages, on user input.

The `sender` thread is where `send` message jobs get dispatched to, from `main_control`.

We also use `tokio`'s runtime. This is because it can help us synchronize simultaneous `read` and `write` to a `TcpStream`, without having to resort to synchronization primitives such as `Mutex`.
