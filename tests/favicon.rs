#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rocket_include_static_resources;
extern crate rocket_etag_if_none_match;

extern crate rocket;
extern crate crc_any;
extern crate mime_guess;

static_resources_initialize!(
    "favicon", "included-static-resources/favicon.ico",
    "favicon-png", "included-static-resources/favicon-16.png"
);

use rocket_etag_if_none_match::EtagIfNoneMatch;

use rocket::local::Client;
use rocket::response::Response;
use rocket::http::Status;

#[get("/favicon.ico")]
fn favicon(etag_if_none_match: EtagIfNoneMatch) -> Response<'static> {
    static_response!(etag_if_none_match, "favicon")
}

#[get("/favicon.png")]
fn favicon_png() -> Response<'static> {
    static_response!("favicon-png")
}

#[test]
fn test_favicon() {
    let rocket = rocket::ignite();

    let rocket = rocket
        .mount("/", routes![favicon]);

    let client = Client::new(rocket).expect("valid rocket instance");

    let req = client.get("/favicon.ico");

    let response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type().unwrap().to_string(), "image/x-icon");
}

#[test]
fn test_favicon_png() {
    let rocket = rocket::ignite();

    let rocket = rocket
        .mount("/", routes![favicon_png]);

    let client = Client::new(rocket).expect("valid rocket instance");

    let req = client.get("/favicon.png");

    let response = req.dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type().unwrap().to_string(), "image/png");
}