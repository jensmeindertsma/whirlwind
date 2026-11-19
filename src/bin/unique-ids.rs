use serde::{Deserialize, Serialize};
use whirlwind::{Body, Message, Node};

fn main() {
    let mut node = Node::initialize();

    let mut counter = 1;

    while let Some(message) = node.receive::<Generate>() {
        let reply = Message {
            source: node.id.clone(),
            destination: message.source,
            body: Body {
                message_id: node.new_message_id(),
                in_reply_to: Some(message.body.message_id),
                payload: GenerateOk {
                    id: format!("{}-{}", node.id, counter),
                },
            },
        };

        node.send(&reply);

        counter += 1;
    }
}

#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename = "generate")]
struct Generate {}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename = "generate_ok")]
struct GenerateOk {
    id: String,
}
