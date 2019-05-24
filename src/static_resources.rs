use std::collections::HashMap;
use std::path::Path;

use crate::{Mime, EntityTag};
use crate::functions::{compute_data_etag, guess_mime};

#[derive(Debug)]
/// Static resources.
pub struct StaticResources {
    resources: HashMap<&'static str, (Mime, &'static [u8], EntityTag)>
}

impl StaticResources {
    #[inline]
    /// Create an instance of `StaticResources`.
    pub fn new() -> StaticResources {
        StaticResources {
            resources: HashMap::new()
        }
    }

    #[inline]
    /// Register a static resource.
    pub fn register_resource_static<P: AsRef<Path>>(&mut self, name: &'static str, path: P, data: &'static [u8]) {
        let etag = compute_data_etag(data);

        let mime = guess_mime(path);

        self.resources.insert(name, (mime, data, etag));
    }

    #[inline]
    /// Get the specific resource.
    pub fn get_resource<S: AsRef<str>>(&self, name: S) -> Option<(&Mime, &'static [u8], &EntityTag)> {
        let name = name.as_ref();

        self.resources.get(name).map(|(mime, data, etag)| (mime, *data, etag))
    }
}