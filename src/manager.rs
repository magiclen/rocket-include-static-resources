#[cfg(debug_assertions)]
use std::sync::Mutex;

#[cfg(debug_assertions)]
use crate::FileResources;

#[cfg(not(debug_assertions))]
use crate::StaticResources;

/// To monitor the state of static resources.
#[cfg(debug_assertions)]
#[derive(Debug)]
pub struct StaticContextManager {
    pub resources: Mutex<FileResources>,
}

/// To monitor the state of static resources.
#[cfg(not(debug_assertions))]
#[derive(Debug)]
pub struct StaticContextManager {
    pub resources: StaticResources,
}

impl StaticContextManager {
    #[cfg(debug_assertions)]
    #[inline]
    pub(crate) fn new(resources: Mutex<FileResources>) -> StaticContextManager {
        StaticContextManager {
            resources,
        }
    }

    #[cfg(not(debug_assertions))]
    #[inline]
    pub(crate) fn new(resources: StaticResources) -> StaticContextManager {
        StaticContextManager {
            resources,
        }
    }
}
