#[cfg(feature = "server")]
pub use self::server::{HttpServerMetricsLayer, HttpServerMetricsService};
#[cfg(feature = "util")]
pub use self::util::ServiceBuilderExt;

#[cfg(feature = "server")]
pub mod server;
#[cfg(feature = "util")]
pub mod util;

pub const HTTP_SERVER_REQUESTS_TOTAL: &str = "http_server_requests_total";
pub const HTTP_SERVER_REQUESTS_PENDING: &str = "http_server_requests_pending";
pub const HTTP_SERVER_REQUEST_DURATION_SECONDS: &str = "http_server_request_duration_seconds";
