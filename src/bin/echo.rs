use serde::{Deserialize, Serialize};
use std::io;
use tracing::Level;
use whirlwind::{Body, Message, Node};

fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_target(false)
        .with_writer(io::stderr)
        .with_max_level(Level::DEBUG)
        .init();

    let mut node = Node::initialize();

    while let Some(message) = node.receive::<Echo>() {
        let Echo { echo } = message.body.payload;

        let reply = Message {
            source: node.id.clone(),
            destination: message.source,
            body: Body {
                message_id: Some(node.next_message_id()),
                in_reply_to: message.body.message_id,
                payload: EchoOk { echo },
            },
        };

        node.send(reply);
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
