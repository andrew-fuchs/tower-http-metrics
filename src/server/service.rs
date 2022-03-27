use std::task::Context;
use std::task::Poll;

use http::Request as HttpRequest;
use http::Response as HttpResponse;
use tower_service::Service;

use crate::server::future::InstrumentedFuture;
use crate::util::label_from_method;

use super::body::InstrumentedBody;

pub struct HttpServerMetricsService<S> {
    inner: S,
}

impl<S> HttpServerMetricsService<S> {
    pub(crate) fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, ReqBody, ResBody> Service<HttpRequest<ReqBody>> for HttpServerMetricsService<S>
where
    S: Service<HttpRequest<ReqBody>, Response = HttpResponse<ResBody>>,
{
    type Response = HttpResponse<InstrumentedBody<ResBody>>;

    type Error = S::Error;

    type Future = InstrumentedFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: HttpRequest<ReqBody>) -> Self::Future {
        let method = label_from_method(req.method());
        InstrumentedFuture::new(self.inner.call(req), method)
    }
}
