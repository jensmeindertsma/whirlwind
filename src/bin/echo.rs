use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};
use tracing::{Level, info_span};

fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_target(false)
        .with_writer(io::stderr)
        .with_max_level(Level::DEBUG)
        .init();

    let node_span = info_span!("node").entered();

    let mut input = io::stdin().lock();
    let mut output = io::stdout().lock();

    let init_span = info_span!("initialization").entered();

    let Initialization {
        node_id,
        node_ids: _,
    } = handle_initialization(&mut input, &mut output);

    init_span.exit();

    for (message_id, incoming) in (2..).zip(input.lines()) {
        let message_span = info_span!("message", id = message_id).entered();

        tracing::info!("processing incoming message");

        let line = incoming.expect("message should be valid UTF-8");

        let message: Message<Echo> = serde_json::from_str(&line)
            .expect("deserialized message should be of the valid format");

        let response = Message {
            source: &node_id,
            destination: message.source,
            body: Body {
                message_id,
                in_reply_to: Some(message.body.message_id),
                payload: EchoOk {
                    echo: message.body.payload.echo,
                },
            },
        };

        let outgoing = serde_json::to_string(&response).expect("response should be serializable");

        writeln!(output, "{outgoing}").expect("output should be writable");

        tracing::info!("completed sending reply");

        message_span.exit();
    }

    node_span.exit();
}

fn handle_initialization(mut input: impl BufRead, mut output: impl Write) -> Initialization {
    let line = {
        let mut buffer = String::new();
        input
            .read_line(&mut buffer)
            .expect("input should be readable");
        buffer
    };

    let message = serde_json::from_str::<Message<Initialization>>(&line)
        .expect("deserialized message should be of the valid format");

    let payload = message.body.payload;

    tracing::debug!("received initialization message from `{}`", message.source);
    tracing::info!(cluster = ?payload.node_ids);

    let response = Message {
        source: &payload.node_id,
        destination: message.source,
        body: Body {
            message_id: 1,
            in_reply_to: Some(message.body.message_id),
            payload: InitializationOk {},
        },
    };

    let outgoing = serde_json::to_string(&response).expect("response should be serializable");

    writeln!(output, "{outgoing}").expect("output should be writable");

    tracing::info!("completed initialization");

    payload
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename = "echo")]
struct Echo<'a> {
    echo: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename = "echo_ok")]
struct EchoOk<'a> {
    echo: &'a str,
}
