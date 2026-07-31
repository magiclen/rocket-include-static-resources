#![allow(unused_must_use)]

use rocket_include_static_resources::*;

#[test]
fn fairing() {
    rocket::build().attach(static_resources_initializer!(
        "favicon" => "examples/front-end/images/favicon.ico",
        "favicon-png" => "examples/front-end/images/favicon-16.png",
        "html-readme" => ("examples", "front-end", "html", "README.html"),
    ));
}
