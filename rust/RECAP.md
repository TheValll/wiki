# Rust — RECAP

Single-glance table of every concept across the Rust domain, organized by chapter.

---

## Ch 01 — [Installation, Hello World & Cargo](./01-installation-hello-cargo/README.md)

| § | Concept | What / for what | Intuition |
|---|---|---|---|
| 1.1 | What is Rust? | Compiled, statically typed, memory-safe-without-GC, fearless concurrency | A modern systems language with ownership instead of a runtime |
| 1.2 | Installing Rust | `rustup`, toolchain channels (stable/nightly), components | Single tool that manages versions and targets |
| 1.3 | Hello, World! | `fn main()`, `println!` macro | The simplest possible binary |
| 1.4 | Cargo | The build tool, package manager, runner | npm/pip equivalent built in |
| 1.5 | Essential Cargo Commands | `new`, `build`, `run`, `test`, `check`, `doc` | Daily verbs |
| 1.6 | Cargo.lock | Lockfile pins exact dependency versions for reproducibility | The "this exact recipe worked" snapshot |

## Ch 02 — [The Guessing Game](./02-guessing-game/README.md)

| § | Concept | What / for what | Intuition |
|---|---|---|---|
| 2.1 | Project setup | `cargo new`, structure | Scaffold for a new binary |
| 2.2 | Reading user input | `io::stdin().read_line(&mut s)` | Handing a buffer for input to fill |
| 2.3 | Random number | `rand` crate, `gen_range` | Pull in an external crate from crates.io |
| 2.4 | `match` for comparison | `cmp::Ordering` enum + match | Three-way comparison via a sum type |
| 2.5 | The game loop | `loop { … break; }` | Infinite loop with explicit exit |
| 2.6 | Final code | Putting it all together | First end-to-end program |

## Ch 03 — [Variables, Mutability & Data Types](./03-variables-types/README.md)

| § | Concept | What / for what | Intuition |
|---|---|---|---|
| 3.1 | Variables & immutability default | `let` binds, immutable by default | Read-only unless you say otherwise |
| 3.2 | `mut` keyword | Opt-in mutability | Explicit "yes, this can change" |
| 3.3 | `const` | Compile-time constant, must be typed, `SCREAMING_CASE` | Inlined at compile time, lives forever |
| 3.4 | Shadowing | Re-`let` the same name, possibly with a new type | Soft replacement — old binding gone, new one starts fresh |
| 3.5 | Scalar Types | `i32`, `u64`, `f64`, `bool`, `char` (4-byte Unicode scalar) | Fixed-size numeric / boolean / char primitives |
| 3.6 | Compound Types | Tuples (heterogeneous), Arrays (fixed-size, same type) | Tuple = anonymous struct; array = fixed-length list |

## Ch 04 — [Functions & Control Flow](./04-functions-control-flow/README.md)

| § | Concept | What / for what | Intuition |
|---|---|---|---|
| 4.1 | Defining/calling functions | `fn name(args) -> Type { … }` | Snake_case names; signature mandatory |
| 4.2 | Parameters | Typed args, no overloading | Each parameter has an explicit type |
| 4.3 | Statements vs Expressions | Statements end in `;`; expressions evaluate to a value | The last expression of a block IS the block's value |
| 4.4 | Return Values | `-> Type`, no semicolon on the last expression to return | `return` keyword optional at end |
| 4.5 | Comments | `//` line, `///` doc, `//!` inner doc | Doc comments feed `cargo doc` |
| 4.6 | `if` Expressions | `if cond { … } else { … }`, returns a value | Ternary equivalent: `let x = if … { a } else { b };` |
| 4.7 | Loops | `loop`, `while`, `for x in iter`; loop labels for nested break | Three flavors; `for` over iterators is idiomatic |

## Ch 05 — [Ownership](./05-ownership/README.md)

| § | Concept | What / for what | Intuition |
|---|---|---|---|
| 5.1 | Why Ownership? | Memory safety without a garbage collector | Compile-time discipline replaces runtime tracing |
| 5.2 | Stack vs Heap | Stack: fixed-size, fast LIFO; Heap: dynamic, allocator-managed | Boxes go on heap, copies stay on stack |
| 5.3 | The 3 Rules | Single owner; one owner at a time; dropped at scope end | The single-owner rule is the heart of Rust |
| 5.4 | Variable Scope | Lexical scope; resources released at `}` | Drop runs deterministically when binding leaves scope |
| 5.5 | The `String` Type | Heap-allocated, growable, owned | `String` owns; `&str` borrows |
| 5.6 | Move Semantics | `=` and function calls move ownership for non-`Copy` types | Handing the package to someone — you don't have it anymore |
| 5.7 | Clone | Explicit deep copy via `.clone()` | Costs are visible, never implicit |
| 5.8 | `Copy` Trait | Stack-only types (integers, bool, char) duplicate on `=` | Cheap-to-copy types opt into bitwise duplication |
| 5.9 | Ownership and Functions | Passing by value moves; receiving means ownership | Functions take ownership unless you lend |
| 5.10 | Return Values and Scope | Returns transfer ownership back | The owner can hand the package back |

## Ch 06 — [References, Borrowing & Slices](./06-references-slices/README.md)

| § | Concept | What / for what | Intuition |
|---|---|---|---|
| 6.1 | The Problem | Functions that consume their argument force callers to hand over | Without borrowing, every call is destructive |
| 6.2 | References & Borrowing | `&T` lends without transferring ownership | Lending a book: you keep the original |
| 6.3 | Immutable References | `&T` — read-only, many allowed simultaneously | Many readers OK in parallel |
| 6.4 | Mutable References | `&mut T` — exclusive write access | Exactly one writer at a time |
| 6.5 | Mixing Mutable / Immutable | Cannot have `&mut` and `&` alive at once | The two borrowing states are exclusive |
| 6.6 | Dangling References | Compiler rejects refs that would outlive their referent | Safety: no pointer to freed memory |
| 6.7 | The 2 Rules of References | Either many `&` OR one `&mut`; must always point to valid data | Aliasing XOR mutation |
| 6.8 | The Problem That Leads to Slices | Indices into a `String` invalidate after mutation | A slice ties a view to the data's lifetime |
| 6.9 | String Slices `&str` | Borrowed view into UTF-8 string data | A window into someone else's `String` |
| 6.10 | String Literals Are Slices | Literal `"hi"` has type `&'static str` | Baked into the binary |
| 6.11 | `&str` as Parameter | Takes `String` and `&str` callers; the universal text input | Borrow rather than own when you only need to read |
| 6.12 | Other Slices | `&[T]` for arrays/vecs | Same idea, generic over element type |

## Ch 07 — [Structs](./07-structs/README.md)

| § | Concept | What / for what | Intuition |
|---|---|---|---|
| 7.1 | Defining/Instantiating | `struct S { f: T }`, `S { f: v }` | Named record |
| 7.2 | Field init shorthand | `S { f }` when local var has same name | Less typing |
| 7.3 | Update syntax `..` | Build a struct from another, overriding some fields | Spread-style construction |
| 7.4 | Tuple structs | `struct Point(i32, i32);` — no named fields | Newtype pattern |
| 7.5 | Unit-like structs | `struct Marker;` — no data | Carry only the type identity |
| 7.6 | Ownership of struct data | Own data → use `String`; borrow → use `&str` + lifetimes | Owned vs borrowed fields decide ergonomics |
| 7.7 | Example: rectangle area | Worked example computing `width * height` | Tying it together |
| 7.8 | Derived Traits | `#[derive(Debug, Clone, PartialEq)]` | Free implementations the compiler writes for you |
| 7.9 | Method syntax | `impl S { fn m(&self) … }` | Methods are functions whose first arg is `self` |
| 7.10 | Methods with more parameters | Combine `&self` with extra args | Same as a function call but namespaced |
| 7.11 | Associated functions | `fn new() -> Self` — no `self` | Constructor / static methods |
| 7.12 | Multiple `impl` blocks | Split methods across blocks for organization | Compiler unifies them |

## Ch 08 — [Enums and Pattern Matching](./08-enums/README.md)

| § | Concept | What / for what | Intuition |
|---|---|---|---|
| 8.1 | Defining an Enum | `enum E { A, B }` | Sum type — exactly one variant at a time |
| 8.2 | Enums with Data | Each variant can carry payload | Struct-per-variant unified into one type |
| 8.3 | Methods on Enums | `impl E { … }` | Enums get methods like structs |
| 8.4 | `Option<T>` | Built-in `Some(T) \| None` — Rust's "no null" | Make absence a type |
| 8.5 | `match` Construct | Pattern-match on a value | Switch with destructuring |
| 8.6 | Patterns That Bind | `match x { Some(v) => v, _ => 0 }` | Destructure inside the arm |
| 8.7 | Matching with `Option<T>` | Forces you to handle `None` | Compiler-enforced null check |
| 8.8 | Matches Are Exhaustive | All variants must be covered | The compiler keeps your switch honest |
| 8.9 | Catch-All and `_` | `_ => …` for "anything else" | Default arm |
| 8.10 | `if let` | Match a single pattern concisely | Sugar for one-arm match |
| 8.11 | `let...else` | Bind or diverge if pattern fails | Early-exit pattern matching |

## Ch 09 — [Packages, Crates & Modules](./09-packages-crates-modules/README.md)

| § | Concept | What / for what | Intuition |
|---|---|---|---|
| 9.1 | Packages and Crates | A package = `Cargo.toml` + crate(s); a crate is a compilation unit | Package ships, crate compiles |
| 9.2 | Defining Modules | `mod name { … }` controls scope and privacy | Folder of code + visibility wall |
| 9.3 | Paths | `crate::`, `super::`, `self::` to refer to items | Absolute vs relative addressing |
| 9.4 | `use` | Bring paths into scope | Aliasing for ergonomics |
| 9.5 | Modules in Different Files | `mod foo;` looks for `foo.rs` or `foo/mod.rs` | Filesystem mirrors module tree |
| 9.6 | Real-World Example | Game of Life broken into modules | End-to-end module organization |

## Ch 10 — [Common Collections](./10-collections/README.md)

| § | Concept | What / for what | Intuition |
|---|---|---|---|
| 10.1 | `Vec<T>` | Growable, heap-allocated array | Resizable list |
| 10.2 | `String` & UTF-8 | Owned, growable, byte-level UTF-8 | A `Vec<u8>` validated as text |
| 10.3 | `HashMap<K, V>` | Hash-based dictionary | Key-to-value lookup in O(1) average |
| 10.4 | Choosing a Collection | When to pick which | Decision rules: ordered, unique, fast lookup |

## Ch 11 — [Error Handling](./11-error-handling/README.md)

| § | Concept | What / for what | Intuition |
|---|---|---|---|
| 11.1 | `panic!` | Unrecoverable errors that abort/unwind | Crash the program intentionally |
| 11.2 | `Result<T, E>` | Recoverable errors as values | Failure is just another return value |
| 11.3 | `unwrap` / `expect` | Quick conversions from `Result` to value, panicking on `Err` | Use sparingly, lose useful context |
| 11.4 | Propagating Errors | Return `Result` up the call stack | Caller decides how to handle |
| 11.5 | The `?` Operator | Auto-propagate or unwrap on `Result` / `Option` | One-character early-return |
| 11.6 | When to `panic!` vs Return `Result` | Bugs panic; expected failures return `Result` | Distinguish bugs from valid failure modes |
| 11.7 | Custom Validation Types | Encode invariants in the type system | Make illegal states unrepresentable |

## Ch 12 — [Generics, Traits & Lifetimes](./12-generics-traits-lifetimes/README.md)

| § | Concept | What / for what | Intuition |
|---|---|---|---|
| 12.1 | Generic Types | `fn f<T>(x: T)` — code that works for many types | One template, many monomorphized copies |
| 12.2 | Traits | Shared behavior contracts (`trait Display { … }`) | Like interfaces but no runtime cost (by default) |
| 12.3 | Lifetimes | Annotations that prove references stay valid | A "this-ref outlives that-ref" proof to the borrow checker |

## Ch 13 — [Tests](./13-tests/README.md)

| § | Concept | What / for what | Intuition |
|---|---|---|---|
| 13.1 | Test Function Anatomy | `#[test] fn t() { … }` | Tests live alongside the code |
| 13.2 | Assertions | `assert!`, `assert_eq!`, `assert_ne!` | Crash the test on failure |
| 13.3 | `should_panic` | Test that a code path panics | Negative-path coverage |
| 13.4 | `Result<T, E>` in Tests | Use `?` in tests; success on Ok | Cleaner than nested unwraps |
| 13.5 | Controlling Test Run | `cargo test`, `--`, `--nocapture`, filter by name | Knobs for faster cycles |
| 13.6 | Test Organization | Unit tests in `#[cfg(test)] mod tests`; integration in `tests/` | Two layers: white-box vs black-box |

## Ch 14 — [I/O Project: minigrep](./14-io-project-minigrep/README.md)

| § | Concept | What / for what | Intuition |
|---|---|---|---|
| 14.1 | CLI Arguments | `std::env::args()` | First real-world entry point |
| 14.2 | Reading a File | `std::fs::read_to_string` | Slurp a file's contents |
| 14.3 | Refactoring for Modularity | Split `main` from library logic | Separate concerns; library is testable |
| 14.4 | Error Handling Details | Replace `panic!` with `Result` propagation | Real errors deserve real types |
| 14.5 | TDD for `search` | Red/green/refactor on the search function | Tests first |
| 14.6 | Case-Insensitive via Env Var | Read `CASE_INSENSITIVE` from `env::var` | Configuration outside the code |

## Ch 15 — [Closures & Iterators](./15-closures-iterators/README.md)

| § | Concept | What / for what | Intuition |
|---|---|---|---|
| 15.1 | Closures: basics | Anonymous functions that capture environment | Functions that remember surrounding bindings |
| 15.2 | How closures capture | By reference, mutable reference, or move | Three capture modes mirror borrowing rules |
| 15.3 | `Fn` / `FnMut` / `FnOnce` | Three traits for the three capture kinds | Choose based on how the closure uses captured vars |
| 15.4 | `Iterator` trait | One method `next()` returns `Option<Item>` | Lazy pull-based pipeline |
| 15.5 | Consuming adapters | `.collect()`, `.sum()`, `.fold()` — drain the iterator | The "go" command for the lazy pipeline |
| 15.6 | Iterator adapters | `.map()`, `.filter()`, `.take()` — produce new iterators | Functional composition |
| 15.7 | Closures capturing env | `move` keyword forces ownership transfer into the closure | Useful for threads / returning closures |
| 15.8 | Writing your own iterator | `impl Iterator for Foo` | Custom lazy sequence |
| 15.9 | Performance | Iterators compile to the same code as hand-written loops | Zero-cost abstraction |
| 15.10 | Closures vs function items | Closures may capture; `fn` items don't | Coexist, used differently |

## Ch 16 — [More about Cargo & Crates.io](./16-more-cargo-crates/README.md)

| § | Concept | What / for what | Intuition |
|---|---|---|---|
| 16.1 | Release Profiles | `[profile.dev]` vs `[profile.release]` — speed vs symbols | Two kitchens, two priorities |
| 16.2 | Publishing to Crates.io | Versioning, metadata, license, `cargo publish` | The shared pantry |
| 16.3 | Workspaces | Multiple crates in one repo, sharing `target/` and dependencies | Mono-repo tooling |
| 16.4 | `cargo install` | Install a binary crate to `~/.cargo/bin` | Personal tool shelf |
| 16.5 | Custom Cargo Commands | Any `cargo-foo` in PATH becomes `cargo foo` | Cargo as a plugin host |

## Ch 17 — [Smart Pointers](./17-smart-pointers/README.md)

| § | Concept | What / for what | Intuition |
|---|---|---|---|
| 17.1 | `Box<T>` | Heap allocation, single ownership | Send a value to the heap |
| 17.2 | `Deref` | Make a type behave like a pointer to its inner | Why `*` and method-call autoderef work |
| 17.3 | `Drop` | Custom cleanup when value goes out of scope | RAII destructor |
| 17.4 | `Rc<T>` | Reference-counted shared ownership (single-thread) | Many owners by counting |
| 17.5 | `RefCell<T>` | Interior mutability with runtime borrow checks | Borrow rules enforced at runtime, not compile |
| 17.6 | `Rc<RefCell<T>>` | Shared mutable state | Many owners + write access through one |
| 17.7 | Reference Cycles & `Weak<T>` | Avoid memory leaks from cyclic `Rc` graphs | A non-owning ref to break the cycle |

## Ch 18 — [Fearless Concurrency](./18-fearless-concurrency/README.md)

| § | Concept | What / for what | Intuition |
|---|---|---|---|
| 18.1 | `thread::spawn` & `JoinHandle` | Spawn an OS thread, wait via `.join()`, `move` to transfer ownership | Hand a closure to a real OS thread; `move` because the thread may outlive its caller |
| 18.2 | Channels (`mpsc`) | Send/recv between threads, ownership transfers across the channel | Conveyor belt — once you put the box down, it's not yours anymore |
| 18.3 | `Mutex<T>` | Mutual exclusion via a `MutexGuard` smart pointer; lock released on drop | Whiteboard with a single marker — only the holder can write |
| 18.4 | `Arc<T>` | Atomically-counted shared ownership across threads | Like `Rc`, but with an atomic counter — combine with `Mutex` for shared mutation |
| 18.5 | `Send` & `Sync` | Marker traits that gate what can cross threads | The compiler-enforced contract behind everything in `std::sync` |

## Ch 19 — [Async / Await](./19-async-await/README.md)

| § | Concept | What / for what | Intuition |
|---|---|---|---|
| 19.1 | Futures, `async`, `.await` | `async fn` returns a lazy state machine; `.await` is "yield until ready" | A paused recipe — nothing cooks until someone runs the future |
| 19.2 | Executors & runtimes | `std` defines `Future`; the runtime (Tokio, smol) actually polls | The kitchen — the expediter (executor) calls tickets, the reactor watches the oven (OS events) |
| 19.3 | `join!`, `select!`, `spawn` | Run futures in parallel, race them, or fire onto another task | Three concurrency dials: all-of, first-of, fire-and-forget |
| 19.4 | `Stream` | Async `Iterator`: many values, eventually | Conveyor belt where each box arrives "eventually" |
| 19.5 | Async vs threads | Async = waiting at scale (I/O); threads = computing in parallel (CPU) | One rule: if it spends time waiting → async; if it spends time computing → threads |
| 19.6 | `Pin`, `Send`, async traits | Why `Pin<&mut Self>` in `poll`; spawn requires `Send`; AFIT stable since 1.75 | The rough edges; 95 % of code never sees them |

---

## Companion: [`rust-intuition.md`](./rust-intuition.md)

Pure-intuition single-file companion (no formulas, no exercises) covering ownership, traits, generics, closures, iterators, smart pointers, fearless concurrency, async/await — read-on-the-train style.
