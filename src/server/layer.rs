use tower_layer::Layer;

use super::HttpServerMetricsService;

#[derive(Clone)]
pub struct HttpServerMetricsLayer;

impl HttpServerMetricsLayer {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S> Layer<S> for HttpServerMetricsLayer {
    type Service = HttpServerMetricsService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        HttpServerMetricsService::new(inner)
    }
}

impl Default for HttpServerMetricsLayer {
    fn default() -> Self {
        Self::new()
    }
}
