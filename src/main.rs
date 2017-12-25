extern crate hyper;
extern crate futures;

use std::env;

mod server;
mod client;

fn help() {
	println!("Usage:\nmain (server|client)");
}

fn main() {
	// Get application arguments. (First is app filename, we need arg on index 1)
	let args: Vec<String> = env::args().collect();

	match args.len() {
		2 => { // in case there are 2 arguments [app filename, cmd]
			match &args[1][..] {
				"server" => server::run(),
				"client" => client::request(),
				_ => help(),
			}
		},
		_ => {
			help();
		}
	}
}
