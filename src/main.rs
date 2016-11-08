#![feature(attr_literals)]
#![feature(custom_derive)]
#![feature(proc_macro)]

#![allow(unknown_lints)] // for clippy
#![warn(fat_ptr_transmutes)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
// TODO #![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_import_braces)]
#![warn(unused_results)]
#![warn(variant_size_differences)]

#[macro_use]
extern crate clap;
extern crate futures;
extern crate ipfs_client;
extern crate tokio_core;
extern crate mhash;
extern crate maddr;
extern crate stomp;
#[macro_use]
extern crate stomp_macros;

mod util;
mod context;
mod subcommands;

use stomp::ParseApp;

fn main() {
    subcommands::App::parse().run();
}
