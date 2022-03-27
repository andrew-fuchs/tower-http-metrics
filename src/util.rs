use http::Method;
#[cfg(feature = "tower")]
use tower::ServiceBuilder;
use tower_layer::Stack;

#[cfg(feature = "server")]
use crate::server::HttpServerMetricsLayer;

#[cfg(feature = "util")]
pub trait ServiceBuilderExt<L> {
    #[cfg(feature = "server")]
    fn http_server_metrics(self) -> ServiceBuilder<Stack<HttpServerMetricsLayer, L>>;
}

#[cfg(feature = "util")]
impl<L> ServiceBuilderExt<L> for ServiceBuilder<L> {
    #[cfg(feature = "server")]
    fn http_server_metrics(self) -> ServiceBuilder<Stack<HttpServerMetricsLayer, L>> {
        self.layer(HttpServerMetricsLayer::new())
    }
}

/// Utility function that maps methods to a *limited* set of strings to
/// prevent too many metrics time-series from being created.
pub(crate) const fn label_from_method(method: &Method) -> &'static str {
    match *method {
        Method::CONNECT => "CONNECT",
        Method::DELETE => "DELETE",
        Method::GET => "GET",
        Method::HEAD => "HEAD",
        Method::OPTIONS => "OPTIONS",
        Method::PATCH => "PATCH",
        Method::POST => "POST",
        Method::PUT => "PUT",
        Method::TRACE => "TRACE",
        _ => "",
    }
}
