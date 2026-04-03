# Part 3 — Variables, Mutability & Data Types

## 3.1 — Variables & Immutability by Default

Imagine a **labeled box**. In Rust, when you create a variable, the box is **sealed** by default: you can read what's inside, but you can't change the contents.

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;  // ERROR!
    println!("The value of x is: {x}");
}
```

The compiler refuses:

```
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++
```

### Why immutable by default?

| Reason | Explanation |
|--------|------------|
| **Safety** | If one part of the code assumes a value won't change, another part can't modify it by accident |
| **Concurrency** | Immutable data is naturally thread-safe |
| **Reasoning** | Code is easier to understand when values don't change |

---

## 3.2 — The `mut` Keyword

To "unseal" the box, add `mut`:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");  // 5
    x = 6;
    println!("The value of x is: {x}");  // 6
}
```

`mut` is an explicit signal of intent: "this variable will change." Future readers of the code immediately know they need to track its mutations.

---

## 3.3 — Constants (`const`)

Constants resemble immutable variables but with stricter rules:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

### Differences between `let`, `let mut`, and `const`

| Property | `let` | `let mut` | `const` |
|----------|-------|-----------|---------|
| Mutable? | No | Yes | **Never** |
| Type required? | No (inference) | No (inference) | **Yes, mandatory** |
| Compile-time value? | Not required | Not required | **Yes, mandatory** |
| Global scope allowed? | No | No | **Yes** |
| Can be shadowed? | Yes | Yes | No |
| Naming convention | snake_case | snake_case | **SCREAMING_SNAKE_CASE** |

Constants are useful for magic values shared across the code: speed of light, maximum score, number of seconds in an hour, etc.

---

## 3.4 — Shadowing

**Shadowing** allows you to redeclare a variable with the same name. The new variable "shadows" the old one:

```rust
fn main() {
    let x = 5;

    let x = x + 1;         // x = 6 (new x, shadows the old one)

    {
        let x = x * 2;     // x = 12 (new x in the inner scope)
        println!("Inner scope: {x}");  // 12
    }

    println!("Outer scope: {x}");      // 6
}
```

### In memory (stack):

```
Step 1: let x = 5;
  STACK
  ┌──────────┐
  │ x = 5    │  ← active
  └──────────┘

Step 2: let x = x + 1;
  STACK
  ┌──────────┐
  │ x = 5    │  ← shadowed (still in memory, but inaccessible)
  │ x = 6    │  ← active
  └──────────┘

Step 3: { let x = x * 2; }
  STACK
  ┌──────────┐
  │ x = 5    │  ← shadowed
  │ x = 6    │  ← shadowed
  │ x = 12   │  ← active (inner scope)
  └──────────┘

Step 4: inner scope ends
  STACK
  ┌──────────┐
  │ x = 5    │  ← shadowed
  │ x = 6    │  ← active again!
  └──────────┘
```

### Shadowing vs `mut`: the crucial difference

Shadowing allows **changing the type** of a variable:

```rust
// OK: shadowing creates a NEW variable
let spaces = "   ";          // &str
let spaces = spaces.len();   // usize

// ERROR: mut does NOT allow changing the type
let mut spaces = "   ";      // &str
spaces = spaces.len();       // ERROR: expected `&str`, found `usize`
```

```
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
2 |     let mut spaces = "   ";
  |                      ----- expected due to this value
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`
```

| | Shadowing (`let x = ...`) | Mutation (`x = ...`) |
|-|---------------------------|---------------------|
| Creates a new variable? | **Yes** | No |
| Can change the type? | **Yes** | No |
| Requires `mut`? | No | Yes |
| Error if you forget `let`? | Yes (compile error) | — |

---

## 3.5 — Scalar Types

Rust is **statically typed**: the compiler must know the type of every variable at compile time. It can often infer it, but sometimes an annotation is needed:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
//         ^^^  annotation needed because parse() can produce multiple types
```

A **scalar** represents a single value. Rust has 4 scalar types:

### 3.5.1 — Integers

| Size | Signed (can be negative) | Unsigned (positive only) |
|------|--------------------------|--------------------------|
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` **(default)** | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| Architecture | `isize` | `usize` |

`isize` and `usize` depend on the architecture: 64 bits on a 64-bit CPU, 32 bits on a 32-bit CPU.

#### Value ranges

| Type | Formula | Range |
|------|---------|-------|
| `i8` | -(2^7) to 2^7 - 1 | -128 to 127 |
| `u8` | 0 to 2^8 - 1 | 0 to 255 |
| `i32` | -(2^31) to 2^31 - 1 | -2,147,483,648 to 2,147,483,647 |
| `u32` | 0 to 2^32 - 1 | 0 to 4,294,967,295 |

#### Representation in memory (i8 vs u8)

```
Value -1 as i8 (two's complement):
  ┌───┬───┬───┬───┬───┬───┬───┬───┐
  │ 1 │ 1 │ 1 │ 1 │ 1 │ 1 │ 1 │ 1 │  = 0xFF = -1
  └───┴───┴───┴───┴───┴───┴───┴───┘
    ^
    sign bit

Value 255 as u8:
  ┌───┬───┬───┬───┬───┬───┬───┬───┐
  │ 1 │ 1 │ 1 │ 1 │ 1 │ 1 │ 1 │ 1 │  = 0xFF = 255
  └───┴───┴───┴───┴───┴───┴───┴───┘
    ^
    no sign bit, all bits count

Same bits, different interpretation!
```

#### Integer literals

| Format | Example | Value |
|--------|---------|-------|
| Decimal | `98_222` | 98222 |
| Hexadecimal | `0xff` | 255 |
| Octal | `0o77` | 63 |
| Binary | `0b1111_0000` | 240 |
| Byte (u8 only) | `b'A'` | 65 |
| With type suffix | `57u8` | 57 (type u8) |

The `_` is a visual separator: `1_000_000` = `1000000`.

#### Integer overflow

What happens if you exceed a type's capacity?

| Mode | Behavior |
|------|---------|
| **Debug** (`cargo build`) | The program **panics** (crashes) at runtime |
| **Release** (`cargo build --release`) | **Wrapping**: 256_u8 → 0, 257_u8 → 1 (two's complement) |

Relying on wrapping behavior is considered an **error**. For explicit control:

| Method family | Behavior |
|--------------|---------|
| `wrapping_add`, `wrapping_mul`... | Wraps in all modes |
| `checked_add`, `checked_mul`... | Returns `None` on overflow |
| `overflowing_add`... | Returns (value, bool) to indicate overflow |
| `saturating_add`... | Clamps at the min/max value |

### 3.5.2 — Floating-Point Numbers

```rust
let x = 2.0;      // f64 (default)
let y: f32 = 3.0;  // f32
```

| Type | Size | Precision |
|------|------|-----------|
| `f32` | 32 bits | Single precision (IEEE-754) |
| `f64` | 64 bits | **Double precision (default)** |

`f64` is the default because on modern CPUs, it's nearly as fast as `f32` but much more precise.

### 3.5.3 — Numeric operations

```rust
let sum = 5 + 10;            // addition
let difference = 95.5 - 4.3; // subtraction
let product = 4 * 30;        // multiplication
let quotient = 56.7 / 32.2;  // division
let truncated = -5 / 3;      // = -1 (integer division truncates toward zero)
let remainder = 43 % 5;      // = 3 (modulo)
```

### 3.5.4 — Booleans

```rust
let t = true;
let f: bool = false;  // with annotation
```

- Type: `bool`
- Size: **1 byte**
- Values: `true` or `false`
- Primary usage: conditions (`if`, `while`)

### 3.5.5 — Characters (`char`)

```rust
let c = 'z';
let z: char = 'ℤ';
let heart_eyed_cat = '😻';
```

| Property | Value |
|----------|-------|
| Size | **4 bytes** |
| Encoding | Unicode scalar value |
| Syntax | **Single** quotes (double quotes = `&str`) |
| Range | U+0000..U+D7FF and U+E000..U+10FFFF |

A `char` can hold: accented letters, CJK characters, emojis, zero-width spaces, etc.

---

## 3.6 — Compound Types

Compound types group multiple values into a single type.

### 3.6.1 — Tuples

A tuple groups values of **potentially different types**, with a **fixed size**:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

**In memory** (stack):
```
  STACK
  ┌────────────────────┐
  │ tup                │
  │ ┌────────────────┐ │
  │ │ .0 : i32 = 500 │ │  4 bytes
  │ │ .1 : f64 = 6.4 │ │  8 bytes
  │ │ .2 : u8  = 1   │ │  1 byte  (+ padding)
  │ └────────────────┘ │
  └────────────────────┘
  Total: memory-aligned (likely 16 bytes)
```

#### Destructuring

Extract values via pattern matching:

```rust
let tup = (500, 6.4, 1);
let (x, y, z) = tup;      // x=500, y=6.4, z=1
println!("y = {y}");       // 6.4
```

#### Index access

```rust
let x: (i32, f64, u8) = (500, 6.4, 1);
let five_hundred = x.0;     // 500
let six_point_four = x.1;   // 6.4
let one = x.2;              // 1
```

Indexing starts at **0** and uses a **dot** (not brackets).

#### The unit type `()`

The empty tuple `()` is called **unit**. It's the implicit return type of expressions that return nothing:

```rust
fn do_something() {
    // no return → implicitly returns ()
}
```

### 3.6.2 — Arrays

An array groups values of the **same type**, with a **fixed size**:

```rust
let a = [1, 2, 3, 4, 5];
```

#### Type annotation

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
//      ^^^  ^
//      type number of elements
```

#### Repeated-value initialization

```rust
let a = [3; 5];   // equivalent to [3, 3, 3, 3, 3]
```

#### Index access

```rust
let a = [1, 2, 3, 4, 5];
let first = a[0];   // 1
let second = a[1];   // 2
```

#### In memory (stack)

```
  STACK
  ┌─────────────────────────────┐
  │ a: [i32; 5]                 │
  │ ┌─────┬─────┬─────┬─────┬─────┐
  │ │  1  │  2  │  3  │  4  │  5  │
  │ └─────┴─────┴─────┴─────┴─────┘
  │   [0]   [1]   [2]   [3]   [4]  │
  └─────────────────────────────┘
  Total: 5 x 4 = 20 contiguous bytes on the stack
```

An array is a **contiguous block of memory** of known size, allocated on the **stack**.

#### Out-of-bounds access = Panic

```rust
let a = [1, 2, 3, 4, 5];
let index = 10;
let element = a[index];  // PANIC at runtime!
```

```
thread 'main' panicked at src/main.rs:4:19:
index out of bounds: the len is 5 but the index is 10
```

Rust checks bounds at runtime and **refuses to access invalid memory**. In C, this would be an unprotected memory access (undefined behavior).

#### Array vs Vec

| | Array `[T; N]` | Vec `Vec<T>` |
|-|-----------------|--------------|
| Size | Fixed at compile time | Variable at runtime |
| Storage | **Stack** | **Heap** |
| Performance | No allocation | Dynamic allocation |
| When to use | Known, fixed size (e.g., months of the year) | Unknown or variable size |

```rust
// Array: the 12 months never change
let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
              "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

// Vec: the number of users changes
let mut users = Vec::new();
users.push("Alice");
users.push("Bob");
```

**Rule**: if you're unsure whether to use an array or Vec, use **Vec**.

---

## 3.7 — Summary

```
Rust Types
├── Scalars (single value)
│   ├── Integers: i8, u8, i16, u16, i32 (default), u32, i64, u64, i128, u128, isize, usize
│   ├── Floats: f32, f64 (default)
│   ├── Boolean: bool
│   └── Character: char (4 bytes, Unicode)
│
└── Compound (multiple values)
    ├── Tuple: (T1, T2, T3) — mixed types, fixed size
    └── Array: [T; N] — same type, fixed size
```

| Concept | Key Takeaway |
|---------|-------------|
| **Immutability** | Default. Use `mut` to allow modification |
| **Constants** | `const NAME: Type = value;` always immutable, global scope allowed |
| **Shadowing** | `let x = ...` redeclares, can change the type |
| **Static typing** | The compiler must know all types at compile time |
| **Overflow** | Panic in debug, wrapping in release |
| **Arrays** | Stack, fixed size, bounds-checked access |
