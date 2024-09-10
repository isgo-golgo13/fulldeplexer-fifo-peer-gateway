# FullDuplexer FIFO Sync Peer Gateway
Rust FullDuplex POSIX FIFO Peers and Central FIFO Peer Gateway using POSIX Libc


## Project Source File Structure
The following is the project source file structure.

```shell
posix-peer-gateway-sync/
├── Makefile
├── Dockerfile.gateway
├── Dockerfile.peer
├── docker-compose.yml
├── Cargo.toml
├── Cargo.lock
├── .env
├── src/
│   └── main.rs
├── svckit/
│   ├── lib.rs
│   ├── fullduplexer-fifo-context.rs
│   ├── fullduplexer.rs
│   ├── fullduplex-peer.rs
│   └── fullduplex-peer-gateway.rs
```

## The API for the Full Duplexer FIFO Async Peer Gateway

The core of the API is provided in the `svckit` crate referencing the the following source files.

- fullduplexer.rs - This is the Rust trait that defines the synchronous send*/receive* APIs that the `FullDuplexPeer`
and the `FullDuplexPeerGateway`implement.

The trait API for `FullDuplexer` is defined as follows.

```rust
use std::io::{Read, Result, Write};

pub struct Context;

pub trait FullDuplexer {
    fn send(&self, ctx: &Context, reader: &mut dyn Read, n: i64) -> Result<usize>;
    fn send_all(&self, ctx: &Context, readers: Vec<&mut dyn Read>, n: i64) -> Result<usize>;
    fn receive(&self, ctx: &Context, writer: &mut dyn Write, n: i64) -> Result<usize>;
    fn receive_all(&self, ctx: &Context, writers: Vec<&mut dyn Write>, n: i64) -> Result<usize>;
}
```

- fullduplexer-peer.rs
- fullduplexer-peer-gateway.rs


## Build Docker Images and Run

```shell
# Build the Docker images
docker-compose build

# Start the services
docker-compose up
```
