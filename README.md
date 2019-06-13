Include Static Resources for Rocket Framework
====================

[![Build Status](https://travis-ci.org/magiclen/rocket-include-static-resources.svg?branch=master)](https://travis-ci.org/magiclen/rocket-include-static-resources)

This is a crate which provides macros `static_resources_initialize!` and `static_response!` to statically include files from your Rust project and make them be the HTTP response sources quickly.

## Example

```rust
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_include_static_resources;

use rocket_include_static_resources::StaticResponse;

#[get("/favicon.ico")]
fn favicon() -> StaticResponse {
    static_response!("favicon")
}

#[get("/favicon-16.png")]
fn favicon_png() -> StaticResponse {
    static_response!("favicon-png")
}

#[get("/")]
fn index() -> StaticResponse {
    static_response!("html-readme")
}

fn main() {
    rocket::ignite()
        .attach(StaticResponse::fairing(|resources| {
            static_resources_initialize!(
                resources,

                "favicon", "examples/front-end/images/favicon.ico",
                "favicon-png", "examples/front-end/images/favicon-16.png",

                "html-readme", "examples/front-end/html/README.html",
            );
        }))
        .mount("/", routes![favicon, favicon_png])
        .mount("/", routes![index])
        .launch();
}
```

* `static_resources_initialize!` is used in the fairing of `StaticResponse` to include static files into your executable binary file. You need to specify each file's name and its path. In order to reduce the compilation time and allow to hot-reload resources, files are compiled into your executable binary file together, only when you are using the **release** profile.
* `static_response!` is used for retrieving the file you input through the macro `static_resources_initialize!` as a Response instance into which three HTTP headers, **Content-Type**, **Content-Length** and **Etag**, will be automatically added.

See `examples`.

## Crates.io

https://crates.io/crates/rocket-include-static-resources

## Documentation

https://docs.rs/rocket-include-static-resources

## License

[MIT](LICENSE)