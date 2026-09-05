use serde::{Serialize, de::DeserializeOwned};
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

    let node = Node::initialize();

    node.handle(|message: Message<Echo>| Reply {
        payload: EchoOk {
            echo: message.payload().echo,
        },
    });
}

#[derive(Debug, DeserializeOwned)]
#[serde(tag = "type")]
#[serde(rename = "echo")]
struct Echo<'a> {
    echo: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename = "echo_ok")]
struct EchoOk<'a> {
    echo: &'a str,
}
