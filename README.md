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

use rocket_include_static_resources::{EtagIfNoneMatch, StaticResponse};

#[get("/favicon.ico")]
fn favicon(etag_if_none_match: EtagIfNoneMatch) -> StaticResponse {
    static_response!(etag_if_none_match, "favicon")
}

#[get("/favicon-16.png")]
fn favicon_png() -> StaticResponse {
    static_response!("favicon-png")
}

#[get("/")]
fn index(etag_if_none_match: EtagIfNoneMatch) -> StaticResponse {
    static_response!(etag_if_none_match, "html-readme")
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

* `static_resources_initialize!` is used for including files into your executable binary file. You need to specify each file's name and its path. For instance, the above example uses **favicon** to represent the file **included-static-resources/favicon.ico** and **favicon_png** to represent the file **included-static-resources/favicon.png**. A name cannot be repeating. In order to reduce the compilation time and allow to hot-reload resources, files are compiled into your executable binary file together, only when you are using the **release** profile.
* `static_response!` is used for retrieving the file you input through the macro `static_resources_initialize!` as a Response instance into which three HTTP headers, **Content-Type**, **Content-Length** and **Etag**, will be automatically added.

See `examples`.

## Crates.io

https://crates.io/crates/rocket-include-static-resources

## Documentation

https://docs.rs/rocket-include-static-resources

## License

[MIT](LICENSE)