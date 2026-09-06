use serde::{Deserialize, Serialize};
use std::io;
use tracing::Level;
use whirlwind::{Incoming, Node, Reply};

fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_target(false)
        .with_writer(io::stderr)
        .with_max_level(Level::DEBUG)
        .init();

    let mut node = Node::initialize();

    let mut counter = 0;

    node.handle(|message: Incoming<Generate>| {
        counter += 1;

        Reply {
            payload: GenerateOk {
                id: format!("{}-{}", node.id, counter),
            },
        }
    })
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
