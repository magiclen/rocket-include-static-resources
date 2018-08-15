Include Static Resources for Rocket Framework
====================

This is a crate which provides macros `static_resources_initialize!` and `static_response!` to static include files from your Rust project and make them be the HTTP response sources quickly.

## Example

```rust
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate lazy_static;

#[macro_use] extern crate rocket_include_static_resources;

extern crate rocket;
extern crate crc;
extern crate mime_guess;

static_resources_initialize!(
   "favicon", "included-static-resources/favicon.ico",
   "favicon-png", "included-static-resources/favicon-16.png"
);

use rocket::response::Response;

#[get("/favicon.ico")]
fn favicon() -> Response<'static> {
   static_response!("favicon")
}

#[get("/favicon.png")]
fn favicon_png() -> Response<'static> {
   static_response!("favicon-png")
}
```

## License

[MIT](LICENSE)