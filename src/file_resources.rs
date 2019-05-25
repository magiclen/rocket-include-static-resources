use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;
use std::fs;
use std::io::{self, ErrorKind};

use crate::{Mime, EntityTag};
use crate::functions::{compute_data_etag, guess_mime};

#[derive(Debug)]
/// Reloadable file resources.
pub struct FileResources {
    resources: HashMap<&'static str, (PathBuf, Mime, Vec<u8>, EntityTag, Option<SystemTime>)>
}

impl FileResources {
    #[inline]
    /// Create an instance of `FileResources`.
    pub fn new() -> FileResources {
        FileResources {
            resources: HashMap::new()
        }
    }

    #[inline]
    /// Register a resource from a path and it can be reloaded automatically.
    pub fn register_resource_file<P: Into<PathBuf>>(&mut self, name: &'static str, file_path: P) -> Result<(), io::Error> {
        let file_path = file_path.into();

        let data = fs::read(&file_path)?;

        let etag = compute_data_etag(&data);

        let mime = guess_mime(&file_path);

        let metadata = file_path.metadata().unwrap();

        let mtime = metadata.modified().ok();

        self.resources.insert(name, (file_path, mime, data, etag, mtime));

        Ok(())
    }

    #[inline]
    /// Unregister a resource from a file by a name.
    pub fn unregister_resource_file<S: AsRef<str>>(&mut self, name: S) -> Option<PathBuf> {
        let name = name.as_ref();

        match self.resources.remove(name) {
            Some((file_path, _, _, _, _)) => {
                Some(file_path)
            }
            None => {
                None
            }
        }
    }

    #[inline]
    /// Reload resources if needed.
    pub fn reload_if_needed(&mut self) -> Result<(), io::Error> {
        for (_, (file_path, _, data, etag, mtime)) in &mut self.resources {
            let metadata = file_path.metadata()?;

            let (reload, new_mtime) = match mtime {
                Some(mtime) => {
                    match metadata.modified() {
                        Ok(new_mtime) => {
                            (new_mtime > *mtime, Some(new_mtime))
                        }
                        Err(_) => {
                            (true, None)
                        }
                    }
                }
                None => {
                    match metadata.modified() {
                        Ok(new_mtime) => {
                            (true, Some(new_mtime))
                        }
                        Err(_) => {
                            (true, None)
                        }
                    }
                }
            };

            if reload {
                let new_data = fs::read(&file_path)?;

                let new_etag = compute_data_etag(&new_data);

                *data = new_data;

                *etag = new_etag;

                *mtime = new_mtime;
            }
        }

        Ok(())
    }

    #[inline]
    /// Get the specific resource.
    pub fn get_resource<S: AsRef<str>>(&mut self, name: S, reload_if_needed: bool) -> Result<(&Mime, &[u8], &EntityTag), io::Error> {
        let name = name.as_ref();

        if reload_if_needed {
            let (file_path, mime, data, etag, mtime) = self.resources.get_mut(name).ok_or(io::Error::new(ErrorKind::NotFound, format!("The name `{}` is not found.", name)))?;

            let metadata = file_path.metadata()?;

            let (reload, new_mtime) = match mtime {
                Some(mtime) => {
                    match metadata.modified() {
                        Ok(new_mtime) => {
                            (new_mtime > *mtime, Some(new_mtime))
                        }
                        Err(_) => {
                            (true, None)
                        }
                    }
                }
                None => {
                    match metadata.modified() {
                        Ok(new_mtime) => {
                            (true, Some(new_mtime))
                        }
                        Err(_) => {
                            (true, None)
                        }
                    }
                }
            };

            if reload {
                let new_data = fs::read(&file_path)?;

                let new_etag = compute_data_etag(&new_data);

                *data = new_data;

                *etag = new_etag;

                *mtime = new_mtime;
            }

            Ok((mime, data, etag))
        } else {
            self.resources.get(name).map(|(_, mime, data, etag, _)| (mime, data.as_slice(), etag)).ok_or(io::Error::new(ErrorKind::NotFound, format!("The name `{}` is not found.", name)))
        }
    }
}