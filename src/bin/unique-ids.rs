use whirlwind::Message;

fn main() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_target(false)
        .with_writer(io::stderr)
        .with_max_level(Level::DEBUG)
        .init();

    let mut node = Node::initialize();

    for message in node.messages() {
        node.send(Message {})
    }
}
