/// Used in the fairing of `StaticResponse` to include static files into your executable binary file. You need to specify each file's name and its path.
#[macro_export]
#[cfg(debug_assertions)]
macro_rules! static_resources_initialize {
    ( $resources:expr, $($name:expr, $path:expr), * $(,)* ) => {
        use std::fs;

        $(
            $resources.register_resource_file($name, $path).unwrap();
        )*
    };
}

/// Used in the fairing of `StaticResponse` to include static files into your executable binary file. You need to specify each file's name and its path.
#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! static_resources_initialize {
    ( $resources:expr, $($name:expr, $path:expr), * $(,)* ) => {
        use std::fs;

        $(
            $resources.register_resource_static($name, concat!(env!("CARGO_MANIFEST_DIR"), "/", $path), include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path)));
        )*
    };
}

/// Used for retrieving the file you input through the macro `static_resources_initialize!` as a Response instance into which three HTTP headers, **Content-Type**, **Content-Length** and **Etag**, will be automatically added.
#[macro_export]
macro_rules! static_response {
    ( $name:expr ) => {
        {
            use ::rocket_include_static_resources::{StaticResponse, EtagIfNoneMatch};

            StaticResponse::build(
                EtagIfNoneMatch {
                    etag: None
                },
                None,
                $name,
            )
        }
    };
    ( $etag_if_none_match:expr, $name:expr ) => {
        {
            use ::rocket_include_static_resources::{StaticResponse, EtagIfNoneMatch};

            StaticResponse::build(
                $etag_if_none_match,
                None,
                $name,
            )
        }
    };
}