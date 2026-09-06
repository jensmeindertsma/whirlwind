use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Lines, StdinLock, StdoutLock};

pub struct Node<'a> {
    input: Lines<StdinLock<'a>>,
    output: StdoutLock<'a>,
    pub id: String,
    counter: u16,
}

impl Node<'_> {
    pub fn initialize() -> Self {
        let input = io::stdin().lock().lines();
        let output = io::stdout().lock();

        let node = Node {
            input,
            output,
            id: String::new(),
            counter: 1,
        };

        node
    }
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
