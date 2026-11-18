pub struct Incoming<Payload> {
    pub id: u32,
    pub source: String,
    pub in_reply_to: Option<u32>,
    pub payload: Payload,
}

pub struct Message<'a, Payload> {
    pub id: u32,
    pub destination: &'a str,
    pub in_reply_to: Option<u32>,
    pub payload: Payload,
}
