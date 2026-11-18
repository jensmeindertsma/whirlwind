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

    while let Some(incoming) = node.receive::<Echo>() {
        let Echo { echo } = incoming.payload;

        let reply = Message {
            id: node.new_message_id(),
            destination: &incoming.source,
            in_reply_to: Some(incoming.id),
            payload: EchoOk { echo },
        };

        node.send(&reply);
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename = "echo")]
struct Echo {
    echo: String,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename = "echo_ok")]
struct EchoOk {
    echo: String,
}
