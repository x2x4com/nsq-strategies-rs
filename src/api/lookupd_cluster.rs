use super::lookupd::Lookupd;
use crate::utils::*;
use std::collections::HashSet;
use reqwest::Response;
use serde_json::Value;
use std::sync::Arc;
use std::sync::Mutex;
use crate::api::nsqd::Nsqd;

pub struct LookupdCluster {
    lookupds: Vec<Lookupd>,
}

impl LookupdCluster {
    pub fn new(addresses: Vec<String>) -> Self {
        LookupdCluster {
            lookupds: addresses.into_iter()
                                .map(|address| Lookupd::new(Some(to_url(address))))
                                .collect()
        }
        
    }

    pub async fn nodes(&self) -> Vec<Nsqd> {
        let set: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
        let nodes: Arc<Mutex<Vec<Nsqd>>> = Arc::new(Mutex::new(vec![]));
        let _ = self.lookupds.iter().map(|lookupd| async {
            let set_lock = Arc::clone(&set);
            let nodes_lock = Arc::clone(&nodes);
            let producers: &Vec<Value>;
            let resp: Response = lookupd.nodes().await.unwrap();
            if resp.status().is_success() {
                let data: Value = resp.json().await.unwrap();
                producers = data.as_object()
                                .and_then(|d| d.get("data")
                                    .and_then(|d| d.get("producers")
                                        .and_then(|d| d.as_array())))
                                .unwrap();

                let _ = producers.iter().map(|p| {
                    let broadcast_address = p.as_object()
                                          .and_then(|d| d.get("broadcast_address")
                                                .and_then(|d| d.as_str()))
                                          .unwrap().to_string();

                    let tcp_port = p.as_object()
                                          .and_then(|d| d.get("tcp_port")
                                                .and_then(|d| d.as_u64()))
                                          .unwrap() as u16;
                    
                    let http_port = p.as_object()
                                          .and_then(|d| d.get("http_port")
                                                .and_then(|d| d.as_u64()))
                                          .unwrap() as u16;
                    
                    let version = p.as_object()
                                          .and_then(|d| d.get("version")
                                                .and_then(|d| d.as_str()))
                                          .unwrap().to_string();
                    // test hostname is in 
                    let mut s = set_lock.lock().unwrap();
                    let mut n = nodes_lock.lock().unwrap();
                    if !s.contains(broadcast_address.as_str()) {
                        s.insert(broadcast_address.clone());
                        n.push(
                            Nsqd::new(
                                broadcast_address,
                                tcp_port,
                                Some(http_port),
                                Some(version),
                                None
                            )
                        );
                    }
                });
            };
        });

        Arc::clone(&nodes)
          .lock()
          .unwrap()
          .to_vec()
    }
}