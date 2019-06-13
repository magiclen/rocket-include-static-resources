/*!
# Include Static Resources for Rocket Framework

This is a crate which provides macros `static_resources_initialize!` and `static_response!` to statically include files from your Rust project and make them be the HTTP response sources quickly.

## Example

```rust,ignore
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

* `static_resources_initialize!` is used for including files into your executable binary file. You need to specify each file's name and its path. For instance, the above example uses **favicon** to represent the file **included-static-resources/favicon.ico** and **favicon_png** to represent the file **included-static-resources/favicon.png**. A name cannot be repeating. In order to reduce the compilation time and allow to hot-reload resources, files are compiled into your executable binary file together, only when you are using the **release** profile.
* `static_response!` is used for retrieving the file you input through the macro `static_resources_initialize!` as a Response instance into which three HTTP headers, **Content-Type**, **Content-Length** and **Etag**, will be automatically added.

See `examples`.
*/

mod functions;
mod file_resources;
mod static_resources;
mod manager;
mod fairing;
mod macros;

extern crate mime;
extern crate mime_guess;
extern crate crc_any;
extern crate rc_u8_reader;

extern crate rocket;

extern crate rocket_etag_if_none_match;

#[cfg(not(debug_assertions))]
use std::io::Cursor;
#[cfg(debug_assertions)]
use std::sync::MutexGuard;

use mime::Mime;
#[cfg(debug_assertions)]
use rc_u8_reader::ArcU8Reader;

use rocket::State;
use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::Status;
use rocket::fairing::Fairing;

use rocket_etag_if_none_match::{EntityTag, EtagIfNoneMatch};

pub use file_resources::FileResources;
pub use static_resources::StaticResources;
pub use manager::StaticContextManager;
use fairing::StaticResponseFairing;

#[derive(Debug)]
/// To respond a static resource.
pub struct StaticResponse {
    name: &'static str,
}

impl StaticResponse {
    #[inline]
    /// Build a `StaticResponse` instance.
    pub fn build(name: &'static str) -> StaticResponse {
        StaticResponse {
            name,
        }
    }
}

impl StaticResponse {
    #[cfg(debug_assertions)]
    #[inline]
    /// Create the fairing of `HandlebarsResponse`.
    pub fn fairing<F>(f: F) -> impl Fairing where F: Fn(&mut MutexGuard<FileResources>) + Send + Sync + 'static {
        StaticResponseFairing {
            custom_callback: Box::new(f)
        }
    }

    #[cfg(not(debug_assertions))]
    #[inline]
    /// Create the fairing of `HandlebarsResponse`.
    pub fn fairing<F>(f: F) -> impl Fairing where F: Fn(&mut StaticResources) + Send + Sync + 'static {
        StaticResponseFairing {
            custom_callback: Box::new(f)
        }
    }
}

impl<'a> Responder<'a> for StaticResponse {
    #[cfg(debug_assertions)]
    fn respond_to(self, request: &Request) -> response::Result<'a> {
        let client_etag = request.guard::<EtagIfNoneMatch>().unwrap();

        let mut response = Response::build();

        let cm = request.guard::<State<StaticContextManager>>().expect("StaticContextManager registered in on_attach");

        let (mime, data, etag) = {
            let mut resources = cm.resources.lock().unwrap();

            match resources.get_resource(self.name, true) {
                Ok((mime, data, etag)) => {
                    let is_etag_match = client_etag.weak_eq(&etag);

                    if is_etag_match {
                        response.status(Status::NotModified);

                        return response.ok();
                    } else {
                        (mime.to_string(), data.clone(), etag.to_string())
                    }
                }
                Err(_) => {
                    return Err(Status::InternalServerError);
                }
            }
        };

        response
            .raw_header("ETag", etag)
            .raw_header("Content-Type", mime)
            .sized_body(ArcU8Reader::new(data));

        response.ok()
    }

    #[cfg(not(debug_assertions))]
    fn respond_to(self, request: &Request) -> response::Result<'a> {
        let client_etag = request.guard::<EtagIfNoneMatch>().unwrap();

        let mut response = Response::build();

        let cm = request.guard::<State<StaticContextManager>>().expect("StaticContextManager registered in on_attach");

        let (mime, data, etag) = {
            let resources: &StaticResources = &cm.resources;

            match resources.get_resource(self.name) {
                Some((mime, data, etag)) => {
                    let is_etag_match = client_etag.weak_eq(&etag);

                    if is_etag_match {
                        response.status(Status::NotModified);

                        return response.ok();
                    } else {
                        (mime.to_string(), data, etag.to_string())
                    }
                }
                None => {
                    return Err(Status::InternalServerError);
                }
            }
        };

        response
            .raw_header("ETag", etag)
            .raw_header("Content-Type", mime)
            .sized_body(Cursor::new(data));

        response.ok()
    }
}