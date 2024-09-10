use super::{Context, FullDuplexPeer, FullDuplexer};
use std::boxed::Box;
use std::collections::HashMap;
use std::env;
use std::io::{Read, Result, Write};

pub struct FullDuplexPeerGateway {
    active_peers: Box<HashMap<String, FullDuplexPeer>>,
    inactive_peers: Box<HashMap<String, FullDuplexPeer>>,
    blacklist: Box<Vec<String>>, // Blacklist to hold prohibited terms
}

impl FullDuplexPeerGateway {
    pub fn new() -> Self {
        let blacklist = FullDuplexPeerGateway::load_blacklist();

        Self {
            active_peers: Box::new(HashMap::new()),
            inactive_peers: Box::new(HashMap::new()),
            blacklist: Box::new(blacklist),
        }
    }

    // Load blacklist terms from the .env file
    fn load_blacklist() -> Vec<String> {
        dotenv::dotenv().ok(); // Load .env file
        let blacklist_str = env::var("BLACKLIST").unwrap_or_default();
        blacklist_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    }

    // Function to check if the message contains any blacklisted terms
    pub fn filter_blacklisted_content(&self, message: &str) -> bool {
        for term in self.blacklist.iter() {
            if message.contains(term) {
                return true; // Blacklisted content found
            }
        }
        false
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
        let mut buffer = vec![0; n as usize];
        reader.read_exact(&mut buffer)?;
        let message = String::from_utf8_lossy(&buffer);

        // Check if the message contains any blacklisted terms
        if self.filter_blacklisted_content(&message) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Message contains blacklisted terms",
            ));
        }

        for peer in self.active_peers.values() {
            peer.send(ctx, &mut buffer.as_slice(), n)?;
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
