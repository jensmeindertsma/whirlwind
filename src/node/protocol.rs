use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IncomingProtocolMessage<Payload> {
    #[serde(rename = "src")]
    pub source: String,

    #[serde(rename = "dest")]
    pub destination: String,

    pub body: Body<Payload>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutgoingProtocolMessage<'a, Payload> {
    #[serde(rename = "src")]
    pub source: &'a str,

    #[serde(rename = "dest")]
    pub destination: &'a str,

    pub body: Body<Payload>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Body<Payload> {
    #[serde(rename = "msg_id")]
    pub message_id: Option<u32>,

    pub in_reply_to: Option<u32>,

    #[serde(flatten)]
    pub payload: Payload,
}
