use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{self},
};
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

    let mut _saved_topology: HashMap<String, Vec<String>>;
    let mut messages_seen = Vec::new();

    while let Some(next) = node.read() {
        let message: Message<IncomingPayload> = match next {
            Ok(m) => m,
            Err(e) => {
                error!("failed to read message: {e:?}");
                continue;
            }
        };

        let outgoing_payload = match message.body.payload {
            IncomingPayload::Broadcast { message } => {
                info!(value = message, "received broadcast message");
                messages_seen.push(message);
                OutgoingPayload::BroadcastOk
            }
            IncomingPayload::Read => {
                info!(
                    ?messages_seen,
                    "received read_ok request, yielding messages seen"
                );
                OutgoingPayload::ReadOk {
                    messages: messages_seen.clone(),
                }
            }
            IncomingPayload::Topology { topology } => {
                info!(?topology, "received topology");
                _saved_topology = topology;
                OutgoingPayload::TopologyOk
            }
        };

        let reply_id = counter.next();
        node.send(Message {
            source: node.state.id.clone(),
            destination: message.source.clone(),
            body: Body {
                id: Some(reply_id),
                in_reply_to: message.body.id,
                payload: outgoing_payload,
            },
        })
        .unwrap();
    }
}

#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum IncomingPayload {
    Broadcast {
        message: usize,
    },
    Read,
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum OutgoingPayload {
    BroadcastOk,
    ReadOk { messages: Vec<usize> },
    TopologyOk,
}
