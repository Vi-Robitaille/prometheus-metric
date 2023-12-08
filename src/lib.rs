pub use prometheus_exporter_base::prelude::*;

pub trait Metric {
    fn get_metrics(&self) -> String;
}