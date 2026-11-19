use super::{Body, Message};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::io::{self, BufRead, Lines, StdinLock, StdoutLock, Write};
use tracing::{debug, info};

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

        debug!("waiting for message");

        let message: Message<Initialization> = node
            .receive()
            .expect("there should be an initialization message");

        debug!("received message");

        let Initialization { node_id, node_ids } = message.body.payload;

        node.id = node_id;
        node.cluster = node_ids;

        let reply = Message {
            source: node.id.clone(),
            destination: message.source,
            body: Body {
                message_id: counter,
                in_reply_to: Some(message.body.message_id),
                payload: InitializationOk {},
            },
        };

        debug!("sending acknowledgement");

        node.send(&reply);

        info!("completed successfully");

        node
    }

    pub fn receive<Payload: DeserializeOwned>(&mut self) -> Option<Message<Payload>> {
        let line = self
            .input
            .next()?
            .expect("standard input should be readable");

        let message: Message<Payload> =
            serde_json::from_str(&line).expect("Message payload should match expected payload");

        Some(message)
    }

    pub fn send<Payload: Serialize>(&mut self, message: &Message<Payload>) {
        let serialized = serde_json::to_string(message).expect("message should be serializable");

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
