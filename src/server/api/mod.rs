use std::collections::HashMap;

pub fn call(request: &str) -> String {
	let mut methods = HashMap::new();
	methods.insert("hello", hello);
	match methods.contains_key(request) {
		true => {
			match methods.get(request) {
				Some(x) => return x(),
				None => return "Internal error".to_string()
			}
		},
		_ => return "method not found".to_string() 
	};
}

pub fn hello() -> String {
	return "Hello!".to_string();
}
