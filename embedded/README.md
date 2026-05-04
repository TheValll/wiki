# Embedded — Wiki Domain

Embedded systems concepts and embedded Rust on Espressif (ESP32) targets. Built around *Simplified Embedded Rust: ESP Core Library Edition* by Omar Hiari (v1.0, May 2024) — the ground truth for the chapter content. The book uses **bare-metal `no_std`** with `esp-hal`.

> **At a glance:** [`RECAP.md`](./RECAP.md) — single-glance summary of every concept in this domain.
>
> **Source material:** [`raw/embedded-book/`](../raw/embedded-book/) — Hiari's PDF, plus the GitHub repo at <https://github.com/theembeddedrustacean/ser-no-std> for code examples and updates.

---

## Why this domain exists

Unlike the [`rust/`](../rust/README.md) domain — where the official Rust book is canonical and well-organized — the embedded Rust ecosystem is more scattered: multiple HALs, multiple platforms, fast-evolving crates. The Hiari book is the most coherent single source for ESP32 + bare-metal that this wiki has found, so it earns a domain.

Companion material (free):
- [The Rust on ESP Book (Espressif)](https://docs.esp-rs.org/book/) — official, complementary
- [Embedded Rust Book (rust-embedded WG)](https://docs.rust-embedded.org/book/) — generalist embedded
- [The Embedded Rustacean blog (Hiari)](https://www.theembeddedrustacean.com) — extended articles

---

## Conventions

- **Pattern**: Layered (Pattern B in [`domains.md`](../domains.md)) — each file is a topic with internal facets (analogy / concept / diagram / details / quick reference).
- **File naming**: `X.Y-slug.md` mirroring the book's chapter.section numbering directly. Example: book §2.6 → `02-microcontroller-concepts/2.6-polling-interrupts.md`. This way, while reading the book, you find the matching wiki file in seconds.
- **Internal sections**: `## X.Y.Z — title` to keep granularity readable on dense topics.
- **Source ground truth**: `raw/embedded-book/`. If the wiki page contradicts the book, the book wins.
- **Code language**: Rust by default (the book's target). Brief asides in C only when illustrating a hardware-level interaction.

---

## Syllabus

| Chapter | Status | Topics |
|---|---|---|
| 01 — Introduction | *(planned)* | Embedded systems, trends, why Rust |
| **02 — Microcontroller Concepts** | **complete (2026-05-04)** | MCU vs MPU, architectures, peripherals, pins, interrupts, memory, clocks, toolchains |
| 03 — Embedded Rust & ESP | *(planned)* | ESPs overview, hardware, software ecosystem, std vs core lib |
| 04+ — Project setup & peripherals | *(planned)* | Per-peripheral chapters (GPIO, ADC, timers, …) |

See [`02-microcontroller-concepts/README.md`](./02-microcontroller-concepts/README.md) for the chapter-level TOC.

---

## Cross-domain links

| From | To | Why |
|---|---|---|
| **Embedded Rust no_std patterns** | [`../rust/rust-intuition.md`](../rust/rust-intuition.md) | `core` vs `std`, `#![no_std]`, panic handlers (advanced Rust topics) |
| **Memory layout, registers, MCU architecture** | *(planned `low-level/`)* | Memory, pointers, OS internals overlap |
| **Pin protocols (UART, SPI, I²C)** | *(planned `electronics/`)* | Signal-level details |
| **Interrupts, real-time control** | DeepSight-Nebula project (off-wiki) | Direct application of these concepts |

---

## Status

Started 2026-05-04. **Chapter 2 (Microcontroller Concepts) complete** as of 2026-05-04 — see [`RECAP.md`](./RECAP.md). Chapters 1, 3, and beyond are pending (read alongside the book at ~20 pages/day).
