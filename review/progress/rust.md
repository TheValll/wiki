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

*(none — entire book chapters 1-11 are in scope; rest of the book will be added as user progresses past chapter 11)*

Planned future additions: chapters 12+ of the Rust book (I/O project, iterators and closures, Cargo, smart pointers, concurrency, OOP, patterns, advanced features, final project).

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
