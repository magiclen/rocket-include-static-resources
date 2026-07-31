use super::StaticResources;
use crate::{EtagIfNoneMatch, StaticResponse};

/// To monitor the state of static resources.
#[derive(Debug)]
pub struct StaticContextManager {
    pub resources: StaticResources,
}

impl StaticContextManager {
    #[inline]
    pub(crate) fn new(resources: StaticResources) -> StaticContextManager {
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
        let name = name.as_ref();

        self.resources
            .get_resource(name)
            .map(|resource| {
                if etag_if_none_match.weak_eq(resource.2) {
                    StaticResponse::not_modified()
                } else {
                    StaticResponse::build(resource.0, resource.1, resource.2)
                }
            })
            .unwrap_or_else(|| panic!("The name `{name}` is not found."))
    }
}
