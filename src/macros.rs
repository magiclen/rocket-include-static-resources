/// Used for including files into your executable binary file. You need to specify each file's ID and its path.
#[macro_export]
macro_rules! static_resources_initialize {
    ( $($id:expr, $path:expr), * $(,)* ) => {
        lazy_static_include_bytes_vec!(STATIC_RESOURCES_DATA $(, $path)* );

        lazy_static! {
            static ref STATIC_RESOURCES: ::std::collections::HashMap<&'static str, ::rocket_include_static_resources::StaticResource> = {
                {
                    use ::rocket_include_static_resources::crc_any::CRC;
                    use ::rocket_include_static_resources::mime_guess::get_mime_type;
                    use ::rocket_include_static_resources::mime::APPLICATION_OCTET_STREAM;
                    use ::rocket_include_static_resources::StaticResource;
                    use ::rocket_include_static_resources::EntityTag;
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

                            let etag = EntityTag::new(true, format!("{:X}", crc64));

                            let path = Path::new($path);

                            let content_type = match path.extension() {
                                Some(extension) => get_mime_type(extension.to_str().unwrap()),
                                None => APPLICATION_OCTET_STREAM
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
            use ::rocket_include_static_resources::rocket::http::hyper::header::ETag;
            use ::rocket_include_static_resources::EntityTag;

            let resource = STATIC_RESOURCES.get($id).unwrap();

            let mut response_builder = Response::build();

            response_builder.header(ETag(resource.etag.clone()));

            response_builder.raw_header("Content-Type", resource.content_type.to_string());

            response_builder.raw_header("Content-Length", resource.data.len().to_string());

            response_builder.streamed_body(resource.data);

            response_builder
        }
    };
    ( $etag_if_none_match:expr, $id:expr ) => {
        {
            use ::rocket_include_static_resources::rocket::response::Response;
            use ::rocket_include_static_resources::rocket::http::{Status, hyper::header::ETag};
            use ::rocket_include_static_resources::EntityTag;

            let resource = STATIC_RESOURCES.get($id).unwrap();

            let mut response_builder = Response::build();

            let is_etag_match = $etag_if_none_match.weak_eq(&resource.etag);

            if is_etag_match {
                response_builder.status(Status::NotModified);
            } else {
                response_builder.header(ETag(resource.etag.clone()));

                response_builder.raw_header("Content-Type", resource.content_type.to_string());

                response_builder.raw_header("Content-Length", resource.data.len().to_string());

                response_builder.streamed_body(resource.data);
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