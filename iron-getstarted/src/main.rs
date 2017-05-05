extern crate iron;
extern crate handlebars_iron;

use iron::{Chain, Request, Response, Iron, IronResult};
use handlebars_iron::{DirectorySource, HandlebarsEngine, Template};

fn main() {
    let mut engine =  HandlebarsEngine::new();
    engine.add(Box::new(DirectorySource::new("views", ".hbs")));
    if let Err(r) = engine.reload() {
        panic!("Error: {}", r.cause);
    }

    let mut chain = Chain::new(greeting);
    chain.link_after(engine);

    let http = "localhost:3000";

    Iron::new(chain).http(http).unwrap();
    println!("serve on {}", http);

}

#[allow(dead_code)]
fn hello_world(_:&mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello, World!")))
}

fn greeting(_: &mut Request) -> IronResult<Response> {
    let mut data = std::collections::HashMap::new();
    data.insert("name", "John Doe");

    let tmpl = Template::new("greeting", data);

    Ok(Response::with((iron::status::Ok, tmpl)))
}
