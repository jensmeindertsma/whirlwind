use serde::{Deserialize, Serialize};
use whirlwind::{Body, Message, Node};

fn main() {
    let mut node = Node::initialize();

    while let Some(message) = node.receive::<Echo>() {
        let Echo { echo } = message.body.payload;

        let reply = Message {
            source: node.id.clone(),
            destination: message.source,
            body: Body {
                message_id: node.new_message_id(),
                in_reply_to: Some(message.body.message_id),
                payload: EchoOk { echo },
            },
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
