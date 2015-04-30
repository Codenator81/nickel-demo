# nickel-demo
Rust Beta 3 [![Build Status](https://travis-ci.org/Codenator81/nickel-demo.png?branch=master)](https://travis-ci.org/Codenator81/nickel-demo)
##Rust web lessons based on nickel.rs and Angular.js
To follow in commits navigate to commit with prefix step in header
<br>
If you have questions or idea report issue please!<br>
Lets build Rust on web together!
<br>
This tutorial follow Rust beta release<br>
###First Step : Create minimal server
in Cargo.toml add:
```toml
[package]
name = "you-app-name"
version = "0.1.0"
authors = ["your-name"]

[dependencies.nickel]

git = "https://github.com/nickel-org/nickel.rs.git"

```
in main.rs add:
```rust
#[macro_use] 
extern crate nickel;

use nickel::Nickel;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            "Hello world!"
        }
    });
    // you can change 8080 to any port 
    server.listen("127.0.0.1:6767");
}
```
Then type in terminal/console `cargo run` and locate to http://localhost:8080/ in your browser. 
After we got informative log message in console:
```
    Listening on http://127.0.0.1:8080
    Ctrl-C to shutdown server
```    
###Second Step : Add template
Add template to `app/views/index.tpl` in root of program:
```html
<!DOCTYPE html>
<html>
<head lang="en">
    <meta charset="UTF-8">
    <title>{{ page_title }}</title>
</head>
<body>
    <h1>
        Hello {{ name }}!
    </h1>
</body>
</html>
```
in main.rs replace:
```rust
    server.utilize(router! {
        get "**" => |_req, _res| {
            "Hello world!"
        }
    });
```
to:
```rust
server.get("/", tmpl_handler);
```
and add before main function:
```rust
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
```
Run `cargo run`. Great I got my template rendered!
###Step 3 : Router and server simple logs
```rust
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

    server.utilize(router);
    // you can change 8080 to any port 
    server.listen("127.0.0.1:8080");
}
```
There we add router and log messages from server.
Now when run and locate to http://localhost:8080/ in browser 
we cen see log messages in console:

    logging request: AbsolutePath("/")
Good!
###Step 4 : header type for content
Add new function for test how we can assign content type in header:
```rust
// this function add header to response for example now we add application/json
fn content_type<'a>(_: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
        //MediaType can be any valid type for reference see 
        res.content_type(MediaType::Json);
        res.send( "{'foo':'bar'}")
}
```
and after `router.get("/", tmpl_handler);` in main function add:
```rust
    // go to http://localhost:8080/content-type to see this route in action
    router.get("/content-type", content_type);
```
Content types available in nickel.rs you can check at [this link](https://github.com/nickel-org/nickel.rs/blob/master/src/mimes.rs#L47)
<br>
<br>
Contributors wanted!
