# Part 16 — More About Cargo & Crates.io

Beyond the basics (`cargo new`, `cargo build`, `cargo run`, `cargo test`), Cargo offers tools for **shipping** your code to the wider Rust ecosystem and for **managing multi-crate projects**.

| Topic | Purpose |
|-------|---------|
| **Release profiles** | Tune compiler behaviour per build type (`dev` vs `release`) |
| **Publishing to crates.io** | Share your crate publicly on the official registry |
| **Workspaces** | Develop multiple related crates side by side in one repo |
| **`cargo install`** | Install binary crates from crates.io into your user-wide `$PATH` |
| **Custom cargo commands** | Extend Cargo with any `cargo-<name>` binary in `$PATH` |

---

## 16.1 — Customizing Builds with Release Profiles

Cargo has two **built-in profiles**:

| Profile | Invoked by | Default optimisation level |
|---------|------------|----------------------------|
| `dev` | `cargo build` | `opt-level = 0` — fast compile, no optimisation |
| `release` | `cargo build --release` | `opt-level = 3` — slow compile, maximum optimisation |

### Tuning a profile

Add a `[profile.<name>]` section in `Cargo.toml`:

```toml
[profile.dev]
opt-level = 1

[profile.release]
opt-level = 3
```

Common tunables:

| Setting | Meaning |
|---------|---------|
| `opt-level` | 0 (none) to 3 (max). `"s"` or `"z"` optimize for size. |
| `debug` | Include debug info (`true`, `false`, `"line-tables-only"`) |
| `lto` | Link-Time Optimisation: `false`, `"thin"`, `true` (fat) |
| `codegen-units` | Parallelism vs optimisation trade-off (1 = slowest, best) |
| `panic` | `"unwind"` (default) or `"abort"` (smaller binary, no cleanup) |

```
┌──────────────────────────────────────────────────────────────────┐
│  Defaults are sensible. Touch profiles only when you have a      │
│  specific reason (binary size, stack traces in release, faster   │
│  dev-profile runs for benchmarks).                                │
└──────────────────────────────────────────────────────────────────┘
```

---

## 16.2 — Publishing a Crate to Crates.io

### Step 1 — Useful documentation comments

Rust has two documentation-comment syntaxes:

| Syntax | Documents | Where |
|--------|-----------|-------|
| `///` | The **item immediately after** (function, struct, ...) | Placed before the item |
| `//!` | The **enclosing item** (the module or crate itself) | At the top of a module or `lib.rs` |

Both accept Markdown:

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let result = my_crate::add_one(5);
/// assert_eq!(6, result);
/// ```
pub fn add_one(x: i32) -> i32 { x + 1 }
```

`cargo doc` generates an HTML rendering in `target/doc/`. `cargo doc --open` opens it in your browser. `cargo test` also **executes code inside `# Examples`** — a beautifully simple way to keep examples correct over time.

Common section headings recognised by convention (not enforced, but rendered):

| Heading | Purpose |
|---------|---------|
| `# Examples` | Usage examples (run as tests) |
| `# Panics` | Conditions that would cause the function to panic |
| `# Errors` | For functions returning `Result`, what errors mean |
| `# Safety` | For `unsafe` functions, what invariants the caller must uphold |

### Step 2 — Re-exporting with `pub use`

Users of your crate see whatever paths you expose. If your internal structure is deep (`mycrate::kinds::color::Red`), re-export commonly-used items at the top level:

```rust
// in lib.rs
pub use self::kinds::color::Red;
pub use self::utils::mix;
```

Now users just write `mycrate::Red` and `mycrate::mix`. Your *internal* organisation can remain whatever is cleanest for maintainers; the *public API* is decoupled from it.

### Step 3 — Set up an account on crates.io

- Log in with GitHub on [crates.io](https://crates.io).
- Get your API token from the account settings page.
- Run `cargo login <token>` once — the token is stored in `~/.cargo/credentials.toml`.

### Step 4 — Add metadata in Cargo.toml

Before `cargo publish` will accept your crate, Cargo requires the `[package]` section to include:

```toml
[package]
name = "my_unique_crate_name"
version = "0.1.0"
edition = "2024"
description = "A short blurb that shows up on crates.io."
license = "MIT OR Apache-2.0"
```

Other useful fields: `authors`, `repository`, `homepage`, `documentation`, `readme`, `keywords`, `categories`.

The **name is first-come, first-served** globally on crates.io — pick carefully.

### Step 5 — Publish

```bash
cargo publish
```

This packages your crate and uploads it. The upload is **permanent** — you can never delete a published version (so that anyone depending on it cannot suddenly lose their dependency).

### Step 6 — New versions

Bump the `version` field following SemVer, then `cargo publish` again:

| Version bump | When |
|--------------|------|
| `MAJOR` (1.0.0 → 2.0.0) | Breaking API changes |
| `MINOR` (1.0.0 → 1.1.0) | Backward-compatible additions |
| `PATCH` (1.0.0 → 1.0.1) | Backward-compatible bug fixes |

### Step 7 — Deprecating with `cargo yank`

If a published version has a serious bug, you cannot delete it, but you can **yank** it:

```bash
cargo yank --vers 1.0.1
cargo yank --vers 1.0.1 --undo   # reverse the yank
```

A yanked version:
- Can still be downloaded by existing `Cargo.lock` files (no breakage for anyone who already has it)
- Will NOT be selected by any new `cargo` resolution

```
┌──────────────────────────────────────────────────────────────────┐
│  Yanking is for "please stop using this version". It is not a    │
│  mechanism for hiding secrets or sensitive content — the code    │
│  remains publicly downloadable.                                  │
└──────────────────────────────────────────────────────────────────┘
```

---

## 16.3 — Cargo Workspaces

A **workspace** groups several related crates under one `Cargo.toml` and shares a single `Cargo.lock` and `target/` directory. Ideal when your project naturally splits into multiple crates (e.g., a library plus one or more binaries using it).

### Structure

```
add/
├── Cargo.toml          ← workspace manifest (no [package], only [workspace])
├── Cargo.lock          ← single lockfile for all crates
├── target/             ← shared build output
├── adder/              ← binary crate
│   ├── Cargo.toml
│   └── src/main.rs
├── add_one/            ← library crate
│   ├── Cargo.toml
│   └── src/lib.rs
└── add_two/            ← another library crate
    ├── Cargo.toml
    └── src/lib.rs
```

Top-level `Cargo.toml`:

```toml
[workspace]
resolver = "3"
members = ["adder", "add_one", "add_two"]
```

### Depending on a workspace sibling

In `adder/Cargo.toml`:

```toml
[dependencies]
add_one = { path = "../add_one" }
```

Running `cargo build` from the workspace root builds everything. Running `cargo run -p adder` runs a specific member.

### Shared external dependencies

If two member crates use `rand = "0.8"`, a single version is compiled and shared — because all members share one `Cargo.lock`. You cannot accidentally mix incompatible versions.

### Tests

```bash
cargo test              # run tests in every member
cargo test -p add_one   # run tests in only add_one
```

### Publishing workspace members

Each member crate must be published **individually** — `cargo publish -p <crate>`. Crates.io doesn't know about workspaces.

---

## 16.4 — Installing Binaries with `cargo install`

```bash
cargo install ripgrep
```

This downloads the source of a crate that provides a binary (has `src/main.rs` or `[[bin]]`), compiles it in release mode, and copies the resulting executable to `~/.cargo/bin/` (which should be in your `$PATH`).

The result is a user-wide installation that doesn't touch your system package manager — similar to `pipx install` for Python or `npm install -g` for JavaScript, but without a central daemon or privileged install.

Useful flags:

| Flag | Purpose |
|------|---------|
| `--path .` | Install from a local directory (great for testing your own crate) |
| `--force` | Reinstall, even if already present |
| `--version <v>` | Pin to a specific version |

---

## 16.5 — Extending Cargo with Custom Commands

Cargo looks at your `$PATH`. If it finds a binary named `cargo-<something>`, you can invoke it as `cargo <something>`:

```bash
# if you have "cargo-watch" installed in PATH:
cargo watch -x test    # runs `cargo test` every time a file changes
```

This is the extension mechanism that makes the ecosystem around Cargo so rich:

| Popular extension | Purpose |
|-------------------|---------|
| `cargo-watch` | Re-run commands on file changes |
| `cargo-edit` | Add / upgrade dependencies from the CLI |
| `cargo-expand` | Print macro-expanded source |
| `cargo-audit` | Scan `Cargo.lock` for security advisories |
| `cargo-tree` | Display the dependency tree |

`cargo --list` enumerates all `cargo-*` binaries it can see in `$PATH`.

---

## 16.6 — Summary

| Feature | Key point |
|---------|-----------|
| **Release profiles** | `[profile.dev]` / `[profile.release]` in `Cargo.toml` — tune `opt-level`, `lto`, `panic`, `codegen-units` |
| **Documentation comments** | `///` for the next item, `//!` for the enclosing module/crate; Markdown; `cargo doc` renders, `cargo test` runs examples |
| **`pub use`** | Re-export deep items at the top level — decouple internal layout from public API |
| **Publishing** | `cargo login` once, then `cargo publish`. Versions are permanent — use `cargo yank` to retire, never to delete |
| **Workspaces** | `[workspace]` in root `Cargo.toml` + `members = [...]`. Single lockfile, shared `target/`, explicit `path = "…"` for sibling deps |
| **`cargo install`** | User-wide binary install into `~/.cargo/bin/` — the `pipx` of Rust |
| **`cargo-<name>`** | Any binary in `$PATH` named `cargo-xyz` becomes `cargo xyz` |

> For the plain-language intuition (no code), see [`rust-intuition.md`](./rust-intuition.md) Part 6.
