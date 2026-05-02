# Rust

This domain is a thin layer on top of the official [Rust book](https://doc.rust-lang.org/book/) (2024 edition). The book is canonical, free, and well-structured — re-summarising every chapter would just create drift. So this folder holds only what's **additive**:

- **[`rust-intuition.md`](./rust-intuition.md)** — every concept of the 21 chapters in 2-4 lines (what + how). A bookmark layer for spot-checks, recaps, and drills.
- **[`../raw/rust-book/`](../raw/rust-book/)** — local mirror of the 2024-edition source markdown (cloned from [rust-lang/book](https://github.com/rust-lang/book)). Ground truth for any synthesis.

For the full content — examples, error walkthroughs, exercises — open the book directly.

## Refresh the local copy

When the book updates upstream:

```bash
git clone --depth=1 https://github.com/rust-lang/book.git /tmp/_rb && \
  cp /tmp/_rb/src/*.md raw/rust-book/ && rm -rf /tmp/_rb
```
