use http::Method;

pub use self::layer::HttpServerMetricsLayer;
pub use self::service::HttpServerMetricsService;

mod body;
mod future;
mod layer;
mod service;

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
