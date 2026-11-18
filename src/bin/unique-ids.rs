use serde::{Deserialize, Serialize};
use std::io;
use tracing::Level;
use whirlwind::{Message, Node};

fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_target(false)
        .with_writer(io::stderr)
        .with_max_level(Level::DEBUG)
        .init();

    let mut node = Node::initialize();

    let mut counter = 1;

    while let Some(incoming) = node.receive::<Generate>() {
        let reply = Message {
            id: node.new_message_id(),
            destination: &incoming.source,
            in_reply_to: Some(incoming.id),

            payload: GenerateOk {
                id: format!("{}-{}", node.id, counter),
            },
        };

        node.send(&reply);

        counter += 1;
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename = "generate")]
struct Generate {}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename = "generate_ok")]
struct GenerateOk {
    id: String,
}
