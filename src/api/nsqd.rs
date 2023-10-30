use reqwest::{Client, Response, Error};
use super::helper::*;

pub struct Nsqd {
    instance: Client,
    base_url: String
}

impl Nsqd {
    pub fn new(lookupd: Option<String>) -> Self {
        Nsqd {
            instance: create_request_instance(),
            base_url: lookupd.unwrap_or("default".to_string())
        }
    }

    pub async fn ping(&self) -> Result<Response, Error> {
        self.instance.get(generate_request_url(&self.base_url, "/ping"))
            .send()
            .await
    }


    pub async fn delete_topic(&self, topic: &str) -> Result<Response, Error> {
        self.instance.post(generate_request_url(&self.base_url, "/topic/delete"))
            .query(&[("topic", topic)])
            .send()
            .await
    }

    pub async fn delete_channel(&self, topic: &str, channel: &str) -> Result<Response, Error> {
        self.instance.post(generate_request_url(&self.base_url, "/channel/delete"))
            .query(&[("topic", topic), ("channel", channel)])
            .send()
            .await
    }
}
