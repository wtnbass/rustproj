extern crate iron;

use iron::{Request, Response, Iron, IronResult};

fn main() {
    Iron::new(hello_world).http("localhost:3000").unwrap();
}

fn hello_world(_:&mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello, World!")))
}
