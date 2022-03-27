use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};

use futures_util::ready;
use http::Response as HttpResponse;
use metrics::{increment_counter, increment_gauge};
use pin_project::pin_project;

use crate::server::body::InstrumentedBody;
use crate::{HTTP_SERVER_REQUESTS_PENDING, HTTP_SERVER_REQUESTS_TOTAL};

#[pin_project]
pub struct InstrumentedFuture<F> {
    #[pin]
    inner: F,
    start: Instant,
    method: &'static str,
}

impl<F> InstrumentedFuture<F> {
    pub(crate) fn new(inner: F, method: &'static str) -> Self {
        let labels = [("method", method)];
        increment_counter!(HTTP_SERVER_REQUESTS_TOTAL, &labels);
        increment_gauge!(HTTP_SERVER_REQUESTS_PENDING, 1.0, &labels);

        Self {
            inner,
            start: Instant::now(),
            method,
        }
    }
}

impl<F, B, E> Future for InstrumentedFuture<F>
where
    F: Future<Output = Result<HttpResponse<B>, E>>,
{
    type Output = Result<HttpResponse<InstrumentedBody<B>>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match ready!(this.inner.poll(cx)) {
            Ok(res) => {
                let status_code = res.status().as_u16();
                Poll::Ready(Ok(res.map(|b| {
                    InstrumentedBody::new(b, *this.start, this.method, status_code)
                })))
            }
            Err(err) => Poll::Ready(Err(err)),
        }
    }
}
