# Part 10 — Common Collections

Unlike arrays and tuples, **collections** store data on the **heap**, so the size doesn't need to be known at compile time and can grow or shrink while the program runs. This chapter covers the three you will reach for most often:

| Collection | Purpose | Generic form |
|------------|---------|--------------|
| **Vector** | A contiguous, growable list of values | `Vec<T>` |
| **String** | A growable, UTF-8 encoded string | `String` |
| **Hash map** | An association of keys to values | `HashMap<K, V>` |

`Vec` and `String` live in the prelude. `HashMap` requires `use std::collections::HashMap;`.

---

## 10.1 — Storing Lists of Values with Vectors

A `Vec<T>` holds any number of values of the **same type** in a single heap-allocated buffer.

### Creating a vector

```rust
// Empty vector — the type annotation is required because Rust can't
// infer T without any values.
let v: Vec<i32> = Vec::new();

// The vec! macro builds a vector from a list of values (type inferred).
let v = vec![1, 2, 3];
```

### Updating a vector

```rust
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

`push` requires `mut` — the vector is modified in place.

### Reading elements — indexing vs. `get`

Rust gives you two ways, and they have **very different failure modes**:

```rust
let v = vec![1, 2, 3, 4, 5];

// Indexing — returns &T directly, panics if out of bounds
let third: &i32 = &v[2];
println!("The third element is {third}");

// .get() — returns Option<&T>, returns None if out of bounds
let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
```

| Access | Out-of-bounds behavior | Use when |
|--------|-----------------------|----------|
| `&v[i]` | **Panic** (program crashes) | You know the index is valid, or want a crash on bug |
| `v.get(i)` | Returns `None` | The index might be invalid and you want to handle it |

### Ownership: can't have a reference and a push

This seems innocent but doesn't compile:

```rust
let mut v = vec![1, 2, 3, 4, 5];
let first = &v[0];       // immutable borrow

v.push(6);               // ERROR: needs mutable borrow

println!("The first element is: {first}");
```

**Why?** `push` may need to reallocate the buffer if there isn't enough capacity. That would move the elements in memory, leaving `first` dangling. The borrow checker prevents this class of bug at compile time.

### Iterating

```rust
let v = vec![100, 32, 57];
for n in &v {
    println!("{n}");
}

// Mutating in place — iterate over &mut and use the dereference operator
let mut v = vec![100, 32, 57];
for n in &mut v {
    *n += 50;
}
```

### Using an enum to store multiple "types"

A `Vec<T>` holds one type, but that type can be an **enum** — letting you store variants that carry different inner data:

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

### Dropping a vector

When a vector goes out of scope, it's dropped — and all its contents are dropped along with it. The borrow checker makes sure nothing still references the freed memory.

### A concrete pattern: `Vec<Vec<T>>`

The Game of Life uses a 2D grid stored as a vector of vectors:

```rust
use rand::Rng;

#[derive(Clone, Copy)]
enum Cells { Alive, Dead }

const GRID_HEIGHT: u32 = 60;
const GRID_WIDTH: u32 = 100;

let mut rng = rand::thread_rng();
let grid: Vec<Vec<Cells>> = (0..GRID_HEIGHT)
    .map(|_| {
        (0..GRID_WIDTH)
            .map(|_| match rng.gen_range(0..2) {
                0 => Cells::Dead,
                _ => Cells::Alive,
            })
            .collect()
    })
    .collect();
```

The pattern `(range).map(...).collect()` is the idiomatic way to build a `Vec`. Each inner `collect()` builds a `Vec<Cells>` (a row); the outer one builds a `Vec<Vec<Cells>>` (the grid).

Indexing then works like a 2D array:

```rust
let height = grid.len();            // number of rows
let width  = grid[0].len();         // cells per row
let top_left = grid[0][0];          // Cells — Copy so this is fine
```

---

## 10.2 — Storing UTF-8 Encoded Text with Strings

Rust has two main string types, and the distinction matters:

| Type | Owned? | Grows? | Lives where |
|------|--------|--------|-------------|
| `String` | Yes | Yes | Heap |
| `&str` | No (borrowed) | No | Anywhere — often a slice of a `String` or a string literal |

String literals like `"hello"` are `&'static str` — they're baked into the binary.

### Creating strings

```rust
let mut s = String::new();                  // empty

let s = "initial contents".to_string();     // from &str
let s = String::from("initial contents");   // same thing, different spelling
```

### Appending

```rust
let mut s = String::from("foo");
s.push_str("bar");          // append a &str — doesn't take ownership
s.push('!');                // append a single char
// s is now "foobar!"
```

`push_str` takes `&str`, so passing a `&String` works too (deref coercion).

### Concatenation with `+`

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;          // s1 is MOVED, s2 is borrowed
// s1 no longer usable; s3 == "Hello, world!"
```

The `+` operator calls `fn add(self, s: &str) -> String` — it takes ownership of the left side and borrows the right.

### `format!` — cleanest for multiple parts

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");
// s is "tic-tac-toe", and s1/s2/s3 are all still usable
```

### Why you can't index a String

```rust
let s1 = String::from("hello");
let h = s1[0];   // ERROR: String doesn't implement Index<usize>
```

Rust **forbids indexing strings by byte offset** because it would give wrong results for anything beyond ASCII.

```
┌──────────────────────────────────────────────────────────────────┐
│  A String is a Vec<u8> of UTF-8 bytes. Many characters take      │
│  more than one byte — `s[0]` would hand you half of a character. │
│                                                                  │
│  Rust refuses to pretend byte 0 = character 0, because for a     │
│  lot of languages (Arabic, Hindi, CJK…) it simply isn't true.    │
└──────────────────────────────────────────────────────────────────┘
```

Example: `"नमस्ते"` is 6 chars to a reader, but **18 bytes** and **4 grapheme clusters**.

### Slicing — by byte range (careful!)

```rust
let hello = "Здравствуйте";        // Cyrillic; each letter is 2 bytes
let s = &hello[0..4];              // "Зд" — 4 bytes = 2 letters
// let s = &hello[0..1];           // PANIC at runtime (not a char boundary)
```

Use slicing only when you're certain about byte boundaries.

### Iterating — the right way

```rust
// Iterate over Unicode scalar values (chars):
for c in "Зд".chars() {
    println!("{c}");
}
// Prints: З  then  д

// Iterate over raw bytes:
for b in "Зд".bytes() {
    println!("{b}");
}
// Prints: 208, 151, 208, 180
```

Grapheme clusters (what humans call "characters") need a crate — not in `std`.

---

## 10.3 — Storing Keys with Associated Values in Hash Maps

`HashMap<K, V>` maps keys of type `K` to values of type `V`. Internally it uses hashing, so lookups are O(1) on average.

```rust
use std::collections::HashMap;
```

### Creating and inserting

```rust
let mut scores = HashMap::new();

scores.insert(String::from("Blue"),   10);
scores.insert(String::from("Yellow"), 50);
```

All keys must be the same type; all values must be the same type. Rust infers them from the first `insert`.

### Accessing values

```rust
let team = String::from("Blue");
let score: i32 = scores.get(&team).copied().unwrap_or(0);
```

Breakdown:

- `.get(&team)` returns `Option<&i32>` — `None` if the key doesn't exist.
- `.copied()` turns `Option<&i32>` into `Option<i32>` (copies the value out).
- `.unwrap_or(0)` returns the inner value or `0` if `None`.

### Iterating

```rust
for (key, value) in &scores {
    println!("{key}: {value}");
}
```

Iteration order is **arbitrary** — don't rely on it.

### Hash maps and ownership

- For `Copy` types (e.g., `i32`): values are **copied** into the map.
- For owned types (e.g., `String`): values are **moved** into the map.

```rust
let field_name  = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are INVALID here — moved into the map
```

If you insert a reference, the referenced value must live at least as long as the map.

### Updating a hash map

There are three things you might want to do when `insert`ing a key that already exists.

#### 1. Overwrite

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);   // replaces 10 with 25
```

#### 2. Insert only if absent — the `entry` API

```rust
scores.entry(String::from("Yellow")).or_insert(50);   // inserted
scores.entry(String::from("Blue"  )).or_insert(50);   // Blue already exists → unchanged
```

`entry(k).or_insert(v)` returns a **mutable reference** to the value (whether it was just inserted or already existed).

#### 3. Update based on the old value

A classic word-count:

```rust
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;              // count is &mut i32, so we deref to update
}

println!("{map:?}");
// {"hello": 1, "world": 2, "wonderful": 1}   (order may vary)
```

```
┌──────────────────────────────────────────────────────────────────┐
│  map.entry(k).or_insert(v)  →  returns &mut V                    │
│  Use this trio when the update depends on whether the key        │
│  already exists or on its current value. It's one lookup, not    │
│  a contains_key + get + insert dance.                            │
└──────────────────────────────────────────────────────────────────┘
```

### Hashing

The default hashing function is **SipHash**, chosen for DoS resistance, not raw speed. If profiling shows hashing is a hot spot, you can swap in a faster hasher (e.g., `rustc-hash`, `ahash`) by specifying a different type for the map's hasher — `HashMap` is generic over its hasher.

---

## 10.4 — Choosing the Right Collection

| You want to... | Use |
|----------------|-----|
| A sequential list of same-typed values | `Vec<T>` |
| A growable UTF-8 string | `String` |
| A borrowed view of a string | `&str` |
| Fast key → value lookup, unordered | `HashMap<K, V>` |
| Key → value lookup with **sorted** keys | `BTreeMap<K, V>` |
| A set with fast membership tests | `HashSet<T>` / `BTreeSet<T>` |
| A double-ended queue | `VecDeque<T>` |

The standard library's [`std::collections`](https://doc.rust-lang.org/std/collections/) module documents all of them.

---

## 10.5 — Summary

| Collection | Create | Read | Insert | Key points |
|------------|--------|------|--------|-----------|
| **`Vec<T>`** | `Vec::new()` or `vec![...]` | `&v[i]` (panics) or `v.get(i)` (Option) | `v.push(x)` | One type per vec; use an enum to mix |
| **`String`** | `String::new()`, `String::from(...)`, `"...".to_string()` | `.chars()`, `.bytes()`, slice by bytes | `push_str`, `push`, `+`, `format!` | UTF-8; **no** `s[i]` indexing |
| **`HashMap<K, V>`** | `HashMap::new()` | `.get(&k)` → `Option<&V>` | `insert`, `entry(k).or_insert(v)` | Unordered; owned values are moved in |

```
┌──────────────────────────────────────────────────────────────────┐
│  Vec, String, and HashMap cover ~80% of day-to-day data needs.   │
│  They are heap-allocated, growable, and play nicely with the     │
│  ownership rules you learned in chapters 5 and 6.                │
└──────────────────────────────────────────────────────────────────┘
```
