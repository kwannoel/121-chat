# Task

Simple one on one chat.

Application should start in two modes:

    as a server, waiting for one client to connect or
    as a client, taking an IP address (or hostname) of server to connect to.

After connection is established, users on either side (server and client) can send messages to the other side. After connection is terminated by the client - server continues waiting for another client. The receiving side should acknowledge every incoming message (automatically send back "message received'' indication), the sending side should show the roundtrip time for acknowledgment. Wire protocol shouldn't make any assumptions on the message contents (e.g. allowed byte values, character encoding, etc).

UI is your choice - can be just a console.

Requirements:

    Application is to be compiled and run on linux
    Implementation language : OCaml and/or Rust
    Can use any 3rd-party general-purpose libraries (extlib, containers, tokio, etc)
    Primary objectives : robustness, code simplicity and maintainability.

# Usage

- Start server
``` sh
MODE=server PORT=<port> cargo run
```

- Start client
``` sh
MODE=client IP_ADDR=<ip_addr> cargo run
```

- Chatting

Type your message and send EOF with `^D`, the text will be forwarded to the other chat client.

- Disconnect

Send SIGINT with `^C`.

# Approach

Client and Server can share the same chat protocol, and utilize their own connection protocols, since their in-chat behaviour is the same.

## Architecture

We try to follow a message passing style, with `main_control` receiving event messages and handling them accordingly.
