//! # Include Static Resources for Rocket Framework
//! This is a crate which provides macros `static_resources_initialize!` and `static_response!` to static include files from your Rust project and make them be the HTTP response sources quickly.
//!
//! ## Example
//!
//! ```
//! #![feature(plugin)]
//! #![plugin(rocket_codegen)]
//!
//! #[macro_use] extern crate lazy_static;
//!
//! #[macro_use] extern crate rocket_include_static_resources;
//!
//! extern crate rocket;
//! extern crate crc;
//! extern crate mime_guess;
//!
//! static_resources_initialize!(
//!    "favicon", "included-static-resources/favicon.ico",
//!    "favicon-png", "included-static-resources/favicon-16.png"
//! );
//!
//! use rocket::response::Response;
//!
//! #[get("/favicon.ico")]
//! fn favicon() -> Response<'static> {
//!    static_response!("favicon")
//! }
//!
//! #[get("/favicon.png")]
//! fn favicon_png() -> Response<'static> {
//!    static_response!("favicon-png")
//! }
//! ```
//!
//! * `static_resources_initialize!` is used for including files into your executable binary file. You need to specify each file's ID and its path. For instance, the above example uses **favicon** to represent the file **included-static-resources/favicon.ico** and **favicon_png** to represent the file **included-static-resources/favicon.png**. An ID cannot be repeating.
//! * `static_response!` is used for retrieving the file you input through the macro `static_resources_initialize!` as a Response instance into which three HTTP headers, **Content-Type**, **Content-Length** and **Etag**, will be automatically added.
//!
//! Refer to `tests/favicon.rs` to see the example completely.

#[doc(hidden)]
pub struct StaticResource {
    pub data: &'static [u8],
    pub content_type: Option<&'static str>,
    pub crc64: u64,
}

#[doc(hidden)]
pub const STATIC_RESOURCE_RESPONSE_CHUNK_SIZE: u64 = 4096;

#[macro_export]
#[doc(hidden)]
macro_rules! add_static_resource {
    ( $map:expr, $($id:expr, $path:expr), * ) => {
        {
            use crc::{crc64 ,Hasher64};
            use rocket_include_static_resources::StaticResource;
            use std::path::Path;

            $(
                let data = include_bytes!($path);

                let mut digest = crc64::Digest::new(crc::crc64::ECMA);
                digest.write(data);

                let crc64 = digest.sum64();

                let path = Path::new($path);

                let extension = path.extension().unwrap().to_str().unwrap().to_lowercase();

                let content_type = mime_guess::get_mime_type_str(&extension);

                if $map.contains_key($id) {
                    panic!("The static resource ID `{}` is duplicated.", $id);
                }

                $map.insert($id , StaticResource{
                    data,
                    content_type,
                    crc64,
                });
            )*
        }
    }
}

/// Used for including files into your executable binary file. You need to specify each file's ID and its path. For instance, the above example uses **favicon** to represent the file **included-static-resources/favicon.ico** and **favicon_png** to represent the file **included-static-resources/favicon.png**. An ID cannot be repeating.
#[macro_export]
macro_rules! static_resources_initialize {
    ( $($id:expr, $path:expr), * ) => {
        lazy_static! {
            pub static ref STATIC_RESOURCES: std::collections::HashMap<&'static str, rocket_include_static_resources::StaticResource> = {
                let mut map = std::collections::HashMap::new();

                $(
                    add_static_resource!(map, $id, concat!(env!("CARGO_MANIFEST_DIR"), "/", $path));
                )*

                map
            };
        }
    };
}

/// Used for retrieving the file you input through the macro `static_resources_initialize!` as a ResponseBuilder instance into which three HTTP headers, **Content-Type**, **Content-Length** and **Etag**, will be automatically added. After fetching the ResponseBuilder instance, you can add extra headers into it!
#[macro_export]
macro_rules! static_response_builder {
    ( $path:expr ) => {
        {
            use rocket::response::Response;
            use rocket::http::hyper::header::{ETag, EntityTag};
            use rocket_include_static_resources::STATIC_RESOURCE_RESPONSE_CHUNK_SIZE;

            let resource = STATIC_RESOURCES.get($path).unwrap();

            let mut response_builder = Response::build();

            response_builder.header(ETag(EntityTag::new(true, resource.crc64.to_string())));

            if let Some(content_type) = resource.content_type {
                response_builder.raw_header("Content-Type", content_type);
            }

            response_builder.raw_header("Content-Length", resource.data.len().to_string());

            response_builder.chunked_body(resource.data, STATIC_RESOURCE_RESPONSE_CHUNK_SIZE);

            response_builder
        }
    };
}

/// Used for retrieving the file you input through the macro `static_resources_initialize!` as a Response instance into which three HTTP headers, **Content-Type**, **Content-Length** and **Etag**, will be automatically added.
#[macro_export]
macro_rules! static_response {
    ( $path:expr ) => {
        {
            static_response_builder!($path).finalize()
        }
    };
}