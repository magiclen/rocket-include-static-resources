use std::sync::{Mutex, MutexGuard, PoisonError};

use crate::rocket::fairing::{Fairing, Info, Kind};
use crate::rocket::{Build, Rocket};

use super::{FileResources, StaticContextManager, StaticResponse};

const FAIRING_NAME: &str = "Static Resources (Debug)";

/// The fairing of `StaticResponse`.
pub struct StaticResponseFairing {
    pub(crate) custom_callback: Box<dyn Fn(&mut MutexGuard<FileResources>) + Send + Sync + 'static>,
}

#[rocket::async_trait]
impl Fairing for StaticResponseFairing {
    #[inline]
    fn info(&self) -> Info {
        Info {
            name: FAIRING_NAME,
            kind: Kind::Ignite,
        }
    }

    #[inline]
    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
        let resources = Mutex::new(FileResources::new());

        (self.custom_callback)(&mut resources.lock().unwrap_or_else(PoisonError::into_inner));

        let state = StaticContextManager::new(resources);

        Ok(rocket.manage(state))
    }
}

impl StaticResponse {
    #[inline]
    /// Create the fairing of `HandlebarsResponse`.
    pub fn fairing<F>(f: F) -> impl Fairing
    where
        F: Fn(&mut MutexGuard<FileResources>) + Send + Sync + 'static, {
        StaticResponseFairing {
            custom_callback: Box::new(f),
        }
    }
}
