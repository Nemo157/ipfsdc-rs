use std::cmp;

use ipfs_client::data::PeerInfo;
use mhash::MultiHash;

use context::Context;

/// Show ipfs peer info
///
/// Prints out information about the specified peer(s).
/// If no peers are specified prints out information about the local peer.
#[derive(StompCommand)]
#[stomp(alias = "id")]
pub struct Info {
    /// Id of peer to lookup
    peerid: Option<MultiHash>,
}

impl Info {
    pub fn run(self, mut context: Context) {
        let future = self.peerid
            .map(|id| context.client.peer_info(&id))
            .unwrap_or_else(|| context.client.local_info());

        print(context.event_loop.run(future).expect("TODO: not crash here"));
    }
}

fn print(info: PeerInfo) {
    println!("peer id:");
    println!("    {}", info.id);
    println!();
    println!("public key:");
    let mut rest = &*info.public_key;
    while !rest.is_empty() {
        let (line, next) = rest.split_at(cmp::min(100, rest.len()));
        println!("    {}", line);
        rest = next;
    }
    println!();
    println!("addresses:");
    for addr in info.addresses {
        println!("    {}", addr);
    }
    println!();
    println!("agent version:");
    println!("    {}", info.agent_version);
    println!();
    println!("protocol version:");
    println!("    {}", info.protocol_version);
}

