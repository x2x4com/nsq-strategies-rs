#![allow(dead_code, unused_variables)]
use std::sync::{Arc, Mutex};
use tokio_nsq::{NSQProducerConfig, NSQProducer, NSQEvent, NSQConfigShared};
use anyhow::Result;

use crate::api::{nsqd::Nsqd, lookupd_cluster::LookupdCluster};

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


pub struct ProducerConn {
    pub broadcast_address: String,
    pub tcp_port: u16,
    pub conn: NSQProducer
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
    conns: Vec<ProducerConn>,
    is_closed: bool,

}

impl Producer {
    pub fn new(opts: ProducerConfig, nsqd_opts: Option<NSQConfigShared>) -> Self {
        if opts.lookupd_url.is_some() {
            Producer {
                opts: opts.clone(),
                nsqd: None,
                lookup_cluster: Some(LookupdCluster::new(opts.lookupd_url.unwrap().split(",").map(|d| d.to_string()).collect::<Vec<String>>())),
                counter: Arc::new(Mutex::new(0)),
                conns: Vec::new(),
                is_closed: false
            }
        } else {
            Producer {
                opts: opts.clone(),
                nsqd: Some(Nsqd::new(opts.nsqd_host.unwrap(), opts.nsqd_port.unwrap(), None, None, nsqd_opts)),
                lookup_cluster: None,
                counter: Arc::new(Mutex::new(0)),
                conns: Vec::new(),
                is_closed: false
            }
        }
    }

    pub async fn connect(&mut self) {
        match self.lookup_cluster {
            Some(ref lookup_cluster) => {
                
                for node in lookup_cluster.nodes().await {
                    if let Some(conn) = self.connect_nsqd(node.broadcast_address.as_str(), node.tcp_port, None).await {
                        self.conns.push(ProducerConn { broadcast_address: node.broadcast_address.clone(), tcp_port: node.tcp_port, conn });
                    }
                }

            },
            None => {
                if let Some(nsqd) = self.nsqd.as_ref() {
                    if let Some(conn) = self.connect_nsqd(nsqd.broadcast_address.as_str(), nsqd.tcp_port, None).await {
                        self.conns.push(ProducerConn { broadcast_address: nsqd.broadcast_address.clone(), tcp_port: nsqd.tcp_port, conn });
                    }
                }
            }
        }
    }

    async fn connect_nsqd(&self, nsqd_host: &str, nsqd_port: u16, opts: Option<NSQConfigShared>) -> Option<NSQProducer> {
        // https://docs.rs/tokio-nsq/latest/tokio_nsq/struct.NSQProducer.html
        let mut producer: NSQProducer;
        if let Some(opts) = opts {
            producer = NSQProducerConfig::new(format!("{}:{}", nsqd_host, nsqd_port)).set_shared(opts).build();
        } else {
            producer = NSQProducerConfig::new(format!("{}:{}", nsqd_host, nsqd_port)).build();
        }
        if let NSQEvent::Healthy() = producer.consume().await.unwrap() {
            Some(producer)
        } else {
            None
        }
        
    }

    fn reconnect_nsqd(&self, nsqd_host: &str, nsqd_port: u16, opts: Option<NSQConfigShared>) {

    }

    pub async fn produce(&self, topic: &str, message: &str) -> Result<()> {

        Ok(())
    }

    async fn produce_once(&self, topic: &str, message: &str) {
        match self.opts.strategy {
            ProducerStrategy::RoundRobin => {
                
            },
            ProducerStrategy::FanOut => {
                
            }
        }
    }

    pub fn close_all(&self) {
        
    }

}

pub fn index_of_connection(conns: &Vec<ProducerConn>, nsqd_host: &str, nsqd_port: u16) -> Option<usize> {
    let mut index: Option<usize> = None;
    for (i, conn) in conns.iter().enumerate() {
        if conn.broadcast_address == nsqd_host && conn.tcp_port == nsqd_port {
            index = Some(i);
            break;
        }
    }
    index
}
