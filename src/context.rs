use std::marker::PhantomData;

use tokio_core::reactor::Core;
use ipfs_client::Client;

pub struct Context {
    pub event_loop: Core,
    pub client: Client,
    private_construction: PhantomData<bool>,
}

impl Context {
    pub fn new() -> Context {
        let event_loop = Core::new().expect("TODO: what could go wrong here?");
        let client = Client::new(event_loop.handle(), "http://localhost:5001/api/v0/");
        Context {
            event_loop: event_loop,
            client: client,
            private_construction: PhantomData,
        }
    }
}