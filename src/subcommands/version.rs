use context::Context;

/// Shows the ipfs daemon's version information
#[derive(StompCommand)]
pub struct Version {
    /// Only show the version number
    #[stomp(short = 'n')]
    number: bool,
    /// Show the commit hash
    commit: bool,
    /// Show repo version
    repo: bool,
}

impl Version {
    pub fn run(self, mut context: Context) {
        let version = context.event_loop
            .run(context.client.version())
            .expect("TODO: not crash here");

        if self.repo {
            println!("{}", version.repo);
        } else {
            if !self.number {
                print!("ipfs daemon version ");
            }
            print!("{}", version.version);
            if self.commit {
                print!("-{}", version.commit);
            }
            println!();
        }
    }
}
