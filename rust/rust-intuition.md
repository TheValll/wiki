# Rust — Intuition

Bookmark layer for [The Rust Programming Language](https://doc.rust-lang.org/book/) (2024 edition). Every concept in 2-4 lines: **what** it does, **how** it works. No code, no exercises — for the full explanation, read the corresponding chapter (mirrored locally in [`../raw/rust-book/`](../raw/rust-book/)).

---

## Ch.1 — Getting Started

- **`rustup`** — toolchain installer + manager. *What:* single CLI to install and switch Rust versions (stable/nightly). *How:* drops `rustc`, `cargo`, etc. into `~/.cargo/bin`; `rustup update` keeps them current.
- **Cargo** — build tool + package manager + runner. *What:* compiles, fetches deps, runs tests, generates docs. *How:* reads `Cargo.toml` (manifest) + `Cargo.lock` (resolved versions).
- **`cargo new`** — scaffold a project. *What:* creates `src/main.rs` (binary) or `src/lib.rs` (library) with a `Cargo.toml`. *How:* one command, ready to `cargo run`.
- **Cargo.lock** — lockfile pinning exact dep versions. *What:* reproducibility — same build everywhere. *How:* auto-updated by Cargo on dep resolution; commit it for binaries, optionally for libs.
- **`println!`** — formatted output macro. *What:* prints to stdout with placeholders. *How:* macro (not function) because it parses the format string at compile time.

## Ch.2 — Programming a Guessing Game

- **`io::stdin().read_line(&mut s)`** — read a line of input. *What:* reads from stdin, appends to a `String`. *How:* mutable borrow of the buffer; the function fills it.
- **External crate (`rand`)** — pull deps from crates.io. *What:* add to `Cargo.toml` (or `cargo add rand`), import in code. *How:* `rand::thread_rng().gen_range(1..=100)`.
- **`match`** — pattern-matching switch. *What:* branch on a value's variant or pattern. *How:* compiler enforces exhaustiveness — every case must be handled.
- **`loop { … break; }`** — explicit infinite loop. *What:* no condition; exit via `break`, can return a value (`break val;`). *How:* the only loop form that's also an expression.

## Ch.3 — Common Programming Concepts

- **`let` + immutability default** — bindings read-only by default. *What:* `let x = 5;` cannot be reassigned. *How:* `let mut x = 5;` opts in to mutation.
- **`const`** — compile-time constant. *What:* inlined everywhere it's used; type required. *How:* `const MAX: u32 = 100_000;` — `SCREAMING_CASE` by convention.
- **Shadowing** — re-`let` the same name. *What:* new binding replaces the old, possibly with a different type. *How:* `let x = "5"; let x: u32 = x.parse().unwrap();`.
- **Scalar types** — `i32`, `u64`, `f64`, `bool`, `char`. *What:* fixed-size primitives. *How:* `char` is a 4-byte Unicode scalar (not C's 1-byte char).
- **Compound types** — tuples + arrays. *What:* tuples = heterogeneous fixed-size; arrays = homogeneous fixed-size. *How:* `(1, "a", 3.0)` and `[1, 2, 3]` — both stack-allocated.
- **Functions** — `fn name(arg: T) -> R { … }`. *What:* snake_case names, signature mandatory. *How:* no overloading; last expression (no `;`) is the return value.
- **Statements vs expressions** — statements end in `;` and produce nothing; expressions evaluate. *What:* `let x = if c { 1 } else { 2 };` works because `if` is an expression. *How:* trailing `;` turns an expression into a statement.
- **`if`/`else`** — branch expression. *What:* evaluates to a value; types of branches must match. *How:* `let n = if x > 0 { x } else { -x };`.
- **Loops** — `loop`, `while`, `for x in iter`. *What:* three flavors; `for` over iterators is idiomatic. *How:* loop labels (`'outer: loop`) for nested `break`.

## Ch.4 — Understanding Ownership

- **Ownership** — single-owner rule. *What:* every value has exactly one owner; when the owner leaves scope, the value is dropped. *How:* compile-time enforced, no GC, no double-free.
- **Stack vs heap** — fixed vs dynamic memory. *What:* stack = fast LIFO, fixed-size; heap = dynamic, allocator-tracked, owned via pointers. *How:* `Box`, `String`, `Vec` allocate on the heap; primitives stay on the stack.
- **Move** — `=` and function calls transfer ownership for non-`Copy` types. *What:* `let s2 = s1;` moves the `String` — `s1` no longer usable. *How:* compiler refuses subsequent reads of `s1`.
- **`Copy` trait** — bitwise duplication for stack-only types. *What:* `i32`, `bool`, `char`, `f64`, fixed-size tuples of `Copy` types. *How:* marker trait; `let y = x;` copies, both remain valid.
- **`Clone`** — explicit deep copy. *What:* `s2 = s1.clone();` duplicates the heap allocation. *How:* always explicit — costs are visible.
- **References** — `&T`, `&mut T`. *What:* borrow without transferring ownership. *How:* lifetime tracked by the compiler; no dangling refs allowed.
- **Borrowing rules** — at any moment, EITHER many `&T` OR one `&mut T`. *What:* aliasing XOR mutation. *How:* compiler refuses code that violates this; data races eliminated at compile time.
- **Slices** — `&[T]`, `&str`. *What:* pointer + length, view into someone else's buffer. *How:* no ownership; valid as long as the source buffer is.

## Ch.5 — Using Structs

- **`struct`** — named record. *What:* `struct Point { x: i32, y: i32 }`. *How:* `Point { x: 1, y: 2 }`; field shorthand if local var has same name.
- **Tuple struct** — `struct Pair(i32, i32);`. *What:* newtype pattern; gives the type identity without naming fields. *How:* `p.0`, `p.1` to access.
- **Unit-like struct** — `struct Marker;`. *What:* carries only a type identity, no data. *How:* used as marker types or for trait impls without state.
- **Update syntax `..`** — copy fields from another struct. *What:* `let p2 = Point { x: 10, ..p1 };` — `p2.y` comes from `p1`. *How:* like JS spread, but only at the end.
- **`impl` block** — methods on a type. *What:* `impl Point { fn area(&self) -> f64 {…} }`. *How:* `&self`, `&mut self`, or `self` decide the borrow style.
- **Associated functions** — no `self`. *What:* `Point::new(...)` — constructor / utility. *How:* called via `Type::fn`, not `instance.fn`.
- **Derived traits** — `#[derive(Debug, Clone, PartialEq)]`. *What:* compiler auto-generates the impl. *How:* available for traits the compiler knows how to derive (Debug, Clone, Copy, Eq, Hash, …).

## Ch.6 — Enums and Pattern Matching

- **`enum`** — sum type. *What:* a value that is *one of* several variants. *How:* `enum Shape { Circle(f64), Square { side: f64 } }` — variants can carry different data.
- **`Option<T>`** — `Some(T)` or `None`. *What:* replaces null; absence is a typed value. *How:* compiler forces you to handle both arms.
- **`Result<T, E>`** — `Ok(T)` or `Err(E)`. *What:* replaces exceptions; failure is a return value. *How:* same exhaustiveness rule as `Option`.
- **`match`** — exhaustive pattern matching. *What:* one arm per variant; compiler refuses partial coverage. *How:* patterns destructure (`Some(x) => …`, `Point { x, .. } => …`).
- **`if let`** — single-pattern match. *What:* `if let Some(v) = opt { use(v); }`. *How:* sugar for a `match` with one interesting arm and an `_` else.
- **`let...else`** — bind or diverge. *What:* `let Some(v) = opt else { return; };` — early-exit. *How:* the `else` branch must diverge (return, panic, break).

## Ch.7 — Packages, Crates, Modules

- **Package** — a `Cargo.toml` + one or more crates. *What:* the unit of distribution. *How:* at most one library crate + many binary crates per package.
- **Crate** — a compilation unit. *What:* `lib.rs` (library) or `main.rs` (binary), forming a tree of modules. *How:* `cargo build` produces one artifact per crate.
- **Module (`mod`)** — scope + privacy boundary. *What:* `mod foo { … }` defines a sub-namespace. *How:* `mod foo;` looks for `foo.rs` or `foo/mod.rs`.
- **Privacy** — private by default. *What:* items invisible outside their module unless marked `pub`. *How:* struct fields stay private even in `pub struct`; enum variants are auto-public in a `pub enum`.
- **Paths** — `crate::`, `super::`, `self::`. *What:* address items in the module tree. *How:* `crate::` = absolute root, `super::` = parent, `self::` = current.
- **`use`** — bring path into scope. *What:* `use std::collections::HashMap;`. *How:* `as` for renaming, `pub use` for re-exporting.

## Ch.8 — Common Collections

- **`Vec<T>`** — growable heap-allocated array. *What:* resizable, ordered. *How:* `vec![1, 2, 3]` macro; `push`, `pop`, indexing with `[i]` (panics on OOB) or `.get(i)` (returns `Option`).
- **`String`** — heap-allocated UTF-8 text. *What:* owned, mutable. *How:* internally a `Vec<u8>` validated as UTF-8; indexing by byte position is forbidden — use `.chars()` or `.bytes()`.
- **`HashMap<K, V>`** — hash-based dictionary. *What:* key-to-value lookup, O(1) avg. *How:* `map.insert(k, v)`, `map.get(&k)`; default hasher is SipHash (DoS-resistant, slower than `FxHash`).

## Ch.9 — Error Handling

- **`panic!`** — unrecoverable error. *What:* aborts (or unwinds) the thread; for bugs and impossible states. *How:* stack trace via `RUST_BACKTRACE=1`; `panic = 'abort'` in `Cargo.toml` skips unwinding.
- **`Result<T, E>`** — recoverable error. *What:* failure as a value, not an exception. *How:* caller must explicitly handle `Err` to access `T`.
- **`unwrap` / `expect`** — extract `Ok`, panic on `Err`. *What:* quick conversion when you "know" the value is there. *How:* `expect("...")` carries a panic message; prefer it over bare `unwrap`.
- **`?` operator** — propagate errors. *What:* `let x = f()?;` returns early on `Err`, unwraps on `Ok`. *How:* works on `Result` and `Option` in functions returning the same kind.
- **When to panic vs Result** — bugs panic, expected failures return `Result`. *What:* distinguish "this should never happen" from "this might happen". *How:* user input → `Result`; broken invariant → `panic!`.

## Ch.10 — Generics, Traits, Lifetimes

- **Generics `<T>`** — code that works for many types. *What:* `fn largest<T: Ord>(slice: &[T]) -> &T`. *How:* monomorphization — compiler generates one specialized copy per concrete `T`. Zero runtime cost.
- **Trait** — shared behavior contract. *What:* `trait Drawable { fn draw(&self); }`. *How:* like an interface, defined separately from the type — implementations can be added later (orphan rule permitting).
- **Trait bound** — constraint on a generic. *What:* `<T: Display + Clone>` — `T` must implement both. *How:* without it, generic methods can only call what every type has.
- **Default trait method** — body in the trait definition. *What:* inherited by implementors unless overridden. *How:* code reuse without inheritance.
- **`impl Trait`** — opaque return type. *What:* `fn make_adder(...) -> impl Fn(i32) -> i32`. *How:* caller knows it implements the trait, doesn't see the concrete type.
- **Lifetimes `'a`** — proofs about reference validity. *What:* `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`. *How:* annotation states the relationship; compiler verifies it.
- **Lifetime elision** — common patterns inferred. *What:* `fn f(s: &str) -> &str` is auto-elided to `<'a>(s: &'a str) -> &'a str`. *How:* three rules in the reference; works for the common cases, fails for ambiguous ones.

## Ch.11 — Writing Automated Tests

- **`#[test]`** — mark a function as a test. *What:* function with no args returning `()` (or `Result`). *How:* `cargo test` discovers and runs all `#[test]` functions.
- **Assertions** — `assert!`, `assert_eq!`, `assert_ne!`. *What:* panic on failure with a useful message. *How:* `assert!(cond, "msg with {}", x)`; `_eq!` shows both values on failure.
- **`#[should_panic]`** — expect a panic. *What:* test passes only if the body panics. *How:* `#[should_panic(expected = "substring")]` to match the message.
- **`Result` in tests** — return `Result<(), Box<dyn Error>>`. *What:* lets you use `?` inside tests. *How:* test passes on `Ok`, fails on `Err`.
- **Unit tests** — same file as code. *What:* `#[cfg(test)] mod tests { … }` — has access to private items. *How:* conditionally compiled, doesn't ship in release builds.
- **Integration tests** — `tests/` directory. *What:* black-box, only the public API. *How:* each `tests/*.rs` is a separate crate.
- **`cargo test --` flags** — control output and run. *What:* `--nocapture` keeps stdout, `-- name_filter` runs subset. *How:* args after `--` go to the test binary, not Cargo.

## Ch.12 — I/O Project: minigrep

- **`std::env::args()`** — CLI arguments. *What:* iterator over the program's arguments. *How:* first element is the program name; collect with `.collect::<Vec<_>>()`.
- **`std::fs::read_to_string`** — slurp a file. *What:* read whole file into a `String`. *How:* returns `Result<String, io::Error>`.
- **Refactor `main`** — split `main` from library logic. *What:* move logic into `lib.rs`, keep `main.rs` thin. *How:* library is unit-testable, binary is just glue.
- **TDD on `search`** — red/green/refactor. *What:* write the failing test first. *How:* the test pins the function's contract before the body exists.
- **Env vars (`std::env::var`)** — runtime config. *What:* read environment variables for behavior toggles. *How:* `env::var("CASE_INSENSITIVE").is_ok()`.
- **`eprintln!`** — print to stderr. *What:* diagnostics that don't pollute stdout. *How:* same syntax as `println!`, different stream.

## Ch.13 — Functional Features (Closures + Iterators)

- **Closure `|args| body`** — anonymous function with captures. *What:* captures variables from the surrounding scope. *How:* compiler infers types from usage.
- **`Fn` / `FnMut` / `FnOnce`** — three closure traits. *What:* read-only / mutating / consuming captures. *How:* compiler picks based on the body; accept the *weakest* trait that fits as parameter.
- **`move` keyword** — force capture by ownership. *What:* `move |x| …` takes ownership of captures. *How:* required for closures sent to threads.
- **`Iterator` trait** — single method `next() -> Option<Item>`. *What:* lazy pull-based sequence. *How:* each call returns the next item or `None` at end.
- **Adapter methods** — `.map`, `.filter`, `.take`, `.skip`. *What:* build a chain of transformations; lazy. *How:* each returns a new iterator; nothing runs until a consumer pulls.
- **Consumer methods** — `.collect`, `.sum`, `.fold`, `.count`, `for`. *What:* drain the iterator and produce a final value. *How:* this is when the chain actually runs, one element at a time.
- **Performance** — iterators compile to the same code as loops. *What:* zero-cost abstraction. *How:* compiler sees the whole pipeline and optimizes aggressively (often *better* than hand-written loops).

## Ch.14 — More about Cargo

- **Release profiles** — `[profile.dev]` vs `[profile.release]`. *What:* two preset compile configs. *How:* `dev` = fast compile, debug info; `release` = optimisation, no debug info, slow compile.
- **Doc comments** — `///` on items, `//!` inside modules. *What:* `cargo doc` renders them as HTML on docs.rs. *How:* code blocks inside docs are run as tests by `cargo test`.
- **Publishing to crates.io** — `cargo publish`. *What:* upload a versioned crate to the public registry. *How:* needs `name`, `version`, `license`, `description` in `Cargo.toml`; versions are immutable (only `cargo yank`-able).
- **Workspaces** — multi-crate repo. *What:* one `Cargo.lock`, shared `target/` across members. *How:* top-level `Cargo.toml` lists `[workspace] members = [...]`.
- **`cargo install`** — install a binary crate. *What:* drops a binary into `~/.cargo/bin`. *How:* user-local, no admin rights, fetched from crates.io.
- **Custom commands** — any `cargo-foo` in `$PATH`. *What:* `cargo foo` invokes it. *How:* how `cargo-watch`, `cargo-edit`, `cargo-expand` plug in.

## Ch.15 — Smart Pointers

- **Smart pointer** — struct that acts like a pointer with extra machinery. *What:* implements `Deref` (so `*` works) and `Drop` (cleanup). *How:* `Box`, `Rc`, `RefCell`, etc. are all built on these two traits.
- **`Box<T>`** — heap allocation, single owner. *What:* move a value to the heap behind a pointer. *How:* `let b = Box::new(5);` — `b` is owned, dropped at scope end. Use cases: huge values, recursive types, trait objects.
- **`Deref` trait** — pointer-like dereferencing. *What:* lets `*x` and method auto-deref work. *How:* chains automatically (`&Box<String>` → `&String` → `&str`).
- **Deref coercion** — implicit conversions at call sites. *What:* `&String` passes where `&str` is expected. *How:* compiler stacks deref calls behind your back, zero runtime cost.
- **`Drop` trait** — custom cleanup. *What:* run code when a value goes out of scope. *How:* cannot call `.drop()` directly; use `std::mem::drop(x)` to fire early.
- **`Rc<T>`** — single-thread reference-counted shared ownership. *What:* many owners, freed when the last one drops. *How:* `Rc::clone(&a)` is just a counter bump (cheap); read-only.
- **`RefCell<T>`** — interior mutability with runtime borrow check. *What:* mutate through `&self` at the cost of runtime panics on rule violations. *How:* `.borrow()` / `.borrow_mut()` track active borrows in counters.
- **`Rc<RefCell<T>>`** — shared mutable state. *What:* many owners + interior mutability. *How:* for threads, swap to `Arc<Mutex<T>>`.
- **`Weak<T>`** — non-owning reference for breaking cycles. *What:* doesn't keep the value alive. *How:* `.upgrade()` returns `Option<Rc<T>>` — `None` if the value is gone.

## Ch.16 — Fearless Concurrency

- **`thread::spawn(|| {…})`** — spawn an OS thread. *What:* real OS thread; closure runs in parallel. *How:* `move` is required so the thread owns its captures; returns a `JoinHandle`.
- **`JoinHandle::join`** — wait for a thread. *What:* blocks until the thread finishes; returns its result. *How:* returns `Result` because the thread might have panicked.
- **Channels (`mpsc`)** — multi-producer, single-consumer. *What:* `tx.send(v)` transfers ownership of `v`; `rx.recv()` receives it. *How:* `tx.clone()` for many producers; ownership transfer prevents data races.
- **`Mutex<T>`** — mutual exclusion. *What:* `m.lock()` blocks until exclusive access; returns a `MutexGuard`. *How:* lock released when guard drops; no manual unlock.
- **`Arc<T>`** — atomic reference counting. *What:* like `Rc`, but counter is atomic — safe across threads. *How:* `Arc<Mutex<T>>` is the canonical shared-mutable pattern; ~10× slower than `Rc` per clone but still nanoseconds.
- **`Send` / `Sync`** — marker traits for thread safety. *What:* `Send` = can be transferred to another thread; `Sync` = `&T` can be shared. *How:* auto-derived; `Rc` and `RefCell` opt out.

## Ch.17 — Async / Await

- **`async fn`** — function returning a `Future`. *What:* calling it produces a paused state machine; nothing runs until awaited. *How:* `async fn f() -> T` desugars to `fn f() -> impl Future<Output = T>`.
- **`.await`** — yield until the future is ready. *What:* polls the future; if `Pending`, hands control back to the executor. *How:* only valid inside `async` contexts.
- **`Future` trait** — `poll(self: Pin<&mut Self>, cx) -> Poll<Output>`. *What:* the contract every async value implements. *How:* `Poll::Ready(v)` or `Poll::Pending`; the executor calls `poll` repeatedly.
- **Runtime / executor** — what actually runs futures. *What:* polls futures, schedules them, talks to the OS for I/O. *How:* not in `std`; bring your own (Tokio, smol, async-std). Tokio is the de-facto choice.
- **`#[tokio::main]`** — set up the runtime. *What:* wraps your async main in a runtime. *How:* equivalent to manually building a `Runtime` and calling `block_on`.
- **`join!`** — run futures concurrently, wait for all. *What:* same task, no spawning; finishes when every future is `Ready`. *How:* returns a tuple of results.
- **`select!`** — race futures, take the first `Ready`. *What:* drops the others mid-flight. *How:* used for timeouts, cancellation, fastest-of patterns.
- **`spawn` (`tokio::spawn`)** — fire onto another task. *What:* returns a `JoinHandle`; future may run on another worker thread. *How:* future must be `Send + 'static`.
- **`Stream`** — async iterator. *What:* many values, each available "eventually". *How:* `while let Some(x) = stream.next().await`; combinators in `futures::StreamExt`.
- **`Pin`** — guarantee a value won't move. *What:* required for self-referential async state machines. *How:* almost always built via `Box::pin` or `tokio::pin!`; rarely constructed by hand.
- **Async vs threads** — different tools for different problems. *What:* async = many things waiting (I/O); threads = many things computing (CPU). *How:* combine them — many threads each running an async runtime is common.

## Ch.18 — OOP Features

- **OOP in Rust** — encapsulation yes, polymorphism yes, inheritance no. *What:* Rust skips inheritance by design. *How:* composition + default trait methods replace it; types stay independent, traits are the glue.
- **Encapsulation** — module privacy + `pub`. *What:* items private by default; `pub` exposes them. *How:* struct fields stay private even in `pub struct`; the type can rely on internal invariants.
- **Trait object `dyn Trait`** — type-erased polymorphism. *What:* one handle for many concrete types implementing a trait. *How:* stored behind a pointer (`Box<dyn T>`, `&dyn T`); fat pointer = data ptr + vtable ptr.
- **Static vs dynamic dispatch** — when the method is resolved. *What:* generics → compile time (monomorphization, inlinable); `dyn` → runtime (vtable lookup). *How:* prefer generics; reach for `dyn` only when mixing types in one container.
- **Dyn-compatibility** — rules a trait must satisfy to be `dyn`. *What:* no methods returning `Self`; no methods with their own generic params; no `Self: Sized` bound. *How:* without these, the vtable can't be built; older docs called this "object safety".
- **State pattern in Rust** — two ways. *What:* faithful port (`Box<dyn State>` consumed and replaced) vs Rust-idiomatic (one type per state, transitions consume `self`). *How:* the second pushes invalid transitions to compile errors via the type system (typestate pattern).

## Ch.19 — Patterns and Matching

- **Where patterns appear** — `match`, `if let`, `while let`, `for`, `let`, `let...else`, function params. *What:* patterns are everywhere bindings are introduced. *How:* `let (a, b) = (1, 2);` is a destructuring pattern.
- **Refutable vs irrefutable** — can the pattern fail to match? *What:* `let x = 5;` (irrefutable) vs `if let Some(v) = opt` (refutable). *How:* `let` only accepts irrefutable; `if let` / `match` arms accept either.
- **Pattern syntax basics** — literals, ranges, `_`, `..`. *What:* `1..=5`, `'a'..='z'`, `_` (catch-all), `..` (rest). *How:* mix freely: `Some(1..=5)`, `(_, y, ..)`.
- **Destructuring** — pull fields out of structs/enums/tuples. *What:* `let Point { x, y } = p;` or `Some(Point { x, .. }) => …`. *How:* renames with `x: a`; wildcards with `_`.
- **Pattern guards** — `if cond` after a pattern. *What:* extra runtime check on the arm. *How:* `Some(n) if n > 0 => …`; doesn't affect exhaustiveness.
- **`@` bindings** — capture and test simultaneously. *What:* `n @ 1..=5 => println!("{n}")` — bind to `n` only if range matches. *How:* useful when you both want to test and use the value.

## Ch.20 — Advanced Features

- **`unsafe`** — opt out of compile-time checks. *What:* five powers — deref raw pointers, call unsafe fn, access mutable static, implement unsafe trait, access `union` fields. *How:* `unsafe { … }` block; you take responsibility for upholding invariants.
- **Raw pointers** — `*const T`, `*mut T`. *What:* like C pointers — no aliasing rules, can be null. *How:* only dereferenced inside `unsafe`; mostly used for FFI or building safe abstractions.
- **FFI (`extern "C"`)** — call other languages. *What:* declare or expose C-ABI functions. *How:* `extern "C" fn` (callable from C) and `extern "C" { fn foo(); }` (calling C from Rust).
- **Associated types** — type placeholders inside a trait. *What:* `trait Iterator { type Item; … }`. *How:* implementor picks `Item` once per `impl`; cleaner than generic trait params for "one impl per type".
- **Default type parameters** — `<T = DefaultType>`. *What:* lets users skip the type when the default fits. *How:* used by `Add<Rhs = Self>` so `impl Add for Point` doesn't need to specify `Rhs`.
- **Fully qualified syntax** — `<Type as Trait>::method(...)`. *What:* pick a specific impl when names collide. *How:* needed for associated functions where Rust can't infer which trait.
- **Supertrait** — `trait Foo: Bar`. *What:* `Foo` requires `Bar` to also be implemented. *How:* `Foo`'s methods can call `Bar`'s methods.
- **Newtype pattern** — `struct Wrapper(Inner);`. *What:* bypass the orphan rule, encode units, restrict an API. *How:* zero runtime cost; access via `self.0`.
- **Type alias** — `type Kilometers = i32;`. *What:* different name, same type. *How:* not a new type — interoperable with the original; useful for shortening verbose generic types.
- **`!` (never type)** — type of expressions that never return. *What:* `panic!()`, `loop {}`, `return` have type `!`. *How:* coerces to any type; lets `match` arms diverge cleanly.
- **DSTs (`dyn Trait`, `[T]`, `str`)** — dynamically-sized types. *What:* size unknown at compile time. *How:* always behind a pointer (`&dyn T`, `Box<[T]>`, `&str`); fat pointer carries length or vtable.
- **`Sized` trait** — implicit bound on every generic. *What:* `<T>` actually means `<T: Sized>` by default. *How:* opt out with `<T: ?Sized>` to accept DSTs.
- **Function pointers** — `fn(i32) -> i32`. *What:* like a closure but cannot capture; implements all three `Fn` traits. *How:* useful for FFI and as a coercion target from non-capturing closures.
- **Returning closures** — `Box<dyn Fn(...)>` or `impl Fn(...)`. *What:* closures have anonymous types, so you wrap or use `impl Trait`. *How:* `impl Fn` is preferred when there's exactly one return path.
- **Macros (`macro_rules!`)** — declarative macros. *What:* pattern-match on token trees, expand at compile time. *How:* `vec!`, `println!`, `assert!` are all macros.
- **Procedural macros** — custom `derive`, attribute, function-like. *What:* Rust code that generates Rust code. *How:* defined in a separate `proc-macro` crate; powers `serde`, `tokio::main`, `clap`.

## Ch.21 — Final Project: Web Server

- **Single-threaded HTTP server** — listen, parse, respond. *What:* `TcpListener::bind`, accept connections, read request bytes, write response. *How:* each request handled in sequence — slow requests block everyone.
- **Thread pool** — bounded set of worker threads. *What:* workers pull jobs from a shared channel. *How:* `Arc<Mutex<Receiver<Job>>>` shared across workers; each worker loops on `recv()`.
- **`Box<dyn FnOnce() + Send + 'static>`** — type-erased job. *What:* the closure type the channel transports. *How:* `Box` for the unknown size, `FnOnce` because each job runs once, `Send + 'static` to cross thread boundaries safely.
- **Graceful shutdown** — drain the pool on exit. *What:* each worker exits when the channel closes. *How:* drop the sender; `recv()` returns `Err(...)`; workers break their loop and drop runs.
- **Dropping the pool** — `impl Drop for ThreadPool`. *What:* on drop, signal shutdown then `join` each worker. *How:* `take` the `JoinHandle` out of an `Option<JoinHandle>` (you can't `join` through `&mut self` otherwise).

---

## How to use this file

Three drill modes:
1. **Spot-check** — read a name (e.g., `RefCell`), articulate what + how before peeking at the line.
2. **Recap** — open the chapter you're reading, scan the relevant entries to anchor what's coming.
3. **Stale-check** — when something feels fuzzy, find the entry; if it doesn't ring a bell, re-read the chapter in `raw/rust-book/`.

For full explanations, examples, error walkthroughs: <https://doc.rust-lang.org/book/> or the local mirror in [`../raw/rust-book/`](../raw/rust-book/).
