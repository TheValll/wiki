# Part 11 — Error Handling

Rust groups errors into two categories and gives each one a dedicated tool:

| Category | Tool | What happens |
|----------|------|-------------|
| **Unrecoverable** | `panic!` | Program prints a message and stops |
| **Recoverable** | `Result<T, E>` | Caller decides what to do |

There are no exceptions, no try/catch. Every error path is visible in the type system.

---

## 11.1 — Unrecoverable Errors with `panic!`

When something goes so wrong that the program **cannot continue safely**, call `panic!`:

```rust
fn main() {
    panic!("crash and burn");
}
```

```
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
```

### When does Rust panic for you?

Some operations panic **automatically** when they detect a bug:

```rust
let v = vec![1, 2, 3];
v[99]; // index out of bounds → panic
```

This is a **safety net**: Rust refuses to read invalid memory (in C this would be undefined behavior — a security vulnerability).

### Reading a backtrace

Set the `RUST_BACKTRACE` environment variable to see the chain of function calls that led to the panic:

```
RUST_BACKTRACE=1 cargo run
```

The backtrace lists every function on the call stack. Look for the **first line that points to your code** — that's usually where the problem is.

```
┌──────────────────────────────────────────────────────────────────┐
│  panic! = "this is a bug, the program cannot continue."          │
│  It's not for expected failures (file not found, bad input).     │
│  It's for situations that should never happen in correct code.   │
└──────────────────────────────────────────────────────────────────┘
```

### Unwinding vs. aborting

By default, a panic **unwinds** the stack — Rust walks back up and cleans up each function's data. If you want the binary to be smaller and let the OS reclaim memory instead, you can switch to **abort** mode in `Cargo.toml`:

```toml
[profile.release]
panic = 'abort'
```

---

## 11.2 — Recoverable Errors with `Result`

Most errors are not bugs — they're expected situations (a file that doesn't exist, a network timeout). Rust models these with `Result<T, E>`:

```rust
enum Result<T, E> {
    Ok(T),    // success — carries the value
    Err(E),   // failure — carries the error
}
```

`Result` is in the prelude — no import needed.

### Example: opening a file

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt");

    let greeting_file = match greeting_file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
```

`File::open` returns `Result<File, io::Error>`:
- If the file exists → `Ok(file)` — you get a file handle.
- If it doesn't → `Err(error)` — you get an `io::Error` describing what went wrong.

### Matching on different error kinds

Not all errors deserve the same response. You can inspect the error kind:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt");

    let greeting_file = match greeting_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}
```

Logic: if the file doesn't exist → create it. If something else went wrong → panic.

### Cleaner version with closures

The nested `match` above is verbose. `unwrap_or_else` reads better:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}
```

---

## 11.3 — Shortcuts: `unwrap` and `expect`

Writing `match` everywhere is verbose. Two shortcuts exist for prototyping or when you are certain an error can't happen:

### `unwrap()`

Returns the `Ok` value, or **panics** with a generic message if `Err`:

```rust
let greeting_file = File::open("hello.txt").unwrap();
// If Err → panic with: "called `Result::unwrap()` on an `Err` value: ..."
```

### `expect(msg)`

Same as `unwrap`, but **you choose the panic message**:

```rust
let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in this project");
// If Err → panic with: "hello.txt should be included in this project: ..."
```

```
┌──────────────────────────────────────────────────────────────────┐
│  Prefer expect() over unwrap() — when you read the panic         │
│  message later, a descriptive string tells you WHY the value     │
│  was expected to be Ok, not just THAT it wasn't.                 │
└──────────────────────────────────────────────────────────────────┘
```

| Method | On `Ok(v)` | On `Err(e)` | Use case |
|--------|-----------|-------------|----------|
| `unwrap()` | Returns `v` | Panics (generic msg) | Quick prototype |
| `expect("msg")` | Returns `v` | Panics with `"msg"` | Production code where failure = bug |

---

## 11.4 — Propagating Errors

Often, a function **doesn't know what to do** with an error — it should pass it back to its caller. This is called **propagating**.

### The manual way (with `match`)

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

The pattern is: try the operation → if it fails, `return Err(e)` → if it succeeds, continue.

---

## 11.5 — The `?` Operator

The `?` operator does **exactly** what the manual propagation above does, in one character:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

### What `?` does, step by step

```
┌──────────────────────────────────────────────────────────────────┐
│  expression?                                                      │
│                                                                   │
│  1. Evaluate the expression (must return Result or Option)        │
│  2. If Ok(value) → unwrap it, continue with value                 │
│  3. If Err(e)    → return Err(e) from the current function        │
│                                                                   │
│  It's syntax sugar for:                                           │
│  match expression {                                               │
│      Ok(value) => value,                                          │
│      Err(e)    => return Err(e.into()),                           │
│  }                                                                │
└──────────────────────────────────────────────────────────────────┘
```

Notice the `.into()` — `?` automatically **converts** the error type using the `From` trait, so you can use `?` even when the error types don't match exactly (as long as a conversion exists).

### Chaining with `?`

Since `?` returns the success value, you can chain calls:

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

Or even shorter — `fs::read_to_string` does everything in one call:

```rust
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

### `?` only works in functions that return `Result` or `Option`

This won't compile:

```rust
fn main() {
    let greeting_file = File::open("hello.txt")?;
}
```

```
error: the `?` operator can only be used in a function that returns
       `Result` or `Option`
```

Fix: change `main` to return a `Result`:

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}
```

`Box<dyn Error>` means "any kind of error" — it's the most flexible return type for `main`.

### `?` with `Option<T>`

`?` also works on `Option`: it returns `None` early instead of `Err`:

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

**You cannot mix** `?` on `Result` and `?` on `Option` in the same function — the return type must match.

---

## 11.6 — When to `panic!` vs. When to Return `Result`

This is the most important section. The rule of thumb:

### Use `Result` by default

When writing a library or a function that others will call, **return `Result`**. Let the caller decide how to handle the error — maybe they want to retry, maybe they want to log and continue, maybe they want to crash. That's their choice, not yours.

### Use `panic!` in these specific cases

| Situation | Why panic is appropriate |
|-----------|------------------------|
| **Examples, prototypes, tests** | You want to see the error fast, not handle it gracefully |
| **You have more info than the compiler** | You *know* a call won't fail, but the type system can't prove it (e.g., `"127.0.0.1".parse::<IpAddr>().unwrap()` — a hardcoded valid IP) |
| **A broken invariant** | Your code reached a state that should be impossible — continuing would be unsafe or produce garbage |
| **Bad state that can't be recovered** | Not "file not found" (that's expected) but "the data structure is internally corrupt" |

### Use `expect` with a reason

When you use `unwrap()` or `expect()`, prefer `expect` with a message explaining **why** the value should be `Ok`:

```rust
// Bad — no context when it panics
let home: IpAddr = "127.0.0.1".parse().unwrap();

// Good — the message explains the assumption
let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");
```

---

## 11.7 — Creating Custom Types for Validation

Instead of scattering validation checks everywhere, you can encode the rules **into a type**. The constructor enforces them once, and every function that takes the type gets the guarantee for free.

### Example: a `Guess` that must be 1–100

```rust
pub struct Guess {
    value: i32,   // private — can only be set through new()
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

| Design decision | Effect |
|-----------------|--------|
| `value` is **private** | Outside code can't set it to an invalid number |
| `new()` **panics** on invalid input | A `Guess` that exists is guaranteed to be in range |
| `value()` is a **getter** | Read-only access to the inner value |

Any function that accepts a `Guess` knows the value is between 1 and 100 — no runtime checks needed.

```
┌──────────────────────────────────────────────────────────────────┐
│  If invalid input is a BUG (the caller should never pass it),    │
│  panic in the constructor.                                       │
│                                                                  │
│  If invalid input is EXPECTED (user input, file data),           │
│  return Result from the constructor instead:                     │
│  pub fn new(value: i32) -> Result<Guess, String>                 │
└──────────────────────────────────────────────────────────────────┘
```

---

## 11.8 — Summary

| Concept | Tool | When |
|---------|------|------|
| **Unrecoverable error** | `panic!` | Bug, broken invariant, impossible state |
| **Recoverable error** | `Result<T, E>` | Expected failure the caller can handle |
| **Quick unwrap** | `unwrap()` / `expect()` | Prototypes, or when you know it can't fail |
| **Propagate to caller** | `?` operator | You can't handle it here, but someone above can |
| **Validate once, trust everywhere** | Custom type with constructor | Enforce invariants in one place |

```
┌──────────────────────────────────────────────────────────────────┐
│  The decision tree:                                               │
│                                                                   │
│  Can the caller reasonably recover?                               │
│    YES → return Result<T, E>                                      │
│    NO  → panic!                                                   │
│                                                                   │
│  Can YOU handle the error here?                                   │
│    YES → match / unwrap_or / unwrap_or_else                       │
│    NO  → propagate with ?                                         │
│                                                                   │
│  Writing a prototype / test?                                      │
│    → unwrap() / expect() are fine, swap them out later            │
└──────────────────────────────────────────────────────────────────┘
```
