help:
    just --list

build:
    cargo build --release

debug:
    ./maelstrom/maelstrom serve

test-echo: build
    ./maelstrom/maelstrom test -w echo --bin ./target/release/echo --node-count 1 --time-limit 10

