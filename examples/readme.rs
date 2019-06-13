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