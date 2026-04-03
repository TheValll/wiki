# Part 1 — Installation, Hello World & Cargo

## 1.1 — What is Rust?

Imagine an **ultra-strict but helpful architect**. Before you lay a single brick, he reviews your blueprints and tells you:

- "This wall will collapse" (memory error)
- "You forgot to lock this door" (data race)
- "This room serves no purpose" (unused variable)

That architect is the **Rust compiler**. It refuses to build until everything is correct.

| Language | Memory Management | Speed | Memory Safety |
|----------|------------------|-------|---------------|
| **C/C++** | Manual (malloc/free) | Very fast | Not guaranteed (segfaults, buffer overflows) |
| **Java/Python/Go** | Garbage Collector (GC) | Medium | Yes, but GC pauses |
| **Rust** | Ownership (compile-time) | Very fast | Guaranteed at compilation, zero runtime cost |

Rust combines the **performance of C** with the **safety of Java**, without a garbage collector.

---

## 1.2 — Installing Rust

Rust is installed via **`rustup`**, a tool that manages Rust versions and associated tools.

### On Linux / macOS

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

A **linker** is also required (usually already present). On macOS:
```bash
$ xcode-select --install
```
On Ubuntu/Debian:
```bash
$ sudo apt install build-essential
```

### On Windows

Go to [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) and follow the instructions. Visual Studio will be needed for the linker and native libraries.

### What gets installed

```
rustup installs:
┌─────────────────────────────────────────┐
│  rustc        The Rust compiler         │
│  cargo        Build system & package    │
│               manager                   │
│  rustup       Version manager           │
│  std library  The standard library      │
│  rustfmt      Code formatter            │
│  clippy       Linter                    │
└─────────────────────────────────────────┘
```

### Verify the installation

```bash
$ rustc --version
rustc 1.85.0 (a34c079 2025-01-18)
```

### Useful commands

| Command | Action |
|---------|--------|
| `rustup update` | Update Rust |
| `rustup self uninstall` | Uninstall Rust |
| `rustup doc` | Open local documentation in the browser |

---

## 1.3 — Hello, World!

### Create the file

```bash
$ mkdir ~/projects/hello_world
$ cd ~/projects/hello_world
```

Create a file `main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

### Anatomy of the program

```
fn main() {
    println!("Hello, world!");
}
│       │        │
│       │        └── Macro (!) that prints text to the screen
│       └── Function body (between { })
└── Entry point: ALWAYS the first function to execute
```

Four important details:

| Element | Explanation |
|---------|------------|
| `fn main()` | Declares the `main` function — the entry point of every executable Rust program |
| `println!` | The `!` indicates this is a **macro**, not a regular function |
| `"Hello, world!"` | A **string literal** passed as an argument to the macro |
| `;` | The semicolon ends the statement. Most lines of Rust code end with one |

### Compile and run

```bash
$ rustc main.rs     # Compilation
$ ./main            # Execution (.\main.exe on Windows)
Hello, world!
```

### What happens behind the scenes?

```
                    Compilation              Execution
                    (once)                   (as many times as you want)

  main.rs ──────> [ rustc ] ──────> main (binary executable)
  (source code)   (compiler)        (standalone file)
                                         │
                                         ├── Contains ALL the machine code
                                         ├── No runtime dependencies
                                         └── Distributable as-is
```

Rust is an **ahead-of-time compiled** (AOT) language:

| Compiled language (Rust, C) | Interpreted language (Python, JS) |
|-----------------------------|----------------------------------|
| Source -> binary -> execution | Source -> interpreter -> execution |
| Recipient only needs the binary | Recipient needs the interpreter installed |
| Faster at runtime | Slower at runtime |
| Slower to compile | No compilation step |

---

## 1.4 — Cargo: The Project Manager

`rustc` is enough for a single file, but for real projects, we use **Cargo**. It is both:
- A **build system** (compiles your code)
- A **package manager** (manages dependencies = "crates")

### Create a project

```bash
$ cargo new hello_cargo
$ cd hello_cargo
```

### Generated structure

```
hello_cargo/
├── Cargo.toml          Project configuration
├── .gitignore          Git ignore file (created automatically)
└── src/
    └── main.rs         Source code
```

Cargo also initializes a **Git repository** automatically.

### Cargo.toml — The configuration file

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

| Section | Role |
|---------|------|
| `[package]` | Project metadata: name, version, Rust edition |
| `name` | Project name (used for the binary) |
| `version` | Project version (SemVer) |
| `edition` | Rust edition (determines language features available) |
| `[dependencies]` | List of external crates the project depends on |

The format is **TOML** (Tom's Obvious, Minimal Language).

### src/main.rs — The generated code

```rust
fn main() {
    println!("Hello, world!");
}
```

**Cargo convention**: all source code lives in `src/`. The root directory contains config, README, license, etc.

---

## 1.5 — Essential Cargo Commands

### `cargo build` — Compile

```bash
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

The binary is created in `target/debug/hello_cargo`:

```
hello_cargo/
├── Cargo.toml
├── Cargo.lock          ← New! Locks dependency versions
├── src/
│   └── main.rs
└── target/
    └── debug/
        └── hello_cargo ← The compiled binary
```

### `cargo run` — Compile AND execute

```bash
$ cargo run
   Compiling hello_cargo v0.1.0
    Finished dev [unoptimized + debuginfo] target(s)
     Running `target/debug/hello_cargo`
Hello, world!
```

**Smart**: if the source code hasn't changed, Cargo skips compilation and directly runs the existing binary.

### `cargo check` — Verify without compiling

```bash
$ cargo check
   Checking hello_cargo v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

- Checks that the code compiles without errors
- Does **NOT** produce a binary
- Much faster than `cargo build`
- Use it in a loop during development

### `cargo build --release` — Optimized compilation

```bash
$ cargo build --release
```

The binary is created in `target/release/` (instead of `target/debug/`).

### Summary table

| Command | Produces binary? | Optimized? | Destination | Usage |
|---------|-----------------|-----------|-------------|-------|
| `cargo check` | No | — | — | Quick syntax verification |
| `cargo build` | Yes | No (debug) | `target/debug/` | Development |
| `cargo run` | Yes + executes | No (debug) | `target/debug/` | Development (most used) |
| `cargo build --release` | Yes | Yes | `target/release/` | Production / benchmarks |

### Two compilation profiles

```
DEBUG profile (default)             RELEASE profile (--release)
┌──────────────────────────┐        ┌──────────────────────────┐
│ - Fast compilation       │        │ - Slow compilation       │
│ - Unoptimized code       │        │ - Optimized code         │
│ - Debug info included    │        │ - No debug info          │
│ - target/debug/          │        │ - target/release/        │
│ - For development        │        │ - For distribution       │
└──────────────────────────┘        └──────────────────────────┘
```

---

## 1.6 — Cargo.lock & Reproducibility

When you run `cargo build` for the first time, Cargo:

1. Reads the version constraints in `Cargo.toml`
2. Resolves the exact versions of all dependencies
3. Writes those exact versions in **`Cargo.lock`**

```
Cargo.toml (what you want)         Cargo.lock (what you have)
┌──────────────────────┐           ┌──────────────────────┐
│ rand = "0.8.5"       │  ──────>  │ rand 0.8.5           │
│ (= ^0.8.5 = >=0.8.5 │           │ rand_core 0.6.4      │
│  and <0.9.0)         │           │ getrandom 0.2.15     │
└──────────────────────┘           │ ...exact versions    │
                                   └──────────────────────┘
```

| File | Role | Commit to Git? |
|------|------|----------------|
| `Cargo.toml` | Version constraints (flexible) | Yes |
| `Cargo.lock` | Exact resolved versions (frozen) | Yes (for binaries) |

- **Reproducible builds**: every `cargo build` uses the same versions
- **Update**: `cargo update` recalculates versions (within `.toml` constraints)

```bash
$ cargo update
    Updating crates.io index
    Updating rand v0.8.5 -> v0.8.6    # Within ^0.8.5 constraint
```

To change major versions (e.g., 0.9.x), you must manually edit `Cargo.toml`.

---

## 1.7 — Summary

| Concept | Key Takeaway |
|---------|-------------|
| **Rust** | Compiled language, no GC, memory safety guaranteed at compile time |
| **rustup** | Installs and manages Rust versions |
| **rustc** | The compiler (rarely used directly) |
| **Cargo** | Build system + package manager (always used) |
| **Cargo.toml** | Project configuration (TOML format) |
| **Cargo.lock** | Exact dependency versions (reproducibility) |
| **cargo run** | The everyday command: compile + execute |
| **cargo check** | Fast verification without producing a binary |
