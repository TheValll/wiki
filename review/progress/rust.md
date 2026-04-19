# Rust — Review Progress

**Current position:** 34/37 — *`#[test]`, assertion macros, custom messages*
**Last session:** 2026-04-18 (first formal session)
**Note:** Pre-system quiz covered book chapters 3-11. Levels below are imported conservatively — revise downward if you want tighter consolidation.

---

## Mastered (Level 4) — archived, no longer quizzed

*(none yet — move items here as they reach Level 4 in formal sessions)*

---

## In review (active spaced repetition)

Imported from the informal quiz we ran previously. Each concept is placed at Level 2 (seen and answered correctly at least once, but not in this formal system yet). Anything the user flagged as shaky is at Level 1.

| # | Concept | Level | Last seen | Note |
|---|---------|-------|-----------|------|
| 3 | Variables, mutability, shadowing vs `mut` | 2 | 2026-04-15 | Binding vs assignment understood |
| 4 | Scalar types, integer overflow, wrapping/checked/saturating/overflowing | 2 | 2026-04-15 | 4 explicit methods solid |
| 6 | Functions, statements vs expressions, `()` unit type | 2 | 2026-04-15 | Semicolon trick, `()` vocabulary |
| 7 | Control flow, `loop` with `break value`, `while` returns `()` | 2 | 2026-04-15 | — |
| 8 | Ownership rules, stack vs heap, move semantics | 2 | 2026-04-17 | Q5 discussed but not fully completed |
| 9-11 | References, slices, borrowing rules | 2 | — | Covered, not individually quizzed |
| 12-17 | Structs, methods, enums, `match`, `if let` | 2 | — | Covered, not individually quizzed |
| 18-19 | Modules, `pub`, visibility (struct fields vs enum variants) | 2 | 2026-04-18 | Struct-vs-enum asymmetry grasped |
| 20 | `use`, `as`, `pub use`, nested paths, glob | 1 | — | **Flagged: visibility rules to consolidate** |
| 21-23 | Collections (`Vec`, `String`, `HashMap`) | 2 | — | — |
| 24-27 | Error handling (`panic`, `Result`, `?`, validation types) | 1 | — | **Flagged: `?` vs `unwrap` distinction, `unwrap` acceptable cases** |
| 28-31 | Generics, traits, trait bounds | 2 | — | — |
| 32-33 | Lifetimes, elision rules | 1 | — | **Flagged: elision rules will become natural with practice** |
| 34 | `#[test]`, `assert!` / `assert_eq!` / `assert_ne!`, custom messages | 1 | 2026-04-18 | **Gaps: `.sum()` on empty iter, `Debug` required by `assert_eq!`, fluency writing float-tolerant tests** |
| 35-37 | `should_panic`, `Result`-returning tests, `cargo test` flags, unit vs integration | 2 | — | Chapter 11 just completed, not yet formally quizzed |

---

## Not yet reached (locked — do not quiz)

### Module 9 — Closures & iterators (chapter 13) — fiches written, pending read-through
- 38-45 (locked — unlock after user validates the fiches against the book)

### Module 10 — Cargo deep dive (chapter 14) — fiches written, pending read-through
- 46-52 (locked — unlock after user validates the fiches against the book)

Planned future additions: chapters 15+ of the Rust book (smart pointers, concurrency, OOP, patterns, advanced features, final project).

---

## Intuition drills (under-the-hood, no-formulas mode)

Concepts articulated in **intuition mode** — re-explained by the user in his own words, using his own analogies and schemas, **without code**. Source: [`rust/rust-intuition.md`](../../rust/rust-intuition.md).

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

## Priority consolidation areas (from past quiz feedback)

These are the weak spots that sessions should prioritize in warm-ups:

1. **Module visibility rules** (#18-19) — user initially assumed everything was accessible within the same crate. Clarify: private by default, `pub` on module vs `pub` on item, visibility of struct fields vs enum variants.
2. **`?` vs `unwrap`** (#25-26) — distinguish error propagation from panic. When is `unwrap` acceptable (logical impossibilities only)?
3. **Lifetime elision rules** (#33) — the 3 rules, when they apply, when to annotate manually.

---

## Session history

| Date | Session focus | Notes |
|------|---------------|-------|
| 2026-04-15 | Q1-Q4 — variables, overflow, expressions/statements, `loop` | All passed, solid mechanics |
| 2026-04-17 | Q5 — move semantics (partial) | Paused, move at some point |
| 2026-04-17 | Review system initialized | State imported from informal quiz |
| 2026-04-18 | Warm-up #18-19 (visibility) ✅ — Lesson #34 (`#[test]` + asserts) ⚠️ | Visibility asymmetry struct/enum solid. Tests: gaps on `.sum()` empty, `Debug`-for-`assert_eq`, float-test fluency |
