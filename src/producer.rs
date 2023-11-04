use std::sync::{Arc, Mutex};
use crate::api::{nsqd::{Nsqd, NsqdConfig}, lookupd_cluster::LookupdCluster};

#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct ProducerConfig {
    pub strategy: ProducerStrategy,
    pub retry: Option<u8>,
    pub delay: u32,
    pub max_fanout_nodes: Option<u32>,
    pub max_inflight: Option<u32>,
    pub lookupd_url: Option<String>,
    pub nsqd_host: Option<String>,
    pub nsqd_port: Option<u16>
}

impl Default for ProducerConfig {
    fn default() -> Self {
        ProducerConfig {
            strategy: ProducerStrategy::RoundRobin,
            retry: Some(3),
            delay: 0,
            max_fanout_nodes: None,
            max_inflight: Some(10),
            lookupd_url: None,
            nsqd_host: Some("localhost".to_string()),
            nsqd_port: Some(4160)        }
    }
}

pub struct Producer {
    opts: ProducerConfig,
    nsqd: Option<Nsqd>,
    lookup_cluster: Option<LookupdCluster>,
    pub counter: Arc<Mutex<u64>>,
}

impl Producer {
    pub fn new(opts: ProducerConfig) -> Self {
        if opts.lookupd_url.is_some() {
            Producer {
                opts: opts.clone(),
                nsqd: None,
                lookup_cluster: Some(LookupdCluster::new(opts.lookupd_url.unwrap().split(",").map(|d| d.to_string()).collect::<Vec<String>>())),
                counter: Arc::new(Mutex::new(0))
            }
        } else {
            let nsqd_opts = NsqdConfig {};
            Producer {
                opts: opts.clone(),
                nsqd: Some(Nsqd::new(opts.nsqd_host.unwrap(), opts.nsqd_port.unwrap(), nsqd_opts)),
                lookup_cluster: None,
                counter: Arc::new(Mutex::new(0))
            }
        }
    }

    pub async fn connect(&mut self) {

    }

    fn connect_nsqd(&self, nsqd_host: &str, nsqd_port: u16, opts: NsqdConfig) {

    }

    fn reconnect_nsqd(&self, nsqd_host: &str, nsqd_port: u16, opts: NsqdConfig) {

    }

    pub async fn produce(&self, topic: &str, message: &str, opts: ProducerConfig) {

    }

    async fn produce_once(&self, topic: &str, message: &str, opts: ProducerConfig) {

    }

    pub fn close_all(&self) {

    }
}
