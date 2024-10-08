help:
    just --list

build:
    cargo build --release

debug:
    ./maelstrom/maelstrom serve

test-echo: build
    ./maelstrom/maelstrom test -w echo --bin ./target/release/echo --node-count 1 --time-limit 10

test-unique-ids: build
    ./maelstrom/maelstrom test -w unique-ids --bin ./target/release/unique-ids --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition

test-single-node-broadcast: build
    ./maelstrom/maelstrom test -w broadcast --bin ./target/release/broadcast --node-count 1 --time-limit 20 --rate 10

test-multi-node-broadcast: build
    ./maelstrom/maelstrom test -w broadcast --bin ./target/release/broadcast --node-count 5 --time-limit 20 --rate 10

test-fault-tolerant-broadcast: build
    ./maelstrom/maelstrom test -w broadcast --bin ./target/release/broadcast --node-count 5 --time-limit 20 --rate 10 --nemesis partition