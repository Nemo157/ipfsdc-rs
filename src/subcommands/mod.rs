use maddr::MultiAddr;

use context::Context;

mod info;
mod version;
mod swarm;

#[derive(StompCommand)]
#[stomp(crate_authors, crate_version)]
#[stomp(name = "IPFS Daemon CLI")]
#[stomp(global_settings(ColoredHelp, DeriveDisplayOrder, VersionlessSubcommands))]
pub struct App {
    #[stomp(default_value = "/ip4/127.0.0.1/tcp/5001/https")]
    api: MultiAddr,
    #[stomp(subcommand)]
    subcommand: Commands,
}

#[derive(StompCommands)]
pub enum Commands {
    Info(info::Info),
    Version(version::Version),
    Swarm(swarm::Swarm),
}

impl App {
    pub fn run(self) {
        self.subcommand.run(Context::new(self.api))
    }
}

impl Commands {
    fn run(self, context: Context) {
        match self {
            Commands::Info(info) => info.run(context),
            Commands::Version(version) => version.run(context),
            Commands::Swarm(swarm) => swarm.run(context),
        }
    }
}
