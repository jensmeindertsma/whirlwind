mod protocol;

use protocol::{Body, IncomingProtocolMessage, OutgoingProtocolMessage};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::io::{self, BufRead, Lines, StdinLock, StdoutLock, Write};

use crate::message::{Incoming, Message};

pub struct Node<'a> {
    pub id: String,
    pub cluster: Vec<String>,

    input: Lines<StdinLock<'a>>,
    output: StdoutLock<'a>,

    counter: u32,
}

impl Node<'_> {
    pub fn initialize() -> Self {
        let input = io::stdin().lock().lines();
        let output = io::stdout().lock();

        let counter = 1;

        let mut node = Node {
            // Temporary non-allocating value until initialization completes
            id: String::new(),
            cluster: Vec::new(),

            input,
            output,

            counter,
        };

        let incoming: Incoming<Initialization> = node
            .receive()
            .expect("there should be an initialization message");

        let Initialization { node_id, node_ids } = incoming.payload;

        node.id = node_id;
        node.cluster = node_ids;

        let reply = Message {
            id: counter,
            destination: &incoming.source,
            in_reply_to: Some(incoming.id),
            payload: InitializationOk {},
        };

        node.send(&reply);

        node
    }

    pub fn receive<Payload: DeserializeOwned>(&mut self) -> Option<Incoming<Payload>> {
        let line = self
            .input
            .next()?
            .expect("standard input should be readable");

        let protocol_message: IncomingProtocolMessage<Payload> =
            serde_json::from_str(&line).expect("incoming payload should match expected payload");

        let Body {
            message_id,
            in_reply_to,
            payload,
        } = protocol_message.body;

        Some(Incoming {
            id: message_id.expect("all messages should have an ID"),
            source: protocol_message.source,
            in_reply_to,
            payload,
        })
    }

    pub fn send<Payload: Serialize>(&mut self, message: &Message<Payload>) {
        let Message {
            id,
            destination,
            in_reply_to,
            payload,
        } = message;

        let protocol_message = OutgoingProtocolMessage {
            source: &self.id,
            destination,
            body: Body {
                message_id: Some(*id),
                in_reply_to: *in_reply_to,
                payload,
            },
        };

        let serialized =
            serde_json::to_string(&protocol_message).expect("message should be serializable");

        writeln!(self.output, "{serialized}").expect("standard output should be writeable");
    }

    pub fn new_message_id(&mut self) -> u32 {
        self.counter += 1;

        self.counter
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename = "init")]
struct Initialization {
    node_id: String,
    node_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename = "init_ok")]
struct InitializationOk {}
