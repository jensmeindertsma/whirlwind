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

test-unique-ids: build
    ./maelstrom/maelstrom test -w unique-ids --bin ./target/release/unique-ids --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
