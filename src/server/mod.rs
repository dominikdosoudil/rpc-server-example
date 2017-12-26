/* entry file for server module */

use futures::future::Future;
use futures::future::ok as FutureOk;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use hyper::Error as HyperError;

mod api;

struct HelloWorld;

impl Service for HelloWorld {
	type Request = Request;
	type Response = Response;
	type Error = HyperError;
	type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

	/* Params:
	 * _req Self::Request
	 * Returns type Self::Future
	 */
	fn call(&self, _req: Self::Request) -> Self::Future {
		let req_method = String::from(_req.uri().to_string().trim_matches('/'));
		let response = api::call(&req_method);

		// Self::Future is actually defined as Box	
		return Box::new(FutureOk(
			Response::new()
				.with_header(ContentLength(response.len() as u64))
				.with_body(response)
		))	
	}
}

pub fn run() {
    let addr = "127.0.0.1:8000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
    server.run().unwrap();
}
