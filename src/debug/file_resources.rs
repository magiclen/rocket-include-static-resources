extern crate mime_guess;

use std::collections::HashMap;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use crate::functions::{compute_data_etag, guess_mime};
use crate::{EntityTag, Mime};

#[derive(Debug)]
struct Resource {
    path: PathBuf,
    // mime could be an atom `Mime`, so just clone it
    mime: Mime,
    data: Arc<Vec<u8>>,
    etag: Arc<EntityTag<'static>>,
    mtime: Option<SystemTime>,
}

#[derive(Debug)]
/// Reloadable file resources.
pub struct FileResources {
    resources: HashMap<&'static str, Resource>,
}

impl FileResources {
    /// Create an instance of `FileResources`.
    #[inline]
    pub fn new() -> FileResources {
        FileResources {
            resources: HashMap::new(),
        }
    }

    /// Register a resource from a path and it can be reloaded automatically.
    #[inline]
    pub fn register_resource_file<P: Into<PathBuf>>(
        &mut self,
        name: &'static str,
        file_path: P,
    ) -> Result<(), io::Error> {
        let path = file_path.into();

        let metadata = path.metadata()?;

        let mtime = metadata.modified().ok();

        let data = fs::read(&path)?;

        let etag = compute_data_etag(&data);

        let mime = guess_mime(&path);

        let resource = Resource {
            path,
            mime,
            data: Arc::new(data),
            etag: Arc::new(etag),
            mtime,
        };

        self.resources.insert(name, resource);

        Ok(())
    }

    /// Unregister a resource from a file by a name.
    #[inline]
    pub fn unregister_resource_file<S: AsRef<str>>(&mut self, name: S) -> Option<PathBuf> {
        let name = name.as_ref();

        self.resources.remove(name).map(|resource| resource.path)
    }

    /// Reload resources if needed.
    #[inline]
    pub fn reload_if_needed(&mut self) -> Result<(), io::Error> {
        for resource in self.resources.values_mut() {
            let metadata = resource.path.metadata()?;

            let (reload, new_mtime) = match resource.mtime {
                Some(mtime) => {
                    match metadata.modified() {
                        Ok(new_mtime) => (new_mtime > mtime, Some(new_mtime)),
                        Err(_) => (true, None),
                    }
                }
                None => {
                    match metadata.modified() {
                        Ok(new_mtime) => (true, Some(new_mtime)),
                        Err(_) => (true, None),
                    }
                }
            };

            if reload {
                let new_data = fs::read(&resource.path)?;

                let new_etag = compute_data_etag(&new_data);

                resource.data = Arc::new(new_data);

                resource.etag = Arc::new(new_etag);

                resource.mtime = new_mtime;
            }
        }

        Ok(())
    }

    #[allow(clippy::type_complexity)]
    /// Get the specific resource.
    #[inline]
    pub fn get_resource<S: AsRef<str>>(
        &mut self,
        name: S,
    ) -> Result<(Mime, Arc<Vec<u8>>, Arc<EntityTag<'static>>), io::Error> {
        let name = name.as_ref();

        let resource = self.resources.get_mut(name).ok_or_else(|| {
            io::Error::new(ErrorKind::NotFound, format!("The name `{}` is not found.", name))
        })?;

        let metadata = resource.path.metadata()?;

        let (reload, new_mtime) = match resource.mtime {
            Some(mtime) => {
                match metadata.modified() {
                    Ok(new_mtime) => (new_mtime > mtime, Some(new_mtime)),
                    Err(_) => (true, None),
                }
            }
            None => {
                match metadata.modified() {
                    Ok(new_mtime) => (true, Some(new_mtime)),
                    Err(_) => (true, None),
                }
            }
        };

        if reload {
            let new_data = fs::read(&resource.path)?;

            let new_etag = compute_data_etag(&new_data);

            resource.data = Arc::new(new_data);

            resource.etag = Arc::new(new_etag);

            resource.mtime = new_mtime;
        }

        Ok((resource.mime.clone(), resource.data.clone(), resource.etag.clone()))
    }
}

impl Default for FileResources {
    #[inline]
    fn default() -> Self {
        FileResources::new()
    }
}
