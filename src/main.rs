use std::env;

mod server;
mod client;

fn help() {
	println!("Usage:\nmain (server|client)");
}

fn main() {
	let args: Vec<String> = env::args().collect();

	match args.len() {
		2 => {
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
