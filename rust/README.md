# Rust Learning Wiki

A structured deep-dive into Rust, from installation to ownership mastery.

Based on [The Rust Programming Language](https://doc.rust-lang.org/book/) (Chapters 1-4).

---

## Syllabus

### Block A: Getting Started

| # | Part | Key Concepts | Rust Book |
|---|------|-------------|-----------|
| 1 | [Installation, Hello World & Cargo](01-installation-hello-cargo.md) | rustup, rustc, ahead-of-time compilation, Cargo, TOML, build profiles | Ch. 1 |
| 2 | [The Guessing Game](02-guessing-game.md) | `let`, `mut`, `match`, `Result`, crates, `loop`, `break`, `continue` | Ch. 2 |

### Block B: Core Concepts

| # | Part | Key Concepts | Rust Book |
|---|------|-------------|-----------|
| 3 | [Variables, Mutability & Types](03-variables-types.md) | Immutability, `mut`, `const`, shadowing, scalars, tuples, arrays | Ch. 3.1-3.2 |
| 4 | [Functions & Control Flow](04-functions-control-flow.md) | `fn`, parameters, statements vs expressions, `if`, `loop`, `while`, `for` | Ch. 3.3-3.5 |

### Block C: Ownership — The Heart of Rust

| # | Part | Key Concepts | Rust Book |
|---|------|-------------|-----------|
| 5 | [Ownership](05-ownership.md) | Stack vs heap, 3 rules, `String`, move, clone, `Copy` trait, ownership & functions | Ch. 4.1 |
| 6 | [References, Borrowing & Slices](06-references-slices.md) | `&`, `&mut`, borrowing rules, dangling refs, `&str`, `&[T]` | Ch. 4.2-4.3 |
