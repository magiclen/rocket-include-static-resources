use std::collections::HashMap;

use crate::{functions::compute_data_etag, mime::Mime, EntityTag};

#[derive(Debug)]
struct Resource {
    mime: Mime,
    data: &'static [u8],
    etag: EntityTag<'static>,
}

#[derive(Debug)]
/// Static resources.
pub struct StaticResources {
    resources: HashMap<&'static str, Resource>,
}

impl StaticResources {
    /// Create an instance of `StaticResources`.
    #[inline]
    pub fn new() -> StaticResources {
        StaticResources {
            resources: HashMap::new()
        }
    }

    /// Register a static resource.
    #[inline]
    pub fn register_resource_static(
        &mut self,
        name: &'static str,
        mime: Mime,
        data: &'static [u8],
    ) {
        let etag = compute_data_etag(data);

        let resource = Resource {
            mime,
            data,
            etag,
        };

        self.resources.insert(name, resource);
    }

    /// Get the specific resource.
    #[inline]
    pub fn get_resource<S: AsRef<str>>(
        &self,
        name: S,
    ) -> Option<(&Mime, &'static [u8], &EntityTag<'static>)> {
        let name = name.as_ref();

        self.resources.get(name).map(|resource| (&resource.mime, resource.data, &resource.etag))
    }
}

impl Default for StaticResources {
    #[inline]
    fn default() -> Self {
        StaticResources::new()
    }
}
