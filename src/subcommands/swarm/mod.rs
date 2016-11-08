use context::Context;

mod peers;
mod addrs;
mod connect;
mod disconnect;

/// Manipulate the network swarm.
///
/// The swarm is the component that opens, listens for, and maintains
/// connections to other ipfs peers in the internet
#[derive(StompCommand)]
pub struct Swarm {
    #[stomp(subcommand)]
    subcommand: Commands,
}

#[derive(StompCommands)]
enum Commands {
    Peers(peers::Peers),
    Addrs(addrs::Addrs),
    Connect(connect::Connect),
    Disconnect(disconnect::Disconnect),
}

impl Swarm {
    pub fn run(self, context: Context) {
        self.subcommand.run(context)
    }
}

impl Commands {
    fn run(self, context: Context) {
        match self {
            Commands::Peers(peers) => peers.run(context),
            Commands::Addrs(addrs) => addrs.run(context),
            Commands::Connect(connect) => connect.run(context),
            Commands::Disconnect(disconnect) => disconnect.run(context),
        }
    }
}
