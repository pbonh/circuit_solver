---
title: TCP Networking (Rust)
type: claim
id: claim-tcp-networking
tags:
- rust
- networking
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/21-chapter-20-final-project-building-a-multithreaded-web-server.txt
confidence:
  base: 0.65
---

## Definition

Rust's standard library exposes blocking TCP networking via `std::net`. `TcpListener` accepts incoming connections; `TcpStream` represents a connected pair of endpoints implementing `Read` and `Write`. For async, non-blocking I/O at scale, applications reach for ecosystem libraries like `tokio` or `async-std`.

## How It Works

`TcpListener::bind("127.0.0.1:7878")` returns a listener; `listener.incoming()` yields `Result<TcpStream>` for each accepted connection. Each `TcpStream` can be read with `BufReader::new(&stream).lines()` or written with `writeln!(stream, ...)`. Listener and stream APIs are blocking by default; switch to non-blocking with `.set_nonblocking(true)` or use an async runtime for cooperative scheduling.

## Key Parameters

- `TcpListener::bind`, `TcpListener::accept`, `TcpListener::incoming`
- `TcpStream::connect` (client side)
- Blocking vs non-blocking mode
- `Read` / `Write` trait impls
- Buffered wrappers (`BufReader`, `BufWriter`)

## When To Use

- Educational and small-server scenarios using only `std`
- One-off network utilities, debug tools, simple proxies
- Foundations for higher-level frameworks
- Production code typically delegates to `tokio` / `hyper` / `actix-web`

## Risks & Pitfalls

- Blocking I/O ties up a thread per connection; doesn't scale
- No HTTP parsing — the stdlib offers raw byte streams only
- TLS requires external crates (`rustls`, `native-tls`)
- Easy to forget to flush a `BufWriter` before close

## Related Concepts

- [[concepts/threads]]
- [[concepts/thread-pool]]
- [[concepts/channels]]
- [[concepts/rust-language]]

## Sources

- [[summaries/rust-book-21-chapter-20-final-project-building-a-multithreaded-web-server]]
