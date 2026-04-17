# Part 14 — An I/O Project: Building a Command Line Program

A practical project that ties together everything from Blocks A-J: **building `minigrep`**, a miniature clone of the classic Unix `grep` tool. It reads a file, searches for a string, and prints every matching line.

| Concept used | Where it was introduced |
|--------------|-------------------------|
| `String`, `&str`, slices | [Part 6 — References & Slices](06-references-slices.md) |
| Structs & methods | [Part 7 — Structs](07-structs.md) |
| Vectors | [Part 10 — Collections](10-collections.md) |
| `Result<T, E>`, `?`, `Box<dyn Error>` | [Part 11 — Error Handling](11-error-handling.md) |
| Traits & lifetimes | [Part 12 — Generics, Traits & Lifetimes](12-generics-traits-lifetimes.md) |
| Unit & integration tests | [Part 13 — Writing Automated Tests](13-tests.md) |

Final usage:

```
$ cargo run -- searchstring example-filename.txt
```

---

## 14.1 — Accepting Command-Line Arguments

Rust exposes process arguments via the standard library: `std::env::args` returns an iterator over the arguments passed to the program.

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
}
```

Running it:

```
$ cargo run -- needle poem.txt
[src/main.rs:5] &args = [
    "target/debug/minigrep",
    "needle",
    "poem.txt",
]
```

| Index | Value | Meaning |
|-------|-------|---------|
| `args[0]` | `"target/debug/minigrep"` | The binary path (always present) |
| `args[1]` | `"needle"` | First user argument |
| `args[2]` | `"poem.txt"` | Second user argument |

```
┌──────────────────────────────────────────────────────────────────┐
│  Use env::args (not env::args_os) when arguments are valid        │
│  Unicode. args panics on invalid UTF-8; args_os returns OsString  │
│  which can hold any byte sequence but is more awkward to work     │
│  with.                                                            │
└──────────────────────────────────────────────────────────────────┘
```

Saving the query and file path:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}
```

---

## 14.2 — Reading a File

`std::fs::read_to_string` opens a file and returns its contents as a `String`:

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
```

This works — but everything lives in `main`, errors are handled with `expect`, and there's no structure. Time to refactor.

---

## 14.3 — Refactoring for Modularity

As a project grows, cramming logic into `main` becomes a liability: hard to test, hard to reason about, hard to reuse. The Rust community's guidelines for CLI programs:

| Guideline | Why |
|-----------|-----|
| Split into `src/main.rs` and `src/lib.rs` | `main` handles the binary; `lib` holds the logic that can be tested |
| Keep `main` small | Its job: parse args, set up config, call the library, handle errors |
| Group related config into a `struct` | Clearer than passing loose arguments around |
| Return `Result` from functions that can fail | Lets the caller decide how to react |

### Extracting argument parsing

```rust
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
```

Why `.clone()`? Because `Config` owns its strings. Cloning costs a small allocation but keeps the lifetime story trivial. A more efficient design (using iterators) is possible but premature here — **clarity first**.

```
┌──────────────────────────────────────────────────────────────────┐
│  Convention: name the constructor `new` if it cannot fail,        │
│  `build` if it returns a Result. A `new` that can return Err      │
│  surprises users — by convention `new` is expected to succeed.    │
└──────────────────────────────────────────────────────────────────┘
```

### Extracting the running logic

```rust
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}
```

- **`Box<dyn Error>`** — a trait object that accepts *any* type implementing the `Error` trait. We don't have to commit to one concrete error type.
- **`?`** — propagates the error instead of panicking.
- **`Ok(())`** — the idiomatic "success, no value to return" signal.

### Moving everything into `src/lib.rs`

```rust
// src/lib.rs
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}
```

Everything that can be tested now lives in the library crate.

### The new `src/main.rs`

```rust
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
```

Three clean responsibilities:

1. **Parse** arguments → `Config::build`
2. **Run** the program → `minigrep::run`
3. **Handle failure** → print to stderr and exit with a non-zero code

---

## 14.4 — Error Handling Details

### `unwrap_or_else` — custom panic-free recovery

```rust
let config = Config::build(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {err}");
    process::exit(1);
});
```

| Method | On `Err`, it... |
|--------|-----------------|
| `unwrap()` | Panics with the error |
| `expect("msg")` | Panics with your message |
| `unwrap_or(default)` | Returns `default` |
| `unwrap_or_else(\|e\| ...)` | Runs your closure with the error, returns its value |

`unwrap_or_else` lets us respond to the error without panicking — here, print a message and exit cleanly.

### `eprintln!` vs `println!`

```rust
eprintln!("Problem parsing arguments: {err}");
```

| Macro | Stream | Used for |
|-------|--------|----------|
| `println!` | **stdout** | Program output (results, data) |
| `eprintln!` | **stderr** | Error messages, diagnostics |

This distinction matters because users redirect the two streams separately:

```
$ cargo run > output.txt            # stdout to file
$ cargo run 2> errors.txt           # stderr to file
$ cargo run > output.txt 2>&1       # both to the same file
```

Error messages should not pollute the stdout pipeline — imagine `minigrep needle poem.txt | wc -l` if errors leaked into stdout.

### `process::exit(code)`

Terminates the program immediately with the given exit code:

| Code | Convention |
|------|-----------|
| `0` | Success (also the default if `main` returns normally) |
| `1` | Generic error |
| Other non-zero values | Program-specific error categories |

```
┌──────────────────────────────────────────────────────────────────┐
│  process::exit does NOT run destructors. Any cleanup that         │
│  matters (flushing buffers, closing files via Drop) will be       │
│  skipped. Prefer returning an error from main when possible.      │
└──────────────────────────────────────────────────────────────────┘
```

---

## 14.5 — Test-Driven Development for the `search` Function

**TDD flow**: write a failing test → make it pass with the minimum code → refactor. Here we apply it to the string-searching function.

### Step 1 — the failing test

```rust
// src/lib.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

### Step 2 — minimum implementation

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

The lifetime `'a` ties the returned slices to **`contents`** — not to `query`. The compiler needs this hint because the return slices point into `contents`' memory, which must outlive the returned `Vec`.

```
┌──────────────────────────────────────────────────────────────────┐
│  Why annotate only `contents` with 'a?                            │
│                                                                   │
│  The returned Vec<&str> holds slices of `contents`. The query     │
│  is only used to compare — none of its bytes end up in the        │
│  output. So the lifetime of the return value depends on           │
│  `contents`, and only `contents` needs the explicit lifetime.     │
└──────────────────────────────────────────────────────────────────┘
```

### Step 3 — wiring it into `run`

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
```

---

## 14.6 — Case-Insensitive Search via an Environment Variable

Real CLI tools often have a flag (like `grep -i`) or an environment variable to toggle case-insensitivity. We'll use the env var approach.

### The insensitive search function (TDD again)

```rust
#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
}
```

```rust
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

`to_lowercase` returns a new `String` — notice that `query` is shadowed from `&str` to `String`. When calling `contains`, we pass `&query` (a `&String`) which coerces to `&str`.

### Reading the env var

```rust
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}
```

`env::var("IGNORE_CASE")`:
- Returns `Ok(String)` if the variable is set (any value).
- Returns `Err(VarError)` if unset.
- `.is_ok()` converts that to a `bool` — we don't care about the value, only whether it's set.

### Choosing which function to call

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
```

### Usage

```
$ cargo run -- to poem.txt
Are you nobody, too?
How dreary to be somebody!

$ IGNORE_CASE=1 cargo run -- to poem.txt
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

On Windows PowerShell:

```
PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
```

---

## 14.7 — Summary

| Concept | Syntax / API | Purpose |
|---------|--------------|---------|
| **Read CLI args** | `std::env::args().collect::<Vec<String>>()` | Get what the user typed |
| **Read file** | `std::fs::read_to_string(path)` | Load a whole file into a `String` |
| **Config struct** | `pub struct Config { ... }` | Group related parameters, testable |
| **Fallible constructor** | `fn build(...) -> Result<Self, &'static str>` | Convention: `build` instead of `new` when it can fail |
| **Generic error type** | `Result<T, Box<dyn Error>>` | Accept any type implementing `Error` |
| **Error propagation** | `?` | Short-circuit on `Err` |
| **Recover without panic** | `result.unwrap_or_else(\|e\| { ... })` | Handle `Err` with a closure |
| **Write to stderr** | `eprintln!("...")` | Keep error output out of the stdout pipeline |
| **Exit with code** | `std::process::exit(1)` | Terminate with a non-zero status |
| **Lifetime-tied return** | `fn search<'a>(q: &str, c: &'a str) -> Vec<&'a str>` | Tell the compiler which input the output borrows from |
| **Env variable** | `env::var("NAME").is_ok()` | Boolean flag based on whether the var is set |

```
┌──────────────────────────────────────────────────────────────────┐
│  The big lesson of this chapter isn't any single API — it's      │
│  the REFACTOR:                                                    │
│                                                                   │
│  main  →  parse + orchestrate + exit                              │
│  lib   →  everything else (testable, reusable)                    │
│                                                                   │
│  Keep main small. Put logic in functions that return Result.      │
│  Write tests against the library, not the binary.                 │
└──────────────────────────────────────────────────────────────────┘
```

> See also: [Part 11 — Error Handling](11-error-handling.md) for `Result`, `?`, and `Box<dyn Error>`, and [Part 13 — Writing Automated Tests](13-tests.md) for the `#[cfg(test)] mod tests` pattern used here.
