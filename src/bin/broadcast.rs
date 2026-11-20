use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    io,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use tracing::{Level, debug, info, info_span};
use whirlwind::{Body, Message, Node};

fn main() {
    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_ansi(false)
        .with_target(false)
        .with_max_level(Level::DEBUG)
        .init();

    let initialization_span = info_span!("initialization").entered();

    let mut node = Node::initialize();

    initialization_span.exit();

    let mut neighbors = Vec::new();

    let mut values = HashSet::new();

    let mut unacknowledged: Arc<Mutex<HashMap<u32, Message<Payload>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    while let Some(message) = node.receive::<Payload>() {
        let response = match message.body.payload {
            Payload::Topology { mut topology } => {
                info!("received new topology");

                if let Some(new_neighbors) = topology.remove(&node.id) {
                    neighbors = new_neighbors;

                    info!("my neighbors are {neighbors:?}")
                }

                Payload::TopologyOk
            }
            Payload::Broadcast { message: value } => {
                let _broadcast_span = info_span!("broadcast").entered();

                info!(source = message.source, value, "received message",);

                // Only broadcast new values
                if !values.contains(&value) {
                    values.insert(value);

                    // Don't broadcast back to source
                    for neighbor in neighbors.iter().filter(|n| **n != message.source) {
                        info!(destination = neighbor, value, "broadcasting new value");

                        let message_id = node.new_message_id();

                        let message = Message {
                            source: node.id.clone(),
                            destination: neighbor.to_string(),
                            body: Body {
                                message_id: Some(message_id),
                                in_reply_to: None,
                                payload: Payload::Broadcast { message: value },
                            },
                        };

                        unacknowledged
                            .lock()
                            .expect("mutex should not deadlock")
                            .insert(message_id, message.clone());

                        let send_handle = node.handler();

                        let unacknowledged_handle = unacknowledged.clone();

                        thread::spawn(move || {
                            loop {
                                thread::sleep(Duration::from_millis(1000));

                                if unacknowledged_handle
                                    .lock()
                                    .expect("mutex should not deadlock")
                                    .contains_key(&message_id)
                                {
                                    send_handle.send(&message);
                                } else {
                                    break;
                                }
                            }
                        });

                        node.send(&message);
                    }
                }

                Payload::BroadcastOk
            }
            Payload::BroadcastOk => {
                let acknowledged_id = message.body.in_reply_to.expect("broadcast acknowledgment");

                debug!("broadcast message {acknowledged_id} was acknowledged");

                unacknowledged
                    .lock()
                    .expect("mutex should not deadlock")
                    .remove(&acknowledged_id);

                continue;
            }
            Payload::Read => {
                let _read_span = info_span!("read").entered();

                info!(source = message.source, "received request");

                info!(
                    "responding to request with values {:?}",
                    values.iter().copied().collect::<Vec<u32>>()
                );

                Payload::ReadOk {
                    messages: values.iter().copied().collect(),
                }
            }
            _ => continue,
        };

        let reply = Message {
            source: node.id.clone(),
            destination: message.source.clone(),
            body: Body {
                message_id: Some(node.new_message_id()),
                in_reply_to: message.body.message_id,
                payload: response,
            },
        };

        node.send(&reply);
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Broadcast {
        message: u32,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<u32>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}
