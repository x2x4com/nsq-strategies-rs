use super::lookupd::Lookupd;
use crate::utils::*;
use std::collections::HashSet;
use reqwest::Response;
use serde_json::Value;
use std::sync::Arc;
use std::sync::Mutex;

pub struct LookupdCluster {
    _lookupds: Vec<Lookupd>,
}

impl LookupdCluster {
    pub fn new(addresses: Vec<&str>) -> Self {
        LookupdCluster {
            _lookupds: addresses.into_iter()
                                .map(|address| Lookupd::new(Some(to_url(address.to_string()))))
                                .collect()
        }
        
    }

    pub async fn nodes(&self) -> Vec<Value> {
        let set: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
        let nodes: Arc<Mutex<Vec<Value>>> = Arc::new(Mutex::new(vec![]));
        let _ = self._lookupds.iter().map(|lookupd| async {
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
                    let hostname = p.as_object()
                                          .and_then(|d| d.get("hostname")
                                                .and_then(|d| d.as_str()))
                                          .unwrap().to_string();
                    // test hostname is in 
                    let mut s = set_lock.lock().unwrap();
                    let mut n = nodes_lock.lock().unwrap();
                    if !s.contains(hostname.as_str()) {
                        s.insert(hostname.clone());
                        n.push(p.clone());
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