use reqwest::{Client, Response, Error};
use super::helper::*;

pub struct Lookupd {
    instance: Client,
    base_url: String
}

impl Lookupd {
    pub fn new(lookupd: Option<String>) -> Self {
        Lookupd {
            instance: create_request_instance(),
            base_url: lookupd.unwrap_or("default".to_string())
        }
    }

    pub async fn ping(&self) -> Result<Response, Error> {
        self.instance.get(generate_request_url(&self.base_url, "/ping"))
            .send()
            .await
    }

    pub async fn lookup(&self, topic: &str) -> Result<Response, Error> {
        self.instance.get(generate_request_url(&self.base_url, "/lookup"))
            .query(&[("topic", topic)])
            .send()
            .await
    }

    pub async fn topics(&self) -> Result<Response, Error> {
        self.instance.get(generate_request_url(&self.base_url, "/topics"))
            .send()
            .await
    }

    pub async fn channels(&self, topic: &str) -> Result<Response, Error> {
        self.instance.get(generate_request_url(&self.base_url, "/channels"))
            .query(&[("topic", topic)])
            .send()
            .await
    }

    pub async fn nodes(&self) -> Result<Response, Error> {
        self.instance.get(generate_request_url(&self.base_url, "/nodes"))
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
