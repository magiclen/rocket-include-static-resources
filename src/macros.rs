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
                static_resources: State<StaticContextManager>,
                etag_if_none_match: EtagIfNoneMatch,
            ) -> StaticResponse {
                static_resources.build(&etag_if_none_match, $name)
            }
        )*
    };
}
