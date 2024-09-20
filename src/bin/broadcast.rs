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

    let mut current_topology: Option<HashMap<String, Vec<String>>> = None;
    let mut messages_seen = Vec::new();

    while let Some(next) = node.read() {
        let message: Message<Payload> = match next {
            Ok(m) => m,
            Err(e) => {
                error!("failed to read message: {e:?}");
                continue;
            }
        };

        let outgoing_payload = match message.body.payload {
            Payload::Broadcast { message } => {
                info!(value = message, "received broadcast message");
                if !messages_seen.contains(&message) {
                    messages_seen.push(message);

                    // We will broadcast to our neighbors
                    match &current_topology {
                        None => {
                            error!("received broadcast message that i haven't seen before but I have no topology!");
                            continue;
                        }
                        Some(topology) => {
                            let neighbors = topology
                                .get(&node.state.id)
                                .expect("we should be part of the topology");

                            for neighbor in neighbors {
                                node.send(Message {
                                    source: node.state.id.clone(),
                                    destination: neighbor.clone(),
                                    body: Body {
                                        id: Some(counter.next()),
                                        in_reply_to: None,
                                        payload: Payload::Broadcast { message },
                                    },
                                })
                                .unwrap();
                            }
                        }
                    }
                }

                Payload::BroadcastOk
            }

            Payload::Read => {
                info!(
                    ?messages_seen,
                    "received read_ok request, yielding messages seen"
                );
                Payload::ReadOk {
                    messages: messages_seen.clone(),
                }
            }
            Payload::Topology { topology } => {
                info!(?topology, "received topology");
                current_topology = Some(topology);
                Payload::TopologyOk
            }
            _ => continue,
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

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<usize>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}
