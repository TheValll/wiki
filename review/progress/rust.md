# Rust — Review Progress

**Review flow:** Flow C — Competence validation (see [`../AGENT.md`](../AGENT.md) §3.C)
**Current position:** 4 concepts in progress / awaiting validation
**Last session:** 2026-04-18 (first formal session — pre-migration)
**Note:** On 2026-04-21 this file was migrated from Flow M (levels 0-4, spaced repetition) to Flow C (binary validation). Concepts that had reached Level 2 without a flagged consolidation note were auto-promoted to Validated. Concepts flagged in the previous "Priority consolidation areas" section were placed In progress for their upcoming validation session.

---

## Validated — concept has passed competence validation, no longer in active review

Post-validation retention comes from actual Rust coding on personal projects (DeepSight, Rust book exercises). These concepts are not re-drilled; if the user reports a real-world gap on a validated concept, he explicitly requests to re-open it.

### Language basics
- **#3** — Variables, mutability, shadowing vs `mut` *(validated 2026-04-15)*
- **#4** — Scalar types, integer overflow, wrapping/checked/saturating/overflowing *(validated 2026-04-15)*
- **#6** — Functions, statements vs expressions, `()` unit type *(validated 2026-04-15)*
- **#7** — Control flow, `loop` with `break value`, `while` returns `()` *(validated 2026-04-15)*

### Ownership & borrowing
- **#8** — Ownership rules, stack vs heap, move semantics *(validated 2026-04-17)*
- **#9-11** — References, slices, borrowing rules *(auto-migrated 2026-04-21)*

### Structs & enums
- **#12-17** — Structs, methods, enums, `match`, `if let` *(auto-migrated 2026-04-21)*

### Modules & paths
- **#18-19** — Modules, `pub`, visibility (struct fields vs enum variants) *(validated 2026-04-18)*

### Collections
- **#21-23** — `Vec`, `String`, `HashMap` *(auto-migrated 2026-04-21)*

### Generics & traits
- **#28-31** — Generics, traits, trait bounds *(auto-migrated 2026-04-21)*

### Testing (partial)
- **#35-37** — `should_panic`, `Result`-returning tests, `cargo test` flags, unit vs integration *(auto-migrated 2026-04-21)*

---

## In progress — pending validation session

Each of these is the next candidate for a Flow C validation block (§3.C). Gaps noted from prior quiz work are the things to address in the Step 3 exercise battery.

| # | Concept | Gaps to address in the validation session |
|---|---------|--------------------------------------------|
| 20 | `use`, `as`, `pub use`, nested paths, glob | Visibility rules — private by default, `pub` on module vs `pub` on item, visibility of struct fields vs enum variants. User initially assumed everything was accessible within the same crate. |
| 24-27 | Error handling (`panic`, `Result`, `?`, validation types) | `?` vs `unwrap` distinction; when `unwrap` is acceptable (logical impossibilities only); propagation patterns with custom error types. |
| 32-33 | Lifetimes, elision rules | The 3 elision rules, when they apply, when to annotate manually. User noted this will become natural with practice — validation bar stays at "able to read & annotate correctly under exercise pressure". |
| 34 | `#[test]`, `assert!` / `assert_eq!` / `assert_ne!`, custom messages | `.sum()` on empty iter (type inference), `Debug` required by `assert_eq!`, fluency writing float-tolerant tests, custom assertion messages. |

---

## Not yet reached (locked — do not quiz)

### Module 9 — Closures & iterators (chapter 13) — fiches written, pending read-through
- 38-45 (locked — unlock after user validates the fiches against the book)

### Module 10 — Cargo deep dive (chapter 14) — fiches written, pending read-through
- 46-52 (locked — unlock after user validates the fiches against the book)

Planned future additions: chapters 15+ of the Rust book (smart pointers, concurrency, OOP, patterns, advanced features, final project).

---

## Intuition drills (under-the-hood, no-formulas mode)

Intuition drills are **orthogonal to Flow C** — run as standalone articulation sessions when the user explicitly requests one, not embedded in the validation flow. Source: [`rust/rust-intuition.md`](../../rust/rust-intuition.md).

Format: each successful articulation = "validated". A second successful articulation a few weeks later = "consolidated".

| Section | Concept | First validated | Consolidated | Notes |
|---------|---------|-----------------|--------------|-------|
| 1.1 | The single-owner rule | — | — | Pending first articulation drill |
| 1.2 | Move vs Copy | — | — | Pending first articulation drill |
| 1.3 | Borrowing: sharing without giving up | — | — | Pending first articulation drill |
| 1.4 | The two borrowing states (and why) | — | — | Pending first articulation drill |
| 1.5 | Lifetimes as proofs | — | — | Pending first articulation drill |
| 1.6 | Slices: views into owned memory | — | — | Pending first articulation drill |
| 2.1 | Enums as sum types (Option / Result) | — | — | Pending first articulation drill |
| 2.2 | Pattern matching and exhaustiveness | — | — | Pending first articulation drill |
| 2.3 | Modules and the visibility rule | — | — | Pending first articulation drill |
| 3.1 | Traits: shared behavior without inheritance | — | — | Pending first articulation drill |
| 3.2 | Generics and monomorphization | — | — | Pending first articulation drill |
| 3.3 | Trait bounds: the contract on T | — | — | Pending first articulation drill |
| 3.4 | Trait objects: the dynamic alternative | — | — | Pending first articulation drill |
| 4.1 | Panic vs Result: two ways to fail | — | — | Pending first articulation drill |
| 4.2 | The `?` operator: the propagation shortcut | — | — | Pending first articulation drill |
| 5.1 | Closures: functions that remember | — | — | Chapter 13 — in reading 2026-04-18 |
| 5.2 | Fn / FnMut / FnOnce: the three capture kinds | — | — | Chapter 13 — in reading 2026-04-18 |
| 5.3 | The `move` keyword | — | — | Chapter 13 — in reading 2026-04-18 |
| 5.4 | Iterators: lazy pipelines | — | — | Chapter 13 — in reading 2026-04-18 |
| 6.1 | Release profiles: two kitchens, two priorities | — | — | Chapter 14 — fiche written pre-read 2026-04-18 |
| 6.2 | Crates.io: the shared pantry | — | — | Chapter 14 — fiche written pre-read 2026-04-18 |
| 6.3 | Workspaces: one repo, many crates | — | — | Chapter 14 — fiche written pre-read 2026-04-18 |
| 6.4 | `cargo install`: the personal tool shelf | — | — | Chapter 14 — fiche written pre-read 2026-04-18 |

---

## Session history

| Date | Session focus | Notes |
|------|---------------|-------|
| 2026-04-15 | Q1-Q4 — variables, overflow, expressions/statements, `loop` | All passed, solid mechanics |
| 2026-04-17 | Q5 — move semantics (partial) | Paused, move at some point |
| 2026-04-17 | Review system initialized | State imported from informal quiz (under Flow M) |
| 2026-04-18 | Warm-up #18-19 (visibility) OK — Lesson #34 (`#[test]` + asserts) gaps | Visibility asymmetry struct/enum solid. Tests: gaps on `.sum()` empty, `Debug`-for-`assert_eq`, float-test fluency |
| 2026-04-21 | **Migration Flow M → Flow C** | Auto-promoted Lv 2 unflagged concepts to Validated (#3, #4, #6, #7, #8, #9-11, #12-17, #18-19, #21-23, #28-31, #35-37). Kept In progress: #20, #24-27, #32-33, #34. "Priority consolidation areas" section removed — gaps inlined in "In progress" table. |
