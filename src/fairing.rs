#[cfg(debug_assertions)]
use std::sync::{Mutex, MutexGuard};

use crate::rocket::Rocket;
#[cfg(debug_assertions)]
use crate::rocket::State;
#[cfg(debug_assertions)]
use crate::rocket::request::Request;
use crate::rocket::fairing::{Fairing, Info, Kind};
#[cfg(debug_assertions)]
use crate::rocket::data::Data;

#[cfg(debug_assertions)]
use crate::FileResources;

#[cfg(not(debug_assertions))]
use crate::StaticResources;

use crate::StaticContextManager;

const FAIRING_NAME: &'static str = "Static Resources";

/// The fairing of `StaticResponse`.
#[cfg(debug_assertions)]
pub struct StaticResponseFairing {
    pub(crate) custom_callback: Box<Fn(&mut MutexGuard<FileResources>) + Send + Sync + 'static>
}

/// The fairing of `StaticResponse`.
#[cfg(not(debug_assertions))]
pub struct StaticResponseFairing {
    pub(crate) custom_callback: Box<Fn(&mut StaticResources) + Send + Sync + 'static>
}

impl Fairing for StaticResponseFairing {
    #[cfg(debug_assertions)]
    fn info(&self) -> Info {
        Info {
            name: FAIRING_NAME,
            kind: Kind::Attach | Kind::Request,
        }
    }

    #[cfg(not(debug_assertions))]
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

    #[cfg(debug_assertions)]
    fn on_request(&self, req: &mut Request, _data: &Data) {
        let cm = req.guard::<State<StaticContextManager>>().expect("StaticContextManager registered in on_attach");

        cm.resources.lock().unwrap().reload_if_needed().unwrap();
    }
}