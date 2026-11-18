use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    io,
};
use tracing::Level;
use tracing::info_span;
use whirlwind::{Message, Node};

fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_target(false)
        .with_writer(io::stderr)
        .with_max_level(Level::DEBUG)
        .without_time()
        .init();

    let mut node = Node::initialize();

    let node_span = info_span!("node").entered();

    let mut values = HashSet::new();

    let mut network_topology: Option<HashMap<String, Vec<String>>> = None;

    while let Some(incoming) = node.receive::<Payload>() {
        let response = match incoming.payload {
            Payload::Broadcast { value } => {
                let broadcast_span = info_span!("broadcast").entered();

                values.insert(value);

                if let Some(neighbors) = network_topology
                    .as_ref()
                    .and_then(|topology| topology.get(&node.id))
                {
                    tracing::info!("sending value {value} to {neighbors:?}");

                    for neighbor in neighbors {
                        for value in &values {
                            let message_id = node.new_message_id();

                            tracing::info!("sending value {value} to {neighbor} id={message_id}",);

                            node.send(&Message {
                                id: message_id,
                                destination: neighbor,
                                in_reply_to: Some(incoming.id),
                                payload: Payload::Broadcast { value: *value },
                            });
                        }
                    }
                }

                broadcast_span.exit();

                Payload::BroadcastOk
            }
            Payload::Read => Payload::ReadOk {
                values: values.iter().copied().collect(),
            },
            Payload::Topology { topology } => {
                tracing::debug!("neighbors: {:?}", topology.get(&node.id));

                network_topology = Some(topology);

                // TODO: if we adjust our topology map we should keep track of which
                // values have already been sent where so we don't duplicate

                Payload::TopologyOk
            }
            Payload::BroadcastOk => {
                let broadcast_span = info_span!("broadcast", id = incoming.in_reply_to).entered();

                tracing::info!("received by {}", incoming.source);

                broadcast_span.exit();

                continue;
            }
            other => {
                tracing::warn!("unhandled payload: {other:?}");
                continue;
            }
        };

        let reply = Message {
            id: node.new_message_id(),
            destination: &incoming.source,
            in_reply_to: Some(incoming.id),
            payload: response,
        };

        node.send(&reply);
    }

    node_span.exit();
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Broadcast {
        #[serde(rename = "message")]
        value: u16,
    },
    BroadcastOk,
    Read,
    ReadOk {
        #[serde(rename = "messages")]
        values: Vec<u16>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}
