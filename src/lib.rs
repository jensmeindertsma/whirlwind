use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{
    io::{self, BufRead, Lines, StdinLock, StdoutLock, Write},
    marker::PhantomData,
};

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

        let incoming: Message<Initialization> =
            Node::read(&mut node).expect("there should be an initialization message");

        let Message {
            source,
            body:
                Body {
                    message_id,
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

        let response = Message {
            source: node.id.clone(),
            destination: source,
            body: Body {
                message_id: node.next_message_id,
                in_reply_to: Some(message_id),
                payload: InitializationOk {},
            },
        };

        Node::send(&mut node, response);

        node.next_message_id += 1;

        node
    }

    fn read<Payload: DeserializeOwned>(&mut self) -> Option<Message<Payload>> {
        let line = self
            .input
            .next()?
            .expect("standard input should be readable");

        let message =
            serde_json::from_str(&line).expect("incoming payload should match expected payload");

        Some(message)
    }

    fn send<Payload: Serialize>(&mut self, message: Message<Payload>) {
        let serialized =
            serde_json::to_string(&message).expect("message serialization should succeed");

        writeln!(self.output, "{serialized}").expect("standard output should be writeable");
    }

    pub fn messages<Payload>(&mut self) -> impl Iterator<Item = Message<Payload>> {
        self.input.map(|line| self.read())
    }

    pub fn handle<Incoming: DeserializeOwned, Outgoing: Serialize>(
        &mut self,
        handler: fn(Message<Incoming>) -> Reply<Outgoing>,
    ) {
        while let Some(message) = self.read() {
            let incoming_message_id = message.body.message_id;
            let incoming_source = message.source.clone();

            let reply = handler(message);

            self.send(Message {
                source: self.id.clone(),
                destination: incoming_source,
                body: Body {
                    message_id: self.next_message_id,
                    in_reply_to: Some(incoming_message_id),
                    payload: reply.payload,
                },
            });
            self.next_message_id += 1;
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message<Payload> {
    #[serde(rename = "src")]
    pub source: String,

    #[serde(rename = "dest")]
    pub destination: String,

    body: Body<Payload>,
}

impl<Payload> Message<Payload> {
    pub fn new() -> MessageBuilder {
        MessageBuilder {}
    }

    pub fn id(&self) -> u16 {
        self.body.message_id
    }

    pub fn in_reply_to(&self) -> Option<u16> {
        self.body.in_reply_to
    }

    pub fn payload(self) -> Payload {
        self.body.payload
    }
}

struct Empty;
struct WithSource;
struct WithDestination;
struct WithReply;
struct WithPayload;

struct MessageBuilder<Stage = Empty> {
    stage: PhantomData<Stage>,
}

impl MessageBuilder<Blank> {
    pub fn with_destination(self, destination: String) -> MessageBuilder<WithDestination> {}
}

#[derive(Debug, Deserialize, Serialize)]
struct Body<Payload> {
    #[serde(rename = "msg_id")]
    message_id: u16,

    in_reply_to: Option<u16>,

    #[serde(flatten)]
    payload: Payload,
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
