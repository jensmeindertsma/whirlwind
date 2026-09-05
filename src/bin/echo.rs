use serde::{Deserialize, Serialize};
use std::io;
use tracing::Level;
use whirlwind::{Message, Node, Reply};

fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_target(false)
        .with_writer(io::stderr)
        .with_max_level(Level::DEBUG)
        .init();

    let mut node = Node::initialize();

    node.handle(|message: Message<Echo>| Reply {
        payload: EchoOk {
            echo: message.body.payload.echo,
        },
    });
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
