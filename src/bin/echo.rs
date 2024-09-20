use serde::{Deserialize, Serialize};
use std::io::{self};
use tracing::{error, info};
use whirlwind::{Body, Counter, Message, Node};

fn main() {
    tracing_subscriber::fmt().with_writer(io::stderr).init();

    let mut counter = Counter::new();

    let mut node = Node::initialize(&mut counter).unwrap();

    info!(?node.state, "initialized node");

    while let Some(next) = node.read() {
        let message: Message<IncomingPayload> = match next {
            Ok(m) => m,
            Err(e) => {
                error!("failed to read message: {e:?}");
                continue;
            }
        };

        let IncomingPayload { echo } = message.body.payload;

        node.send(Message {
            source: node.state.id.clone(),
            destination: message.source,
            body: Body {
                id: Some(counter.next()),
                in_reply_to: message.body.id,
                payload: OutgoingPayload { echo },
            },
        })
        .unwrap();
    }
}

#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename = "echo")]
struct IncomingPayload {
    echo: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename = "echo_ok")]
struct OutgoingPayload {
    echo: String,
}
