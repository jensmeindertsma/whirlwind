use serde::{Deserialize, Serialize};
use whirlwind::{Message, Node};

fn main() {
    let mut node = Node::initialize();

    let mut counter = 1;

    while let Some(incoming) = node.receive::<Generate>() {
        let reply = Message {
            id: node.new_message_id(),
            destination: incoming.source.as_str(),
            in_reply_to: Some(incoming.id),

            payload: GenerateOk {
                id: format!("{}-{}", node.id, counter),
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
