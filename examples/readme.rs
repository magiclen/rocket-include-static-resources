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

#[cfg(feature = "cache")]
cached_static_response_handler! {
    259_200;
    "/favicon.ico" => cached_favicon => "favicon",
    "/favicon-16.png" => cached_favicon_png => "favicon-png",
}

#[get("/")]
fn index(
    static_resources: &State<StaticContextManager>,
    etag_if_none_match: EtagIfNoneMatch,
) -> StaticResponse {
    static_resources.build(&etag_if_none_match, "html-readme")
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build()
        .attach(static_resources_initializer!(
            "favicon" => "examples/front-end/images/favicon.ico",
            "favicon-png" => "examples/front-end/images/favicon-16.png",
            "html-readme" => ("examples", "front-end", "html", "README.html"),
        ))
        .mount("/", routes![favicon, favicon_png])
        .mount("/", routes![index]);

    #[cfg(feature = "cache")]
    {
        rocket.mount("/cache", routes![cached_favicon, cached_favicon_png])
    }

    #[cfg(not(feature = "cache"))]
    {
        rocket
    }
}
