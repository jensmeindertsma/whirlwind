use serde::{Deserialize, Serialize};
use std::io::{self};
use tracing::{error, info};
use whirlwind::{Body, Counter, Message, Node};

fn main() {
    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_ansi(false)
        .init();

    let mut counter = Counter::new();
    let mut node = Node::initialize(&mut counter).unwrap();

    info!(state = ?node.state, "initialized node");

    while let Some(next) = node.read() {
        let message: Message<Echo> = match next {
            Ok(m) => m,
            Err(e) => {
                error!("failed to read message: {e:?}");
                continue;
            }
        };

        let Echo { echo } = message.body.payload;

        info!(
            source = message.source,
            echo,
            id = message.body.id,
            "receiving echo message"
        );

        let reply_id = counter.next();
        node.send(Message {
            source: node.state.id.clone(),
            destination: message.source.clone(),
            body: Body {
                id: Some(reply_id),
                in_reply_to: message.body.id,
                payload: EchoOk { echo },
            },
        })
        .unwrap();

        info!(
            destination = message.source,
            id = reply_id,
            in_reply_to = message.body.id,
            "replying with echo_ok"
        )
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
