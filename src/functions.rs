extern crate mime_guess;

use crate::EntityTag;

#[inline]
pub(crate) fn compute_data_etag<B: AsRef<[u8]> + ?Sized>(data: &B) -> EntityTag<'static> {
    EntityTag::from_data(data)
}
