help:
    just --list

build:
    cargo build --release

check:
    cargo clippy

clean:
    rm -rf store target

debug:
    ./maelstrom/maelstrom serve

format:
    cargo fmt --all

test-echo: build
    ./maelstrom/maelstrom test -w echo --bin ./target/release/echo --node-count 1 --time-limit 10
