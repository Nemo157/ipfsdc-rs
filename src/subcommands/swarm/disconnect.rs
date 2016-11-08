use futures::Future;
use maddr::MultiAddr;

use context::Context;
use util;

/// Close connection(s) to the given address(es)
///
/// The disconnect is not permanent; if ipfs needs to talk to that address
/// later, it will reconnect.
#[derive(StompCommand)]
pub struct Disconnect {
    /// Address(es) of peer(s) to disconnect from
    #[stomp(arg, min_values = 1)]
    addresses: Vec<MultiAddr>,
}

impl Disconnect {
    pub fn run(self, context: Context) {
        let Context { client, mut event_loop, .. } = context;

        let disconnections = self.addresses.into_iter().map(|addr| {
            client.swarm().disconnect(&addr).map(move |result| {
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

        util::run_all(&mut event_loop, disconnections);
    }
}
