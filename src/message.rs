use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Message<Payload> {
    #[serde(rename = "src")]
    pub source: String,

    #[serde(rename = "dest")]
    pub destination: String,

    pub body: Body<Payload>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Body<Payload> {
    #[serde(rename = "msg_id")]
    pub message_id: Option<u32>,

    pub in_reply_to: Option<u32>,

    #[serde(flatten)]
    pub payload: Payload,
}
