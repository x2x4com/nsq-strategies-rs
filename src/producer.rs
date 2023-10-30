use std::sync::{Arc, Mutex};

use crate::api::{nsqd::Nsqd, lookupd_cluster::LookupdCluster};
pub enum ProducerStrategy {
    RoundRobin,
    FanOut
}

impl ProducerStrategy {
    fn as_str(&self) -> &'static str {
        match self {
            ProducerStrategy::RoundRobin => "round_robin",
            ProducerStrategy::FanOut => "fan_out"
        }
    }
}

pub struct ProducerConfig {
    pub strategy: ProducerStrategy,
    pub retry: Option<u8>,
    pub delay: Option<u32>,
    pub max_fanout_nodes: Option<u32>
}

impl Default for ProducerConfig {
    fn default() -> Self {
        ProducerConfig {
            strategy: ProducerStrategy::RoundRobin,
            retry: None,
            delay: None,
            max_fanout_nodes: None
        }
    }
}

pub struct Producer {
    opts: ProducerConfig,
    nsqd: Option<Nsqd>,
    lookup_cluster: Option<LookupdCluster>,
    pub counter: Arc<Mutex<u64>>,
}
