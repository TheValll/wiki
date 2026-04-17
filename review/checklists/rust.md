# Rust — Curriculum Checklist

Ordered list of Rust concepts for review, aligned with the chapters of [The Rust Programming Language](https://doc.rust-lang.org/book/) and the wiki pages in [`rust/`](../../rust/).

Each item is one reviewable concept, not necessarily one full chapter — long chapters are split into focused sub-concepts where it makes sense.

---

## Module 0 — Setup and first contact

| # | Concept | Wiki ref |
|---|---------|----------|
| 1 | Installation, `cargo new`, `cargo build`, `cargo run`, build profiles | [`01-installation-hello-cargo.md`](../../rust/01-installation-hello-cargo.md) |
| 2 | Guessing game: `let`, `mut`, `match`, `Result`, loops | [`02-guessing-game.md`](../../rust/02-guessing-game.md) |

## Module 1 — Core concepts (chapter 3)

| # | Concept | Wiki ref |
|---|---------|----------|
| 3 | Variables, mutability, shadowing vs `mut` | [`03-variables-types.md`](../../rust/03-variables-types.md) §3.1-3.4 |
| 4 | Scalar types (integers, floats, bool, char) and integer overflow | [`03-variables-types.md`](../../rust/03-variables-types.md) §3.5 |
| 5 | Compound types: tuples, arrays, unit type `()` | [`03-variables-types.md`](../../rust/03-variables-types.md) §3.6 |
| 6 | Functions, parameters, statements vs expressions | [`04-functions-control-flow.md`](../../rust/04-functions-control-flow.md) §4.1-4.4 |
| 7 | Control flow: `if` as expression, `loop` with `break value`, `while`, `for`, labels | [`04-functions-control-flow.md`](../../rust/04-functions-control-flow.md) §4.6-4.7 |

## Module 2 — Ownership (chapter 4)

| # | Concept | Wiki ref |
|---|---------|----------|
| 8 | Stack vs heap, the 3 ownership rules, scope and `drop` | [`05-ownership.md`](../../rust/05-ownership.md) §5.2-5.4 |
| 9 | Move semantics, `Copy` trait, `Clone`, ownership in functions | [`05-ownership.md`](../../rust/05-ownership.md) §5.6-5.10 |
| 10 | References `&T` / `&mut T`, borrowing rules, NLL, dangling prevention | [`06-references-slices.md`](../../rust/06-references-slices.md) §6.2-6.7 |
| 11 | Slices `&str` / `&[T]`, deref coercion, `&str` as parameter | [`06-references-slices.md`](../../rust/06-references-slices.md) §6.8-6.12 |

## Module 3 — Structuring data (chapters 5-6)

| # | Concept | Wiki ref |
|---|---------|----------|
| 12 | Structs: definition, instantiation, update syntax `..`, tuple and unit structs | [`07-structs.md`](../../rust/07-structs.md) §7.1-7.5 |
| 13 | Methods, `&self` / `&mut self` / `self`, associated functions, multiple `impl` blocks | [`07-structs.md`](../../rust/07-structs.md) §7.9-7.12 |
| 14 | `#[derive(Debug)]`, `{:?}` vs `{:#?}`, `dbg!` macro | [`07-structs.md`](../../rust/07-structs.md) §7.8 |
| 15 | Enums with data, `Option<T>`, methods on enums | [`08-enums.md`](../../rust/08-enums.md) §8.1-8.4 |
| 16 | `match`: exhaustiveness, binding patterns, catch-all `_` | [`08-enums.md`](../../rust/08-enums.md) §8.5-8.9 |
| 17 | `if let`, `if let ... else`, `let ... else` | [`08-enums.md`](../../rust/08-enums.md) §8.10-8.11 |

## Module 4 — Growing projects (chapter 7)

| # | Concept | Wiki ref |
|---|---------|----------|
| 18 | Packages vs crates vs modules, `mod` file lookup, `src/bin/` | [`09-packages-crates-modules.md`](../../rust/09-packages-crates-modules.md) §9.1-9.2 |
| 19 | `pub`, absolute vs relative paths, `super`, visibility of struct fields vs enum variants | [`09-packages-crates-modules.md`](../../rust/09-packages-crates-modules.md) §9.2-9.3 |
| 20 | `use`, `as`, `pub use`, nested paths, glob | [`09-packages-crates-modules.md`](../../rust/09-packages-crates-modules.md) §9.4 |

## Module 5 — Common collections (chapter 8)

| # | Concept | Wiki ref |
|---|---------|----------|
| 21 | `Vec<T>`: indexing vs `.get()`, iteration, enum-based heterogeneity | [`10-collections.md`](../../rust/10-collections.md) §10.1 |
| 22 | `String` vs `&str`, UTF-8, slicing by bytes, iterating `.chars()` / `.bytes()` | [`10-collections.md`](../../rust/10-collections.md) §10.2 |
| 23 | `HashMap<K, V>`: insert, get, ownership, `entry().or_insert()`, word-count pattern | [`10-collections.md`](../../rust/10-collections.md) §10.3 |

## Module 6 — Error handling (chapter 9)

| # | Concept | Wiki ref |
|---|---------|----------|
| 24 | `panic!`, unwinding vs aborting, when to panic | [`11-error-handling.md`](../../rust/11-error-handling.md) §11.1, §11.6 |
| 25 | `Result<T, E>`, `unwrap` vs `expect`, matching on `ErrorKind` | [`11-error-handling.md`](../../rust/11-error-handling.md) §11.2-11.3 |
| 26 | The `?` operator: propagation, `From` conversion, `Option` variant | [`11-error-handling.md`](../../rust/11-error-handling.md) §11.4-11.5 |
| 27 | Validation types (e.g. `Guess`), encoding invariants in types | [`11-error-handling.md`](../../rust/11-error-handling.md) §11.7 |

## Module 7 — Generic types, traits, lifetimes (chapter 10)

| # | Concept | Wiki ref |
|---|---------|----------|
| 28 | Generics in functions, structs, enums, methods; monomorphization | [`12-generics-traits-lifetimes.md`](../../rust/12-generics-traits-lifetimes.md) §12.1 |
| 29 | Traits: defining, implementing, orphan rule, default implementations | [`12-generics-traits-lifetimes.md`](../../rust/12-generics-traits-lifetimes.md) §12.2 |
| 30 | Trait bounds: `impl Trait` vs `<T: Trait>`, `+`, `where` clauses, returning `impl Trait` | [`12-generics-traits-lifetimes.md`](../../rust/12-generics-traits-lifetimes.md) §12.2 |
| 31 | Conditional method impls, blanket implementations | [`12-generics-traits-lifetimes.md`](../../rust/12-generics-traits-lifetimes.md) §12.2 |
| 32 | Lifetimes: annotations, function signatures, struct fields, `'static` | [`12-generics-traits-lifetimes.md`](../../rust/12-generics-traits-lifetimes.md) §12.3 |
| 33 | Lifetime elision rules (3 rules), methods with lifetimes | [`12-generics-traits-lifetimes.md`](../../rust/12-generics-traits-lifetimes.md) §12.3 |

## Module 8 — Automated tests (chapter 11)

| # | Concept | Wiki ref |
|---|---------|----------|
| 34 | `#[test]`, assertion macros (`assert!`, `assert_eq!`, `assert_ne!`), custom messages | [`13-tests.md`](../../rust/13-tests.md) §13.1-13.2 |
| 35 | `#[should_panic]` with `expected`, `Result<T, E>` in tests | [`13-tests.md`](../../rust/13-tests.md) §13.3-13.4 |
| 36 | `cargo test` flags: `--test-threads`, `--show-output`, filtering, `#[ignore]` | [`13-tests.md`](../../rust/13-tests.md) §13.5 |
| 37 | Unit vs integration tests, `#[cfg(test)]`, `tests/` directory, `tests/common/mod.rs` | [`13-tests.md`](../../rust/13-tests.md) §13.6 |

---

## Notes

- Exercises must be **original** — the wiki is for theory, never for copy-pasted exercises.
- For Rust, exercises work especially well as **code reading** ("will this compile?"), **bug spotting**, or **scenario-based** ("refactor this to use `?`").
- Each module maps loosely to one Rust book chapter. Concepts within a module are sometimes intertwined in the book — quiz them together when relevant.
