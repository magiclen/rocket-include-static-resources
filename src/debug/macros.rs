/// Used in the fairing of `StaticResponse` to include static files into your executable binary file. You need to specify each file's name and its path relative to the directory containing the manifest of your package. In order to reduce the compilation time and allow to hot-reload resources, files are compiled into your executable binary file together, only when you are using the **release** profile.
#[macro_export]
macro_rules! static_resources_initialize {
    ( $resources:expr, $($name:expr => $path:expr), * $(,)* ) => {
        $(
            $resources.register_resource_file($name, $crate::manifest_dir_macros::not_directory_path!($path)).unwrap();
        )*
    };
}
