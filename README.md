# nickel-demo
Rust Beta 3 [![Build Status](https://travis-ci.org/Codenator81/nickel-demo.png?branch=master)](https://travis-ci.org/Codenator81/nickel-demo)
##Rust web lessons based on nickel.rs and Angular.js
Just follow my commits <br>
I try do every commits step by step where each compile<br>
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

    server.listen("127.0.0.1:6767");
}
```
When type in terminal/console `cargo run` and locate to http://localhost:8080/ in your browser. 
After we got informative log message in console:
```
    Listening on http://127.0.0.1:8080
    Ctrl-C to shutdown server
```    
###Second Step : Add template
Add template to `app/views/index.tpl` in root of my program:
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
```
I run again `cargo run`. Great I got my template rendered!
###Step 3 : Router and server simple logs
Now when locate to http://localhost:8080/<br>
we cen see log messages in console:

    logging request: AbsolutePath("/")
Good!
###Step 4 : header type for content
```rust
    //res: Response
    res.content_type(MediaType::Json);
```

<br>
<br>
Contributors wanted!
