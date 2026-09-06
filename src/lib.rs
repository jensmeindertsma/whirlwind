mod node;

pub use node::Node;

pub struct Incoming<Payload> {
    pub source: String,
    pub id: u16,
    pub payload: Payload,
}

pub struct Reply<Payload> {
    pub payload: Payload,
}
