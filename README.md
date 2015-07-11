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
    server.listen("127.0.0.1:8080");
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
        res.set(MediaType::Json);
        res.send( "{'foo':'bar'}")
}
```
and after `router.get("/", tmpl_handler);` in main function add:
```rust
    // go to http://localhost:8080/content-type to see this route in action
    router.get("/content-type", content_type);
```
or another variant without function with macro `middleware!` :
```rust
let mut router = Nickel::router();

// go to http://localhost:8080/content-type to see this route in action
router.get("/content-type", middleware! { |request, mut response|
    response.set(MediaType::Json);
    "{'foo':'bar'}"
});
```
Content types available in nickel.rs you can check at [this link](https://github.com/nickel-org/nickel.rs/blob/master/src/mimes.rs#L47)
In our code we add "Application/Json" content type. You can check it when run code and navigate in your browser to http://localhost:8080/content-type
###Step 5 : Test our code on travis-c
So create file `.travis.yml` in root of your code and add:
```yaml
1    language: rust
2    rust:
3        - 1.0.0-beta.3
4    os:
5        - linux
6    script:
7      - cargo build -v
8      - cargo test
```
Connect travis-c to your GitHub account very easy and if you before don't try it now is time to do so.
Travis run test on every commit to GitHub repo.<br> 
On first line in our code we tell travis-c about our language and in line 2-3 we add language version he try to build. <br>
 4-5 is OS in which we test our code. In our example we use `Linux`. <br>
 Lines 6-7 run `cargo build` and `cargo test`. <br>
 Now every time code deplojed to GitHub travis build our code and run test. <br>
 To show  [![Build Status](https://travis-ci.org/Codenator81/nickel-demo.png?branch=master)](https://travis-ci.org/Codenator81/nickel-demo) in your repo 
 write <br>`[![Build Status](https://travis-ci.org/Codenator81/nickel-demo.png?branch=master)](https://travis-ci.org/Codenator81/nickel-demo)`<br>
 Where:<br>
 `[![Build Status](https://travis-ci.org/Codenator81/nickel-demo.png?branch=master)]` - show image from travis-c<br>
 `https://travis-ci.org` - link to travis-c<br>
 `Codenator81` - accaunt name on travis-c<br>
 `nickel-demo.png?branch=master` - repo name with added .png at end and branch tested on<br>
 `(https://travis-ci.org/Codenator81/nickel-demo)` - link to travis<br>
###Step 6 : save data to Postgres 
  My Cargo.toml now look like this:
  ```toml
  [package]
  name = "nickel-demo"
  version = "0.0.6"
  
  [dependencies]
  nickel = "*"
  postgres = "*"
  
  ```
  in console run `cargo update` and `cargo build`<br>
  `cargo update` refresh your locale crates 
  create table blogs in your db:
```sql
  CREATE TABLE blogs
  (
    id serial NOT NULL,
    content text,
    author varchar(32),
    varchar(10) default to_char(CURRENT_DATE, 'yy-mm-dd')
    CONSTRAINT dataset_pkey PRIMARY KEY (id)
  )
```
change main.rs to 
```rust
#[macro_use]
extern crate nickel;
extern crate postgres;

use postgres::{Connection, SslMode};

use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult, MediaType};

use std::collections::HashMap;

struct Blog {
    id: i32,
    content: String, //text
    author: String, //varchar(32)
    datepost: String
}

fn save_db<'a>(_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    //connect to database 
    let conn = Connection::connect("postgres://root:root@localhost/json-test", &SslMode::None).unwrap();
    let blog = Blog {
        id: 0,
        content: "My second blogpost".to_string(),
        author: "Mike".to_string(),
        datepost: "".to_string()
    };
    // insert data in DB
    conn.execute("INSERT INTO blogs (content, author) VALUES ($1, $2)",
    &[&blog.content, &blog.author]).unwrap();
    // render in template save.tpl
    let mut data = HashMap::<&str, String>::new();
    data.insert("content", blog.content);
    data.insert("author", blog.author);
    data.insert("page_title", "Save blog data".to_string());
    res.render("app/views/blog/save.tpl", &data)
}

fn get_db_data<'a>(req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let conn = Connection::connect("postgres://root:root@localhost/json-test", &SslMode::None).unwrap();
    // select data with condition by id WHERE id = $1
    let stmt = conn.prepare("SELECT id, content, author, datepost FROM blogs WHERE id = $1").unwrap();
    // get request blogid and parse it from &str to i32
    let id: Option<i32> = req.param("blogid").trim().parse::<i32>().ok();
    //run query
    let rows = match stmt.query(&[&id]) {
        Ok(rows) => rows,
        Err(err) => panic!("Error running query: {:?}", err)
    };
    // create init date. Should be new fn in real app
    let mut blog = Blog {
        id: 0,
        content: "".to_string(),
        author: "".to_string(),
        datepost: "".to_string()
    };
    //get data from query and break it becouse we need one row data
    for row in &rows {
        blog = Blog {
        id: row.get(0),
        content: row.get(1),
        author: row.get(2),
        datepost: row.get(3)
        };
        break;
    }
    let mut data = HashMap::<&str, String>::new();
    data.insert("content", blog.content);
    data.insert("author", blog.author);
    data.insert("datepost", blog.datepost);
    data.insert("page_title", "Show blog data".to_string());
    res.render("app/views/blog/show.tpl", &data)
}

fn main() {
    let mut server = Nickel::new();
    // start using router
    let mut router = Nickel::router();

    //http://localhost:8080/test-save-db
    router.get("/test-save-db", save_db);
    //http://localhost:8080/get-blog/1   where 1 is id in your database table
    router.get("/get-blog/:blogid", get_db_data);

    server.utilize(router);
    server.listen("127.0.0.1:8080");
}
```
Connect to DB `Connection::connect("postgres://root:root@localhost/json-test", &SslMode::None).unwrap();`<br>
where `"postgres://user:pass@localhost/db_name"`
more details in code comment. 
For base postgres usage in Rust look to [link](https://github.com/sfackler/rust-postgres)
so now when we navigate to //http://localhost:8080/test-save-db we save data to DB an //http://localhost:8080/get-blog/1 retrieve data from DB by id<br>
Now we ready to start build blog/wiki in Rust! <br>
Next we try send post from form to server and save it.
<br>
<br>
Contributors wanted!
