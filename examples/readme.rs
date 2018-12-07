#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_include_static_resources;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate lazy_static_include;

use rocket_include_static_resources::{EtagIfNoneMatch, StaticResponse};

static_resources_initialize!(
    "favicon", "examples/front-end/images/favicon.ico",
    "favicon-png", "examples/front-end/images/favicon-16.png",

    "html-readme", "examples/front-end/html/README.html",
);

#[get("/favicon.ico")]
fn favicon(etag_if_none_match: EtagIfNoneMatch) -> StaticResponse {
    static_response!(etag_if_none_match, "favicon")
}

#[get("/favicon-16.png")]
fn favicon_png(etag_if_none_match: EtagIfNoneMatch) -> StaticResponse {
    static_response!(etag_if_none_match, "favicon-png")
}

#[get("/")]
fn index(etag_if_none_match: EtagIfNoneMatch) -> StaticResponse {
    static_response!(etag_if_none_match, "html-readme")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![favicon])
        .mount("/", routes![favicon_png])
        .mount("/", routes![index])
        .launch();
}