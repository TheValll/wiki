# Part 13 — Writing Automated Tests

Rust has a **first-class testing framework** built into the language and toolchain. No external library needed — `cargo test` compiles your code in test mode, runs every function marked `#[test]`, and reports results.

| Term | What it means |
|------|---------------|
| **Test function** | A function annotated with `#[test]` |
| **Passing** | The function completes without panicking |
| **Failing** | The function panics (via `assert!`, `panic!`, or any other panic) |

---

## 13.1 — Anatomy of a Test Function

A test follows a simple pattern: **arrange → act → assert**.

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

Run with:

```
$ cargo test
running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Key points:
- `#[test]` marks the function as a test — without it, `cargo test` ignores it.
- Each test runs in its **own thread**. If the thread panics, the test is marked as failed.
- `use super::*;` brings the parent module's items into the test module's scope.

---

## 13.2 — Checking Results with Assertions

### `assert!` — boolean condition

```rust
#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle { width: 8, height: 7 };
    let smaller = Rectangle { width: 5, height: 1 };

    assert!(larger.can_hold(&smaller));
}
```

Passes if the expression is `true`. Panics (= fails) if `false`.

Use negation for inverse checks:

```rust
assert!(!smaller.can_hold(&larger));
```

### `assert_eq!` and `assert_ne!` — equality with diagnostics

```rust
pub fn add_two(a: u64) -> u64 {
    a + 2
}

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}
```

When `assert_eq!` fails, it prints **both values** — much more helpful than a plain `assert!`:

```
assertion `left == right` failed
  left: 5
 right: 4
```

`assert_ne!` is the opposite — passes when the values differ.

| Macro | Passes when | Fails when |
|-------|-------------|------------|
| `assert!(expr)` | `expr` is `true` | `expr` is `false` |
| `assert_eq!(left, right)` | `left == right` | `left != right` |
| `assert_ne!(left, right)` | `left != right` | `left == right` |

```
┌──────────────────────────────────────────────────────────────────┐
│  assert_eq! and assert_ne! require that both values implement     │
│  PartialEq (for comparison) and Debug (for printing).             │
│                                                                   │
│  For your own types: #[derive(Debug, PartialEq)]                  │
└──────────────────────────────────────────────────────────────────┘
```

### Custom failure messages

All three macros accept an optional format string after the required arguments:

```rust
pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{result}`"
    );
}
```

When the test fails, the custom message appears — much easier to debug than a generic "assertion failed."

---

## 13.3 — Testing for Panics with `should_panic`

Sometimes you want to verify that code **panics** when it should. The `#[should_panic]` attribute does this:

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }
}

#[test]
#[should_panic]
fn greater_than_100() {
    Guess::new(200);
}
```

| What happens | Test result |
|--------------|-------------|
| Code panics | **Pass** |
| Code does NOT panic | **Fail** |

### Making `should_panic` precise with `expected`

A bare `#[should_panic]` passes on **any** panic. That's fragile — a completely unrelated panic would pass the test. Add `expected` to check that the panic message contains a substring:

```rust
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }
        Guess { value }
    }
}

#[test]
#[should_panic(expected = "less than or equal to 100")]
fn greater_than_100() {
    Guess::new(200);
}
```

Now the test only passes if the panic message **contains** `"less than or equal to 100"`.

---

## 13.4 — Using `Result<T, E>` in Tests

Instead of panicking, tests can return `Result`:

```rust
#[test]
fn it_works() -> Result<(), String> {
    let result = add(2, 2);

    if result == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
```

The main benefit: you can use the `?` operator to propagate errors cleanly instead of scattering `unwrap()` calls.

```
┌──────────────────────────────────────────────────────────────────┐
│  You CANNOT use #[should_panic] on a Result-returning test.       │
│  To assert that something returns an Err, use:                    │
│  assert!(value.is_err())                                          │
└──────────────────────────────────────────────────────────────────┘
```

---

## 13.5 — Controlling How Tests Run

`cargo test` accepts two sets of flags, separated by `--`:

```
cargo test [flags for cargo] -- [flags for the test binary]
```

### Parallel vs. sequential

Tests run in **parallel** by default (one thread per test). If tests share state (files, environment variables), they can interfere. Force sequential execution with:

```
$ cargo test -- --test-threads=1
```

### Showing `println!` output

By default, `cargo test` **captures** stdout for passing tests (only failed tests show their output). To see everything:

```
$ cargo test -- --show-output
```

### Running a subset of tests

```
$ cargo test it_adds_two            # run one specific test
$ cargo test add                    # run all tests with "add" in the name
```

Filtering matches against the **full test path** including the module: `tests::it_adds_two`. So you can also filter by module name.

### Ignoring expensive tests

Mark slow tests with `#[ignore]`:

```rust
#[test]
#[ignore]
fn expensive_test() {
    // code that takes a long time
}
```

```
$ cargo test                        # skips ignored tests
$ cargo test -- --ignored           # runs ONLY ignored tests
$ cargo test -- --include-ignored   # runs ALL tests including ignored
```

---

## 13.6 — Test Organization

Rust separates tests into two categories:

| Category | Where | What it tests | Accesses private functions? |
|----------|-------|---------------|---------------------------|
| **Unit tests** | Same file as the code, in a `#[cfg(test)] mod tests` | One module in isolation | **Yes** |
| **Integration tests** | Separate `tests/` directory at project root | Public API only | **No** |

### Unit tests

Convention: a `tests` module at the bottom of each source file.

```rust
// src/lib.rs

pub fn add_two(a: u64) -> u64 {
    internal_adder(a, 2)
}

fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let result = internal_adder(2, 2);   // private function — accessible!
        assert_eq!(result, 4);
    }
}
```

`#[cfg(test)]` means this module is **only compiled when running `cargo test`** — it doesn't end up in the release binary.

**Testing private functions**: Rust allows it. The `tests` module is a child of the module containing the code, so it can access private items via `use super::*;`. This is a deliberate design choice — not all languages allow this.

### Integration tests

Create a `tests/` directory at the project root:

```
my_project/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs
```

```rust
// tests/integration_test.rs

use my_project::add_two;

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}
```

Key differences from unit tests:
- **No `#[cfg(test)]` needed** — Cargo treats the `tests/` directory specially.
- **Must import your crate** — integration tests are separate crates, so `use my_project::...;` is required.
- **Only public API** — private functions are not accessible.

Run a specific integration test file:

```
$ cargo test --test integration_test
```

### Sharing helper code between integration tests

If multiple integration test files need a shared `setup()` function, **don't** put it in `tests/common.rs` (Cargo would treat it as its own test file). Use the subdirectory convention:

```
tests/
├── common/
│   └── mod.rs          ← shared helpers, NOT a test file
└── integration_test.rs
```

```rust
// tests/common/mod.rs
pub fn setup() {
    // shared setup code
}

// tests/integration_test.rs
use my_project::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}
```

Files in subdirectories of `tests/` are **not** compiled as separate test crates — they won't appear in the test output.

### Integration tests for binary crates

```
┌──────────────────────────────────────────────────────────────────┐
│  If your project only has src/main.rs (no src/lib.rs),            │
│  you CANNOT write integration tests — there's no library          │
│  crate to import.                                                 │
│                                                                   │
│  Best practice: put logic in src/lib.rs, keep src/main.rs         │
│  as a thin wrapper that calls library functions.                  │
│  Then integration tests can use your_crate::... to test the       │
│  logic.                                                           │
└──────────────────────────────────────────────────────────────────┘
```

---

## 13.7 — Summary

| Concept | Syntax / Command | Purpose |
|---------|-----------------|---------|
| **Mark a test** | `#[test]` | Tell `cargo test` this is a test function |
| **Assert true** | `assert!(expr)` | Pass if `expr` is true |
| **Assert equal** | `assert_eq!(a, b)` | Pass if `a == b`, prints both on failure |
| **Assert not equal** | `assert_ne!(a, b)` | Pass if `a != b` |
| **Custom message** | `assert!(x, "msg {}", val)` | Better failure diagnostics |
| **Expect panic** | `#[should_panic(expected = "...")]` | Pass if code panics with matching message |
| **Return Result** | `fn test() -> Result<(), String>` | Use `?` in tests instead of unwrap |
| **Sequential** | `cargo test -- --test-threads=1` | Prevent shared-state interference |
| **Show output** | `cargo test -- --show-output` | See `println!` from passing tests |
| **Filter** | `cargo test name_pattern` | Run matching tests only |
| **Ignore** | `#[ignore]` | Skip slow tests by default |
| **Unit tests** | `#[cfg(test)] mod tests` | In-file, can test private functions |
| **Integration tests** | `tests/` directory | Separate crate, public API only |
| **Shared helpers** | `tests/common/mod.rs` | Shared code without being a test file |

```
┌──────────────────────────────────────────────────────────────────┐
│  The testing mental model:                                        │
│                                                                   │
│  Unit tests   → "does this function work correctly?"              │
│  Integration  → "do these modules work together correctly?"       │
│                                                                   │
│  #[cfg(test)]  → only compiled for cargo test                     │
│  tests/        → only compiled for cargo test                     │
│                                                                   │
│  Both are zero cost in your release binary.                       │
└──────────────────────────────────────────────────────────────────┘
```
