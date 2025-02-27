// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use aptos_infallible::Mutex;
use aptos_metrics_core::const_metric::ConstMetric;
use prometheus::{
    core::{Collector, Desc, Describer},
    Opts,
};
use sysinfo::{RefreshKind, System, SystemExt};

use super::common::NAMESPACE;

const MEM_METRICS_COUNT: usize = 6;

const SYSTEM_MEM_TOTAL: &str = "system_mem_total";
const SYSTEM_MEM_USED: &str = "system_mem_used";
const SYSTEM_MEM_FREE: &str = "system_mem_free";

const SYSTEM_SWAP_TOTAL: &str = "system_swap_total";
const SYSTEM_SWAP_USED: &str = "system_swap_used";
const SYSTEM_SWAP_FREE: &str = "system_swap_free";

/// A Collector for exposing system memory metrics
pub(crate) struct MemoryMetricsCollector {
    system: Arc<Mutex<System>>,

    mem_total: Desc,
    mem_used: Desc,
    mem_free: Desc,

    swap_total: Desc,
    swap_used: Desc,
    swap_free: Desc,
}

impl MemoryMetricsCollector {
    fn new() -> Self {
        let system = Arc::new(Mutex::new(System::new_with_specifics(
            RefreshKind::new().with_memory(),
        )));

        let mem_total = Opts::new(SYSTEM_MEM_TOTAL, "Memory total.")
            .namespace(NAMESPACE)
            .describe()
            .unwrap();
        let mem_used = Opts::new(SYSTEM_MEM_USED, "Memory used.")
            .namespace(NAMESPACE)
            .describe()
            .unwrap();
        let mem_free = Opts::new(SYSTEM_MEM_FREE, "Memory free.")
            .namespace(NAMESPACE)
            .describe()
            .unwrap();

        let swap_total = Opts::new(SYSTEM_SWAP_TOTAL, "Swap memory total.")
            .namespace(NAMESPACE)
            .describe()
            .unwrap();
        let swap_used = Opts::new(SYSTEM_SWAP_USED, "Swap memory used.")
            .namespace(NAMESPACE)
            .describe()
            .unwrap();
        let swap_free = Opts::new(SYSTEM_SWAP_FREE, "Swap memory free.")
            .namespace(NAMESPACE)
            .describe()
            .unwrap();

        Self {
            system,
            mem_total,
            mem_used,
            mem_free,

            swap_total,
            swap_used,
            swap_free,
        }
    }
}

impl Default for MemoryMetricsCollector {
    fn default() -> Self {
        MemoryMetricsCollector::new()
    }
}

impl Collector for MemoryMetricsCollector {
    fn desc(&self) -> Vec<&Desc> {
        vec![
            &self.mem_total,
            &self.mem_used,
            &self.mem_free,
            &self.swap_total,
            &self.swap_used,
            &self.swap_free,
        ]
    }

    fn collect(&self) -> Vec<prometheus::proto::MetricFamily> {
        let mut system = self.system.lock();
        system.refresh_memory();

        let mem_total =
            ConstMetric::new_counter(self.mem_total.clone(), system.total_memory() as f64, None)
                .unwrap();
        let mem_used =
            ConstMetric::new_gauge(self.mem_used.clone(), system.used_memory() as f64, None)
                .unwrap();
        let mem_free =
            ConstMetric::new_gauge(self.mem_free.clone(), system.free_memory() as f64, None)
                .unwrap();

        let swap_total =
            ConstMetric::new_counter(self.swap_total.clone(), system.total_swap() as f64, None)
                .unwrap();
        let swap_used =
            ConstMetric::new_gauge(self.swap_used.clone(), system.used_swap() as f64, None)
                .unwrap();
        let swap_free =
            ConstMetric::new_gauge(self.swap_free.clone(), system.free_swap() as f64, None)
                .unwrap();

        let mut mfs = Vec::with_capacity(MEM_METRICS_COUNT);
        mfs.extend(mem_total.collect());
        mfs.extend(mem_used.collect());
        mfs.extend(mem_free.collect());
        mfs.extend(swap_total.collect());
        mfs.extend(swap_used.collect());
        mfs.extend(swap_free.collect());

        mfs
    }
}

#[cfg(test)]
mod tests {
    use super::MemoryMetricsCollector;
    use prometheus::Registry;

    #[test]
    fn test_cpu_collector_register() {
        let collector = MemoryMetricsCollector::default();

        let r = Registry::new();
        let res = r.register(Box::new(collector));
        assert!(res.is_ok());
    }
}
