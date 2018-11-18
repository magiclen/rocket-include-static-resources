Include Static Resources for Rocket Framework
====================

[![Build Status](https://travis-ci.org/magiclen/rocket-include-static-resources.svg?branch=master)](https://travis-ci.org/magiclen/rocket-include-static-resources)

This is a crate which provides macros `static_resources_initialize!` and `static_response!` to statically include files from your Rust project and make them be the HTTP response sources quickly.

## Example

```rust
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate lazy_static_include;

#[macro_use] extern crate rocket_include_static_resources;

extern crate rocket;

static_resources_initialize!(
   "favicon", "examples/front-end/images/favicon.ico",
   "favicon-png", "examples/front-end/images/favicon-16.png"
);

use rocket_include_static_resources::EtagIfNoneMatch;

use rocket::response::Response;

#[get("/favicon.ico")]
fn favicon(etag_if_none_match: EtagIfNoneMatch) -> Response<'static> {
   static_response!(etag_if_none_match, "favicon")
}

#[get("/favicon.png")]
fn favicon_png() -> Response<'static> {
   static_response!("favicon-png")
}
```

* `static_resources_initialize!` is used for including files into your executable binary file. You need to specify each file's ID and its path. For instance, the above example uses **favicon** to represent the file **included-static-resources/favicon.ico** and **favicon_png** to represent the file **included-static-resources/favicon.png**. An ID cannot be repeating.
* `static_response!` is used for retrieving the file you input through the macro `static_resources_initialize!` as a Response instance into which three HTTP headers, **Content-Type**, **Content-Length** and **Etag**, will be automatically added.

In order to reduce the compilation time, files are compiled into your executable binary file together, only when you are using the **release** profile.

See `examples`.

## Crates.io

https://crates.io/crates/rocket-include-static-resources

## Documentation

https://docs.rs/rocket-include-static-resources

## License

[MIT](LICENSE)