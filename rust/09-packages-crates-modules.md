# Part 9 — Managing Growing Projects with Packages, Crates, and Modules

As a project grows, you need a way to organize code into separate files, reuse it, and control which parts are visible from the outside. Rust's **module system** gives you four tools for this: **packages**, **crates**, **modules**, and **paths**.

---

## 9.1 — Packages and Crates

A **crate** is the smallest unit the Rust compiler considers at a time. There are two kinds:

| Crate type | Entry file | Produces |
|------------|-----------|----------|
| **Binary crate** | `src/main.rs` | An executable |
| **Library crate** | `src/lib.rs`  | Code meant to be shared (no `main` function) |

The entry file is called the **crate root** — the compiler starts reading from there.

A **package** is one or more crates bundled with a `Cargo.toml`. A package must contain at least one crate, and can contain **at most one library crate** but many binary crates.

```
game_of_life/            <- package
├── Cargo.toml           <- describes the package
├── Cargo.lock
└── src/
    └── main.rs          <- crate root of the binary crate
```

```
┌──────────────────────────────────────────────────────────────────┐
│  Package  →  contains 0-1 library crate + N binary crates        │
│  Crate    →  a tree of modules, compiled as one unit             │
│  Module   →  a namespace inside a crate                          │
│  Path     →  how you name an item inside the module tree         │
└──────────────────────────────────────────────────────────────────┘
```

### Binary crates in `src/bin/`

Any `.rs` file placed in `src/bin/` becomes an **additional binary crate**:

```
my_package/
├── Cargo.toml
└── src/
    ├── lib.rs           <- library crate (optional)
    ├── main.rs          <- default binary
    └── bin/
        ├── tool_a.rs    <- extra binary: `cargo run --bin tool_a`
        └── tool_b.rs    <- extra binary: `cargo run --bin tool_b`
```

---

## 9.2 — Defining Modules to Control Scope and Privacy

A **module** groups related items (functions, structs, enums, other modules) under a name and controls their visibility. You declare a module with the `mod` keyword:

```rust
mod garden {
    pub mod vegetables {
        pub struct Asparagus {}
    }
}
```

### The compiler's module lookup rules

When the compiler sees `mod garden;` in the crate root, it looks for the module's code in this order:

1. **Inline** — inside curly brackets after `mod garden { ... }`
2. `src/garden.rs`
3. `src/garden/mod.rs` (older style)

For **submodules** (e.g., `mod vegetables;` inside `garden`):

1. Inline inside `mod vegetables { ... }`
2. `src/garden/vegetables.rs`
3. `src/garden/vegetables/mod.rs`

### Privacy: everything is private by default

Items inside a module are **private to their parent module** unless marked `pub`. Child modules can see items of their ancestors, but ancestors can't see children's private items.

```rust
mod front_of_house {
    pub mod hosting {           // module must be pub to be reached
        pub fn add_to_waitlist() {}   // and the function too
    }
}

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
}
```

```
┌──────────────────────────────────────────────────────────────────┐
│  pub on a module      →  lets parents SEE the module             │
│  pub on a function    →  lets the function be CALLED from        │
│                          outside the module                      │
│  Both are required to reach `add_to_waitlist` from outside.      │
└──────────────────────────────────────────────────────────────────┘
```

---

## 9.3 — Paths for Referring to an Item in the Module Tree

A **path** names an item in the module tree. There are two kinds:

| Path type | Starts with | Example |
|-----------|------------|---------|
| **Absolute** | `crate::` (the crate root) | `crate::front_of_house::hosting::add_to_waitlist()` |
| **Relative** | a name in the current scope | `front_of_house::hosting::add_to_waitlist()` |

### `super` — go up one level

`super` is like `..` in a filesystem: it refers to the parent module.

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();   // calls the parent's deliver_order
    }

    fn cook_order() {}
}
```

### `pub` on structs vs enums

For a `struct`, `pub` on the struct itself does **not** make the fields public — you mark each field individually:

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,   // still private
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
```

For an `enum`, `pub` on the enum makes **all variants public** — which is almost always what you want.

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
```

---

## 9.4 — Bringing Paths into Scope with `use`

Writing `front_of_house::hosting::add_to_waitlist()` everywhere gets tiring. `use` creates a **shortcut** in the current scope:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();   // no more long path
}
```

### Idiomatic `use` paths

- For **functions**: `use` the **parent module**, then call `parent::function()`. This makes it clear the function is not local.
- For **structs, enums, and other items**: `use` the **full path** directly.

```rust
use std::collections::HashMap;   // bring the type in directly

let mut map = HashMap::new();
map.insert(1, 2);
```

### Handling name clashes with `as`

Two types with the same name? Rename one with `as`:

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn f1() -> Result      { /* ... */ }
fn f2() -> IoResult<()> { /* ... */ }
```

### Re-exporting with `pub use`

`pub use` brings a name into scope **and** makes it available for outside code to use through your module:

```rust
pub use crate::front_of_house::hosting;
```

Now external code can write `your_crate::hosting::add_to_waitlist()` instead of the deeper path.

### Nested paths and globs

```rust
// Instead of:
use std::cmp::Ordering;
use std::io;

// Write:
use std::{cmp::Ordering, io};

// Bring in `self` + children:
use std::io::{self, Write};

// Glob — everything public in a module (use sparingly, mostly for tests/prelude):
use std::collections::*;
```

### External crates

In `Cargo.toml`:

```toml
[dependencies]
rand = "0.8.5"
```

In code:

```rust
use rand::Rng;

let n = rand::thread_rng().gen_range(0..100);
```

The standard library (`std`) is also a crate, but it ships with Rust — no entry in `Cargo.toml` needed.

---

## 9.5 — Separating Modules into Different Files

As modules grow, move them out of the crate root into their own files.

### Single-file module

**`src/lib.rs`** (or `main.rs`):

```rust
mod front_of_house;          // load from src/front_of_house.rs

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

**`src/front_of_house.rs`**:

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

### Submodule in its own file

If `hosting` also gets big, split it further:

**`src/front_of_house.rs`**:

```rust
pub mod hosting;             // load from src/front_of_house/hosting.rs
```

**`src/front_of_house/hosting.rs`**:

```rust
pub fn add_to_waitlist() {}
```

```
┌──────────────────────────────────────────────────────────────────┐
│  Key rule: `mod foo;` is declared ONCE in the module tree —      │
│  not repeated in every file that uses items from foo.            │
│  To USE items from foo elsewhere, write `use crate::foo::...`.   │
└──────────────────────────────────────────────────────────────────┘
```

---

## 9.6 — A Real-World Example: Game of Life

Look at the actual structure of the `game_of_life` crate — it uses every concept from this chapter.

### File tree

```
game_of_life/
├── Cargo.toml
└── src/
    ├── main.rs              <- binary crate root
    ├── grid.rs              <- module `grid`
    ├── grid/
    │   └── cell.rs          <- submodule `grid::cell`
    ├── game.rs              <- module `game`
    └── game/
        └── rules.rs         <- submodule `game::rules`
```

### `src/main.rs` — declaring modules and bringing items into scope

```rust
pub mod grid;
pub mod game;

use std::{thread, time};
use rand::Rng;
use grid::display_grid;
use game::rules::apply_rules;
use grid::cell::Cells;
```

| Line | What it does |
|------|--------------|
| `pub mod grid;` | Tells the compiler "load `src/grid.rs` as module `grid`" |
| `pub mod game;` | Same for `src/game.rs` |
| `use std::{thread, time};` | Nested path — brings two `std` modules in at once |
| `use rand::Rng;` | Brings the `Rng` trait from the external `rand` crate |
| `use grid::display_grid;` | Brings one function from the `grid` module |
| `use game::rules::apply_rules;` | Traverses two module levels |
| `use grid::cell::Cells;` | Brings the `Cells` enum into scope |

### `src/grid.rs` — module with a submodule + public function

```rust
pub mod cell;               // load src/grid/cell.rs
use cell::Cells;            // relative use: cell is a child of grid

pub fn display_grid(grid: &Vec<Vec<Cells>>) {
    // ...
}
```

### `src/grid/cell.rs` — leaf module with a public enum

```rust
#[derive(Clone, Copy)]
pub enum Cells {
    Alive,
    Dead,
}
```

Because `Cells` is `pub`, both variants (`Alive`, `Dead`) are exported automatically.

### `src/game/rules.rs` — accessing a sibling module via `crate::`

```rust
use crate::grid::cell::Cells;   // absolute path from crate root

pub fn apply_rules(grid: &Vec<Vec<Cells>>, generation: u32)
    -> (Vec<Vec<Cells>>, u32)
{
    // ...
}
```

From inside `game::rules`, the `grid` module is a **sibling** (same level in the tree). The easiest way to reach it is with an **absolute path** starting at `crate::`.

### Module tree diagram

```
crate
├── grid               (pub)
│   ├── display_grid   (pub fn)
│   └── cell           (pub mod)
│       └── Cells      (pub enum)
└── game               (pub)
    └── rules          (pub mod)
        └── apply_rules (pub fn)
```

---

## 9.7 — Summary

| Concept | Purpose | Example |
|---------|---------|---------|
| **Package** | Cargo project with `Cargo.toml` | `game_of_life/` |
| **Crate** | Compilation unit (binary or library) | `src/main.rs`, `src/lib.rs` |
| **Module** | Namespace inside a crate | `mod grid;` |
| **`pub`** | Makes an item visible to parents | `pub fn display_grid(...)` |
| **Absolute path** | Starts at `crate::` | `crate::grid::cell::Cells` |
| **Relative path** | Starts at current scope | `cell::Cells` |
| **`super`** | Go up one module level | `super::deliver_order()` |
| **`use`** | Bring a path into the current scope | `use grid::cell::Cells;` |
| **`use` + `as`** | Rename on import | `use std::io::Result as IoResult;` |
| **`pub use`** | Re-export to the outside world | `pub use crate::front_of_house::hosting;` |
| **Nested path** | Multiple `use`s in one line | `use std::{cmp::Ordering, io};` |
| **Module file lookup** | `mod foo;` → `foo.rs` or `foo/mod.rs` | `mod grid;` → `src/grid.rs` |

```
┌──────────────────────────────────────────────────────────────────┐
│  Modules are how you keep Rust projects readable as they grow:   │
│  split code by responsibility, expose a clean public API with    │
│  `pub`, and use `use` to keep call sites short.                  │
└──────────────────────────────────────────────────────────────────┘
```
