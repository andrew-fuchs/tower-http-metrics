use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};

use http::HeaderMap;
use http_body::Body;
use metrics::{decrement_gauge, histogram};
use pin_project::{pin_project, pinned_drop};

use crate::{HTTP_SERVER_REQUESTS_PENDING, HTTP_SERVER_REQUEST_DURATION_SECONDS};

#[pin_project(PinnedDrop)]
pub struct InstrumentedBody<B> {
    #[pin]
    inner: B,
    start: Instant,
    method: &'static str,
    status: u16,
}

impl<B> InstrumentedBody<B> {
    pub(crate) fn new(inner: B, start: Instant, method: &'static str, status: u16) -> Self {
        Self {
            inner,
            start,
            method,
            status,
        }
    }
}

impl<B: Body> Body for InstrumentedBody<B>
where
    B: Body,
{
    type Data = B::Data;

    type Error = B::Error;

    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        let this = self.project();
        this.inner.poll_data(cx)
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<HeaderMap>, Self::Error>> {
        let this = self.project();
        this.inner.poll_trailers(cx)
    }
}

#[pinned_drop]
impl<B> PinnedDrop for InstrumentedBody<B> {
    fn drop(self: Pin<&mut Self>) {
        let duration_seconds = self.start.elapsed().as_secs_f64();

        decrement_gauge!(
            HTTP_SERVER_REQUESTS_PENDING,
            1.0,
            &[("method", self.method)]
        );
        histogram!(
            HTTP_SERVER_REQUEST_DURATION_SECONDS,
            duration_seconds,
            &[
                ("method", self.method.to_string()),
                ("status", self.status.to_string())
            ]
        );
    }
}
