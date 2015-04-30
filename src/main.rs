#[macro_use]
extern crate nickel;

use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult, MediaType};

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


// this function add header to response for example now we add application/json
fn content_type<'a>(_: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
        //MediaType can be any valid type for reference see https://github.com/nickel-org/nickel.rs/blob/master/src/mimes.rs#L47
        res.content_type(MediaType::Json);
        res.send( "{'foo':'bar'}")
}

fn main() {
    let mut server = Nickel::new();

    //middleware function logs each request to console
    server.utilize(middleware! { |request|
        println!("logging request: {:?}", request.origin.uri);
    });

    // start using router
    let mut router = Nickel::router();

    //works only on route http://localhost:8080
    router.get("/", tmpl_handler);

    // go to http://localhost:8080/content-type to see this route in action
    router.get("/content-type", content_type);

    server.utilize(router);
    server.listen("127.0.0.1:8080");
}