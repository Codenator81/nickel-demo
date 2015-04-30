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
When do `cargo run` locate to http://localhost:8080/<br>
Got informative log message in console:

    Listening on http://127.0.0.1:8080
    Ctrl-C to shutdown server
###Second Step : Add template
You can see two variants there how call function<br>
I add template to `app/views/index.tpl` in root of my program<br>
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
