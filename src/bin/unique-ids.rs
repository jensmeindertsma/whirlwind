use serde::{Deserialize, Serialize};
use std::io::{self};
use tracing::{error, info};
use whirlwind::{Body, Counter, Message, Node};

fn main() {
    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_ansi(false)
        .init();

    let mut message_counter = Counter::new();
    let mut generate_counter = Counter::new();

    let mut node = Node::initialize(&mut message_counter).unwrap();

    info!(state = ?node.state, "initialized node");

    while let Some(next) = node.read() {
        let message: Message<Generate> = match next {
            Ok(m) => m,
            Err(e) => {
                error!("failed to read message: {e:?}");
                continue;
            }
        };

        info!(
            source = message.source,
            id = message.body.id,
            "receiving generate message"
        );

        let reply_id = message_counter.next();
        let generated_id = format!("{}-{}", node.state.id, generate_counter.next());
        node.send(Message {
            source: node.state.id.clone(),
            destination: message.source.clone(),
            body: Body {
                id: Some(reply_id),
                in_reply_to: message.body.id,
                payload: GenerateOk {
                    id: generated_id.clone(),
                },
            },
        })
        .unwrap();

        info!(
            destination = message.source,
            in_reply_to = message.body.id,
            generated_id,
            "replying with generate_ok"
        )
    }
}

#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename = "generate")]
struct Generate {}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename = "generate_ok")]
struct GenerateOk {
    id: String,
}
