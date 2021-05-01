/*!
# Include Static Resources for Rocket Framework

This is a crate which provides macros `static_resources_initializer!` and `static_response_handler!` to statically include files from your Rust project and make them be the HTTP response sources quickly.

## Example

```rust,ignore
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_include_static_resources;

use rocket::State;

use rocket_include_static_resources::{EtagIfNoneMatch, StaticContextManager, StaticResponse};

static_response_handler! {
    "/favicon.ico" => favicon => "favicon",
    "/favicon-16.png" => favicon_png => "favicon-png",
}

#[get("/")]
fn index(
    static_resources: State<StaticContextManager>,
    etag_if_none_match: EtagIfNoneMatch,
) -> StaticResponse {
    static_resources.build(&etag_if_none_match, "html-readme")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(static_resources_initializer!(
            "favicon" => "examples/front-end/images/favicon.ico",
            "favicon-png" => "examples/front-end/images/favicon-16.png",
            "html-readme" => "examples/front-end/html/README.html",
        ))
        .mount("/", routes![favicon, favicon_png])
        .mount("/", routes![index])
}
```

* `static_resources_initializer!` is used for including files into your executable binary file. You need to specify each file's name and its path relative to the directory containing the manifest of your package. For instance, the above example uses **favicon** to represent the file **included-static-resources/favicon.ico** and **favicon_png** to represent the file **included-static-resources/favicon.png**. A name cannot be repeating. In order to reduce the compilation time and allow to hot-reload resources, files are compiled into your executable binary file together, only when you are using the **release** profile.
* `static_response_handler!` is used for quickly creating **GET** route handlers to retrieve static resources.

See `examples`.
*/

#[doc(hidden)]
pub extern crate rocket;

extern crate rocket_etag_if_none_match;

#[cfg(feature = "cache")]
extern crate rocket_cache_response;

extern crate mime;

#[doc(hidden)]
pub extern crate slash_formatter;

mod functions;

mod macros;

#[cfg(debug_assertions)]
mod debug;

#[cfg(not(debug_assertions))]
mod release;

#[cfg(debug_assertions)]
pub use debug::*;

#[cfg(not(debug_assertions))]
pub use release::*;

use mime::Mime;

pub use rocket_etag_if_none_match::entity_tag::EntityTag;
pub use rocket_etag_if_none_match::EtagIfNoneMatch;

#[cfg(feature = "cache")]
pub use rocket_cache_response::CacheResponse;
