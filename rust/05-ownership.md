# Part 5 — Ownership: The Heart of Rust

Ownership is **THE** concept that sets Rust apart from every other language. Understanding this part is essential for everything that follows.

---

## 5.1 — Why Ownership?

Every program must manage memory. Three approaches exist:

| Approach | Languages | Advantage | Disadvantage |
|----------|----------|-----------|-------------|
| **Garbage Collector** | Java, Python, Go, JS | No need to think about it | GC pauses, runtime overhead |
| **Manual** | C, C++ | Total control, zero overhead | Segfaults, leaks, double free, use-after-free |
| **Ownership** | **Rust** | Zero overhead, zero memory bugs | Learning curve |

### Analogy: the property deed

Imagine every value in Rust is a **house**:

```
┌─────────────────────────────────────────────────────────────┐
│  Each house has exactly ONE owner                           │
│  If the owner moves away, they sell the house (move)        │
│  If the owner dies (goes out of scope), the house           │
│    is demolished automatically (drop)                       │
│  They can lend the keys to someone (borrow / reference)     │
│    but the house remains theirs                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 5.2 — Stack vs Heap

These are two regions of memory available at runtime, but they work very differently.

### The Stack

```
The stack works like a pile of plates:
  - You place a plate on top (push)
  - You remove the one on top (pop)
  - You can NOT remove a plate from the middle

  ┌───────────────┐
  │  z = true     │  ← top (last added, first removed)
  ├───────────────┤
  │  y = 3.14     │
  ├───────────────┤
  │  x = 42       │
  └───────────────┘
```

| Property | Value |
|----------|-------|
| Structure | LIFO (Last In, First Out) |
| Data size | **Fixed and known** at compile time |
| Speed | **Very fast** (just move a pointer) |
| Allocation | Automatic (no searching for space) |

### The Heap

```
The heap works like a restaurant:
  1. You arrive and say "table for 5"
  2. The host finds a free table that's big enough
  3. They give you the table's address (pointer)
  4. Late-arriving friends ask "where are you?" → follow the pointer

  HEAP (free memory, fragmented)
  ┌─────┐  ┌──────────────┐  ┌────┐
  │free │  │ "hello world"│  │free│
  └─────┘  └──────────────┘  └────┘
               ^
               │
  STACK        │
  ┌──────────┐ │
  │ ptr ─────┼─┘
  │ len = 11 │
  └──────────┘
```

| Property | Value |
|----------|-------|
| Structure | Unordered, fragmented |
| Data size | **Variable**, known at runtime |
| Speed | **Slower** (searching for space + following pointers) |
| Allocation | The allocator finds a free block large enough |

### Why does this matter?

Ownership exists to manage the **heap**. Memory problems come from the heap:
- Which parts of code use which heap data?
- How to minimize duplicated data?
- How to free unused memory?

---

## 5.3 — The 3 Rules of Ownership

```
┌──────────────────────────────────────────────────────────┐
│  1. Each value has ONE owner                             │
│  2. There can only be ONE owner at a time                │
│  3. When the owner goes out of scope,                    │
│     the value is destroyed (dropped)                     │
└──────────────────────────────────────────────────────────┘
```

The entire system rests on these three rules. The compiler checks them at compile time. If a rule is violated, the program won't compile.

---

## 5.4 — Variable Scope

A **scope** is the region of the program where a variable is valid:

```rust
fn main() {
    {                        // s doesn't exist yet
        let s = "hello";    // s is valid from this point
        println!("{s}");     // s is usable
    }                        // scope ends → s is no longer valid
    // println!("{s}");      // ERROR: s no longer exists
}
```

When a variable goes out of scope, Rust automatically calls the **`drop`** function to release its resources.

---

## 5.5 — The `String` Type

To illustrate ownership, we need a heap-allocated type. `String` is perfect.

### String literal vs `String`

| | String literal (`&str`) | `String` |
|-|------------------------|----------|
| Storage | Directly in the binary | Heap |
| Mutable? | No | Yes |
| Size | Known at compile time | Variable at runtime |
| Example | `"hello"` | `String::from("hello")` |

### Creating and modifying a `String`

```rust
let mut s = String::from("hello");
s.push_str(", world!");
println!("{s}");  // "hello, world!"
```

### In memory

```
  STACK                           HEAP
  ┌──────────────┐               ┌───┬───┬───┬───┬───┐
  │ s             │               │ h │ e │ l │ l │ o │
  │ ┌──────────┐ │               └───┴───┴───┴───┴───┘
  │ │ ptr ─────┼─┼──────────────→  index 0 1 2 3 4
  │ │ len = 5  │ │
  │ │ cap = 5  │ │
  │ └──────────┘ │
  └──────────────┘
```

| Field | Role |
|-------|------|
| **ptr** | Pointer to the data on the heap |
| **len** | Number of bytes currently in use |
| **cap** | Total capacity allocated (can be > len) |

### Allocation and deallocation

```rust
{
    let s = String::from("hello");  // 1. String::from requests memory from the heap
    // use s...
}                                    // 2. Rust calls drop(s) → memory freed
```

In C/C++, you must call `free()` or `delete`. In Rust, it's **automatic** at the end of the scope. This pattern is called **RAII** (Resource Acquisition Is Initialization).

---

## 5.6 — Move Semantics

### Simple case: integers (Copy)

```rust
let x = 5;
let y = x;    // Copies the value 5
println!("x = {x}, y = {y}");  // OK: both x and y are valid
```

Integers live on the stack with known size. Copying is trivial and fast.

```
  STACK
  ┌──────────┐
  │ x = 5    │  ← still valid
  │ y = 5    │  ← independent copy
  └──────────┘
```

### Complex case: String (Move)

```rust
let s1 = String::from("hello");
let s2 = s1;
```

What happens in memory?

```
  Step 1: let s1 = String::from("hello");

  STACK                           HEAP
  ┌──────────────┐               ┌───┬───┬───┬───┬───┐
  │ s1            │               │ h │ e │ l │ l │ o │
  │ ┌──────────┐ │               └───┴───┴───┴───┴───┘
  │ │ ptr ─────┼─┼──────────────→
  │ │ len = 5  │ │
  │ │ cap = 5  │ │
  │ └──────────┘ │
  └──────────────┘


  Step 2: let s2 = s1;

  STACK                           HEAP
  ┌──────────────┐
  │ s1 (INVALID) │               ┌───┬───┬───┬───┬───┐
  │ ┌──────────┐ │               │ h │ e │ l │ l │ o │
  │ │ ptr ─ ─ ─│ │               └───┴───┴───┴───┴───┘
  │ │ len = 5  │ │                         ^
  │ │ cap = 5  │ │                         │
  │ └──────────┘ │                         │
  ├──────────────┤                         │
  │ s2 (VALID)   │                         │
  │ ┌──────────┐ │                         │
  │ │ ptr ─────┼─┼─────────────────────────┘
  │ │ len = 5  │ │
  │ │ cap = 5  │ │
  │ └──────────┘ │
  └──────────────┘
```

Rust copies the 3 fields (ptr, len, cap) on the stack, but does **NOT copy** the heap data. Then it **invalidates s1**.

### Why invalidate s1?

If both `s1` and `s2` pointed to the same heap, at end of scope:

```
  } ← s2 goes out of scope → drop(s2) → frees the heap
    ← s1 goes out of scope → drop(s1) → frees the SAME heap!

  = DOUBLE FREE = memory bug = corruption = vulnerability
```

To prevent this, Rust forbids using `s1` after the move.

### Using s1 after a move = error

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{s1}, world!");  // ERROR!
```

```
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:16
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`,
  |            which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
5 |     println!("{s1}, world!");
  |                ^^ value borrowed here after move
```

### Move vs Shallow Copy

| | Shallow Copy (other languages) | Move (Rust) |
|-|-------------------------------|-------------|
| Copies stack metadata? | Yes | Yes |
| Copies heap data? | No | No |
| Original remains valid? | **Yes** | **No** (invalidated) |
| Risk of double free? | **Yes** | **No** |

### Fundamental design rule

```
┌────────────────────────────────────────────────────────────┐
│  Rust will NEVER automatically deep-copy your data.        │
│  Any automatic copy can be assumed to be cheap              │
│  in terms of runtime performance.                          │
└────────────────────────────────────────────────────────────┘
```

---

## 5.7 — Clone: Deep Copy

If you **do** want to copy the heap data, use `clone()`:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {s1}, s2 = {s2}");  // OK: both are valid
```

```
  STACK                           HEAP
  ┌──────────────┐               ┌───┬───┬───┬───┬───┐
  │ s1            │               │ h │ e │ l │ l │ o │  ← s1's data
  │ ┌──────────┐ │               └───┴───┴───┴───┴───┘
  │ │ ptr ─────┼─┼──────────────→
  │ │ len = 5  │ │
  │ │ cap = 5  │ │               ┌───┬───┬───┬───┬───┐
  │ └──────────┘ │               │ h │ e │ l │ l │ o │  ← s2's data (copy)
  ├──────────────┤               └───┴───┴───┴───┴───┘
  │ s2            │                         ^
  │ ┌──────────┐ │                         │
  │ │ ptr ─────┼─┼─────────────────────────┘
  │ │ len = 5  │ │
  │ │ cap = 5  │ │
  │ └──────────┘ │
  └──────────────┘
```

Two independent heap allocations. Modifying `s2` doesn't affect `s1`.

`clone()` is a **visual indicator** in the code: "heads up, potentially expensive operation here."

---

## 5.8 — The `Copy` Trait (Stack-Only Data)

Types whose size is known at compile time and live entirely on the stack implement the **`Copy`** trait:

```rust
let x = 5;
let y = x;
println!("x = {x}, y = {y}");  // OK! No move.
```

For these types, deep copy = shallow copy (there's no heap). The copy is trivial and instant.

### Types that implement `Copy`

| Type | Examples |
|------|---------|
| All integers | `i8`, `u8`, `i32`, `u32`, `i64`, `u64`, `isize`, `usize`... |
| All floats | `f32`, `f64` |
| Booleans | `bool` |
| Characters | `char` |
| Tuples of `Copy` types | `(i32, i32)` ✓, `(i32, String)` ✗ |

### Rule: `Copy` and `Drop` are mutually exclusive

A type cannot implement `Copy` if it (or any of its parts) implements `Drop`. This makes sense: if a type needs special cleanup (`drop`), trivially copying it would be dangerous.

```
String type:
  - Contains a pointer to the heap
  - Implements Drop (to free the heap)
  - CANNOT implement Copy
  → Assignment = Move

i32 type:
  - Lives entirely on the stack
  - No Drop needed
  - Implements Copy
  → Assignment = Trivial copy
```

---

## 5.9 — Ownership and Functions

Passing a value to a function follows the same rules as assignment: **move or copy**.

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s is MOVED into the function
                                    // s is no longer valid here!

    // println!("{s}");             // ERROR: s was moved

    let x = 5;                     // x comes into scope

    makes_copy(x);                 // x is COPIED (i32 = Copy)
                                    // x is still valid!

    println!("{x}");               // OK: x = 5
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}   // some_string goes out of scope → drop() → heap memory freed

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}   // some_integer goes out of scope → nothing special (stack)
```

### Ownership diagram

```
main()                          takes_ownership()
┌─────────────────┐
│ s = String("hello") ──MOVE──→ some_string = String("hello")
│                  │             │
│ (s INVALID)      │             │ println! → "hello"
│                  │             │
│                  │             └─ } drop(some_string) → heap freed
│                  │
│ x = 5 ─────COPY──→ makes_copy()
│                  │   some_integer = 5
│ (x STILL 5)     │   │
│                  │   └─ } some_integer popped off stack
│                  │
│ println!("{x}")  │
└──────────────────┘
```

---

## 5.10 — Return Values and Scope

Returning from a function **transfers ownership** to the caller:

```rust
fn main() {
    let s1 = gives_ownership();         // move from function into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 moves into the function
                                         // return value moves into s3
}
// s3 → drop (heap freed)
// s2 → already moved, nothing to do
// s1 → drop (heap freed)

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string    // move to the caller
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string       // move to the caller
}
```

### The problem: it's tedious

If you want to use a value in a function without losing ownership, you have to return it:

```rust
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);  // s1 moves, comes back in s2
    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)    // return the String + the length
}
```

That's a lot of ceremony just to "read the length of a string." There must be a better way...

### The solution: references

The next part introduces **references** (`&`): a way to use a value without taking ownership. This is the key to writing ergonomic Rust.

---

## 5.11 — Summary

| Concept | Description | Example |
|---------|------------|---------|
| **Stack** | LIFO, fixed size, fast | `i32`, `bool`, `char` |
| **Heap** | Dynamic, allocator, pointer | `String`, `Vec` |
| **Owner** | Each value has ONE owner | `let s = String::from("hi")` |
| **Scope** | Owner goes out of scope → auto `drop()` | `{ let s = ...; }` |
| **Move** | Ownership transfer, original invalidated | `let s2 = s1;` (s1 invalid) |
| **Clone** | Explicit deep copy (expensive) | `let s2 = s1.clone();` |
| **Copy** | Trivial copy for stack-only types | `let y = x;` (i32) |
| **Functions** | Params = move or copy, return = move | `fn f(s: String)` moves s |

```
┌──────────────────────────────────────────────────────────┐
│  Ownership guarantees:                                   │
│  ✓ No double free                                        │
│  ✓ No use-after-free                                     │
│  ✓ No memory leaks (in most cases)                       │
│  ✓ ZERO runtime cost (everything checked at compile time)│
└──────────────────────────────────────────────────────────┘
```
