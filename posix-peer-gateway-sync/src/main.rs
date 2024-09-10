use std::io::Cursor;
use svckit::{
    fullduplexer_fifo_context::FifoContext, Context, FullDuplexPeer, FullDuplexPeerGateway,
};

fn main() {
    // Load FifoContext for peer1 and peer2 from environment variables
    let peer1_context = FifoContext::from_env();
    let peer1 = FullDuplexPeer::new(peer1_context);

    let peer2_context = FifoContext::from_env();
    let peer2 = FullDuplexPeer::new(peer2_context);

    // Initialize the gateway
    let mut gateway = FullDuplexPeerGateway::new();

    // Add peers to the gateway
    gateway.add_peer(peer1);
    gateway.add_peer(peer2);

    // Create a context for sending and receiving messages
    let ctx = Context;

    // Simulated peer communication using Cursors
    let mut message = Cursor::new(b"Message from peer1");
    let mut output = Cursor::new(Vec::new());

    // Send a message from peer1 to peer2 via the gateway
    gateway
        .send(&ctx, &mut message, 18)
        .expect("Failed to send message");

    // Receive the message at peer2
    gateway
        .receive(&ctx, &mut output, 18)
        .expect("Failed to receive message");

    // Print the received message
    println!(
        "Received message: {:?}",
        String::from_utf8(output.into_inner()).unwrap()
    );
}
