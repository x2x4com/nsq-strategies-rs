pub enum ProducerStrategy {
    RoundRobin,
    FanOut
}

impl ProducerStrategy {
    fn as_str(&self) -> &'static str {
        match self {
            ProducerStrategy::RoundRobin => "round_robin",
            ProducerStrategy::FanOut => "round_robin"
        }
    }
}

pub struct Producer {
    opts: String,
}
