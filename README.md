<div align="center">
    <h1>🌪️ whirlwind</h1>
    <i>Implementing distributed systems that can weather any network storm.</i>
</div>
<br/>

## Introduction

This is my attempt at solving the [Fly.io Gossip Glomers distributed systems challenges](https://fly.io/dist-sys/). Each challenge asks you to provide a binary capable of communicating inside a simulation where there may be network faults present. The [Maelstrom platform](https://github.com/jepsen-io/maelstrom) is used to verify that your workload implementation handles these issues correctly inside a distributed system.

## Getting Started

1. Make sure the [prerequisites](https://github.com/jepsen-io/maelstrom/blob/main/doc/01-getting-ready/index.md#prerequisites) are met by your system.
2. Download the latest tarball (`tar.bz2` file) from the [Releases](https://github.com/jepsen-io/maelstrom/releases/latest) page.
3. Extract the tarball to the `./maelstrom` directory in the root of this repository:

```bash
$ tar -xf maelstrom.tar.bz2
```

4. Run any of the implemented workloads using the `justfile`.

## Solutions

| Challenge | Title                                              | Implementation          | Verification             |
| --------- | -------------------------------------------------- | ----------------------- | ------------------------ |
| 1         | [Echo](https://fly.io/dist-sys/1/)                 | `src/bin/echo.rs`       | `$ just test-echo`       |
| 2         | [Unique ID Generation](https://fly.io/dist-sys/2/) | `src/bin/unique-ids.rs` | `$ just test-unique-ids` |

## References

- [Maelstrom Protocol](https://github.com/jepsen-io/maelstrom/blob/main/doc/protocol.md)
- [Workloads](https://github.com/jepsen-io/maelstrom/blob/main/doc/workloads.md)
