# Part 17 — Smart Pointers

A **smart pointer** is a struct that acts like a pointer but carries extra capabilities — heap allocation, reference counting, runtime borrow tracking, custom cleanup. Unlike plain references (`&T`, `&mut T`) which **borrow**, smart pointers usually **own** their data.

| Type | What it gives you | When to reach for it |
|------|-------------------|----------------------|
| `Box<T>` | Owned heap allocation, single owner, known pointer size | Recursive types, trait objects, large values |
| `Rc<T>` | **Shared** ownership, reference-counted | Many parts of the code must read the same value |
| `RefCell<T>` | **Interior mutability** — borrow rules at runtime instead of compile time | Mutating through a shared reference |
| `Weak<T>` | Non-owning reference counted pointer | Breaking `Rc` cycles (parent ↔ child) |

All of these are just **regular types** — the smart-pointer behaviour comes from implementing two traits: `Deref` (act like a pointer) and `Drop` (clean up when dropped).

| Prerequisite | Chapter |
|--------------|---------|
| Traits & generic bounds | [Part 12](12-generics-traits-lifetimes.md) |
| Ownership, borrowing, `&T` / `&mut T` | [Parts 5-6](05-ownership.md) |

---

## 17.1 — `Box<T>`: the simplest smart pointer

`Box<T>` stores a value of type `T` on the **heap** instead of the stack, and gives you a stack-sized pointer to it.

```rust
let b = Box::new(5);   // 5 is on the heap; b is a stack-sized pointer
println!("{}", b);     // 5 — transparent dereference
```

When `b` goes out of scope, both the stack pointer and the heap value are freed — no leak, no double-free.

Three situations where you actually need `Box<T>`:

| Situation | Why `Box` is the answer |
|-----------|-------------------------|
| **Recursive type** (size cannot be known at compile time) | `Box` has a fixed size, so `T` containing `Box<T>` is well-defined |
| **Trait object** (`Box<dyn Trait>`) | Lets you store any type implementing a trait behind one pointer |
| **Large value** (avoid copying on move) | Move a `Box<[u8; 1_000_000]>` = move 8 bytes |

### Recursive types: the cons list

```rust
enum List {
    Cons(i32, List),     // ERROR: recursive type has infinite size
    Nil,
}
```

The compiler cannot compute the size of `List`: it contains another `List`, which contains another, ad infinitum. Fix it by putting the recursion behind a `Box`:

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};
let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
```

Now each `Cons` holds an `i32` + a pointer (fixed size). The list structure lives on the heap.

```
Stack             Heap
─────             ────
list ──►  Cons(1, ●)  ──► Cons(2, ●) ──► Cons(3, ●) ──► Nil
```

---

## 17.2 — `Deref`: making a type behave like a pointer

The `Deref` trait is what makes `*b` work. It has one method, `deref`, returning a reference to the inner value:

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> { MyBox(x) }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

let b = MyBox::new(5);
assert_eq!(*b, 5);   // *b  ≡  *(b.deref())
```

The compiler rewrites `*b` into `*(b.deref())` behind the scenes — that's why `*` works uniformly on `Box`, `Rc`, `Arc`, `MyBox`, and plain references.

### Deref coercion

If a function expects `&str` and you have a `&String`, Rust **automatically** converts: `String: Deref<Target=str>`, so `&String` can coerce to `&str`. The same happens for `&Box<T>` → `&T`.

```rust
fn hello(name: &str) { println!("Hello, {name}!"); }

let m = MyBox::new(String::from("Rust"));
hello(&m);   // &MyBox<String> → &String → &str — two coercions chained
```

Without deref coercion, you would write `hello(&(*m)[..])`. With it, `&m` is enough.

### `DerefMut` for mutable access

`*b = 3;` needs `DerefMut`, which returns `&mut T`. Same shape, but mutable.

```
┌──────────────────────────────────────────────────────────────────┐
│  Deref coercion runs at compile time. There is no runtime cost.  │
│  It only works for &T, &mut T, and types that implement Deref.   │
└──────────────────────────────────────────────────────────────────┘
```

---

## 17.3 — `Drop`: custom cleanup on scope exit

The `Drop` trait lets you run code when a value is about to be destroyed — closing files, releasing locks, freeing a C resource, logging.

```rust
struct CustomSmartPointer { data: String }

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping {}", self.data);
    }
}

fn main() {
    let _a = CustomSmartPointer { data: String::from("a") };
    let _b = CustomSmartPointer { data: String::from("b") };
    println!("end of main");
}
// end of main
// Dropping b     ← reverse order of declaration
// Dropping a
```

`drop` is called **automatically** at the end of the scope. Two rules:

- **You cannot call `drop` manually** — the language forbids it, otherwise the value would be dropped twice (once by you, once by the compiler at scope end).
- To drop something **early**, use the free function `std::mem::drop`:

```rust
let p = CustomSmartPointer { data: String::from("early") };
drop(p);                 // destructor runs here
println!("p is gone");
```

`std::mem::drop` is in the prelude, so you just write `drop(p)`.

---

## 17.4 — `Rc<T>`: shared ownership by reference counting

Sometimes one value needs **multiple owners** — for example, a node in a graph that several edges point to. `Rc<T>` (Reference Counted) lets you clone cheaply: every clone increments a counter, every drop decrements it, and the value is freed only when the count hits zero.

```rust
use std::rc::Rc;

let a = Rc::new(String::from("shared"));
let b = Rc::clone(&a);          // count = 2, no deep copy
let c = Rc::clone(&a);          // count = 3

println!("count = {}", Rc::strong_count(&a));   // 3
```

Each `Rc::clone` just **bumps the counter** — it does **not** deep-copy the underlying data. Use `Rc::clone(&a)` rather than `a.clone()` to make the intent visible at call sites.

### `Rc<T>` only grants shared, **read-only** access

An `Rc<T>` implements `Deref<Target=T>` but not `DerefMut`. You cannot mutate through an `Rc` alone — if two owners could write through their shared pointer, you'd have two mutable references to the same data, which breaks Rust's aliasing rules.

To mutate, combine with `RefCell`: `Rc<RefCell<T>>` (see §17.6).

### `Rc<T>` is single-threaded only

`Rc`'s counter is not atomic — incrementing it from two threads would race. For multi-threaded shared ownership, use **`Arc<T>`** (Atomic Rc) — same API, atomic counter, slightly slower but thread-safe.

---

## 17.5 — `RefCell<T>`: interior mutability

The borrow checker is **static** — it proves at compile time that there is never more than one `&mut T`, and never an `&T` and `&mut T` at the same time. Sometimes you need to break out of that into a **runtime-checked** version — for example to mutate something through an `Rc` shared reference, or inside a type that exposes `&self` methods but needs internal mutation (memoization, caching).

`RefCell<T>` enforces the **same rules**, but at runtime. Violating them is a **panic**, not a compile error.

```rust
use std::cell::RefCell;

let cell = RefCell::new(vec![1, 2, 3]);

{
    let r1 = cell.borrow();        // shared borrow, count = 1
    let r2 = cell.borrow();        // shared borrow, count = 2
    println!("{:?} {:?}", r1, r2);
}  // both dropped

cell.borrow_mut().push(4);         // exclusive borrow
println!("{:?}", cell.borrow());   // [1, 2, 3, 4]
```

The API:

| Method | Returns | Runtime rule |
|--------|---------|--------------|
| `.borrow()` | `Ref<T>` (acts like `&T`) | Many allowed; zero mutable borrows must exist |
| `.borrow_mut()` | `RefMut<T>` (acts like `&mut T`) | Exclusive; zero other borrows must exist |

Violations (`borrow_mut` while a `borrow` is live, for instance) **panic at runtime**. That's the cost of runtime checking.

```
┌──────────────────────────────────────────────────────────────────┐
│  RefCell does not give you TWO mutable borrows — it gives you    │
│  the ability to get a single mutable borrow through a shared     │
│  reference. The aliasing rule still holds; only the place where  │
│  it's checked moves from compile time to runtime.                │
└──────────────────────────────────────────────────────────────────┘
```

### When interior mutability is legitimate

- **Testing with mock objects.** A test double needs to record calls (*mutation*) from within methods exposed as `&self` — `RefCell` in the struct lets you do this without changing the public API.
- **Mutating through shared ownership.** Combine with `Rc` to allow many readers/writers of a shared value (§17.6).
- **Lazy initialization / caching.** Compute once, cache, expose as `&self`.

For thread-safe interior mutability, use `Mutex<T>` or `RwLock<T>` instead of `RefCell<T>`.

---

## 17.6 — Combining `Rc<RefCell<T>>`: shared mutable state

A frequent pattern: you want **multiple owners** AND the ability to **mutate** what they share.

- `Rc<T>` alone: multiple owners, read-only.
- `RefCell<T>` alone: single owner, interior mutability.
- `Rc<RefCell<T>>`: multiple owners, each can call `.borrow_mut()` to mutate.

```rust
use std::rc::Rc;
use std::cell::RefCell;

let shared = Rc::new(RefCell::new(5));

let a = Rc::clone(&shared);
let b = Rc::clone(&shared);

*a.borrow_mut() += 10;
*b.borrow_mut() += 100;

println!("{}", shared.borrow());   // 115
```

Mental layer diagram:

```
 Rc< RefCell< T > >
 │   │         └─ the actual data
 │   └─ runtime-checked borrow rules (interior mutability)
 └─ reference-counted shared ownership
```

Read inside-out: a value `T`, wrapped in a `RefCell` (so it can be mutated through shared refs), wrapped in an `Rc` (so multiple owners exist).

---

## 17.7 — Reference cycles and `Weak<T>`

Reference counting has one failure mode: **cycles**. If `a` holds an `Rc` to `b`, and `b` holds an `Rc` to `a`, neither counter ever hits zero — the pair leaks forever, even after both local variables are gone.

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

use List::{Cons, Nil};

let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

// Make a's tail point back to b → cycle
if let Cons(_, link) = &*a {
    *link.borrow_mut() = Rc::clone(&b);
}
// Leak: a.count = 2, b.count = 2; dropping a and b brings both to 1, never to 0.
```

Rust does **not** detect this at compile time. `Rc` leaks silently — no unsafety, no crash, just memory that will never be reclaimed.

### `Weak<T>`: a non-owning pointer

A `Weak<T>` is an `Rc<T>` that **does not count toward ownership**. It does not keep the value alive. To access the value, call `.upgrade()`, which returns `Option<Rc<T>>` — `Some` if the value still exists, `None` if it has been dropped.

Typical use case: a tree where each child holds a **strong `Rc`** to its parent's children (owning), and the parent holds a **`Weak`** back to each child (non-owning) — or vice versa.

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,       // non-owning upward pointer
    children: RefCell<Vec<Rc<Node>>>,  // owning downward pointers
}

let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
});

let branch = Rc::new(Node {
    value: 5,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![Rc::clone(&leaf)]),
});

*leaf.parent.borrow_mut() = Rc::downgrade(&branch);

// Access the parent (if still alive):
if let Some(parent) = leaf.parent.borrow().upgrade() {
    println!("leaf's parent value = {}", parent.value);   // 5
}
```

The rule: **strong `Rc` goes one way, `Weak` goes the other.** The "strong direction" decides who owns whom.

```
 branch (strong)  ──►  leaf
 leaf   (weak)    ──►  branch          ← does not keep branch alive
```

### Strong vs weak counts

| Function | Counts |
|----------|--------|
| `Rc::strong_count(&rc)` | Number of `Rc` clones — value lives while ≥ 1 |
| `Rc::weak_count(&rc)` | Number of `Weak` pointers — does not keep value alive |

---

## 17.8 — Summary

| Tool | Key shape | When to pick it |
|------|-----------|-----------------|
| `Box<T>` | Owned heap pointer, single owner | Recursive types, trait objects, large values |
| `Deref` trait | Makes a type act like a pointer | Implementing your own smart pointer, enabling `*` and coercion |
| `Drop` trait | Runs custom code at scope exit | File/socket/C-resource cleanup; `drop(x)` to fire early |
| `Rc<T>` | Reference-counted **shared** ownership | Multiple read-only owners, single-threaded |
| `Arc<T>` | Atomic Rc | Multi-threaded shared ownership |
| `RefCell<T>` | Interior mutability, **runtime** borrow rules | Mutate through `&self`; pair with Rc for shared mutable state |
| `Rc<RefCell<T>>` | Shared mutable state | Graph nodes, multi-owner mutable data |
| `Weak<T>` | Non-owning pointer, upgradeable | Breaking parent/child cycles |

### Choosing: a decision tree

```
 Do I need multiple owners of this value?
 ├─ No  → Box<T> (if heap needed), or just stack.
 └─ Yes → Rc<T>  (single-thread)   or   Arc<T>  (multi-thread)
          │
          └─ Do any owner need to mutate?
             ├─ No  → done.
             └─ Yes → Rc<RefCell<T>>  or  Arc<Mutex<T>> / Arc<RwLock<T>>
                     │
                     └─ Could the graph form a cycle?
                        ├─ No  → done.
                        └─ Yes → use Weak<T> on the back-edge.
```

### The underlying idea

Ownership and borrowing are **not just one system**. They are a spectrum:

- `Box<T>` — single owner, compile-time checked, heap-allocated.
- `Rc<T>` — multi-owner, compile-time checked, **reference count** decides lifetime.
- `RefCell<T>` — single-owner, **runtime-checked** borrows, same aliasing rules enforced later.
- Combined types push the check further away from compile time in exchange for flexibility.

Each relaxation trades one static guarantee for one dynamic check. Rust lets you pick the exact trade-off per type.

> For the plain-language intuition (no code), see [`rust-intuition.md`](./rust-intuition.md) Part 7.

See also: [Part 5 — Ownership](05-ownership.md) and [Part 6 — References & Slices](06-references-slices.md) for the foundations these types extend.
