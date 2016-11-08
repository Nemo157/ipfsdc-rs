use futures::Future;
use maddr::MultiAddr;

use context::Context;
use util;

/// Open a new connection to given peer(s)
#[derive(StompCommand)]
pub struct Connect {
    /// Address(es) of peer(s) to connect to
    #[stomp(arg, min_values = 1)]
    addresses: Vec<MultiAddr>,
}

impl Connect {
    pub fn run(self, context: Context) {
        let Context { client, mut event_loop, .. } = context;

        let connections = self.addresses.into_iter().map(|addr| {
            client.swarm().connect(&addr).map(move |result| {
                match result {
                    Ok(msgs) => {
                        for msg in msgs {
                            println!("{}", msg);
                        }
                    }
                    Err(msg) => {
                        println!("{}: {}", addr, msg);
                    }
                }
            })
        });

        util::run_all(&mut event_loop, connections);
    }
}
