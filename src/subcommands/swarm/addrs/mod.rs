use context::Context;

mod local;

/// List known addresses
#[derive(StompCommand)]
pub struct Addrs {
    #[stomp(subcommand)]
    subcommand: Option<Commands>
}

#[derive(StompCommands)]
enum Commands {
    Local(local::Local),
}

impl Addrs {
    pub fn run(self, mut context: Context) {
        if let Some(subcommand) = self.subcommand {
            return subcommand.run(context);
        }

        let peers = context.event_loop
            .run(context.client.swarm().addresses())
            .expect("TODO: not crash here")
            .peers;

        for (peer, addrs) in peers {
            println!("{} ({}):", peer, addrs.len());
            for addr in addrs {
                println!("        {}", addr);
            }
        }
    }
}

impl Commands {
    fn run(self, context: Context) {
        match self {
            Commands::Local(local) => local.run(context),
        }
    }
}
