# Rust — Under the Hood

A companion to the Rust wiki pages. The numbered pages ([`01`](01-installation-hello-cargo.md)-[`17`](17-smart-pointers.md)) are **reference** — syntax, examples, code snippets. This page is the **pure intuition**: why Rust's rules exist, how to picture each mechanism, told in plain language without code. Readable on a train.

Covers concepts from chapters 1-15 of the Rust book.

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

## Part 7 — Smart pointers (chapter 15)

### 7.1 — What "smart pointer" even means

A plain reference (`&T`, `&mut T`) only **borrows** — it points at something owned by someone else, and it must never outlive that someone. A **smart pointer** is a regular struct that *also* acts like a pointer, but usually **owns** the thing it points at, and carries extra machinery along: it may allocate on the heap, count references, run custom cleanup, or enforce borrow rules at runtime.

What makes a type a smart pointer is not a keyword — it is implementing two traits: `Deref` (so `*` and `.method()` feel pointer-like) and `Drop` (so cleanup happens automatically on scope exit). Every smart pointer in this part is built from those two primitives.

The mental image: a plain reference is a **sticky note** on someone else's book — you can peek at a page, but you don't own the book and you must put the note back in time. A smart pointer is a **briefcase that happens to hold the book itself** — heavier, but yours, and it takes care of returning the book to the library when you put the briefcase down.

---

### 7.2 — `Box`: send a value to the heap

By default, Rust puts your values on the **stack** — fast, small, automatically reclaimed. `Box<T>` says: *"put this value on the heap instead, and give me a tiny pointer on the stack so I can carry it around cheaply."*

You don't reach for `Box` to make things fast (the heap is slower). You reach for it for three reasons: the value is **huge** (you don't want to copy it every time you move a variable), the type is **recursive** (a list that contains a list that contains a list — infinite on the stack, finite through a pointer), or you want a **trait object** (one pointer type that can stand for many concrete types implementing a trait).

The mental image: your stack is your **desk**. Most values live there because they fit and you reach them instantly. A `Box` is a **cardboard box in the warehouse**: the item is big or oddly shaped, so you store it downstairs and keep only the label on your desk. When you're done with the label, the warehouse clerk also throws away the box — no leak.

---

### 7.3 — Deref: why `*` works on everything

`Box`, `Rc`, `&T`, your own hand-rolled smart pointer — all of them support `*thing` to "peek inside." That's not magic, it's the `Deref` trait: any type that implements it becomes *dereferenceable*, and the compiler rewrites `*x` into `*x.deref()` transparently.

Even better, Rust quietly chains deref coercions at function-call sites: if you have a `&Box<String>` and the function wants `&str`, the compiler does **two** coercions in a row (`&Box<String> → &String → &str`) without you writing a single cast. This is why Rust APIs feel so forgiving about what you pass in — the coercion network is doing background work.

The mental image: `Deref` is an **adapter plug**. Your `Box<String>` doesn't natively fit into the `&str`-shaped socket, but the compiler stacks two adapters behind your back and it plugs in cleanly. All the adapters are compile-time — no runtime cost.

---

### 7.4 — Drop: the destructor you control

Every type in Rust already has an implicit "destructor" that frees memory when a value goes out of scope. The `Drop` trait lets you **hook into** that moment: when my value is about to die, run this code. Closing a file handle, releasing a lock, logging, decrementing a reference count — all of this happens in `drop`.

Two surprising details: drops run in **reverse order of declaration** (last declared, first dropped — because the stack unwinds like a stack of plates), and you **cannot call `drop(&mut self)` directly** — Rust forbids it because your manual call plus the automatic end-of-scope call would run cleanup twice. To fire cleanup early, use the free function `std::mem::drop(x)` — it takes ownership, so the value is *actually* gone and the destructor only runs once.

The mental image: `Drop` is the **goodbye routine** a value runs on its way out the door. You cannot interrupt someone else's goodbye, but you can say *"okay, go now"* by handing them to `std::mem::drop`, which escorts them out and runs their routine on the way.

---

### 7.5 — `Rc`: shared ownership by counting

Single ownership is great for most things, but sometimes **multiple parts** of the code genuinely need to own the same value — think of a graph node that several edges point to. `Rc<T>` (Reference Counted) gives you this: every clone bumps a counter, every drop decrements it, and the value is freed exactly when the last clone dies.

Cloning an `Rc` does **not** deep-copy the data — it only increments the counter. So `Rc::clone(&a)` is cheap (a pointer and a `+1`), regardless of how large `T` is. Convention is to write `Rc::clone(&a)` rather than `a.clone()`, so readers of your code immediately see that the clone is just a count bump, not a full duplicate.

One important constraint: `Rc` is **read-only** and **single-threaded**. Read-only because multiple owners writing through the same pointer would break aliasing rules. Single-threaded because the counter is not atomic — for threads, use `Arc` (Atomic Rc), same API, slightly slower.

The mental image: `Rc` is a **party balloon** with a tally chalked on the string. Every time a friend grabs the string, they add a tick mark. Every time a friend lets go, they cross one off. When the last tick is crossed, the balloon is released. Nobody can pop or repaint the balloon while it's being shared — you can only admire it.

---

### 7.6 — `RefCell`: runtime borrow checking

Rust's borrow checker is **compile-time**. It proves at build time that you never have two mutable references at once. Sometimes this proof is too conservative — you know the code is safe, but the compiler cannot see why. `RefCell<T>` is the escape hatch: it enforces the **same rules** (one writer OR many readers, never both), but checks them at **runtime**. A violation is a panic, not a compile error.

You use it precisely when you need to mutate something **through a shared reference** — for example, a cache inside a struct whose `&self` methods need to update it, or a test double that records calls. The public API still looks immutable, but internally a `RefCell` keeps the ability to `borrow_mut`.

`RefCell` does **not** give you two mutable references. It gives you the *ability* to take a single mutable reference through a shared outer reference. The aliasing rule is untouched — only the place it's checked moves from compile time to runtime.

The mental image: the compile-time borrow checker is a **bouncer at the door** who decides who enters before the club opens. `RefCell` is **no bouncer at the door, but alarms inside**: you can walk in freely, but if you break the rule (two writers, or a writer while readers are inside), the alarms go off at runtime — a panic. Same rules; different enforcement timing.

---

### 7.7 — `Rc<RefCell<T>>`: shared mutable state

Each layer gives you one property: `Rc` gives you multiple owners (at the cost of read-only access); `RefCell` gives you interior mutability (at the cost of runtime checking). Stack them, and you get **many owners** that can **each mutate** the shared value:

```
  Rc <  RefCell <  T  > >
   ↑      ↑         ↑
   │      │         └─ the actual data
   │      └─ runtime-checked borrow rules (interior mutability)
   └─ shared ownership by reference counting
```

Read inside-out: a value, wrapped in a RefCell so it can be mutated through shared references, wrapped in an Rc so many owners exist. Each owner calls `.borrow_mut()` when they actually want to write — only one writer at a time, enforced at runtime.

For the thread-safe version, swap the layers: `Arc<Mutex<T>>` (many threads, one writer at a time) or `Arc<RwLock<T>>` (many threads, many readers OR one writer). Same idea, different locks.

The mental image: a **shared whiteboard**. `Rc` is the list of people who have a key to the room — any of them can come in. `RefCell` is the **marker**: only one person can be holding it at a time. Many people in the room, but only one writes at a time — and the system panics if two people try to grab the marker at once.

---

### 7.8 — Cycles and `Weak`: the one failure mode

Reference counting has a blind spot: **cycles**. If `a` holds an `Rc` to `b`, and `b` holds one back to `a`, neither counter ever hits zero — even when all your local variables have been dropped. The two values just quietly live on, leaking memory forever. Rust does not detect this at compile time, and `Rc` does not leak-check at drop. No crash, no warning — just a slow drip.

The fix is to break the cycle by making **one direction not own**. That's what `Weak<T>` is for: it's an `Rc` sibling that **does not count**. A `Weak` pointer says *"I know about this value, but I don't vote on whether it should stay alive."* When you actually want to use it, call `.upgrade()` — you get `Some(Rc<T>)` if the value still exists, `None` if it has been dropped.

The canonical shape is a tree or a DAG with back-pointers: children own themselves strongly (parents `Rc` their children), but children hold only a `Weak` back to their parent. If the tree gets cut off from the root, the parent drops, the `Weak` upgrades to `None` on the next visit, and everything unwinds cleanly.

The mental image: a regular `Rc` is a **cable holding the chandelier up** — cut all cables and it falls. A `Weak` is a **safety string** dangling from the chandelier — it tracks that you're still connected, but it is not holding the chandelier. If the last cable is cut, the chandelier falls and the string goes slack (`.upgrade()` returns `None`).

---

### 7.9 — The spectrum: one aliasing rule, many enforcement points

Ownership and borrowing in Rust is not one system — it's a **spectrum**. Each type in this part trades one static guarantee for one dynamic check, in exchange for more flexibility:

| Type | Owner count | Check | Mutability |
|------|-------------|-------|------------|
| plain `T` | 1 | compile time | direct |
| `Box<T>` | 1 (on heap) | compile time | direct |
| `&T` / `&mut T` | borrower, not owner | compile time | depends |
| `Rc<T>` / `Arc<T>` | N | compile time (refcount is runtime) | read-only |
| `RefCell<T>` | 1 | **runtime** | interior |
| `Rc<RefCell<T>>` | N | **runtime** (borrows) | interior |
| `Arc<Mutex<T>>` | N threads | **runtime** (lock) | interior |

Everywhere you see *runtime*, you've paid for **flexibility** with a **panic risk**: the aliasing rule is still enforced, just later. Rust gives you the knob. Turn it toward compile time when you can, toward runtime only when you must.

---

## Part 8 — Fearless concurrency (chapter 16)

### 8.1 — What "fearless" means

Concurrency in C is a minefield: data races, dangling pointers across threads, locks acquired in the wrong order — and most of it isn't caught until production. Rust's bet is that the **same** ownership and borrowing rules that prevent bugs in single-threaded code also prevent **data races** in multi-threaded code. You don't learn a new system for threads; you reuse the one you already know. The compiler refuses to compile most of the racy patterns. That's why it's "fearless" — not because nothing can go wrong, but because the categories of bugs that haunt other languages are simply rejected at the door.

The mental image: ownership in single-threaded Rust is a **traffic system inside one city**. Concurrency is the **same traffic system extended across cities** — same rules of right-of-way, but now there are highway markers (`Send`, `Sync`) that say which vehicles are allowed to cross the bridge between cities, and which must stay in their hometown.

---

### 8.2 — Threads: real OS threads, sent off with `move`

`thread::spawn(|| { … })` hands a closure to a fresh OS thread — same primitive as `pthread_create`, just safe. The closure starts running in parallel; you get a `JoinHandle` you can `.join()` on later to wait for the result. Almost every spawn closure starts with `move` because the spawned thread might outlive the function that created it — capturing by reference would risk dangling, so the compiler insists you transfer ownership.

The mental image: `thread::spawn` is **handing an envelope to a courier**. Once the courier walks out the door, you don't know when they'll be back. Anything inside the envelope must belong to the courier outright — you can't lend them something you might want back. `JoinHandle` is the **return receipt**: when you `join` it, you wait at the door until the courier comes back with whatever they were sent to fetch.

---

### 8.3 — Channels: share by communicating

Instead of two threads poking at the same memory, what if one thread **sends** a value and the other **receives** it? The value's ownership transfers across the channel — once you've sent it, you can't read it anymore. No aliasing, no race. `mpsc::channel()` (multi-producer, single-consumer) gives you a `Sender` and a `Receiver`; `send` puts a value on the belt, `recv` waits at the other end.

This is the Go-style philosophy that Rust embraces: *do not communicate by sharing memory; instead, share memory by communicating.* You almost never reach for raw shared state in well-designed Rust concurrency — you set up channels, and each thread owns its own slice of the world.

The mental image: a **conveyor belt** between two workers. Worker A places boxes on the belt; worker B picks them up at the other end. Once a box leaves A's hand, it isn't theirs anymore — they can't reach back and modify the contents. The belt moves ownership, not just data. (And `tx.clone()` lets you have multiple workers feeding the same belt — many producers, one consumer.)

---

### 8.4 — `Mutex<T>`: shared state, one writer at a time

Sometimes channels are wrong for the problem — you really do want **all threads looking at the same value** and taking turns writing. That's a `Mutex<T>`. Calling `m.lock()` blocks until the lock is free, then hands you a `MutexGuard` that acts like `&mut T`. When the guard goes out of scope, it drops, the lock releases automatically. There is no `.unlock()` — the type system enforces release.

`Mutex` is the threaded sibling of `RefCell` (§7.5). Same idea: pretend you have shared access (`&Mutex<T>`), get back exclusive access (`&mut` to inner) at the cost of a runtime check. `RefCell` panics on violation; `Mutex` blocks until the violation can no longer happen.

The mental image: a **shared whiteboard with a single marker**. Many people in the room, but only the one holding the marker can write on the board. Put the marker down (drop the guard), and the next person picks it up. There is exactly one marker; the type system makes sure of it.

---

### 8.5 — `Arc<T>`: shared ownership, but atomic

`Mutex<T>` alone has one owner. To share it across threads you wrap it again: `Arc<Mutex<T>>` — many owners, each can briefly become **the** writer. `Arc` is `Rc` with one change: the reference counter is updated with **atomic CPU instructions**, so two threads can `clone` and `drop` simultaneously without corrupting the count. The API is identical; the only cost is that the atomic op is ~10× slower than a plain integer increment (still fast — single-digit nanoseconds).

The pattern `Arc<Mutex<T>>` is the canonical shape for shared mutable state. Read inside-out: a value, wrapped in a Mutex so only one thread writes at a time, wrapped in an Arc so many threads can all hold a copy of the handle. Use `Arc<RwLock<T>>` when reads vastly outnumber writes — many readers OR one writer, instead of always one.

The mental image: `Rc` is **a string with paper tally marks** — fine in a quiet room, but if two people try to add a mark at the same time, they smudge each other's. `Arc` is **a digital counter** — atomic increment, no smudging, slightly slower per click.

---

### 8.6 — `Send` and `Sync`: the contract behind everything

Two **marker traits**, no methods, just labels:
- **`Send`**: this type can be **transferred** to another thread.
- **`Sync`**: this type can be **shared** between threads — a `&T` is `Send`.

You almost never implement them by hand. The compiler auto-derives them for any type whose fields are all `Send` / `Sync`. The interesting types are the ones that **opt out**: `Rc<T>` is `!Send` (its counter isn't atomic), `RefCell<T>` is `!Sync` (its runtime check isn't thread-safe), raw pointers are neither.

Every API in `std::sync` reads these marker traits as preconditions. `thread::spawn` takes `F: FnOnce() -> T + Send + 'static` — the closure must be sendable to the new thread. `tokio::spawn` does the same for futures. `Arc<T>` is `Send + Sync` only when `T: Send + Sync`. The whole concurrency stack is a network of these requirements; the compiler walks it for you and complains if any link is broken.

The mental image: `Send` is a **passport** — this type is allowed to cross the border to another thread. `Sync` is a **shared library card** — many threads can simultaneously check this type out (read-only borrow). Most types have both passport and library card. The dangerous types (`Rc`, `RefCell`, raw pointers) are barred at the border, by name, by the compiler.

---

## Part 9 — Async / Await (chapter 17)

### 9.1 — Threads vs async: two tools, two problems

Threads (Part 8) give you **parallelism** — many things really happening at once on different cores. Async gives you **concurrency** — many things *making progress* without dedicating a whole OS thread to each. They're not interchangeable. The single rule: if your code spends most of its time **waiting** (network, disk, sensors, locks), use async. If your code spends most of its time **computing** (math, parsing, hashing), use threads.

A web server holding 10 000 simultaneous TCP connections is the canonical async case — each connection is idle 99 % of the time, waiting on the network. 10 000 threads would burn 20 GB of stack space; 10 000 async tasks fit on one runtime, comfortably. A ray tracer is the canonical thread case — every CPU cycle is doing useful work, there's nothing to wait on, async would just add overhead.

The mental image: threads are **employees** — each one gets their own desk and works one job at a time. Async tasks are **paper tickets** the same employees pass around — when a ticket says "wait for the oven," the employee sets it down and grabs another one. Many tickets per employee, no idle staff.

---

### 9.2 — Futures: lazy by default

In most languages, calling an async function **starts** the work. In Rust, calling `async fn foo()` **does nothing**. It just hands you a `Future` — a struct representing the paused computation. Until something actually runs that future to completion, the body never executes.

```rust
let fut = fetch(url);   // nothing happens
let r = fut.await;      // now the work runs
```

This is jarring at first ("I called the function, why didn't it run?") but it's the foundation of how async composes: futures are **values**, you can hold them, combine them with `join!` and `select!`, throw them away, restart them. Eager async (Python, JS) doesn't let you do that — the work has already started, you can't take it back.

The mental image: an `async fn` is **a recipe card**. Calling it gives you the card; nothing cooks until you hand it to a chef and say "make this." `.await` is the chef pausing at a step that says "wait 20 minutes for the dough to rise" — they walk away, work on another order, come back when the timer rings. Each `.await` in your code is a **pause-and-resume point**.

---

### 9.3 — Executors: the chef Rust doesn't ship with

A future is just a state machine. Something has to actually call `poll` in a loop, watch for I/O readiness, reschedule futures when they wake up. That something is the **executor**, and Rust's `std` does **not** provide one. You pick a runtime as a library — almost everyone picks Tokio.

This is unusual: in Go, Python, JS, the runtime is the language. Rust split the language (`Future`, `.await`) from the runtime (Tokio, smol, async-std) so the same syntax can drive a 16-core production server **and** a microcontroller. The cost: you have to add `tokio = { version = "1", features = ["full"] }` to every async project, and `#[tokio::main]` decorates your `main`. The benefit: async Rust runs on bare metal too.

The mental image: `std` ships you the **recipe cards and the pause-and-resume mechanism**, but not the kitchen. You import a kitchen (Tokio) — that brings the expediter (executor) calling out tickets, the workers (worker threads) cooking them, and the oven timer (reactor) listening for OS events and pinging the expediter when something is ready.

---

### 9.4 — `join!`, `select!`, `spawn`: three concurrency dials

You have N futures and you want to run them in parallel. There are three flavors:

- **`join!`** — *all of them, together.* Polls each, finishes when every one is `Ready`, returns a tuple of results. Same task, no spawning. Use this for parallel fetches that all need to complete.
- **`select!`** — *first one wins, cancel the rest.* As soon as one branch is `Ready`, the others are dropped mid-flight. Use this for timeouts, racing the fastest mirror, cancellation.
- **`spawn`** — *fire onto another task.* Returns a `JoinHandle`, like `thread::spawn`. The future may move to a different worker thread (so it must be `Send`). Use this for fire-and-forget, or when the work outlives the caller.

The mental image: `join!` is **putting three tickets in a row on the rail and waiting for all three to come back**. `select!` is **a horse race** — the first finisher wins, the others are pulled off the track. `spawn` is **dropping a ticket into a separate kitchen** — you'll check on it later, and meanwhile both kitchens cook in parallel.

---

### 9.5 — `Stream`: many values, eventually

A `Future` resolves to **one** value. A `Stream` produces **many** — it's the async equivalent of `Iterator`. `while let Some(item) = stream.next().await` is the async-for loop. Streams are everywhere I/O is: a TCP listener accepting connections, a database query yielding rows, a timer ticking every second, a channel receiver yielding messages.

The same combinator vocabulary as iterators works on streams (`.map`, `.filter`, `.fold`), with the twist that the closures often return **futures** themselves — at every step, you might want to do more I/O. Backpressure falls out naturally: if no one polls the stream, the source doesn't run faster than the consumer collects.

The mental image: an iterator is a **conveyor belt where every box is already on the belt** — you grab them as fast as you want. A stream is a **conveyor belt where each box arrives eventually** — sometimes you wait at the end, sometimes the box is already there. Empty belt = the producer hasn't sent anything yet, and isn't being asked to.

---

### 9.6 — The rough edges: `Pin`, `Send` across awaits, async traits

Most of async Rust is just `async`/`.await` and Tokio. Three corners feel rough, and you'll see them mentioned:

**`Pin<&mut Self>`** in `Future::poll`: the compiler-generated state machine of an `async fn` may hold self-referential pointers (a local borrowing another local, both in the same struct). If the runtime moved that struct in memory between polls, the pointers would dangle. `Pin` is the type-system promise *"this value will not move."* You almost never construct `Pin` by hand — `Box::pin`, `tokio::pin!` do it for you.

**`Send` across `.await`**: spawning a future requires it to be `Send`, which means every value held across an `.await` must be `Send`. The classic offender is an `Rc` that survives an await — the compiler refuses to spawn that future. Fix: drop the `Rc` before the await, or swap it for `Arc`, or use `spawn_local` (which doesn't require `Send`).

**Async in traits** was awkward until Rust 1.75 (Dec 2023). Before that, you needed the `#[async_trait]` macro, which boxed the future. Since 1.75, you can write `async fn foo(&self) -> T` directly in a trait — with caveats around `dyn Trait` dispatch and explicit `Send` bounds. For most code today, write it directly.

The mental image: these three are the **concrete and rebar** behind the smooth wall. You usually don't see them. When the compiler error mentions one, find the load-bearing reference; either move it, drop it earlier, or wrap it in something `Send`.

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
| `Box<T>` | the warehouse — value stored off-stack, stack holds only the label |
| `Deref` | an adapter plug — chained quietly by the compiler, zero runtime cost |
| `Drop` | the goodbye routine — runs at scope exit, reverse declaration order, fire early via `drop(x)` |
| `Rc<T>` | the party balloon — tally marks on the string, released at the last tick |
| `RefCell<T>` | no bouncer at the door, alarms inside — same rules, checked at runtime |
| `Rc<RefCell<T>>` | shared whiteboard with one marker — many key-holders, one writer at a time |
| `Weak<T>` | safety string, not a cable — doesn't hold the chandelier up, just tracks it |
| `thread::spawn` | hand the courier an envelope — `move` is the only way to fill it |
| `JoinHandle` | the return receipt — `.join()` waits at the door for the courier |
| Channels (`mpsc`) | conveyor belt — once the box leaves your hand, it isn't yours anymore |
| `Mutex<T>` | shared whiteboard with a single marker — drop the guard, next person picks it up |
| `Arc<T>` | digital atomic counter — many threads click without smudging |
| `Arc<Mutex<T>>` | many keyholders, one marker — the canonical shared-mutable-state shape |
| `Send` / `Sync` | passport (cross threads) and library card (read concurrently) |
| Threads vs async | employees with their own desks vs paper tickets they pass around |
| Future | a recipe card — nothing cooks until a chef picks it up |
| `.await` | the chef pausing for the timer — walks away, comes back when it rings |
| Executor / runtime | Rust ships the cards, you bring the kitchen (Tokio) |
| `join!` / `select!` / `spawn` | all-of / first-of / fire-onto-another-line |
| `Stream` | conveyor belt where each box arrives eventually |
| `Pin` | concrete-and-rebar — keeps self-referential futures from moving |
