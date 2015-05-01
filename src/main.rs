#[macro_use]
extern crate nickel;

use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult, MediaType};

fn main() {
    let mut server = Nickel::new();
    // start using router
    let mut router = Nickel::router();

    // go to http://localhost:6767/content-type to see this route in action
    router.get("/content-type", middleware! { |request, mut response|
        response.content_type(MediaType::Json);
        "{'foo':'bar'}"
    });

    server.utilize(router);
    server.listen("127.0.0.1:8080");
}