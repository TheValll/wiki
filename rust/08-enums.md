# Part 8 — Enums and Pattern Matching

An **enum** (enumeration) lets you define a type by listing its possible **variants**. Where structs group related fields together, enums say "this value is **one of** these possibilities." Combined with `match` and `if let`, enums become one of Rust's most expressive tools.

---

## 8.1 — Defining an Enum

Use the `enum` keyword, give it a name, then list the variants inside curly brackets:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

Variants are **namespaced** under the enum — access them with `::`:

```rust
let four = IpAddrKind::V4;
let six  = IpAddrKind::V6;
```

Both values have the **same type** (`IpAddrKind`), so you can write functions that accept any variant:

```rust
fn route(ip_kind: IpAddrKind) {}

route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

---

## 8.2 — Enums with Data

### The struct approach (verbose)

You might be tempted to pair an enum with a struct:

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};
```

### The enum approach (concise)

Rust lets you attach data **directly** to each variant:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home     = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

Each variant name becomes a **constructor function** automatically — `IpAddr::V4(...)` takes a `String` and returns an `IpAddr`.

### Different types per variant

Unlike structs, each variant can hold **different types and amounts** of data:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home     = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

### A richer example

```rust
enum Message {
    Quit,                          // no data
    Move { x: i32, y: i32 },      // named fields (like a struct)
    Write(String),                 // single String
    ChangeColor(i32, i32, i32),    // three i32 values
}
```

| Variant | Equivalent struct | Data |
|---------|------------------|------|
| `Quit` | `struct QuitMessage;` (unit struct) | None |
| `Move { x, y }` | `struct MoveMessage { x: i32, y: i32 }` | Named fields |
| `Write(String)` | `struct WriteMessage(String);` (tuple struct) | One `String` |
| `ChangeColor(i32, i32, i32)` | `struct ChangeColorMessage(i32, i32, i32);` | Three `i32` |

The advantage: all four are **one type** (`Message`), so a single function can handle them all.

---

## 8.3 — Methods on Enums

Just like structs, enums can have methods defined in an `impl` block:

```rust
impl Message {
    fn call(&self) {
        // method body
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

Inside the method, `self` is the enum value — you typically `match` on it to decide what to do.

---

## 8.4 — The `Option<T>` Enum

Rust has **no null**. Instead, the standard library provides `Option<T>`:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`Option<T>` is included in the **prelude** — you use `Some` and `None` directly without importing.

```rust
let some_number = Some(5);          // Option<i32>
let some_char   = Some('e');        // Option<char>
let absent: Option<i32> = None;     // must annotate type with None
```

### Why not null?

```
┌──────────────────────────────────────────────────────────────────┐
│  In languages with null, ANY variable might be null.             │
│  You forget to check → runtime crash (the "billion-dollar       │
│  mistake").                                                      │
│                                                                  │
│  In Rust, Option<T> and T are DIFFERENT TYPES.                   │
│  The compiler FORCES you to handle the None case before          │
│  you can use the inner value.                                    │
└──────────────────────────────────────────────────────────────────┘
```

### You can't mix `Option<T>` and `T`

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;  // ERROR: cannot add Option<i8> to i8
```

```
error[E0277]: cannot add `Option<i8>` to `i8`
 --> src/main.rs:5:17
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + Option<i8>`
```

You must **extract** the `T` from `Option<T>` first — and that means handling the `None` case. This is where `match` comes in.

---

## 8.5 — The `match` Control Flow Construct

`match` compares a value against a series of **patterns** and runs the code for the first match. Think of it like a coin-sorting machine.

### Basic example: Coin values

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny   => 1,
        Coin::Nickel  => 5,
        Coin::Dime    => 10,
        Coin::Quarter => 25,
    }
}
```

Each **arm** has two parts separated by `=>`:
- **Left**: a pattern
- **Right**: an expression (the return value)

Arms are separated by commas. For multi-line code, use curly brackets:

```rust
Coin::Penny => {
    println!("Lucky penny!");
    1
}
```

---

## 8.6 — Patterns That Bind to Values

Match arms can **bind** to the data inside enum variants:

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny   => 1,
        Coin::Nickel  => 5,
        Coin::Dime    => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```

When `Coin::Quarter(UsState::Alaska)` is matched, the variable `state` binds to `UsState::Alaska` and can be used in the arm's code.

---

## 8.7 — Matching with `Option<T>`

One of the most common `match` patterns:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None    => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six  = plus_one(five);   // Some(6)
let none = plus_one(None);   // None
```

### Concrete walkthrough

```
Call: plus_one(Some(5))
  ├─ Pattern None    → does not match
  └─ Pattern Some(i) → MATCH! i binds to 5
       → returns Some(5 + 1) = Some(6)

Call: plus_one(None)
  └─ Pattern None    → MATCH!
       → returns None
```

---

## 8.8 — Matches Are Exhaustive

You **must** cover every possible variant. Forgetting one is a compile error:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // Missing None arm!
    }
}
```

```
error[E0004]: non-exhaustive patterns: `None` not covered
 --> src/main.rs:3:15
  |
3 |         match x {
  |               ^ pattern `None` not covered
```

This is a **safety net**: the compiler guarantees you handle every case, especially the `None` case with `Option<T>`.

---

## 8.9 — Catch-All Patterns and the `_` Placeholder

For types with many possible values (like `u8` with 256 values), use a **catch-all** arm:

### Named catch-all (uses the value)

```rust
let dice_roll = 9;
match dice_roll {
    3     => add_fancy_hat(),
    7     => remove_fancy_hat(),
    other => move_player(other),  // catch-all, uses the value
}
```

### `_` placeholder (ignores the value)

```rust
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),          // catch-all, value not needed
}
```

### `_ => ()` (do nothing)

```rust
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),                // nothing happens for other rolls
}
```

| Pattern | Binds value? | Use case |
|---------|-------------|----------|
| `other` | Yes | Need the value in the arm |
| `_` | No | Don't need the value |
| `_ => ()` | No | Explicitly do nothing |

**Important**: the catch-all arm must be **last** — patterns are evaluated in order.

---

## 8.10 — Concise Control Flow with `if let`

When you only care about **one** pattern, `match` can be verbose:

```rust
// Verbose match
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
}
```

`if let` is syntactic sugar for a `match` with one arm + a wildcard:

```rust
// Concise if let
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}
```

### `if let` with `else`

The `else` block corresponds to the `_` arm in a `match`:

```rust
let coin = Coin::Penny;
let mut count = 0;

// Using match:
match coin {
    Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    _ => count += 1,
}

// Equivalent if let + else:
if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
} else {
    count += 1;
}
```

### Trade-off

| | `match` | `if let` |
|-|---------|----------|
| **Exhaustive checking** | Yes — compiler enforces | No — you might miss cases |
| **Conciseness** | Verbose for single-pattern | Clean and short |
| **Best for** | Multiple patterns, safety-critical | One pattern you care about |

---

## 8.11 — The `let...else` Syntax

When you want to **extract a value or return early**, `let...else` keeps the code on the "happy path":

```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;   // early return if not a Quarter
    };

    // "happy path" continues here — state is in scope
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```

### How it works

```
┌──────────────────────────────────────────────────────────────────┐
│  let PATTERN = EXPRESSION else {                                 │
│      // must diverge: return, break, continue, or panic!         │
│  };                                                              │
│                                                                  │
│  • If pattern matches → binds the value in the outer scope       │
│  • If pattern doesn't match → runs the else block (must exit)    │
└──────────────────────────────────────────────────────────────────┘
```

### Comparison: three approaches

```rust
// 1. Nested if let (deep indentation)
fn describe(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is old"))
        } else {
            Some(format!("{state:?} is new"))
        }
    } else {
        None
    }
}

// 2. if let with early return (awkward binding)
fn describe(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };
    // use state...
}

// 3. let...else (cleanest)
fn describe(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };
    // use state...
}
```

---

## 8.12 — Summary

| Concept | Description | Example |
|---------|------------|---------|
| **Enum** | Type with a fixed set of variants | `enum Coin { Penny, Dime }` |
| **Data in variants** | Each variant can hold different data | `V4(u8, u8, u8, u8)` |
| **Methods on enums** | `impl` blocks work on enums too | `impl Message { fn call(&self) {} }` |
| **`Option<T>`** | Replaces null — `Some(T)` or `None` | `let x: Option<i32> = Some(5);` |
| **`match`** | Pattern matching, exhaustive | `match coin { Coin::Penny => 1, ... }` |
| **Binding in patterns** | Extract data from variants | `Some(i) => Some(i + 1)` |
| **Exhaustiveness** | Compiler requires all cases covered | Forgetting `None` → compile error |
| **Catch-all** | `other` (binds) or `_` (ignores) | `_ => reroll()` |
| **`if let`** | Match a single pattern concisely | `if let Some(max) = config_max { ... }` |
| **`if let` + `else`** | Single pattern with fallback | `if let P = x { ... } else { ... }` |
| **`let...else`** | Extract or return early | `let P = x else { return None; };` |

```
┌──────────────────────────────────────────────────────────────────┐
│  Enums + match = Rust's way of modeling "one of many" values.    │
│  Option<T> + exhaustive matching = no more null-pointer bugs.    │
│  if let / let...else = concise shortcuts when one pattern        │
│  is all you need.                                                │
└──────────────────────────────────────────────────────────────────┘
```
