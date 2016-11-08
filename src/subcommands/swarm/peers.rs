use context::Context;

/// List the set of peers this node is connected to
#[derive(StompCommand)]
pub struct Peers {
}

impl Peers {
    pub fn run(self, mut context: Context) {
        let peers = context.event_loop
            .run(context.client.swarm().peers())
            .expect("TODO: not crash here");

        for addr in peers.addresses {
            println!("{}", addr);
        }
    }
}
