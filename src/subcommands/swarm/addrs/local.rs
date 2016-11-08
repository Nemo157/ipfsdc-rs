use context::Context;

/// List local addresses
#[derive(StompCommand)]
pub struct Local {
    /// Show peer id in addresses
    id: bool,
}

impl Local {
    pub fn run(self, mut context: Context) {
        let addresses = context.event_loop
            .run(context.client.swarm().local_addresses(self.id))
            .expect("TODO: not crash here")
            .addresses;

        for addr in addresses {
            println!("{}", addr);
        }
    }
}
