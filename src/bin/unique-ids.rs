use serde::{Deserialize, Serialize};
use whirlwind::{Message, Node, Reply};

fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_target(false)
        .with_writer(io::stderr)
        .with_max_level(Level::DEBUG)
        .init();

    let mut node = Node::initialize();

    let node_id = node.id();
    let mut counter = 1;

    while let Some(message) = node.messages().next() {
        node.send(Message {
            destination: message.source,
            in_reply_to: message.id,
            payload: GenerateOk {
                id: format!("{node_id}-{counter}"),
            },
        });

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
