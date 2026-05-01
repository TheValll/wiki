# 18 — Fearless Concurrency

Rust's headline promise: **data races are caught at compile time**, not at 3 AM in production. The same ownership and borrowing rules that govern single-threaded code extend, almost unchanged, to threads — with two extra marker traits (`Send`, `Sync`) doing the heavy lifting under the hood.

Each page covers **one concept**.

| § | Page |
|---|------|
| 18.1 | [Threads with `thread::spawn`](./18.1-threads-spawn.md) |
| 18.2 | [Message passing with channels (`mpsc`)](./18.2-message-passing.md) |
| 18.3 | [Shared state with `Mutex<T>`](./18.3-mutex.md) |
| 18.4 | [`Arc<T>`: shared ownership across threads](./18.4-arc.md) |
| 18.5 | [`Send` and `Sync`: the marker traits behind it all](./18.5-send-sync.md) |

> Based on [The Rust Programming Language — Chapter 16](https://doc.rust-lang.org/book/ch16-00-concurrency.html).
