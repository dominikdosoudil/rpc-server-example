
use std::env;

fn client() {
	println!("clienting");
}

fn server() {
	println!("starting server");
}

fn help() {
	println!("Usage:\nmain (server|client)");
}

fn main() {
	let args: Vec<String> = env::args().collect();

	match args.len() {
		2 => {
			match &args[1][..] {
				"server" => server(),
				"client" => client(),
				_ => help(),
			}
		},
		_ => {
			help();
		}
	}
}
