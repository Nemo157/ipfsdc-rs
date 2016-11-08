use std::fmt::Debug;

use futures::{ self, Future };
use tokio_core::reactor::Core;

pub fn run_all<F, I>(event_loop: &mut Core, futures: I)
    where F: Future<Item=()>,
          I: IntoIterator<Item=F>,
          <F as Future>::Error: Debug
{
    let mut futures: Vec<_> = futures.into_iter().collect();
    while !futures.is_empty() {
        futures = match event_loop.run(futures::select_all(futures)) {
            Ok((_, _, remaining)) => remaining,
            Err((err, _, remaining)) => {
                println!("TODO handle err: {:?}", err);
                remaining
            }
        };
    }
}
