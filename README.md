# kv-store

An in-memory TCP key-value store in Rust, built to learn networking and concurrency from first principles.

## What This Is

A small, Redis-inspired key-value store accessible over a raw TCP connection using a simple text protocol. Built without any web/async framework — just the Rust standard library — to actually understand what's happening at the networking and concurrency layer, rather than relying on a library to hide it.

## Status

🚧 Work in progress — Phase 1 (single client, single thread).

## Project Phases

- **Phase 1** — single client, single thread. Accept one connection, handle a full sequence of commands from it, move to the next connection once it closes.
- **Phase 2** — multiple clients, concurrent. Thread-per-connection, shared state via.
- **Phase 3** — additional commands (`EXISTS`, `KEYS`, `FLUSHALL`), a dedicated client binary, basic append-only persistence.

## Protocol

Line-based, space-separated, all text.

| Command | Behavior | Response |
|---|---|---|
| `SET key value` | Store `value` under `key` | `OK` |
| `GET key` | Retrieve the value for `key` | the value, or `NIL` if absent |
| `DEL key` | Remove `key` if present | `OK` or `NOT FOUND` |

Malformed input returns a clear error response rather than crashing the connection or server.

## Running

```bash
cargo run
```

Then, in another terminal, connect with `telnet` or `nc`:

```bash
nc localhost 6379
```

## Testing

```bash
cargo test
```

Unit tests cover the store and command parser in isolation; integration tests exercise the server over an actual TCP connection.

## Why This Project

Built as a scoped, finishable step between learning Rust fundamentals and eventually building bigger systems projects (a database engine, a matching engine). Small enough to complete in one to two weeks, while still exercising:

- TCP networking
- Protocol design and parsing
- Multi-file project structure
- Error handling without panicking
- Thread-per-connection concurrency
- Shared mutable state across threads

## Author

Great Ezenna — [greatm3.tech](https://greatm3.tech)