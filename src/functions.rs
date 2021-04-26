extern crate mime_guess;

use std::path::Path;

use mime_guess::from_ext;

use crate::mime::APPLICATION_OCTET_STREAM;
use crate::{EntityTag, Mime};

#[inline]
pub(crate) fn compute_data_etag<B: AsRef<[u8]> + ?Sized>(data: &B) -> EntityTag<'static> {
    EntityTag::from_data(data)
}

#[inline]
pub(crate) fn guess_mime<P: AsRef<Path>>(path: P) -> Mime {
    let path = path.as_ref();

    match path.extension() {
        Some(extension) => {
            match extension.to_str() {
                Some(extension) => from_ext(extension).first_or_octet_stream(),
                None => APPLICATION_OCTET_STREAM,
            }
        }
        None => APPLICATION_OCTET_STREAM,
    }
}
