use super::{Context, FullDuplexPeer, FullDuplexer};
use std::boxed::Box;
use std::collections::HashMap;
use std::io::{Read, Result, Write};

pub struct FullDuplexPeerGateway {
    active_peers: Box<HashMap<String, FullDuplexPeer>>,
    inactive_peers: Box<HashMap<String, FullDuplexPeer>>,
}

impl FullDuplexPeerGateway {
    pub fn new() -> Self {
        Self {
            active_peers: Box::new(HashMap::new()),
            inactive_peers: Box::new(HashMap::new()),
        }
    }

    pub fn add_peer(&mut self, peer: FullDuplexPeer) {
        self.active_peers.insert(peer.id.clone(), peer);
    }

    pub fn remove_peer(&mut self, peer_id: &str) {
        if let Some(peer) = self.active_peers.remove(peer_id) {
            self.inactive_peers.insert(peer_id.to_string(), peer);
        }
    }

    pub fn get_peer(&self, peer_id: &str) -> Option<&FullDuplexPeer> {
        self.active_peers.get(peer_id)
    }
}

impl FullDuplexer for FullDuplexPeerGateway {
    fn send(&self, ctx: &Context, reader: &mut dyn Read, n: i64) -> Result<usize> {
        for peer in self.active_peers.values() {
            peer.send(ctx, reader, n)?;
        }
        Ok(n as usize)
    }

    fn send_all(&self, ctx: &Context, readers: Vec<&mut dyn Read>, n: i64) -> Result<usize> {
        let mut total_bytes = 0;
        for reader in readers {
            total_bytes += self.send(ctx, reader, n)?;
        }
        Ok(total_bytes)
    }

    fn receive(&self, ctx: &Context, writer: &mut dyn Write, n: i64) -> Result<usize> {
        for peer in self.active_peers.values() {
            peer.receive(ctx, writer, n)?;
        }
        Ok(n as usize)
    }

    fn receive_all(&self, ctx: &Context, writers: Vec<&mut dyn Write>, n: i64) -> Result<usize> {
        let mut total_bytes = 0;
        for writer in writers {
            total_bytes += self.receive(ctx, writer, n)?;
        }
        Ok(total_bytes)
    }
}
