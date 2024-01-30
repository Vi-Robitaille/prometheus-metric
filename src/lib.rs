pub use prometheus_exporter_base::prelude::*;
pub use prometheus_metric_derive::*;

pub trait Metric {
    fn get_metrics(&self) -> String;
    fn get_metrics_with_prefix(&self, prefix: String) -> String;
}