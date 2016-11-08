use std::marker::PhantomData;

use maddr::MultiAddr;
use ipfs_client::Client;
use tokio_core::reactor::Core;

pub struct Context {
    pub event_loop: Core,
    pub client: Client,
    private_construction: PhantomData<bool>,
}

impl Context {
    pub fn new(host: MultiAddr) -> Context {
        let event_loop = Core::new().expect("TODO: what could go wrong here?");
        let client = Client::new(event_loop.handle(), host);
        Context {
            event_loop: event_loop,
            client: client,
            private_construction: PhantomData,
        }
    }
}
