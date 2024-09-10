use std::io::Cursor;
use svckit::{Context, FullDuplexPeer, FullDuplexPeerGateway};

fn main() {
    let mut gateway = FullDuplexPeerGateway::new();

    let peer1 = FullDuplexPeer::new("peer1", "/tmp/peer1_read_fifo", "/tmp/peer1_write_fifo");
    let peer2 = FullDuplexPeer::new("peer2", "/tmp/peer2_read_fifo", "/tmp/peer2_write_fifo");

    gateway.add_peer(peer1);
    gateway.add_peer(peer2);

    let ctx = Context;

    let mut message = Cursor::new(b"Message from peer1");
    let mut output = Cursor::new(Vec::new());

    gateway
        .send(&ctx, &mut message, 18)
        .expect("Failed to send message");
    gateway
        .receive(&ctx, &mut output, 18)
        .expect("Failed to receive message");

    println!(
        "Received message: {:?}",
        String::from_utf8(output.into_inner()).unwrap()
    );
}
