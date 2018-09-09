//! # Include Static Resources for Rocket Framework
//! This is a crate which provides macros `static_resources_initialize!` and `static_response!` to statically include files from your Rust project and make them be the HTTP response sources quickly.
//!
//! ## Example
//!
//! ```
//! #![feature(plugin)]
//! #![plugin(rocket_codegen)]
//!
//! #[macro_use] extern crate lazy_static;
//! #[macro_use] extern crate lazy_static_include;
//!
//! #[macro_use] extern crate rocket_include_static_resources;
//! extern crate rocket_etag_if_none_match;
//!
//! extern crate rocket;
//!
//! static_resources_initialize!(
//!    "favicon", "included-static-resources/favicon.ico",
//!    "favicon-png", "included-static-resources/favicon-16.png"
//! );
//!
//! use rocket_etag_if_none_match::EtagIfNoneMatch;
//!
//! use rocket::response::Response;
//!
//! #[get("/favicon.ico")]
//! fn favicon(etag_if_none_match: EtagIfNoneMatch) -> Response<'static> {
//!    static_response!(etag_if_none_match, "favicon")
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
//!
//! In order to reduce the compilation time, files are compiled into your executable binary file together, only when you are using the **release** profile.

pub extern crate rocket;
pub extern crate rocket_etag_if_none_match;
#[doc(hidden)]
pub extern crate crc_any;
#[doc(hidden)]
pub extern crate mime_guess;

#[doc(hidden)]
pub struct StaticResource {
    pub data: &'static [u8],
    pub content_type: Option<&'static str>,
    pub etag: String,
}

#[doc(hidden)]
pub const STATIC_RESOURCE_RESPONSE_CHUNK_SIZE: u64 = 4096;

/// Used for including files into your executable binary file. You need to specify each file's ID and its path. For instance, the above example uses **favicon** to represent the file **included-static-resources/favicon.ico** and **favicon_png** to represent the file **included-static-resources/favicon.png**. An ID cannot be repeating.
#[macro_export]
macro_rules! static_resources_initialize {
    ( $($id:expr, $path:expr), * ) => {
        lazy_static_include_bytes!(STATIC_RESOURCES_DATA $(, $path)* );

        lazy_static! {
            static ref STATIC_RESOURCES: ::std::collections::HashMap<&'static str, ::rocket_include_static_resources::StaticResource> = {
                {
                    use ::rocket_include_static_resources::crc_any::CRC;
                    use ::rocket_include_static_resources::mime_guess::get_mime_type_str;
                    use ::rocket_include_static_resources::StaticResource;
                    use ::std::path::Path;
                    use ::std::collections::HashMap;

                    let mut map = HashMap::new();

                    let mut p = 0usize;

                    $(
                        {
                            let data = STATIC_RESOURCES_DATA[p];

                            p += 1;

                            let mut crc64ecma = CRC::crc64ecma();
                            crc64ecma.digest(data.as_ref());

                            let crc64 = crc64ecma.get_crc();

                            let etag = format!("{:X}", crc64);

                            let path = Path::new($path);

                            let content_type = match path.extension() {
                                Some(extension) => {
                                    get_mime_type_str(&extension.to_str().unwrap().to_lowercase())
                                }
                                None => None
                            };

                            if map.contains_key($id) {
                                panic!("The static resource ID `{}` is duplicated.", $id);
                            }

                            map.insert($id , StaticResource{
                                data,
                                content_type,
                                etag,
                            });
                        }
                    )*

                    map
                }
            };
        }
    };
}

/// Used for retrieving the file you input through the macro `static_resources_initialize!` as a ResponseBuilder instance into which three HTTP headers, **Content-Type**, **Content-Length** and **Etag**, will be automatically added. After fetching the ResponseBuilder instance, you can add extra headers into it!
#[macro_export]
macro_rules! static_response_builder {
    ( $id:expr ) => {
        {
            use ::rocket_include_static_resources::rocket::response::Response;
            use ::rocket_include_static_resources::rocket::http::hyper::header::{ETag, EntityTag};
            use ::rocket_include_static_resources::STATIC_RESOURCE_RESPONSE_CHUNK_SIZE;

            let resource = STATIC_RESOURCES.get($id).unwrap();

            let mut response_builder = Response::build();

            response_builder.header(ETag(EntityTag::new(true, resource.etag.clone())));

            if let Some(content_type) = resource.content_type {
                response_builder.raw_header("Content-Type", content_type);
            }

            response_builder.raw_header("Content-Length", resource.data.len().to_string());

            response_builder.chunked_body(resource.data, STATIC_RESOURCE_RESPONSE_CHUNK_SIZE);

            response_builder
        }
    };
    ( $etag_if_none_match:expr, $id:expr ) => {
        {
            use ::rocket_include_static_resources::rocket::response::Response;
            use ::rocket_include_static_resources::rocket::http::{Status, hyper::header::{ETag, EntityTag}};
            use ::rocket_include_static_resources::STATIC_RESOURCE_RESPONSE_CHUNK_SIZE;

            let resource = STATIC_RESOURCES.get($id).unwrap();

            let mut response_builder = Response::build();

            let etag = $etag_if_none_match.etag;

            let mut is_etag_match = false;

            if let Some(etag) = etag {
                if etag.tag().eq(&resource.etag){
                    is_etag_match = true;
                }
            }

            if is_etag_match {
                response_builder.status(Status::NotModified);
            } else {
                response_builder.header(ETag(EntityTag::new(true, resource.etag.clone())));

                if let Some(content_type) = resource.content_type {
                    response_builder.raw_header("Content-Type", content_type);
                }

                response_builder.raw_header("Content-Length", resource.data.len().to_string());

                response_builder.chunked_body(resource.data, STATIC_RESOURCE_RESPONSE_CHUNK_SIZE);
            }

            response_builder
        }
    };
}

/// Used for retrieving the file you input through the macro `static_resources_initialize!` as a Response instance into which three HTTP headers, **Content-Type**, **Content-Length** and **Etag**, will be automatically added.
#[macro_export]
macro_rules! static_response {
    ( $id:expr ) => {
        {
            static_response_builder!($id).finalize()
        }
    };
    ( $etag_if_none_match:expr, $id:expr ) => {
        {
            static_response_builder!($etag_if_none_match, $id).finalize()
        }
    };
}