# Part 2 — The Guessing Game: First Complete Project

## 2.1 — Create the project

```bash
$ cargo new guessing_game
$ cd guessing_game
```

Cargo generates the usual structure:
```
guessing_game/
├── Cargo.toml
└── src/
    └── main.rs    ← contains println!("Hello, world!")
```

---

## 2.2 — Reading user input

Replace the contents of `src/main.rs`:

```rust
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

### Line-by-line breakdown

#### `use std::io;` — Importing a library

Rust's **prelude** automatically imports a set of common types into every program. If a type isn't in the prelude, you must import it with `use`.

`std::io` provides input/output functions.

#### `let mut guess = String::new();` — Creating a mutable variable

```
let mut guess = String::new();
│   │   │       │       │
│   │   │       │       └── Associated function: creates an empty String
│   │   │       └── String type (heap-allocated, growable, UTF-8)
│   │   └── Variable name
│   └── Mutable (can be modified)
└── Variable declaration
```

| Concept | Explanation |
|---------|------------|
| `let` | Declares a variable (immutable by default) |
| `mut` | Makes the variable modifiable |
| `String::new()` | **Associated function** (called on the type, not an instance). The `::` indicates a namespace |
| `String` | String type: allocated on the **heap**, variable size, UTF-8 encoded |

**In memory**:
```
  STACK                          HEAP
  ┌─────────────┐               ┌──────────────┐
  │ guess       │               │  (empty)     │
  │ ┌─────────┐ │               │              │
  │ │ ptr ────┼─┼─────────────> │              │
  │ │ len = 0 │ │               └──────────────┘
  │ │ cap = 0 │ │
  │ └─────────┘ │
  └─────────────┘
```

A `String` is made of 3 fields on the stack:
- **ptr**: pointer to data on the heap
- **len**: number of bytes currently in use
- **cap**: total allocated capacity

#### `io::stdin().read_line(&mut guess)` — Reading input

```rust
io::stdin()                 // Returns a handle to standard input
    .read_line(&mut guess)  // Reads a line and appends it to guess
    .expect("Failed to read line");  // Handles potential errors
```

| Element | Role |
|---------|------|
| `io::stdin()` | Returns a `Stdin` object (handle to the terminal) |
| `.read_line(...)` | Reads until `\n` and **appends** to the String |
| `&mut guess` | **Mutable reference**: allows `read_line` to modify `guess` without taking ownership |

The `&` creates a **reference** (a pointer to the data, without becoming the owner). References are immutable by default, hence `&mut` to allow modification. We'll cover this in detail in Parts 5 and 6.

#### `.expect("Failed to read line")` — Handling `Result`

`read_line` returns a **`Result`**, an enum with two variants:

```
Result<usize, io::Error>
├── Ok(usize)   ← Success: contains the number of bytes read
└── Err(error)  ← Failure: contains the error
```

| Method | Behavior |
|--------|---------|
| `.expect("msg")` | If `Ok`: returns the value. If `Err`: **crashes** (panics) with the message |
| `.unwrap()` | Like `expect` but with a generic message |
| `match` | Fine-grained handling of each case (we'll see later) |

If you don't handle the `Result`, the compiler shows a **warning**:

```
warning: unused `Result` that must be used
```

#### `println!("You guessed: {guess}")` — Displaying with placeholders

The `{}` are **placeholders**:

```rust
let x = 5;
let y = 10;
println!("x = {x} and y + 2 = {}", y + 2);
// Prints: x = 5 and y + 2 = 12
```

- `{variable}`: directly inserts a variable
- `{}`: replaced by the next expression in the argument list

---

## 2.3 — Generating a random number

Rust doesn't provide a random number generator in the standard library. You need an external **crate**.

### Crates: binary vs library

| Type | Description |
|------|------------|
| **Binary crate** | Produces an executable (our project) |
| **Library crate** | Reusable code for other programs (e.g., `rand`) |

### Adding the dependency

In `Cargo.toml`:

```toml
[dependencies]
rand = "0.8.5"
```

`"0.8.5"` is shorthand for `"^0.8.5"` (**SemVer** — Semantic Versioning):

```
^0.8.5 means:
  >= 0.8.5  (at least this version)
  <  0.9.0  (but not the next minor version)

Accepted: 0.8.5, 0.8.6, 0.8.99
Rejected: 0.9.0, 1.0.0
```

### First build with dependencies

```bash
$ cargo build
  Updating crates.io index
  Downloading rand v0.8.5
  Downloading rand_core v0.6.4
  Downloading getrandom v0.2.15
  ...
  Compiling rand v0.8.5
  Compiling guessing_game v0.1.0
```

Cargo downloads `rand` and **all its transitive dependencies** from [crates.io](https://crates.io) (the central registry of the Rust ecosystem).

### Using `rand`

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

#### Breakdown

```rust
use rand::Rng;  // Import the Rng trait (defines methods for random generators)

let secret_number = rand::thread_rng().gen_range(1..=100);
//                  │                  │         │
//                  │                  │         └── Inclusive range: 1 to 100
//                  │                  └── Method from the Rng trait
//                  └── Thread-local generator, seeded by the OS
```

| Range syntax | Meaning |
|-------------|---------|
| `1..100` | From 1 to 99 (end **exclusive**) |
| `1..=100` | From 1 to 100 (end **inclusive**) |

### Exploring crate documentation

```bash
$ cargo doc --open
```

Generates and opens HTML documentation for all your dependencies in the browser.

---

## 2.4 — Comparing with `match`

### Importing `Ordering`

```rust
use std::cmp::Ordering;
```

`Ordering` is an **enum** with 3 variants:

```
Ordering
├── Less       (smaller)
├── Greater    (bigger)
└── Equal      (equal)
```

### The `match` expression

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

`match` is made of **arms**:

```
match value {
    pattern1 => code1,    ← First arm
    pattern2 => code2,    ← Second arm
    pattern3 => code3,    ← Third arm
}
```

**How it works**: Rust compares the value against each pattern in order. As soon as a pattern matches, the associated code executes and `match` stops.

#### Concrete walkthrough

If `guess = 50` and `secret_number = 38`:

```
1. guess.cmp(&secret_number) returns Ordering::Greater (50 > 38)
2. match Ordering::Greater {
3.     Ordering::Less    =>  ...  // Greater != Less    → SKIP
4.     Ordering::Greater =>  ...  // Greater == Greater → MATCH!
5.                                   println!("Too big!")
6.     Ordering::Equal   =>  ...  // Never reached
7. }
```

### Type mismatch error: String vs i32

This code won't compile! `guess` is a `String`, `secret_number` is an `i32`. You can't compare different types.

```
error[E0308]: mismatched types
  --> src/main.rs:23:21
   |
   |     match guess.cmp(&secret_number) {
   |                     ^^^^^^^^^^^^^^ expected `&String`, found `&{integer}`
```

### Type conversion with shadowing

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

This is **shadowing**: we redeclare `guess` with a new type.

```
BEFORE:  guess = "  76\n"  (String, with whitespace and newline)

guess.trim()        →  "76"       (removes whitespace and \n)
     .parse()       →  Ok(76)     (converts to number)
     .expect(...)   →  76         (extracts value from Ok)

AFTER:   guess = 76_u32  (unsigned 32-bit integer)
```

| Method | Role |
|--------|------|
| `trim()` | Removes whitespace and `\n` (`\r\n` on Windows) from both ends |
| `parse()` | Converts a `String` to another type. Returns a `Result` |
| `: u32` | **Type annotation**: tells `parse()` what type to produce |

The `u32` annotation on `guess` propagates: Rust infers that `secret_number` must also be `u32`.

---

## 2.5 — The game loop

### Infinite loop with `loop`

```rust
loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;                     // ← Exit the loop
        }
    }
}
```

| Keyword | Effect |
|---------|--------|
| `loop { }` | Infinite loop |
| `break` | Exit the loop |
| `continue` | Skip to the next iteration |

### Handling invalid input

If the user types "abc", `parse()` returns `Err` and `expect` crashes the program. We can do better with `match`:

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,       // parse succeeded: return the number
    Err(_) => continue,   // parse failed: skip to next iteration
};
```

```
User input: "abc"

guess.trim().parse()  →  Err(ParseIntError)

match Err(ParseIntError) {
    Ok(num) => num,        // Err != Ok → SKIP
    Err(_)  => continue,   // Err == Err → MATCH!
                           //   _ = catch-all (ignores error details)
                           //   continue = jump back to loop start
}
```

The program silently ignores invalid input and asks for another number.

---

## 2.6 — Complete final code

```rust
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

### Sample run

```
$ cargo run
Guess the number!
Please input your guess.
50
You guessed: 50
Too big!
Please input your guess.
25
You guessed: 25
Too small!
Please input your guess.
foo
Please input your guess.
37
You guessed: 37
You win!
```

---

## 2.7 — Summary of concepts introduced

| Concept | Example | Explanation |
|---------|---------|------------|
| **Immutable variable** | `let x = 5;` | Fixed value |
| **Mutable variable** | `let mut x = 5;` | Modifiable value |
| **Associated function** | `String::new()` | Called on a type (`::`) |
| **Reference** | `&guess`, `&mut guess` | Borrowed pointer, no ownership transfer |
| **`Result` enum** | `Ok(val)` / `Err(e)` | Success or failure of an operation |
| **`expect`** | `.expect("msg")` | Crashes if `Err`, returns value if `Ok` |
| **`match`** | `match val { P => C }` | Exhaustive pattern matching |
| **Shadowing** | `let x: u32 = x.parse()...` | Redeclare a variable with a new type |
| **`loop`** | `loop { break; }` | Infinite loop with explicit exit |
| **`continue`** | `Err(_) => continue` | Jump to next iteration |
| **External crate** | `rand = "0.8.5"` | Dependency in `Cargo.toml` |
| **Trait** | `use rand::Rng;` | Interface of methods that must be imported to use them |
| **Inclusive range** | `1..=100` | From 1 to 100 inclusive |
