#[macro_use] extern crate nickel;

use nickel::Nickel;

fn main() {
    let mut server = Nickel::new();
    // print in browser hello world in any location after /
    // for example http://localhost:8080/foo/bar
    server.utilize(router! {
        get "**" => |_req, _res| {
        "Hello world!"
        }
    });

    server.listen("127.0.0.1:8080");
}