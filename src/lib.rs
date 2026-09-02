use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Lines, StdinLock, StdoutLock};

pub struct Node<'a> {
    input: Lines<StdinLock<'a>>,
    output: StdoutLock<'a>,
}

impl Node<'_> {
    pub fn initialize() -> Self {
        let input = io::stdin().lock().lines();
        let output = io::stdout().lock();

        let mut node = Self { input, output };

        let message: Message<Initialization> = Node::read(&mut node);

        Node::send(
            &mut node,
            Message {
                source: message.body.payload.node_id,
                destination: message.source,
                body: Body {
                    message_id: 1,
                    in_reply_to: Some(message.body.message_id),
                    payload: InitializationOk {},
                },
            },
        );

        node
    }

    fn read<Payload>(&mut self) -> Message<Payload> {}

    fn send<Payload>(&mut self, message: Message<Payload>) {}

    // pub fn messages<Payload>() -> impl Iterator<Item = Message<Payload>> {
    //     todo!()
    // }

    pub fn foo(&mut self, i: String) {}

    // pub fn handle<Incomig, Outgoing>(
    //     &mut self,
    //     handler: fn(Message<Imcomimg>) -> Message<Outgoing>,
    // ) {
    // }
}

#[derive(Debug, Deserialize, Serialize)]
struct Message<Payload> {
    #[serde(rename = "src")]
    source: String,

    #[serde(rename = "dest")]
    destination: String,

    body: Body<Payload>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Body<Payload> {
    #[serde(rename = "msg_id")]
    message_id: u16,

    in_reply_to: Option<u16>,

    #[serde(flatten)]
    payload: Payload,
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
