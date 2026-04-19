# Part 15 — Closures & Iterators

Two "functional" features of Rust that turn out to be central to idiomatic code. Both are **zero-cost abstractions** — the compiler optimizes them away.

| Feature | One-line picture |
|---------|------------------|
| **Closures** | Anonymous functions that can grab variables from their surrounding scope |
| **Iterators** | Lazy sequences — a pipeline that produces values on demand, one at a time |

Both combine beautifully: you build iterator chains using closures, and Rust compiles the whole thing down to tight, allocation-free loops.

| Concept | Chapter |
|---------|---------|
| Builds on `impl Trait`, trait bounds, lifetimes | [Part 12](12-generics-traits-lifetimes.md) |
| Errors / `Option` inside closures | [Part 11](11-error-handling.md) |

---

## 15.1 — Closures: the basics

A **closure** is an anonymous function written with vertical bars for parameters:

```rust
let add_one = |x| x + 1;
println!("{}", add_one(5));   // 6
```

Type annotations are allowed but usually inferred:

```rust
let add_one_explicit = |x: u32| -> u32 { x + 1 };
```

The compiler infers the type **from the first call**. Once inferred, the type is fixed — calling the same closure with a different type is an error:

```rust
let id = |x| x;
let s = id(String::from("hi"));
let n = id(5);       // ERROR: expected String, found integer
```

Each closure has a **unique anonymous type** — you cannot write it down by hand. You refer to it through a trait (`Fn`, `FnMut`, `FnOnce` — see §15.3).

---

## 15.2 — How closures capture their environment

Closures can use variables from the enclosing scope, which regular functions cannot:

```rust
let list = vec![1, 2, 3];
let only_borrows = || println!("{:?}", list);
only_borrows();          // OK: reads list
println!("{:?}", list);  // OK: list still owned by us
```

Closures capture in one of **three ways**, chosen automatically based on what the body does:

| Capture mode | Triggered when | Trait |
|--------------|----------------|-------|
| **Shared reference** (`&T`) | Closure only reads | `Fn` |
| **Mutable reference** (`&mut T`) | Closure modifies | `FnMut` |
| **Move** (takes ownership) | Closure consumes or the `move` keyword is used | `FnOnce` (and possibly `Fn`/`FnMut` too) |

Example of a mutable capture:

```rust
let mut list = vec![1, 2, 3];
let mut borrows_mutably = || list.push(7);
borrows_mutably();         // list becomes [1, 2, 3, 7]
println!("{:?}", list);    // OK, no outstanding borrow
```

### Forcing ownership with `move`

Use `move` when the closure must outlive the scope where captures were created — typically when spawning a thread:

```rust
use std::thread;

let list = vec![1, 2, 3];
thread::spawn(move || println!("{:?}", list))
    .join()
    .unwrap();
```

Without `move`, the thread would borrow `list`, and the main thread's `list` could go out of scope before the child thread reads it — Rust refuses to compile.

```
┌──────────────────────────────────────────────────────────────────┐
│  `move` is about ownership transfer, not about mutability. A     │
│  `move` closure still obeys Fn/FnMut/FnOnce based on what the    │
│  body does to the captures.                                      │
└──────────────────────────────────────────────────────────────────┘
```

---

## 15.3 — The Fn / FnMut / FnOnce traits

Every closure implements **one or more** of these three traits, depending on what it does with its captures:

| Trait | Body behaviour | Callable |
|-------|----------------|----------|
| `FnOnce` | Moves a captured value out (consumes it) | **At least once** — may be exhausted after first call |
| `FnMut` | Mutates captures but does not move them out | **Many times** — but not in parallel |
| `Fn` | Only reads captures | **Many times, even concurrently** |

The relationship is a hierarchy:

```
Fn   ⊂   FnMut   ⊂   FnOnce
(most restrictive)   (least restrictive)
```

Every `Fn` is also `FnMut` and `FnOnce`; every `FnMut` is also `FnOnce`.

### Choosing the trait bound as a caller

When accepting a closure, ask for the **weakest** trait you need:

```rust
fn call_once<F: FnOnce()>(f: F) { f(); }        // accepts all closures
fn call_many<F: FnMut()>(mut f: F) { f(); f(); } // accepts FnMut + Fn
fn call_concurrent<F: Fn()>(f: F) { f(); f(); }  // accepts only Fn
```

### The `unwrap_or_else` case study

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where F: FnOnce() -> T {
        match self {
            Some(x) => x,
            None    => f(),
        }
    }
}
```

The bound is `FnOnce` — the closure is called at most once (when the value is `None`), so any closure works, including one that consumes its captures.

### Contrast: `sort_by_key`

```rust
impl<T> [T] {
    pub fn sort_by_key<K, F>(&mut self, f: F)
    where F: FnMut(&T) -> K, K: Ord {
        // calls f many times during the sort
    }
}
```

Here `FnMut` is needed because the sort calls `f` repeatedly.

---

## 15.4 — The `Iterator` trait

An iterator is any type that implements this trait:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;

    // ... many default methods built on top of next()
}
```

The only method you must write is `next`. Each call produces the next element wrapped in `Some`, or `None` when the sequence is exhausted.

### Obtaining an iterator over a collection

| Method | Yields | Consumes collection? |
|--------|--------|----------------------|
| `iter()` | `&T` (shared refs) | No |
| `iter_mut()` | `&mut T` (mutable refs) | No |
| `into_iter()` | `T` (owned values) | **Yes — consumes the collection** |

```rust
let v = vec![1, 2, 3];
for n in v.iter()      { /* n: &i32 */ }
for n in v.iter_mut()  { /* n: &mut i32 — but v must be mut */ }
for n in v.into_iter() { /* n: i32, v is gone */ }
```

### Iterators are lazy

Creating an iterator does **nothing**. The computation happens only when something asks for values via `next` (explicitly or through a consumer like `for` or `collect`):

```rust
let v = vec![1, 2, 3];
let it = v.iter().map(|x| x * 10);  // nothing runs here
for n in it { println!("{}", n); }  // now the mapping executes
```

---

## 15.5 — Consuming adapters (drain the iterator)

Methods that call `next` internally until exhaustion are called **consumers**. They take ownership of the iterator and return a final value.

| Method | Result |
|--------|--------|
| `sum()` | Total of all items |
| `product()` | Product of all items |
| `count()` | Number of items |
| `collect::<T>()` | Items packed into collection `T` (often `Vec`, `HashMap`, `String`) |
| `for_each(f)` | Calls `f` on each item, returns `()` |
| `fold(init, f)` | Accumulate with a binary function |
| `reduce(f)` | Like `fold` but uses the first element as init |
| `min() / max()` | Smallest / largest |
| `find(pred)` | First item satisfying `pred`, as `Option` |
| `any(pred)` / `all(pred)` | Boolean short-circuit |

```rust
let total: i32 = vec![1, 2, 3].iter().sum();           // 6
let doubled: Vec<i32> = (1..=3).map(|x| x * 2).collect(); // [2, 4, 6]
```

```
┌──────────────────────────────────────────────────────────────────┐
│  `collect` needs to know the target type: use a turbofish         │
│  `collect::<Vec<_>>()` or an explicit binding `let v: Vec<_> = ..` │
└──────────────────────────────────────────────────────────────────┘
```

---

## 15.6 — Iterator adapters (produce new iterators)

Methods that return **another iterator** are called **adapters**. They are lazy — nothing executes until a consumer pulls.

| Adapter | Purpose |
|---------|---------|
| `map(f)` | Transform each item |
| `filter(pred)` | Keep items where `pred` is true |
| `take(n)` | Keep at most `n` items |
| `skip(n)` | Drop the first `n` items |
| `zip(other)` | Pair items from two iterators |
| `chain(other)` | Concatenate two iterators |
| `enumerate()` | Turn items into `(index, item)` |
| `rev()` | Reverse order (only for `DoubleEndedIterator`) |
| `flat_map(f)` | Map then flatten one level |
| `step_by(n)` | Yield every n-th item |
| `peekable()` | Allow looking at the next item without consuming |

```rust
let v: Vec<i32> = (1..=10)
    .filter(|x| x % 2 == 0)      // 2, 4, 6, 8, 10
    .map(|x| x * x)              // 4, 16, 36, 64, 100
    .take(3)                     // 4, 16, 36
    .collect();
```

The chain is a template for the pipeline. Nothing runs until `collect`.

---

## 15.7 — Closures that capture their environment

Iterator adapters almost always take a closure, and that closure commonly captures values from the surrounding scope:

```rust
#[derive(Debug, PartialEq)]
struct Shoe { size: u32, style: String }

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
         .filter(|s| s.size == shoe_size)     // captures shoe_size
         .collect()
}
```

The closure `|s| s.size == shoe_size` borrows `shoe_size` from the function's scope. `filter` needs `FnMut`, which any non-consuming closure satisfies.

---

## 15.8 — Writing your own iterator

Implement the trait for any struct, and you instantly get all the adapter methods for free:

```rust
struct Counter { count: u32 }

impl Counter {
    fn new() -> Self { Counter { count: 0 } }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// Usage — every Iterator method is now available:
let sum_of_pairs_products: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();
// (1·2) + (2·3) + (3·4) + (4·5) = 2 + 6 + 12 + 20 → keep multiples of 3 → 6 + 12 = 18
```

`Counter::new()` yields 1, 2, 3, 4, 5 then `None`. Everything else is built on top of that single `next` method.

---

## 15.9 — Performance: the zero-cost promise

Iterator chains compile down to the **same machine code** as hand-rolled `for` loops — often identical, sometimes better (the compiler can unroll, inline, and vectorize more aggressively when it sees the whole pipeline).

Official benchmark from the Rust book: an audio decoder written with iterator chains produced the same assembly as the C++ version using manual loops, and ran at the same speed.

The takeaway: **do not hand-roll loops to avoid iterator overhead** — there is none. Prefer the iterator pipeline for clarity; the compiler handles the rest.

```
┌──────────────────────────────────────────────────────────────────┐
│  "Zero-cost abstraction" in Rust means: if you had written the    │
│  machine code by hand, you would not have done better. The high- │
│  level construct and the low-level version compile to the same   │
│  thing.                                                           │
└──────────────────────────────────────────────────────────────────┘
```

---

## 15.10 — Closures vs function items

A regular function can be passed wherever a closure is expected, because every function implements `Fn`, `FnMut`, and `FnOnce`:

```rust
let v: Vec<String> = vec![1, 2, 3].iter().map(ToString::to_string).collect();
// identical to:
let v: Vec<String> = vec![1, 2, 3].iter().map(|x| x.to_string()).collect();
```

Use a function reference when it reads better; use a closure when you need to capture context or do something inline.

---

## 15.11 — Summary

| Concept | Key point |
|---------|-----------|
| **Closure syntax** | `\|args\| expr` — types usually inferred from first call, then fixed |
| **Capture modes** | Shared ref (Fn), mut ref (FnMut), move (FnOnce) — chosen automatically |
| **`move` keyword** | Forces ownership transfer; required for threads |
| **`Fn` / `FnMut` / `FnOnce`** | Trait hierarchy — pick the weakest bound you need |
| **`Iterator` trait** | One required method: `next() -> Option<Item>` — everything else is free |
| **`iter`, `iter_mut`, `into_iter`** | Borrow, borrow mut, consume |
| **Laziness** | Adapters compute nothing until a consumer pulls |
| **Consumers vs adapters** | Consumers return a value; adapters return another iterator |
| **Zero-cost** | Iterator chains compile to the same code as hand-written loops |

> For the plain-language intuition (no code), see [`rust-intuition.md`](./rust-intuition.md) Part 5.

See also: [Part 16 — More About Cargo & Crates.io](16-more-cargo-crates.md) for publishing crates that expose iterator-based APIs.
