#[macro_use] extern crate nickel;

use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult};
use std::collections::HashMap;

//there is another variant where don't need return MiddlewareResult
// but then not so clean call in main function (in my opinion)
// if you remove return value here
// fn handler (_: &Request, res: &mut Response) {
// so in main func your call look like this:
// server.get("/", middleware!(@tmpl_handler));
fn tmpl_handler<'a> (_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let mut data = HashMap::<&str, &str>::new();
    // add data for render
    // name = {{ name }} in template
    // page_title = {{ page_title }}
    data.insert("name", "Alex");// change "Alex" to your name )
    data.insert("page_title", "lesson 2");
    res.render("app/views/index.tpl", &data)
}

fn main() {
    let mut server = Nickel::new();

    //works only on route http://localhost:8080
    server.get("/", tmpl_handler);

    server.listen("127.0.0.1:8080");
}