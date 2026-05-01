# 19 — Async / Await

Threads (Ch 18) give you parallelism: many things really happening at once on different cores. **Async** gives you concurrency: many things *making progress* without dedicating a whole OS thread to each. The use case is **I/O-bound** code — waiting on the network, the disk, a sensor — where 99 % of "work" is sitting idle.

Each page covers **one concept**.

| § | Page |
|---|------|
| 19.1 | [Futures, `async`, `.await`](./19.1-futures-async-await.md) |
| 19.2 | [Executors and runtimes (Tokio, smol, async-std)](./19.2-executors-runtimes.md) |
| 19.3 | [Async concurrency: `join!`, `select!`, `spawn`](./19.3-concurrency-primitives.md) |
| 19.4 | [`Stream`: async iteration](./19.4-streams.md) |
| 19.5 | [Async vs threads: when to pick which](./19.5-async-vs-threads.md) |
| 19.6 | [`Pin`, `Send`, and async traits](./19.6-pin-traits.md) |

> Based on [The Rust Programming Language — Chapter 17](https://doc.rust-lang.org/book/ch17-00-async-await.html).
> Async in Rust is **mature for I/O code (Tokio is rock-solid)** but the language layer keeps evolving — async traits stabilized in Rust 1.75 (Dec 2023), AFIT (async-fn-in-traits) is fully usable but with caveats. Pages flag what is still moving.
