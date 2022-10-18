## RPC date server var grpc
Implemented in Rust. The clients get date 
from the server and display locally.

## How to run

Build:
```bash
$ cargo build
```

Run the server:
```bash
$ ./target/debug/server
```

Run a client (in a new terminal):
```bash
$ ./target/debug/client -t ysu
```