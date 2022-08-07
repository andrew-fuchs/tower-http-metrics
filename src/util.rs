#[cfg(feature = "tower")]
use tower::ServiceBuilder;
use tower_layer::Stack;

#[cfg(feature = "server")]
use crate::server::HttpServerMetricsLayer;

#[cfg(feature = "tower")]
pub trait HttpMetricsServiceBuilderExt<L> {
    #[cfg(feature = "server")]
    fn http_server_metrics(self) -> ServiceBuilder<Stack<HttpServerMetricsLayer, L>>;
}

#[cfg(feature = "tower")]
impl<L> HttpMetricsServiceBuilderExt<L> for ServiceBuilder<L> {
    #[cfg(feature = "server")]
    fn http_server_metrics(self) -> ServiceBuilder<Stack<HttpServerMetricsLayer, L>> {
        self.layer(HttpServerMetricsLayer::new())
    }
}
