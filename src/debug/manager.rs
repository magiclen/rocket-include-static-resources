use std::sync::{Mutex, PoisonError};

use super::FileResources;
use crate::{EtagIfNoneMatch, StaticResponse};

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
        self.try_build(etag_if_none_match, name).unwrap()
    }

    /// Attempt to build a `StaticResponse`.
    #[inline]
    pub fn try_build<S: AsRef<str>>(
        &self,
        etag_if_none_match: &EtagIfNoneMatch<'_>,
        name: S,
    ) -> Result<StaticResponse, std::io::Error> {
        self.resources
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get_resource(name.as_ref())
            .map(|resource| {
                if etag_if_none_match.weak_eq(resource.2) {
                    StaticResponse::not_modified()
                } else {
                    StaticResponse::build(&resource.0, resource.1.clone(), resource.2)
                }
            })
    }
}
