use std::path::Path;

use crate::crc_any::CRCu64;
use crate::mime::APPLICATION_OCTET_STREAM;
use crate::mime_guess::from_ext;
use crate::{Mime, EntityTag};

#[inline]
pub(crate) fn compute_data_etag<B: AsRef<[u8]> + ?Sized>(data: &B) -> EntityTag {
    let mut crc64ecma = CRCu64::crc64();
    crc64ecma.digest(data.as_ref());
    let crc64 = crc64ecma.get_crc();
    EntityTag::new(true, format!("{:X}", crc64))
}

#[inline]
pub(crate) fn guess_mime<P: AsRef<Path>>(path: P) -> Mime {
    let path = path.as_ref();

    match path.extension() {
        Some(extension) => {
            from_ext(extension.to_string_lossy().as_ref()).first_or_octet_stream()
        }
        None => {
            APPLICATION_OCTET_STREAM
        }
    }
}