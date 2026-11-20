use serde::{Deserialize, Serialize};
use std::io;
use tracing::{Level, info_span};
use whirlwind::{Body, Message, Node};

fn main() {
    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_ansi(false)
        .with_target(false)
        .with_max_level(Level::DEBUG)
        .init();

    let initialization_span = info_span!("initialization").entered();

    let mut node = Node::initialize();

    initialization_span.exit();

    while let Some(message) = node.receive::<Echo>() {
        let Echo { echo } = message.body.payload;

        let reply = Message {
            source: node.id.clone(),
            destination: message.source,
            body: Body {
                message_id: Some(node.new_message_id()),
                in_reply_to: message.body.message_id,
                payload: EchoOk { echo },
            },
        };

        node.send(&reply);
    }
}

#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename = "echo")]
struct Echo {
    echo: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename = "echo_ok")]
struct EchoOk {
    echo: String,
}
