use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io};
use tracing::{Level, debug, info};
use whirlwind::{Body, Message, Node};

fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(io::stderr)
        .with_max_level(Level::DEBUG)
        .init();

    let mut node = Node::initialize();

    debug!("assigned node ID {}", node.id);

    info!("initialization completed");

    let mut topology = HashMap::new();

    while let Some(message) = node.receive::<Payload>() {
        let reply = Message {
            source: node.id.clone(),
            destination: message.source,
            body: Body {
                message_id: node.new_message_id(),
                in_reply_to: Some(message.body.message_id),
                payload: match message.body.payload {
                    Payload::Topology {
                        topology: new_topology,
                    } => {
                        debug!("received new topology: {topology:?}");
                        debug!("my neighbors are: {:?}", topology["x"]);

                        topology = new_topology;

                        Payload::BroadcastOk
                    }
                    _ => todo!(),
                },
            },
        };

        node.send(&reply);
    }
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Broadcast {
        message: u32,
    },
    BroadcastOk,
    Read,
    ReadOk {
        values: Vec<u32>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}
