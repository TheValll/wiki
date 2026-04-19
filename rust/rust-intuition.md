# Rust — Under the Hood

A companion to the Rust wiki pages. The numbered pages ([`01`](01-installation-hello-cargo.md)-[`14`](14-io-project-minigrep.md)) are **reference** — syntax, examples, code snippets. This page is the **pure intuition**: why Rust's rules exist, how to picture each mechanism, told in plain language without code. Readable on a train.

Covers concepts from chapters 1-14 of the Rust book.

---

## Part 1 — Memory and ownership

### 1.1 — The single-owner rule

Every value in Rust has exactly one **owner** — the variable that currently holds it. When that variable goes out of scope, the value is destroyed and its memory freed. This rule is the foundation of everything: there is never any confusion about *who is responsible* for cleaning up a resource.

Why this matters: in languages like C++, you can accidentally free the same memory twice (two owners, each thinks they should clean up) or leak it (no one thinks they should). In languages with a garbage collector, a runtime figures it out for you, at the cost of pauses. Rust's single-owner rule lets the **compiler** figure it out at compile time — no runtime, no pauses, no double-frees.

The mental image: every value is a package, and exactly one variable is holding it. When the holder leaves, the package is thrown away.

---

### 1.2 — Move vs Copy: what happens on `=`

When you write `let y = x;`, what happens depends on the type of `x`.

- If `x` holds something **cheap and self-contained** (an integer, a bool, a char — fixed-size, lives entirely on the stack), Rust makes a quiet bit-for-bit copy. Both `x` and `y` are usable afterwards. These types implement the `Copy` trait.
- If `x` **owns a resource** (a `String`, `Vec`, `Box` — anything that points to heap memory), duplicating the bytes would create two owners of the same allocation — which breaks the single-owner rule. So Rust **moves**: `y` now owns the allocation, and `x` is treated as gone (the compiler refuses to let you use it).

```
Copy types (i32, bool, f64, ...):
   let x = 5;
   let y = x;     →   x: 5     y: 5     (both usable — independent)

Move types (String, Vec, Box, ...):
   let s1 = String::from("hi");
   let s2 = s1;

                    s1 ✗ invalid       s2 ──┐
                                             ↓
                                       [ "hi" on heap ]

   Ownership transferred. s1 can no longer be used.
```

The golden question to ask yourself: "is this type cheap and self-contained, or does it own an external resource?" If the second, assume move.

The mental image: Copy types are like photocopying paper — both copies exist. Move types are like handing someone a door key — once you give it away, you don't have it.

---

### 1.3 — Borrowing: sharing without giving up

Moving ownership every time you pass a value to a function would be exhausting. So Rust lets you **lend** a value with a reference (`&T`) — like lending a book without giving it away. The borrower can read (or write, if you explicitly allow it), but you remain the owner, and you get it back when the loan ends.

There are two kinds of loans:
- `&T` — a **shared reference**. Read-only. Many can coexist.
- `&mut T` — an **exclusive reference**. Can read *and* modify. Only one can exist at a time, and no shared refs can coexist with it.

The mental image: a shared reference is a library book — many people can read it at once, none can write in it. A mutable reference is a whiteboard you've reserved — you can write on it, but no one else is even looking while you have it.

---

### 1.4 — The two borrowing states (and why)

At any moment, the compiler enforces **one of these two states** for every value:

```
State A — many readers, no writer:
   owner ─── &val ─── &val ─── &val         (all shared refs, read-only)

State B — one writer, nothing else:
   owner ─── &mut val                       (exclusive ref, can modify)

Never both states at once. Never two mutable refs.
```

Why? **Data races**. If one part of the code modifies a value while another reads it, the reader can see a half-finished change or a corrupted state. Most languages handle this with runtime locks or by accepting bugs. Rust forbids the situation at compile time — no locks, no race bugs, no overhead. The rule falls out of the type system.

The mental image: `&mut` is a "do not disturb" sign on the door. You can hang one, but only one, and nobody can peek in while it's up.

---

### 1.5 — Lifetimes as proofs

A reference points somewhere. If the thing it points to disappears, the reference dangles — pointing at garbage. Other languages let this happen and you crash at runtime; Rust refuses to compile the code.

For the compiler to enforce this, it needs to know **how long each reference is valid**. In most code, it figures this out automatically from scope. But when a function takes two references and returns one, it sometimes cannot tell which input the output was borrowed from. That's when you write a lifetime annotation like `'a`.

The annotation does **not create** a lifetime. It **states a relationship**: *"the output reference lives at least as long as `'a`, which is the lifetime of input `x`."* It's a proof you hand the compiler so it can check your logic.

The mental image: lifetimes are expiration dates stamped on loans. The compiler refuses any code where a loan would outlive the thing it was borrowed from.

---

### 1.6 — Slices: views into owned memory

A slice (`&[T]`, `&str`) is a small two-field struct: **a pointer and a length**. It doesn't own any data — it borrows a contiguous window into someone else's buffer.

This is how you write a function that takes *"the first word of any string"* without caring whether the input is an owned `String`, a string literal (`&'static str`), or a sub-region of another string. The function takes `&str` — a view — and works uniformly across all of them.

The mental image: a slice is a **bookmark range** ("pages 45 to 62"). You don't own the book; you just specify a region. As long as the book exists, the bookmark is valid.

---

## Part 2 — Shaping data

### 2.1 — Enums as sum types (Option and Result)

In Rust, an enum is not just a list of tags (like in C). **Each variant can carry its own data**, and the data can differ per variant. This is called a **sum type**: the value is *either* variant A with this data, *or* variant B with that data, and the compiler always knows which one you're holding.

Two enums are so central that they're in the standard library:
- `Option<T>` = `Some(T)` (a value is present) or `None` (it's absent). **Replaces null.**
- `Result<T, E>` = `Ok(T)` (success with a value) or `Err(E)` (failure with an error). **Replaces exceptions.**

The mental image: a variant is a labeled drawer. The enum is a cabinet — at any moment, exactly one drawer is open, and what's inside depends on which drawer it is. The compiler always knows which drawer is open.

---

### 2.2 — Pattern matching and exhaustiveness

The `match` expression checks which variant of an enum you have, and runs different code per variant. The magic: the compiler **refuses to compile** if you forget to handle a variant. No silent bugs where you "forgot about `None`" or "forgot about `Err`".

Combined with sum types, this forces the right thinking: `Option<T>` forces you to handle absence; `Result<T, E>` forces you to handle failure. There's no way to "just grab the value" without acknowledging what could go wrong — except by explicitly panicking.

The mental image: `match` is a checklist with a tickbox for every possible case. The compiler refuses to submit the form if any box is unticked.

---

### 2.3 — Modules and the visibility rule

Rust is **private by default**. Every item (function, struct, module) is invisible outside its module unless you mark it `pub`. This is the opposite of languages like Python or JavaScript, where everything is exposed until you hide it.

Two subtle asymmetries catch beginners:
- **Struct fields** — even in a `pub struct`, fields stay private by default. You must `pub` each field you want exposed.
- **Enum variants** — in a `pub enum`, all variants are automatically public. Because an enum's purpose is to be pattern-matched against, hiding variants would make the type unusable.

The mental image: a module is a room with a door, closed by default. Items are furniture inside. `pub` props the door open for one piece of furniture at a time.

---

## Part 3 — Abstraction

### 3.1 — Traits: shared behavior without inheritance

A trait defines a set of methods. Any type that "implements" the trait provides those methods, in its own way. This lets you write code that works on **any type with a certain capability**, without caring about the type's other details.

Rust has no class inheritance. You don't say "my type extends `Animal`." You say "my type implements the `Display` trait" or "my type implements both `Read` and `Write`." Types are independent; traits are the glue.

The mental image: traits are **labels you stick on types**. "This type is displayable." "That type is comparable." Code that needs a displayable thing asks for "anything with the `Display` sticker" — it doesn't care what kind of thing it is.

---

### 3.2 — Generics and monomorphization

A generic function like `fn largest<T>(slice: &[T]) -> &T` works for any type `T`. At compile time, the compiler **specializes** the function for each concrete type you actually use it with — it generates one copy for `i32`, one for `String`, one for your custom struct. This is called **monomorphization**.

The upside: **zero runtime cost**. The compiled binary has one version of `largest` per type you called it with, each optimized for that type. Generic Rust code performs exactly like hand-written specialized code.

The downside: the binary is slightly larger. Rarely a concern.

The mental image: a generic function is a **recipe template**. At compile time, the compiler reads every place the recipe is used, fills in the blank with the concrete type, and stores one baked copy of the recipe per type.

---

### 3.3 — Trait bounds: the contract on T

Inside a generic function, you can only call methods the compiler *knows* exist on `T`. If you want to compare two generic values, the compiler needs a guarantee that `T` is comparable. You give it that guarantee with a **trait bound**: `fn max<T: PartialOrd>(a: T, b: T) -> T`.

`T: PartialOrd` reads as *"T must implement `PartialOrd`"*. The compiler enforces this at the call site — if you try to call `max` with a type that can't be compared, it refuses.

The mental image: a trait bound is a **filter on the recipe template**. `fn largest<T: PartialOrd>(...)` is a recipe that says "I'll cook any ingredient `T`, as long as you can rank them against each other."

---

### 3.4 — Trait objects: the dynamic alternative

Generics + monomorphization is **static dispatch** — the compiler picks the right version at compile time. But sometimes you want a collection of different types sharing a common behavior — a `Vec` of mixed `Dog`, `Cat`, `Parrot`, all treated as `Animal`. That's what **trait objects** are for: `Box<dyn Animal>` or `&dyn Animal`.

Under the hood, a trait object is a **fat pointer**: one pointer to the value, one pointer to a **vtable** — a little lookup table of function pointers for that type's implementation of the trait. At runtime, calling a method looks up the function in the vtable and jumps there. This is **dynamic dispatch** — slightly slower than static, but it lets you mix types.

Rule of thumb: prefer generics (static) unless you genuinely need to mix types in one collection.

The mental image: generics **bake one specialized copy of the recipe per type**. Trait objects **keep a single recipe + a phonebook** (vtable) — at runtime, they look up the right method for whoever is at the counter.

---

## Part 4 — Errors as values

### 4.1 — Panic vs Result: two ways to fail

Rust has two kinds of failure:
- **Panic** — the program is in an impossible state, unrecoverable, abort. Triggered by `panic!(...)`, `unwrap()` on a failing `Result`, array out-of-bounds, etc. Use it for **bugs**, never for expected failures.
- **Result** — a value representing either success (`Ok(T)`) or a recoverable error (`Err(E)`). Returned by anything that can legitimately fail: file reads, network calls, parsing.

The golden rule for `unwrap`: it's a **promise to the compiler** that you know the `Result` is `Ok`. If you turn out to be wrong, the program panics. Save `unwrap` for cases where `Err` is *logically* impossible (e.g., parsing a hardcoded literal).

The mental image: panic is the **emergency brake** — something went catastrophically wrong. Result is the **normal failure channel** — a letter that might say "done" or "sorry, here's why."

---

### 4.2 — The `?` operator: the propagation shortcut

Chaining operations that each return a `Result` would be ugly without help — every step would need a `match` to extract the value and handle the error. The `?` operator collapses this to **one character**.

`x?` reads as: *"if x is `Ok(value)`, give me `value` and continue. If x is `Err(e)`, stop here and return `Err(e)` from the enclosing function."* It's early-return-on-failure, baked into the syntax. It works on `Option` too, with `None` playing the `Err` role.

This is how Rust makes error propagation feel clean: each step of a pipeline uses `?`, and the first failure short-circuits the whole thing without losing the error.

The mental image: `?` is an **elevator emergency stop**. Each floor (each step) checks "anything wrong?" If yes, it stops the elevator and exits with the error. If no, it continues to the next floor.

---

## Part 5 — Functional features (chapter 13)

### 5.1 — Closures: functions that remember

A closure is an anonymous function that can **capture variables from its surrounding scope**. You write it with `|arg| expression`, and any variable mentioned inside but defined outside is pulled in automatically.

Regular functions are blind to their surroundings — you must pass every dependency as a parameter. Closures are aware of their surroundings — they wrap up the bits they need and carry them along. This is extremely useful for iterator pipelines, callbacks, and anywhere a function needs context without you having to thread it through a parameter.

The mental image: a function is a chef who only knows the ingredients you hand them. A closure is a chef who also grabs whatever they need from the pantry behind them and carries it along when they leave.

---

### 5.2 — Fn / FnMut / FnOnce: the three capture kinds

A closure's behavior depends on what it **does** with its captures. Rust expresses this with three traits, from most permissive to most restrictive:

| Trait | What it does | Can be called... |
|-------|--------------|------------------|
| `Fn` | only **reads** captures | many times, many copies simultaneously |
| `FnMut` | **modifies** captures | many times, but not in parallel |
| `FnOnce` | **consumes** captures (moves them out) | once — after the first call, captures are gone |

You rarely write these traits yourself — Rust **infers** which applies based on what the closure's body does. But when you accept a closure as a parameter, you pick the **weakest** trait that still covers your needs: `F: Fn()` accepts only Fn closures; `F: FnOnce()` accepts all three (since any closure can be called once).

The mental image:
- `Fn` is a **reference book** — read-only, many readers.
- `FnMut` is a **whiteboard** — one person writes at a time.
- `FnOnce` is a **single-use ticket** — once called, it's spent.

---

### 5.3 — The `move` keyword

By default, a closure **borrows** its captures — same rules as any other reference. This is usually fine, but it means the captures must outlive the closure (otherwise you'd have a dangling reference).

When you write `move |arg| ...`, you tell the closure to **take ownership** of its captures instead of borrowing them. This is the only way to send a closure to another thread, because the other thread runs on its own timeline and can't rely on references from the spawning thread's scope — they could disappear at any moment.

The mental image: a default closure **borrows the pantry** behind it — the pantry must stay put where it is. `move` means the closure **packs the pantry into its suitcase** and carries it along. Now it can go anywhere, including another thread.

---

### 5.4 — Iterators: lazy pipelines

An iterator is any type that implements the `Iterator` trait — a single method, `next()`, that returns `Option<Item>` (the next value, or `None` when the sequence is exhausted). That single method is the entire contract.

The magic is in the **laziness**. Iterator adapter methods (`map`, `filter`, `take`, `skip`, ...) don't compute anything — they return *another* iterator that knows how to produce its next value on demand. You can chain a dozen adapters and the chain does nothing until a **consumer** (`collect`, `sum`, `count`, a `for` loop) starts pulling. Then the whole chain runs **one element at a time**, without ever allocating intermediate vectors.

```
  source  ──►  [ map ]  ──►  [ filter ]  ──►  [ take(5) ]  ──►  collect()
                                                                     ↑
                                                               consumer pulls

  Each element travels through the whole belt one at a time.
  Nothing moves until the consumer pulls.
```

This is why Rust iterators are both **expressive and fast** — often faster than hand-written loops. The compiler sees the entire pipeline as one computation and optimizes it aggressively. In practice: zero-cost abstraction.

The mental image: an iterator chain is a **conveyor belt** with stations (map, filter, ...). Nothing moves until the consumer at the end pulls. Then each element travels through all stations, one at a time, no buffers in between.

---

## Part 6 — Cargo and the ecosystem (chapter 14)

### 6.1 — Release profiles: two kitchens, two priorities

Cargo has two default **profiles**: `dev` and `release`. The distinction is a classic compile-time tradeoff.

- `dev` — the compiler does **minimal work**: no optimisation, debug info included. The binary is slow, but compile times are fast. Perfect for the tight edit-run-test loop of development.
- `release` — the compiler does **maximal work**: all optimisation passes, aggressive inlining, no debug info. Compile times are slow, but the binary runs fast. This is what you ship.

A `[profile.dev]` or `[profile.release]` section in `Cargo.toml` lets you tune the dials (optimisation level, link-time optimisation, panic behaviour, etc.).

The mental image: two kitchens in the same restaurant. The **dev kitchen** is a prep station — plates come out fast and rough because chefs are experimenting. The **release kitchen** is the plating station — the last one before the customer — where every detail is polished even if it takes longer.

---

### 6.2 — Crates.io: the shared pantry

A **crate** is a Rust library or binary. **crates.io** is the central registry where anyone can publish a crate and anyone can depend on it by name. Cargo downloads, compiles, and links dependencies automatically.

What makes the ecosystem work:
- A crate is uniquely identified by **name + version**. Names are first-come, first-served globally.
- Published versions are **permanent** — you can never delete a version, only mark it "do not use anymore" via `cargo yank`. This guarantees that anyone who already depends on a version keeps working forever.
- Documentation is standardised: `///` on an item becomes rendered docs on [docs.rs](https://docs.rs), automatically built for every published version.
- Examples inside doc comments are **run as tests** — the docs can't lie without `cargo test` noticing.

The mental image: crates.io is a **shared pantry** that everyone contributes to. Once you put a jar on the shelf, it stays there forever (you can mark it "expired" but never remove it). Everyone who wrote "I need that jar" gets to keep cooking with exactly the version they picked when they wrote it.

---

### 6.3 — Workspaces: one repo, many crates

A **workspace** is a single repository containing several related crates — typically a library plus one or more binaries that use it. The workspace shares a single `Cargo.lock` and `target/` directory across all members.

Why this matters:
- **One lockfile** means all members agree on dependency versions. You cannot accidentally end up with two incompatible versions of the same external crate in the same binary.
- **One `target/`** means shared compilation output — if `crate-a` compiles `serde` once, `crate-b` reuses it instead of recompiling.
- You develop all members at once: `cargo build` at the workspace root builds everything.

The mental image: a workspace is a **shared kitchen** for a family of restaurants. Each restaurant (crate) has its own menu, but they share the same fridge and the same set of spices — so nobody ends up with two incompatible jars of the same ingredient, and prep work is done once for everyone.

---

### 6.4 — `cargo install`: the personal tool shelf

`cargo install <crate>` downloads the source of a binary crate, compiles it in release mode, and drops the executable into `~/.cargo/bin/` (which should be in your `$PATH`).

This is **user-local**, not system-wide — nothing is installed into `/usr` or `C:\Program Files`, no admin rights needed, no package manager involved. It's how you get tools like `ripgrep`, `cargo-watch`, or `tokei` onto your machine as a Rust developer.

The mental image: `cargo install` is your **personal tool shelf** next to your desk. Different from the system's public toolbox (which your OS manages); different from the per-project dev dependencies (which live inside each project). It's the place where your own everyday power tools sit, compiled for your machine, always at hand.

---

## What to remember

| Concept cluster | The one-line picture |
|-----------------|----------------------|
| Ownership | every value has exactly one holder; when the holder leaves, the value dies |
| Move vs Copy | cheap types photocopy; resource-owning types hand over the key |
| Borrowing | lend the value without giving it up: many readers OR one writer |
| Lifetimes | expiration dates on loans, so references never outlive what they point to |
| Slices | a (pointer, length) pair — a window, not a copy |
| Enums | labeled drawers; the compiler always knows which one is open |
| Pattern matching | a checklist that refuses to submit if any case is unticked |
| Traits | labels stuck on types — "anything with this sticker can do X" |
| Generics | a recipe template, baked per concrete type at compile time |
| Trait bounds | a filter on the template — "only ingredients that can be ranked" |
| Trait objects | keep one recipe + a phonebook; look up the method at runtime |
| Panic vs Result | emergency brake vs normal failure channel |
| `?` operator | elevator emergency stop — propagate the error upward |
| Closures | functions that grab stuff from their surrounding pantry |
| Fn / FnMut / FnOnce | read-only book / write-one-at-a-time whiteboard / single-use ticket |
| `move` | pack the pantry into the closure's suitcase |
| Iterators | conveyor belt — nothing moves until the consumer pulls |
| Release profiles | two kitchens — dev preps fast and rough, release polishes for the plate |
| Crates.io | shared pantry — jars go in permanently, can be marked "expired" but never removed |
| Workspaces | shared kitchen — many restaurants, one fridge, one set of spices |
| `cargo install` | your personal tool shelf — user-wide binaries, no admin rights needed |
