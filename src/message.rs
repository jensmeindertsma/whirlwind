use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message<Payload> {
    #[serde(rename = "src")]
    pub source: String,
    #[serde(rename = "dest")]
    pub destination: String,
    pub body: Body<Payload>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Body<Payload> {
    #[serde(rename = "msg_id")]
    pub id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}
