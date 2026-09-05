use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::io::{self, BufRead, Lines, StdinLock, StdoutLock, Write};

pub struct Node<'a> {
    input: Lines<StdinLock<'a>>,
    output: StdoutLock<'a>,
    id: String,
    cluster: Vec<String>,
    next_message_id: u16,
}

impl Node<'_> {
    pub fn initialize() -> Self {
        let input = io::stdin().lock().lines();
        let output = io::stdout().lock();

        let mut node = Self {
            input,
            output,

            // No problem to place temporary values here as both constructors do not
            // allocate memory until data is inserted and we replace these later.
            id: String::new(),
            cluster: Vec::new(),

            next_message_id: 1,
        };

        let incoming: RawMessage<Initialization> =
            Node::read(&mut node).expect("there should be an initialization message");

        let RawMessage {
            source,
            body:
                Body {
                    id: message_id,
                    payload:
                        Initialization {
                            node_id,
                            node_ids: cluster,
                        },
                    ..
                },
            ..
        } = incoming;

        node.id = node_id;
        node.cluster = cluster;

        let response = RawMessage {
            source: node.id.clone(),
            destination: source,
            body: Body {
                id: node.next_message_id,
                in_reply_to: Some(message_id),
                payload: InitializationOk {},
            },
        };

        Node::send(&mut node, response);

        node.next_message_id += 1;

        node
    }

    fn read<Payload: DeserializeOwned>(&mut self) -> Option<RawMessage<Payload>> {
        let line = self
            .input
            .next()?
            .expect("standard input should be readable");

        let message =
            serde_json::from_str(&line).expect("incoming payload should match expected payload");

        Some(message)
    }

    pub fn send<Payload: Serialize>(&mut self, message: impl IntoMessage<Payload>) {
        let serialized =
            serde_json::to_string(&message).expect("message serialization should succeed");

        writeln!(self.output, "{serialized}").expect("standard output should be writeable");
    }

    pub fn receive(&mut self) {}

    pub fn messages<Payload: DeserializeOwned>(
        &mut self,
    ) -> impl Iterator<Item = Message<Payload>> {
        std::iter::from_fn(|| self.read())
    }

    pub fn handle<Incoming: DeserializeOwned, Outgoing: Serialize>(
        &mut self,
        handler: fn(Message<Incoming>) -> Reply<Outgoing>,
    ) {
        while let Some(message) = self.read() {
            let incoming_message_id = message.body.id;
            let incoming_source = message.source.clone();

            let reply = handler(message);

            self.write(RawMessage {
                source: self.id.clone(),
                destination: incoming_source,
                body: Body {
                    id: self.next_message_id,
                    in_reply_to: Some(incoming_message_id),
                    payload: reply.payload,
                },
            });
            self.next_message_id += 1;
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct RawMessage<Payload> {
    #[serde(rename = "src")]
    source: String,

    #[serde(rename = "dest")]
    destination: String,

    body: Body<Payload>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Body<Payload> {
    #[serde(rename = "msg_id")]
    id: u16,

    in_reply_to: Option<u16>,

    #[serde(flatten)]
    payload: Payload,
}

pub fn Message {
    source: String,
    d
}

pub struct Reply<Payload> {
    pub payload: Payload,
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
