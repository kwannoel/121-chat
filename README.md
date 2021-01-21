# Flow

- [Server] Wait for client to connect
- [Server] Client connected -> "Message received" + log request (just dump bytes since that is most general)
- [Client] Server "Message received" -> log roundtrip time
- [Server] Received roundtrip time -> Disconnect
- Repeat.

## Intiial observations

- Since connecting, disconnecting is done *explicitly*, we need lower level network interface.
- Synchronous request processing i.e. one client at a time, block on the client.
