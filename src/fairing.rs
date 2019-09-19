#[cfg(debug_assertions)]
use std::sync::{Mutex, MutexGuard};

use crate::rocket::fairing::{Fairing, Info, Kind};
use crate::rocket::Rocket;

#[cfg(debug_assertions)]
use crate::FileResources;

#[cfg(not(debug_assertions))]
use crate::StaticResources;

use crate::StaticContextManager;

const FAIRING_NAME: &str = "Static Resources";

/// The fairing of `StaticResponse`.
#[cfg(debug_assertions)]
pub struct StaticResponseFairing {
    pub(crate) custom_callback: Box<dyn Fn(&mut MutexGuard<FileResources>) + Send + Sync + 'static>,
}

/// The fairing of `StaticResponse`.
#[cfg(not(debug_assertions))]
pub struct StaticResponseFairing {
    pub(crate) custom_callback: Box<dyn Fn(&mut StaticResources) + Send + Sync + 'static>,
}

impl Fairing for StaticResponseFairing {
    fn info(&self) -> Info {
        Info {
            name: FAIRING_NAME,
            kind: Kind::Attach,
        }
    }

    #[cfg(debug_assertions)]
    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        let resources = Mutex::new(FileResources::new());

        (self.custom_callback)(&mut resources.lock().unwrap());

        let state = StaticContextManager::new(resources);

        Ok(rocket.manage(state))
    }

    #[cfg(not(debug_assertions))]
    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        let mut resources = StaticResources::new();

        (self.custom_callback)(&mut resources);

        let state = StaticContextManager::new(resources);

        Ok(rocket.manage(state))
    }
}
