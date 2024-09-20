use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::{self, BufRead, Lines, StdinLock, Write};

use crate::{Body, Counter, Message};

pub struct Node {
    input: Lines<StdinLock<'static>>,
    output: io::StdoutLock<'static>,
    pub state: State,
}

#[derive(Debug)]
pub struct State {
    pub id: String,
    pub nodes: Vec<String>,
}

impl Node {
    pub fn initialize(counter: &mut Counter) -> Result<Self, InitializationError> {
        let mut input = io::stdin().lock().lines();
        let mut output = io::stdout().lock();

        #[derive(Deserialize)]
        #[serde(tag = "type")]
        #[serde(rename = "init")]
        struct InitPayload {
            #[serde(rename = "node_id")]
            id: String,
            #[serde(rename = "node_ids")]
            nodes: Vec<String>,
        }

        let init_message: Message<InitPayload> = serde_json::from_str(
            &input
                .next()
                .ok_or(InitializationError::NoMessage)?
                .map_err(|e| InitializationError::Io(e))?,
        )
        .map_err(|e| InitializationError::FailedToDeserialize(e))?;

        let InitPayload { id, nodes } = init_message.body.payload;

        let state = State { id, nodes };

        #[derive(Serialize)]
        #[serde(tag = "type")]
        #[serde(rename = "init_ok")]
        struct InitOkPayload {}

        let reply = Message {
            source: state.id.clone(),
            destination: init_message.source,
            body: Body {
                id: Some(counter.next()),
                in_reply_to: init_message.body.id,
                payload: InitOkPayload {},
            },
        };

        writeln!(
            output,
            "{}",
            serde_json::to_string(&reply).map_err(|e| InitializationError::FailedToSerialize(e))?
        )
        .map_err(|e| InitializationError::Io(e))?;

        Ok(Self {
            input,
            output,
            state,
        })
    }

    pub fn read<Payload: DeserializeOwned>(
        &mut self,
    ) -> Option<Result<Message<Payload>, ReadError>> {
        let next = self.input.next()?;

        Some(match next {
            Err(e) => Err(ReadError::Io(e)),
            Ok(line) => serde_json::from_str(&line).map_err(|e| ReadError::FailedToDeserialize(e)),
        })
    }

    pub fn send<Payload: Serialize>(&mut self, message: Message<Payload>) -> Result<(), SendError> {
        writeln!(
            self.output,
            "{}",
            serde_json::to_string(&message).map_err(|e| SendError::FailedToSerialize(e))?
        )
        .map_err(|e| SendError::Io(e))
    }
}

#[derive(Debug)]
pub enum InitializationError {
    FailedToDeserialize(serde_json::Error),
    FailedToSerialize(serde_json::Error),
    Io(io::Error),
    NoMessage,
}

#[derive(Debug)]
pub enum ReadError {
    FailedToDeserialize(serde_json::Error),
    Io(io::Error),
}

#[derive(Debug)]
pub enum SendError {
    FailedToSerialize(serde_json::Error),
    Io(io::Error),
}
