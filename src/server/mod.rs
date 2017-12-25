/* entry file for server module */
use futures::future::Future;
use futures::future::ok as FutureOk;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use hyper::Error as HyperError;

struct HelloWorld;

const PHRASE: &'static str = "Hello, world";

impl Service for HelloWorld {
	type Request = Request;
	type Response = Response;
	type Error = HyperError;
	type Future = Box<Future<Item=Self::Response, Error=HyperError>>;

	fn call(&self, _req: Request) -> Self::Future {
		Box::new(FutureOk(
			Response::new()
				.with_header(ContentLength(PHRASE.len() as u64))
				.with_body(PHRASE)
		))
	}
}

pub fn run() {
    let addr = "127.0.0.1:8000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
    server.run().unwrap();
}
