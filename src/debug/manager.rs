use std::sync::{Mutex, PoisonError};

use crate::{EtagIfNoneMatch, StaticResponse};

use super::FileResources;

/// To monitor the state of static resources.
#[derive(Debug)]
pub struct StaticContextManager {
    pub resources: Mutex<FileResources>,
}

impl StaticContextManager {
    #[inline]
    pub(crate) fn new(resources: Mutex<FileResources>) -> StaticContextManager {
        StaticContextManager {
            resources,
        }
    }

    /// Build a `StaticResponse`.
    #[inline]
    pub fn build<S: AsRef<str>>(
        &self,
        etag_if_none_match: &EtagIfNoneMatch<'_>,
        name: S,
    ) -> StaticResponse {
        self.resources
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get_resource(name.as_ref())
            .map(|resource| {
                if etag_if_none_match.weak_eq(&resource.2) {
                    StaticResponse::not_modified()
                } else {
                    StaticResponse::build(&resource.0, resource.1.clone(), &resource.2)
                }
            })
            .unwrap()
    }
}
