# Rust Learning Wiki

A structured deep-dive into Rust, from installation to automated testing.

Based on [The Rust Programming Language](https://doc.rust-lang.org/book/) (Chapters 1-11).

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

### Block D: Structuring Data

| # | Part | Key Concepts | Rust Book |
|---|------|-------------|-----------|
| 7 | [Structs](07-structs.md) | `struct`, field init shorthand, update syntax `..`, tuple structs, unit structs, `#[derive(Debug)]`, `dbg!`, methods, `&self`, associated functions, `impl` | Ch. 5 |

### Block E: Enums & Pattern Matching

| # | Part | Key Concepts | Rust Book |
|---|------|-------------|-----------|
| 8 | [Enums & Pattern Matching](08-enums.md) | `enum`, variants with data, `Option<T>`, `Some`, `None`, `match`, exhaustive patterns, catch-all `_`, `if let`, `let...else` | Ch. 6 |

### Block F: Growing Projects

| # | Part | Key Concepts | Rust Book |
|---|------|-------------|-----------|
| 9 | [Packages, Crates & Modules](09-packages-crates-modules.md) | Packages vs crates, `mod`, module file lookup, `pub`, absolute vs relative paths, `super`, `use`, `as`, `pub use`, nested paths | Ch. 7 |

### Block G: Common Collections

| # | Part | Key Concepts | Rust Book |
|---|------|-------------|-----------|
| 10 | [Common Collections](10-collections.md) | `Vec<T>`, indexing vs `.get()`, `Vec<Vec<T>>`, `String` vs `&str`, UTF-8, `push_str`, `format!`, `HashMap<K, V>`, `entry().or_insert()` | Ch. 8 |

### Block H: Error Handling

| # | Part | Key Concepts | Rust Book |
|---|------|-------------|-----------|
| 11 | [Error Handling](11-error-handling.md) | `panic!`, unwinding vs aborting, `Result<T, E>`, `unwrap`, `expect`, propagating errors, the `?` operator, panic vs Result, custom validation types | Ch. 9 |

### Block I: Generic Types, Traits & Lifetimes

| # | Part | Key Concepts | Rust Book |
|---|------|-------------|-----------|
| 12 | [Generics, Traits & Lifetimes](12-generics-traits-lifetimes.md) | Generic functions/structs/enums/methods, monomorphization, `trait`, default impls, `impl Trait`, trait bounds, `+`, `where` clauses, blanket impls, lifetime annotations, elision rules, `'static` | Ch. 10 |

### Block J: Writing Automated Tests

| # | Part | Key Concepts | Rust Book |
|---|------|-------------|-----------|
| 13 | [Writing Automated Tests](13-tests.md) | `#[test]`, `assert!`, `assert_eq!`, `assert_ne!`, custom messages, `#[should_panic]`, `Result` in tests, `cargo test` flags, `#[ignore]`, `#[cfg(test)]`, unit vs integration tests, `tests/` directory, `tests/common/mod.rs` | Ch. 11 |
