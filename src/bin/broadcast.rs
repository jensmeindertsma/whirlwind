use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io};
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

    let mut values = Vec::new();

    while let Some(message) = node.receive::<Payload>() {
        let response = match message.body.payload {
            Payload::Broadcast { value } => {
                values.push(value);

                Response::BroadcastOk
            }
            Payload::Read => Response::ReadOk { values: &values },
            Payload::Topology { topology } => {
                tracing::debug!("topology: {topology:?}");

                Response::TopologyOk
            }
        };

        let reply = Message {
            source: node.id.clone(),
            destination: message.source,
            body: Body {
                message_id: Some(node.next_message_id()),
                in_reply_to: message.body.message_id,
                payload: response,
            },
        };

        node.send(reply);
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Broadcast {
        #[serde(rename = "message")]
        value: u16,
    },
    Read,
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Response<'a> {
    BroadcastOk,
    ReadOk {
        #[serde(rename = "messages")]
        values: &'a [u16],
    },
    TopologyOk,
}
