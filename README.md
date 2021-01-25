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
APP_MODE=server PORT=<port> cargo run
```

- Start client
``` sh
APP_MODE=client IP_ADDR=<ip_addr> cargo run
```
