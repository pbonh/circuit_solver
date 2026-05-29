---
title: 'The Rust Programming Language — Chapter 20: Final Project: Building a Multithreaded
  Web Server'
type: source
id: summaries/rust-book-21-chapter-20-final-project-building-a-multithreaded-web-server
kind: publication
tags:
- rust
- project
- concurrency
- networking
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/21-chapter-20-final-project-building-a-multithreaded-web-server.txt
---

## Key Points

- The capstone project builds a minimal multithreaded HTTP server from scratch using only the standard library, exercising TCP I/O, threading, channels, mutexes, atomic reference counting, smart pointers, and `Drop`-based cleanup — a tour of everything covered so far.
- `std::net::TcpListener::bind("127.0.0.1:7878")` opens a listening socket. `listener.incoming()` yields `Result<TcpStream>` values for each accepted connection.
- Reading an HTTP request uses `std::io::BufReader::new(&stream)` and `.lines()`; the request line is `GET / HTTP/1.1`. Writing a response writes status line + headers + blank line + body to the stream.
- The initial single-threaded server cannot handle slow requests without blocking; simulating with `thread::sleep` demonstrates the need for concurrency.
- A `ThreadPool` is implemented with a fixed number of worker threads. Each `Worker` runs a loop receiving jobs from a shared `mpsc::Receiver` wrapped in an `Arc<Mutex<...>>`.
- Jobs are sent across the channel as `Box<dyn FnOnce() + Send + 'static>` — heap-allocated trait objects carrying the closure's captured state.
- `ThreadPool::new(size)` validates `size > 0` (asserting / panicking in the chapter; a `build` method returning `Result` is recommended in production).
- A graceful shutdown is implemented via `Drop for ThreadPool`: drop the sender (signaling channel close), then `join` each worker thread. Workers detect the closed channel via `recv()` returning `Err` and exit their loop.
- The `JoinHandle` is held in an `Option<JoinHandle<()>>` so that `take()` can move ownership into `join()` during drop without invalidating the field.
- The chapter explicitly acknowledges its server is a teaching tool: real production servers use libraries like `hyper`, `tokio`, `actix-web` that handle HTTP correctly, manage many more concurrency patterns, and provide async I/O.

## Relevant Concepts

- [[concepts/tcp-networking]] — `TcpListener`, `TcpStream`.
- [[concepts/thread-pool]] — fixed worker pool for incoming jobs.
- [[concepts/threads]] — `std::thread::spawn`, `JoinHandle`.
- [[concepts/channels]] — sending jobs to workers.
- [[concepts/mutex]] — sharing the receiver among workers.
- [[concepts/arc-type]] — multi-thread shared ownership.
- [[concepts/drop-trait]] — graceful shutdown via `Drop`.
- [[concepts/closures]] — captured-state work items.
- [[concepts/fn-traits]] — `FnOnce() + Send + 'static` job signature.

## Source Metadata

- Source type: book chapter (project chapter)
- Book title: The Rust Programming Language
- Chapter: 20 — Final Project: Building a Multithreaded Web Server
- File path: `raw/rust_book/_txt/21-chapter-20-final-project-building-a-multithreaded-web-server.txt`
- Authors: Steve Klabnik and Carol Nichols
