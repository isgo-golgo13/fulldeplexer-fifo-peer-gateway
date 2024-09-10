# Full Duplex (Duplexer) POSIX FIFO Peer and Central Peer Gateway
Rust FullDuplex POSIX FIFO Peers and Central FIFO Peer Gateway using POSIX Libc


## Project Source File Structure
The following is the project source file structure.

```shell
posix-peer-gateway-sync/
├── Dockerfile.gateway
├── Dockerfile.peer
├── docker-compose.yml
├── Cargo.toml
├── Cargo.lock
├── src/
│   └── main.rs
├── svckit/
│   ├── lib.rs
│   ├── fullduplexer.rs
│   ├── fullduplex-peer.rs
│   └── fullduplex-peer-gateway.rs
```


## Build Docker Images and Run

```shell
# Build the Docker images
docker-compose build

# Start the services
docker-compose up
```
