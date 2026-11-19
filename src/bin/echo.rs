use serde::{Deserialize, Serialize};
use whirlwind::{Message, Node};

fn main() {
    let mut node = Node::initialize();

    while let Some(incoming) = node.receive::<Echo>() {
        let Echo { echo } = incoming.payload;

        let reply = Message {
            id: node.new_message_id(),
            destination: &incoming.source,
            in_reply_to: Some(incoming.id),
            payload: EchoOk { echo },
        };

        node.send(&reply);
    }
}

#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename = "echo")]
struct Echo {
    echo: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename = "echo_ok")]
struct EchoOk {
    echo: String,
}
