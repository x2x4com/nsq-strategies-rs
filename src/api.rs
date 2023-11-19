pub mod lookupd;
pub mod lookupd_cluster;
pub mod nsqd;
mod helper;

#[cfg(test)]
mod tests {
    use tokio_nsq::{NSQProducerConfig, NSQTopic, NSQEvent, NSQChannel, NSQConsumerConfig, NSQConsumerConfigSources, NSQConsumerLookupConfig};
    use std::collections::HashSet;
    // use tokio::runtime::Runtime;

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    pub async fn nsq_single() {

        // let rt = Runtime::new().unwrap();

        let topic = NSQTopic::new("x2x4").unwrap();

        let mut producer = NSQProducerConfig::new("127.0.0.1:9020").build();

        // Wait until a connection is initialized
        if let NSQEvent::Healthy() = producer.consume().await.unwrap() {
            // Publish a single message
            producer.publish(&topic, b"alice1".to_vec()).await.unwrap();
        } else {
            panic!("failed to connect to nsqd");
        }

        if let NSQEvent::Ok() = producer.consume().await.unwrap() {
            println!("send successed")
        } else {
            panic!("failed to send message")
        }

        let mut addresses = HashSet::new();
        addresses.insert("http://127.0.0.1:9001".to_string());
        addresses.insert("http://127.0.0.1:9011".to_string());
        let channel = NSQChannel::new("first").unwrap();
        let mut consumer = NSQConsumerConfig::new(topic, channel)
            .set_max_in_flight(15)
            .set_sources(
                NSQConsumerConfigSources::Lookup(
                    NSQConsumerLookupConfig::new().set_addresses(addresses)
                )
            )
        .build();
        let message = consumer.consume_filtered().await.unwrap();

        let message_body_str = std::str::from_utf8(&message.body).unwrap();
        println!("message body = {}", message_body_str);

        message.finish().await;
        println!("End run consumer");
    }
}