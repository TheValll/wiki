# Part 4 — Functions & Control Flow

## 4.1 — Defining and Calling Functions

In Rust, functions use the **snake_case** convention (lowercase, words separated by `_`):

```rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

```
$ cargo run
Hello, world!
Another function.
```

Key points:
- `fn` declares a new function
- **Definition order** doesn't matter: `another_function` is defined after `main` but can be called from `main`
- Rust only cares that the function is defined **somewhere** in a visible scope

---

## 4.2 — Parameters

Parameters are special variables in the function **signature**:

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

### Rule: the type of EVERY parameter must be declared

```rust
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn main() {
    print_labeled_measurement(5, 'h');  // Prints: The measurement is: 5h
}
```

This is a **deliberate** design choice: by requiring types in signatures, the compiler almost never needs type annotations elsewhere in the code. It can also give much more precise error messages.

| Term | Definition |
|------|-----------|
| **Parameter** | Variable in the function definition |
| **Argument** | Concrete value passed when calling |

In practice, the two terms are often used interchangeably.

---

## 4.3 — Statements vs Expressions (crucial concept)

Rust is an **expression-based language**. This distinction is fundamental.

### Definitions

| | Statement | Expression |
|-|-----------|-----------|
| **What** | Instruction that performs an action | Computation that **produces a value** |
| **Returns** | Nothing (`()`) | A value |
| **Examples** | `let y = 6;`, function definitions | `5 + 6`, function calls, `{}` blocks |

### Statements do NOT return a value

```rust
fn main() {
    let x = (let y = 6);  // ERROR!
}
```

```
error: expected expression, found `let` statement
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^
```

In C or Ruby, you can write `x = y = 6`. **Not in Rust**: `let y = 6` is a statement, it doesn't produce a value.

### Expressions produce a value

Almost everything in Rust is an expression:

```
Expressions:
├── 5 + 6                    → 11
├── function call            → return value
├── macro call               → return value
└── block { ... }            → value of the last expression
```

### The `{}` block as an expression

This is the **key** to understanding Rust:

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1       // ← NO semicolon = this is the block's return value
    };

    println!("The value of y is: {y}");  // 4
}
```

```
Block evaluation:
┌─────────────────────┐
│ let x = 3;          │  ← statement
│ x + 1               │  ← expression (last line, no ;)
└──────────┬──────────┘
           │
           └─→ The entire block evaluates to 4
                 │
    let y = ─────┘   y = 4
```

### THE golden rule

```
┌──────────────────────────────────────────────────────┐
│  Expression WITHOUT semicolon  →  returns its value   │
│  Expression WITH semicolon     →  becomes a statement  │
│                                   (returns ())        │
└──────────────────────────────────────────────────────┘
```

---

## 4.4 — Return Values

The return type is declared with `->`:

```rust
fn five() -> i32 {
    5       // ← no ; = implicit return value
}

fn main() {
    let x = five();
    println!("The value of x is: {x}");  // 5
}
```

The last expression in the body is the **implicit return value**. The `return` keyword exists for early returns, but most Rust functions return the last expression.

### Example with parameter

```rust
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = plus_one(5);
    println!("The value of x is: {x}");  // 6
}
```

### The semicolon trap

```rust
fn plus_one(x: i32) -> i32 {
    x + 1;  // ← SEMICOLON = statement = returns ()
}
```

```
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: remove this semicolon to return this value
```

```
fn plus_one(x: i32) -> i32 {
    x + 1;     // ← statement, returns ()
}                  Function promises i32 but returns ()
                   → ERROR

fn plus_one(x: i32) -> i32 {
    x + 1      // ← expression, returns i32
}                  OK!
```

---

## 4.5 — Comments

```rust
// Single-line comment

// Multi-line comments
// use // on each line

let lucky_number = 7; // End-of-line comment (less common)

// Preferred style: comment ABOVE the code
let lucky_number = 7;
```

**Documentation comments** (`///`) generate HTML docs — covered in Chapter 14.

---

## 4.6 — `if` Expressions

### Basic syntax

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

### The condition MUST be a `bool`

No "truthy/falsy" like in JavaScript or Python:

```rust
fn main() {
    let number = 3;
    if number {  // ERROR!
        println!("number was three");
    }
}
```

```
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer
```

You must be explicit:

```rust
if number != 0 {
    println!("number was something other than zero");
}
```

### `else if` for multiple conditions

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("divisible by 4");
    } else if number % 3 == 0 {
        println!("divisible by 3");       // ← Only this one executes
    } else if number % 2 == 0 {
        println!("divisible by 2");       // ← Not executed (even though true!)
    } else {
        println!("not divisible by 4, 3, or 2");
    }
}
```

Rust executes **only the first** branch whose condition is true, then exits the `if`.

Too many `else if`? Use `match` (Chapter 6 of the Rust Book).

### `if` in a `let` (ternary-like)

Since `if` is an **expression**, it can be used on the right side of a `let`:

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
// number = 5
```

**Caution**: both arms must return the **same type**:

```rust
let number = if condition { 5 } else { "six" };  // ERROR!
```

```
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ expected integer, found `&str`
```

Rust must know the type of `number` at compile time. It can't be "sometimes i32, sometimes &str."

---

## 4.7 — Loops

Rust has three types of loops: `loop`, `while`, `for`.

### 4.7.1 — `loop`: infinite loop

```rust
fn main() {
    loop {
        println!("again!");  // prints indefinitely
    }
}
// Ctrl+C to stop
```

#### Returning a value from `loop`

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;    // ← break WITH a value
        }
    };

    println!("The result is {result}");  // 20
}
```

`break value;` exits the loop AND returns the value. It's an idiomatic way to wait for a result.

#### `return` vs `break`

| Keyword | Exits... |
|---------|---------|
| `break` | The current loop |
| `return` | The **entire function** |

### 4.7.2 — Loop labels

With nested loops, `break` and `continue` apply to the **innermost** loop. **Labels** let you target a specific loop:

```rust
fn main() {
    let mut count = 0;

    'counting_up: loop {          // ← Label on the outer loop
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;             // ← Exits the INNER loop
            }
            if count == 2 {
                break 'counting_up; // ← Exits the OUTER loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");  // 2
}
```

```
Execution:
count = 0
  remaining = 10
  remaining = 9      → break (inner)
count = 1
  remaining = 10
  remaining = 9      → break (inner)
count = 2
  remaining = 10     → break 'counting_up (outer!)
End count = 2
```

Labels begin with a **single quote**: `'label_name`.

### 4.7.3 — `while`: conditional loop

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

```
3!
2!
1!
LIFTOFF!!!
```

Equivalent to `loop { if !condition { break; } ... }` but more readable.

### 4.7.4 — `for`: loop over a collection

#### With `while` (fragile)

```rust
let a = [10, 20, 30, 40, 50];
let mut index = 0;

while index < 5 {
    println!("the value is: {}", a[index]);
    index += 1;
}
```

Problems:
- If you change the array size without updating the `5` → **panic** or missed elements
- The compiler adds a bounds check at **every** iteration → slower

#### With `for` (idiomatic)

```rust
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("the value is: {element}");
}
```

- **No index** to manage
- **No risk** of out-of-bounds access
- **Faster** (no bounds check at each iteration)
- Works even if you change the array size

#### `for` with Range

```rust
// Count from 1 to 3
for number in 1..4 {
    println!("{number}!");  // 1! 2! 3!
}

// Countdown
for number in (1..4).rev() {
    println!("{number}!");  // 3! 2! 1!
}
println!("LIFTOFF!!!");
```

| Range | Values produced |
|-------|----------------|
| `1..4` | 1, 2, 3 (end exclusive) |
| `1..=4` | 1, 2, 3, 4 (end inclusive) |
| `(1..4).rev()` | 3, 2, 1 (reversed) |

### 4.7.5 — Which loop to use?

| Loop | When to use |
|------|------------|
| `for` | **Default choice**. Iterating over collections or ranges. Safest and most idiomatic |
| `while` | When the stopping condition isn't tied to an iterator |
| `loop` | When you need an infinite loop or `break` with a return value |

---

## 4.8 — Summary

| Concept | Syntax | Key Takeaway |
|---------|--------|-------------|
| **Function** | `fn name(p: Type) -> RetType { }` | snake_case, types mandatory for params |
| **Statement** | `let y = 6;` | Does NOT return a value |
| **Expression** | `x + 1`, `{ ... }`, `if ... { } else { }` | Returns a value |
| **Semicolon** | `x + 1;` | Turns an expression into a statement |
| **Implicit return** | `fn f() -> i32 { 42 }` | Last expression WITHOUT `;` |
| **if** | `if bool_expr { } else { }` | Condition MUST be `bool`, can be used in `let` |
| **loop** | `loop { break value; }` | Infinite loop, can return a value |
| **while** | `while condition { }` | Conditional loop |
| **for** | `for x in collection { }` | Idiomatic loop, safe and fast |
| **Labels** | `'label: loop { break 'label; }` | Target a specific loop in nested structures |
