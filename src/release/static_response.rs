extern crate mime;
extern crate rc_u8_reader;

use std::io::Cursor;

use mime::Mime;

use crate::rocket::http::Status;
use crate::rocket::request::Request;
use crate::rocket::response::{self, Responder, Response};
use crate::EntityTag;

#[derive(Debug)]
struct StaticResponseInner {
    mime: String,
    data: &'static [u8],
    etag: String,
}

#[derive(Debug)]
/// To respond a static resource.
pub struct StaticResponse {
    inner: Option<StaticResponseInner>,
}

impl StaticResponse {
    #[inline]
    pub(crate) fn build(
        mime: &Mime,
        data: &'static [u8],
        etag: &EntityTag<'static>,
    ) -> StaticResponse {
        StaticResponse {
            inner: Some(StaticResponseInner {
                mime: mime.to_string(),
                data,
                etag: etag.to_string(),
            }),
        }
    }

    #[inline]
    pub(crate) const fn not_modified() -> StaticResponse {
        StaticResponse {
            inner: None,
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for StaticResponse {
    #[inline]
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'o> {
        let mut response = Response::build();

        if let Some(inner) = self.inner {
            response.raw_header("Etag", inner.etag.to_string());
            response.raw_header("Content-Type", inner.mime.to_string());

            response.sized_body(inner.data.len(), Cursor::new(inner.data));
        } else {
            response.status(Status::NotModified);
        }

        response.ok()
    }
}
