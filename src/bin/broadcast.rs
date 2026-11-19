use std::io;
use tracing::{Level, info};
// use whirlwind::Node;

fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(io::stderr)
        .with_max_level(Level::DEBUG)
        .init();

    // let node = Node::initialize();

    info!("initialization completed")
}
