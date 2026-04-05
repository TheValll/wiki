# Part 7 — Using Structs to Structure Related Data

A **struct** (structure) lets you group related values into a single, named type. Like a tuple, the pieces can be different types — but unlike tuples, you **name each field**, making the code self-documenting.

---

## 7.1 — Defining and Instantiating Structs

### Defining a struct

Use the `struct` keyword, give the struct a name, then list its **fields** (name: type) inside curly brackets:

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

### Creating an instance

Specify concrete values for each field. The order doesn't need to match the definition:

```rust
let user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};
```

### Accessing and modifying fields

Use **dot notation**. To modify, the entire instance must be `mut` — Rust doesn't allow marking individual fields as mutable:

```rust
let mut user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

### Returning a struct from a function

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```

---

## 7.2 — Field Init Shorthand

When the parameter name matches the field name exactly, you can skip the repetition:

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,       // shorthand for username: username
        email,          // shorthand for email: email
        sign_in_count: 1,
    }
}
```

---

## 7.3 — Struct Update Syntax (`..`)

Create a new instance that reuses most values from an existing one:

```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1  // take the rest from user1
};
```

`..user1` **must come last**. You can override as many fields as you want before it.

### Ownership impact

```
┌──────────────────────────────────────────────────────────────────┐
│  The update syntax uses = (assignment) → it MOVES heap data.     │
│                                                                  │
│  • If you override both String fields (email + username),        │
│    only Copy types (active, sign_in_count) come from user1       │
│    → user1 REMAINS valid                                         │
│                                                                  │
│  • If a String field comes from user1 (like username here),      │
│    that field is MOVED → user1 is NO LONGER valid as a whole     │
│    (but user1.email, which was not moved, can still be used)     │
└──────────────────────────────────────────────────────────────────┘
```

---

## 7.4 — Tuple Structs

Structs that look like tuples: they have a name but their fields are **unnamed**. Useful when naming each field would be redundant:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

### Key points

| Property | Detail |
|----------|--------|
| **Different types** | `Color` and `Point` are distinct types even if their fields are identical |
| **Access** | By index: `black.0`, `black.1`, `black.2` |
| **Destructure** | `let Color(r, g, b) = black;` |
| **Type safety** | A function taking `Color` won't accept a `Point` |

---

## 7.5 — Unit-Like Structs

Structs with **no fields at all**. Useful when you need to implement a trait on a type but don't need to store any data:

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

These behave similarly to `()` (the unit type). We'll see them in action when we cover traits (Chapter 10).

---

## 7.6 — Ownership of Struct Data

In the `User` struct, we used `String` (owned) rather than `&str` (borrowed). This is deliberate: each instance **owns all its data**, and the data lives as long as the struct.

### What if you try `&str`?

```rust
struct User {
    active: bool,
    username: &str,  // reference, not owned
    email: &str,
    sign_in_count: u64,
}
```

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:3:15
  |
3 |     username: &str,
  |               ^ expected named lifetime parameter
```

To store references in a struct, you need **lifetimes** (covered in Chapter 10). For now, use owned types like `String`.

---

## 7.7 — Example Program: Calculating Area

This section shows the evolution from raw variables → tuples → structs.

### Version 1: separate variables

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

**Problem**: the signature doesn't show that `width` and `height` are related.

### Version 2: tuple

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

**Better** (grouped), but `dimensions.0` and `dimensions.1` are cryptic. Easy to confuse width/height.

### Version 3: struct

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

**Best**: named fields, clear intent, and `area` takes an **immutable reference** (`&Rectangle`) so `main` keeps ownership.

### Comparison

| Approach | Clarity | Grouping | Ownership |
|----------|---------|----------|-----------|
| Separate variables | Low | None | N/A |
| Tuple | Medium | Yes, unnamed | N/A |
| Struct | **High** | Yes, named | `&Rectangle` borrows |

---

## 7.8 — Adding Useful Functionality with Derived Traits

### The `Debug` trait

Trying to `println!("{rect1}")` won't work — structs don't implement `Display` by default. But we can derive `Debug`:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    // Output: rect1 is Rectangle { width: 30, height: 50 }

    println!("rect1 is {:#?}", rect1);
    // Output (pretty-printed):
    // rect1 is Rectangle {
    //     width: 30,
    //     height: 50,
    // }
}
```

| Format specifier | Output style |
|-----------------|-------------|
| `{:?}` | Compact, single line |
| `{:#?}` | Pretty-printed, multi-line |

### The `dbg!` macro

An alternative for quick debugging. Unlike `println!`:
- Takes **ownership** of the expression (and returns it)
- Prints **file name + line number**
- Outputs to **stderr** (not stdout)

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),  // dbg! returns the value, so width = 60
        height: 50,
    };

    dbg!(&rect1);  // pass a reference to avoid losing ownership
}
```

```
[src/main.rs:10:16] 30 * scale = 60
[src/main.rs:14:5] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

---

## 7.9 — Method Syntax

Methods are functions defined **inside an `impl` block**, whose first parameter is always `self`:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

### The `self` parameter

`&self` is shorthand for `self: &Self`, where `Self` is the type the `impl` block is for.

| Signature | Meaning |
|-----------|---------|
| `&self` | Borrows immutably (read) |
| `&mut self` | Borrows mutably (modify) |
| `self` | Takes ownership (consume/transform — rare) |

### Methods with the same name as fields

```rust
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

// rect1.width   → the field (u32)
// rect1.width() → the method (bool)
```

Rust distinguishes by the presence of `()`. Methods that just return the field value are called **getters** — useful to make a field private while exposing read-only access.

### Automatic referencing and dereferencing

In C/C++ you need `->` to call methods through pointers. Rust has **no `->` operator**. Instead, when you call `object.method()`, Rust automatically adds `&`, `&mut`, or `*` to match the method signature:

```
p1.distance(&p2)      // Rust auto-borrows p1 as &p1
(&p1).distance(&p2)   // Equivalent — but nobody writes this
```

This works because methods have a clear receiver (`self`), so Rust can always determine the correct borrow type.

---

## 7.10 — Methods with More Parameters

Methods can take additional parameters after `self`:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));  // true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));  // false
}
```

---

## 7.11 — Associated Functions

Functions in an `impl` block that do **NOT** take `self` are called **associated functions**. They are called with `::` syntax, not dot notation:

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

let sq = Rectangle::square(3);
```

| | Method | Associated function |
|-|--------|-------------------|
| First param | `self` / `&self` / `&mut self` | No `self` |
| Call syntax | `rect1.area()` | `Rectangle::square(3)` |
| Common use | Operations on an instance | **Constructors** |
| Known example | — | `String::from("hello")` |

`Self` in the function body is an alias for the type after `impl` (here, `Rectangle`).

---

## 7.12 — Multiple `impl` Blocks

A struct can have **multiple** `impl` blocks. This is valid:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

There's no reason to split here, but this becomes useful with **generic types and traits** (Chapter 10).

---

## 7.13 — Summary

| Concept | Description | Example |
|---------|------------|---------|
| **Struct** | Named fields, custom type | `struct User { name: String }` |
| **Tuple struct** | Unnamed fields, named type | `struct Color(i32, i32, i32)` |
| **Unit struct** | No fields | `struct AlwaysEqual;` |
| **Field init shorthand** | Skip `field: field` | `User { email, .. }` |
| **Update syntax** | Copy fields from another instance | `User { email: x, ..user1 }` |
| **`#[derive(Debug)]`** | Enable `{:?}` / `{:#?}` formatting | `println!("{rect:?}")` |
| **`dbg!`** | Debug to stderr with file/line | `dbg!(&rect1)` |
| **Method** | Function with `self`, called with `.` | `rect.area()` |
| **Associated function** | No `self`, called with `::` | `Rectangle::square(3)` |
| **Multiple `impl`** | Split impl blocks (useful with generics) | Valid syntax |

```
┌──────────────────────────────────────────────────────────────────┐
│  Structs + methods = Rust's way of organizing data + behavior.   │
│  No classes, no inheritance — just types with associated         │
│  functions. Traits (Chapter 10) provide polymorphism.            │
└──────────────────────────────────────────────────────────────────┘
```
