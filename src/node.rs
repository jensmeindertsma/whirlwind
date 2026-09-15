use crate::{Body, Message};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::io::{self, BufRead, Lines, StdinLock, StdoutLock, Write};

pub struct Node<'a> {
    input: Lines<StdinLock<'a>>,
    output: StdoutLock<'a>,
    pub id: String,
    counter: u16,
}

impl Node<'_> {
    pub fn initialize() -> Self {
        let input = io::stdin().lock().lines();
        let output = io::stdout().lock();

        let counter = 1;

        let mut node = Node {
            input,
            output,
            id: String::new(),
            counter,
        };

        let incoming: Message<Initialization> = node
            .receive()
            .expect("there should be an initialization message");

        let Initialization { node_id, node_ids } = incoming.body.payload;

        node.id = node_id;

        tracing::debug!("cluster: {node_ids:?}");

        let reply = Message {
            source: node.id.clone(),
            destination: incoming.source,
            body: Body {
                message_id: Some(counter),
                in_reply_to: incoming.body.message_id,
                payload: InitializationOk {},
            },
        };

        node.send(reply);

        node
    }

    pub fn receive<Payload: DeserializeOwned>(&mut self) -> Option<Message<Payload>> {
        let line = self
            .input
            .next()?
            .expect("standard input should be readable");

        let message =
            serde_json::from_str(&line).expect("incoming payload should match expected payload");

        Some(message)
    }

    pub fn send<Payload: Serialize>(&mut self, message: Message<Payload>) {
        let serialized = serde_json::to_string(&message).expect("message should be serializable");

        writeln!(self.output, "{serialized}").expect("standard output should be writeable");
    }

    pub fn next_message_id(&mut self) -> u16 {
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
