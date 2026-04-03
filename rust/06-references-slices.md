# Part 6 — References, Borrowing & Slices

## 6.1 — The Problem

We saw in Part 5 that passing a `String` to a function **transfers ownership**. To keep using the value, you have to return it:

```rust
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)  // return s so we don't lose it
}
```

That's tedious. For every function that "looks at" some data, you'd need a round-trip of ownership. The solution: **references**.

---

## 6.2 — References & Borrowing

### Analogy: borrowing a book

Your friend owns a book. You want to read it, but you don't need to keep it. They **lend** it to you (borrow). You can read it, but:
- The book remains **their property**
- You must **give it back**
- You can't **rip out pages** (immutable reference)

### Syntax

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);   // &s1 = reference to s1

    println!("The length of '{s1}' is {len}.");  // s1 still valid!
}

fn calculate_length(s: &String) -> usize {  // s is a reference
    s.len()
}   // s goes out of scope, but since it's NOT the owner, nothing is dropped
```

The `&` creates a **reference**: a pointer to the value, without becoming its owner.

### In memory

```
  calculate_length          main                       HEAP
  ┌──────────────┐         ┌──────────────┐           ┌───┬───┬───┬───┬───┐
  │ s             │         │ s1            │           │ h │ e │ l │ l │ o │
  │ ┌──────────┐ │         │ ┌──────────┐ │           └───┴───┴───┴───┴───┘
  │ │ ptr ─────┼─┼────────>│ │ ptr ─────┼─┼──────────→
  │ └──────────┘ │         │ │ len = 5  │ │
  └──────────────┘         │ │ cap = 5  │ │
  (reference to s1)        │ └──────────┘ │
                           └──────────────┘
                           (owns the data)
```

`s` is a reference to `s1`, not to the heap directly. It's conceptually a **pointer to a pointer**.

### Borrowing = creating a reference

The act of creating a reference is called **borrowing**. When the reference goes out of scope, the borrowed value is NOT freed (not the owner).

---

## 6.3 — Immutable References (default)

By default, references are **immutable**. You can read, but not modify:

```rust
fn change(some_string: &String) {
    some_string.push_str(", world");  // ERROR!
}
```

```
error[E0596]: cannot borrow `*some_string` as mutable,
              as it is behind a `&` reference
 --> src/main.rs:8:5
  |
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^ `some_string` is a `&` reference,
  |                 so the data it refers to cannot be borrowed as mutable
```

---

## 6.4 — Mutable References

To modify borrowed data, you need a **mutable reference**:

```rust
fn main() {
    let mut s = String::from("hello");  // 1. The variable MUST be mut
    change(&mut s);                      // 2. Create a &mut reference
    println!("{s}");                     // "hello, world"
}

fn change(some_string: &mut String) {    // 3. The parameter is &mut String
    some_string.push_str(", world");
}
```

All three ingredients are required:
1. `let mut s` — the variable is mutable
2. `&mut s` — we create a mutable reference
3. `&mut String` — the function accepts a mutable reference

### Restriction: only ONE mutable reference at a time

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;  // ERROR!

println!("{r1}, {r2}");
```

```
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
7 |     println!("{r1}, {r2}");
  |                -- first borrow later used here
```

### Why this restriction?

To prevent **data races** at compile time:

```
Data race = 3 simultaneous conditions:
  1. Two pointers access the same data at the same time
  2. At least one of them is writing
  3. No synchronization mechanism

Other languages: detected at runtime (expensive, sometimes missed)
Rust: prevented at compile time (zero cost, never missed)
```

### Workaround: use scopes

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
    // use r1...
}   // r1 goes out of scope here

let r2 = &mut s;  // OK! r1 no longer exists
```

---

## 6.5 — Mixing Mutable and Immutable References

You cannot have a mutable reference while immutable references exist:

```rust
let mut s = String::from("hello");

let r1 = &s;      // OK
let r2 = &s;      // OK (multiple & = OK)
let r3 = &mut s;  // ERROR!

println!("{r1}, {r2}, and {r3}");
```

```
error[E0502]: cannot borrow `s` as mutable because it is also
              borrowed as immutable
```

**Logic**: holders of immutable references don't expect the value to suddenly change under their feet.

### Summary table

| Situation | Allowed? |
|-----------|---------|
| Multiple `&` (immutable) | **Yes** |
| A single `&mut` (mutable) | **Yes** |
| `&` + `&mut` simultaneously | **No** |
| Multiple `&mut` | **No** |

### NLL: Non-Lexical Lifetimes

A reference's scope goes from its creation to its **last usage** (not until the end of the block):

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
println!("{r1} and {r2}");
// r1 and r2 are no longer used after this line
// → their scope ends HERE

let r3 = &mut s;   // OK! r1 and r2 are "dead"
println!("{r3}");
```

The compiler is smart enough to detect that `r1` and `r2` are no longer used.

---

## 6.6 — Dangling References

A **dangling pointer** is a pointer to freed memory. Rust prevents them at compile time:

```rust
fn dangle() -> &String {          // returns a reference
    let s = String::from("hello"); // s is created inside the function
    &s                              // return a reference to s
}   // s goes out of scope → drop(s) → memory freed
    // The reference now points to FREED memory!
```

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value,
          but there is no value for it to be borrowed from
```

```
Timeline:
  1. s is created          → heap allocated
  2. &s is returned        → reference to the heap
  3. s goes out of scope   → drop(s) → heap FREED
  4. The returned reference → points to NOTHING

  ┌─────────┐         ┌───────────┐
  │ &s ─────┼────────→│ ???????? │  ← freed memory!
  └─────────┘         └───────────┘
```

### Solution: return the value (move ownership)

```rust
fn no_dangle() -> String {         // returns a String (not a reference)
    let s = String::from("hello");
    s                               // ownership transferred to the caller
}   // s is moved, not dropped → no problem
```

---

## 6.7 — The 2 Rules of References

```
┌──────────────────────────────────────────────────────────────┐
│  1. At any given time: EITHER one &mut, OR many &,           │
│     but NOT both                                             │
│  2. References must ALWAYS be valid                          │
│     (no dangling)                                            │
└──────────────────────────────────────────────────────────────┘
```

These rules are checked at compile time. They guarantee the absence of data races and dangling pointers, with zero runtime cost.

---

## 6.8 — The Problem That Leads to Slices

Imagine a function that returns the index of the end of the first word:

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

The problem: the returned index is **not tied** to the source String.

```rust
let mut s = String::from("hello world");
let word = first_word(&s);  // word = 5

s.clear();  // s is now ""

// word is still 5, but s is empty!
// word is now INVALID, and the compiler can't detect it.
```

The index `word` and the String `s` can become desynchronized without the compiler noticing. It's a bug waiting to happen.

---

## 6.9 — String Slices (`&str`)

A **string slice** is a reference to a **portion** of a `String`:

```rust
let s = String::from("hello world");

let hello = &s[0..5];    // "hello"
let world = &s[6..11];   // "world"
```

### Range syntax

| Syntax | Equivalent | Result |
|--------|-----------|--------|
| `&s[0..5]` | — | From the start to index 4 inclusive |
| `&s[..5]` | `&s[0..5]` | Same (implicit start) |
| `&s[6..]` | `&s[6..len]` | From index 6 to the end |
| `&s[..]` | `&s[0..len]` | The entire String |

### In memory

```
  s (String):                      HEAP:
  ┌──────────────┐                ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
  │ ptr ─────────┼───────────────→│ h │ e │ l │ l │ o │   │ w │ o │ r │ l │ d │
  │ len = 11     │                └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
  │ cap = 11     │                  0   1   2   3   4   5   6   7   8   9  10
  └──────────────┘                  ^                       ^
                                    │                       │
  hello (&str):                     │   world (&str):       │
  ┌──────────────┐                  │   ┌──────────────┐    │
  │ ptr ─────────┼──────────────────┘   │ ptr ─────────┼────┘
  │ len = 5      │                      │ len = 5      │
  └──────────────┘                      └──────────────┘
```

A slice is made of:
- **ptr**: pointer to the first byte of the portion
- **len**: number of bytes in the portion

It does NOT contain `cap` (it doesn't own the memory, just a view into it).

### Rewriting `first_word` with slices

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];    // slice from start to the first space
        }
    }

    &s[..]  // no space found: the entire word
}
```

Now the return value is **tied** to the source String. The compiler can detect bugs:

```rust
let mut s = String::from("hello world");
let word = first_word(&s);   // word = &str, immutable reference to s

s.clear();  // ERROR! s.clear() needs a &mut, but word (&) still exists

println!("the first word is: {word}");
```

```
error[E0502]: cannot borrow `s` as mutable because it is also
              borrowed as immutable
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
18 |     s.clear();
   |     ^^^^^^^^^ mutable borrow occurs here
20 |     println!("the first word is: {word}");
   |                                   ---- immutable borrow later used here
```

The compiler caught the desynchronization that was invisible in the `usize` version. **An entire class of bugs eliminated at compile time.**

---

## 6.10 — String Literals Are Slices

```rust
let s = "Hello, world!";
```

The type of `s` is `&str`: it's a **slice** pointing to a specific location in the compiled binary. That's why string literals are **immutable**: `&str` is an immutable reference.

```
  STACK                    BINARY (.rodata section)
  ┌──────────────┐        ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
  │ s             │        │ H │ e │ l │ l │ o │ , │   │ w │ o │ r │ l │ d │ ! │
  │ ┌──────────┐ │        └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
  │ │ ptr ─────┼─┼──────→
  │ │ len = 13 │ │
  │ └──────────┘ │
  └──────────────┘
```

---

## 6.11 — `&str` as Parameter (best practice)

Instead of accepting `&String`, prefer `&str`:

```rust
// Good: accepts both &String and &str
fn first_word(s: &str) -> &str { ... }

// Less good: only accepts &String
fn first_word(s: &String) -> &str { ... }
```

With `&str` as the parameter, the function accepts all of these:

```rust
let my_string = String::from("hello world");

// All of these work:
first_word(&my_string[0..6]);   // partial slice
first_word(&my_string[..]);     // full slice
first_word(&my_string);         // reference to String (deref coercion)

let my_literal = "hello world";

first_word(&my_literal[0..6]);  // slice of a literal
first_word(my_literal);         // the literal IS already a &str
```

This works thanks to **deref coercions**: Rust automatically converts `&String` to `&str`.

---

## 6.12 — Other Slices

The slice concept isn't limited to strings. You can slice arrays too:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];  // type: &[i32]

assert_eq!(slice, &[2, 3]);
```

The type `&[i32]` works exactly like `&str`: a pointer + a length. This pattern is ubiquitous in Rust.

```
  a: [i32; 5]                    STACK (full array)
  ┌─────┬─────┬─────┬─────┬─────┐
  │  1  │  2  │  3  │  4  │  5  │
  └─────┴─────┴─────┴─────┴─────┘
    [0]   [1]   [2]   [3]   [4]
           ^
           │
  slice: &[i32]
  ┌──────────────┐
  │ ptr ─────────┼──→ points to a[1]
  │ len = 2      │
  └──────────────┘
  // slice = &[2, 3]
```

---

## 6.13 — Summary: Ownership + Borrowing + Slices

These three concepts form Rust's **memory safety** system:

```
                    OWNERSHIP
                    (property)
                   /          \
                  /            \
          BORROWING           SLICES
          (lending)        (partial views)
          &  &mut            &str  &[T]
```

| Concept | Guarantee |
|---------|----------|
| **Ownership** | No double free, automatic deallocation |
| **Borrowing** | No data races (& vs &mut are exclusive) |
| **Slices** | References tied to source data (no desynchronization) |
| **Lifetimes** (implicit here) | No dangling references |

### Complete reference table

| Syntax | Name | Owns the data? | Can modify? |
|--------|------|---------------|------------|
| `String` | Owned string | **Yes** | If `mut` |
| `&String` | Immutable reference | No | **No** |
| `&mut String` | Mutable reference | No | **Yes** |
| `&str` | String slice | No | **No** |
| `&mut str` | Mutable string slice | No | **Yes** (rare) |
| `[i32; 5]` | Owned array | **Yes** | If `mut` |
| `&[i32]` | Array slice | No | **No** |
| `&mut [i32]` | Mutable array slice | No | **Yes** |

```
┌────────────────────────────────────────────────────────────────┐
│  Everything is checked at COMPILE TIME.                        │
│  Zero cost at RUNTIME.                                         │
│  That's Rust's promise: memory safety without overhead.        │
└────────────────────────────────────────────────────────────────┘
```
