/// Used for generating a fairing for static resources.
#[macro_export]
macro_rules! static_resources_initializer {
    ( $($name:expr => $path:expr), * $(,)* ) => {
        {
            $crate::StaticResponse::fairing(|resources| {
                $crate::static_resources_initialize!(
                    resources
                    $(, $name => $path)*
                );
            })
        }
    };
}

/// Used for quickly creating **GET** route handlers to retrieve static resources.
#[macro_export]
macro_rules! static_response_handler {
    ( $($route:expr => $handler_name:ident => $name:expr), * $(,)* ) => {
        $(
            #[get($route)]
            fn $handler_name(
                static_resources: $crate::rocket::State<$crate::StaticContextManager>,
                etag_if_none_match: $crate::EtagIfNoneMatch,
            ) -> $crate::StaticResponse {
                static_resources.build(&etag_if_none_match, $name)
            }
        )*
    };
}

#[cfg(feature = "cache")]
/// Used for quickly creating **GET** route handlers to retrieve static resources with cache control.
#[macro_export]
macro_rules! cached_static_response_handler {
    ( $max_age:expr, $must_revalidate:expr ; $($route:expr => $handler_name:ident => $name:expr), * $(,)* ) => {
        $(
            #[get($route)]
            fn $handler_name(
                static_resources: $crate::rocket::State<$crate::StaticContextManager>,
                etag_if_none_match: $crate::EtagIfNoneMatch,
            ) -> $crate::CacheResponse<$crate::StaticResponse> {
                let responder =  static_resources.build(&etag_if_none_match, $name);

                $crate::CacheResponse::public_only_release(responder, $max_age, $must_revalidate)
            }
        )*
    };
    ( $max_age:expr ; $($route:expr => $handler_name:ident => $name:expr), * $(,)* ) => {
        $crate::cached_static_response_handler! {
            $max_age, false;
            $(
                $route => $handler_name => $name,
            )*
        }
    };
}
