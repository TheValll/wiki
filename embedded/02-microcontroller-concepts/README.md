# 02 — Microcontroller Concepts

A generalist primer on what's *inside* a microcontroller — the building blocks any embedded developer must know before writing firmware. The **ESP32-C3** is used as the running concrete example, but the concepts apply to any modern MCU (ARM Cortex-M, RISC-V, AVR, …).

This chapter covers **hardware**, not Rust specifically. Rust-on-ESP starts in chapter 3.

---

## Pages

| § | Page | What it covers | Source (book) |
|---|------|----------------|---------------|
| 2.1 | [MCU vs MPU](./2.1-mcu-vs-mpu.md) | What distinguishes a microcontroller from a microprocessor | book §2.1 |
| 2.2 | [SoC](./2.2-soc.md) | "System on Chip" terminology and how it relates to MCU | book §2.2 |
| 2.3 | [Processor Architectures](./2.3-processor-architectures.md) | ISA (ARM/RISC-V/Xtensa) + Von-Neumann vs Harvard memory | book §2.3 (incl. 2.3.1, 2.3.2) |
| 2.4 | [Peripherals](./2.4-peripherals.md) | GPIO, Timers/Counters, PWM, ADC/DAC, Serial (UART/SPI/I²C) | book §2.4 |
| 2.5 | [Pin Interface](./2.5-pin-interface.md) | Pinout, multiplexer, 3-stage pin function selection | book §2.5 |
| **2.6** | [**Polling vs Interrupts**](./2.6-polling-interrupts.md) | Pull vs push, IVT, ISR, 4-step flow, when to use which | book §2.6 (incl. 2.6.1–2.6.3) |
| 2.7 | [Memory Mapping](./2.7-memory-mapping.md) | How peripherals/memory are addressed; ESP32-C3 memory map | book §2.7 |
| 2.8 | [Application Memory Layout](./2.8-application-memory-layout.md) | `.text`, `.rodata`, `.data`, `.bss`, heap, stack, IVT; flash → SRAM startup copy | book §2.8 |
| 2.9 | [Clocks](./2.9-clocks.md) | Oscillator, PLL, clock tree, gating, domains | book §2.9 |
| 2.10 | [Toolchains](./2.10-toolchains.md) | Bare-metal vs RTOS, compile chain, debug chain (probe/OpenOCD/GDB) | book §2.10 (incl. 2.10.1–2.10.3) |

---

## Reading order

The book is linear and concepts build, but the dependencies are loose. If jumping to a topic:
- 2.6 (polling/interrupts) — self-contained
- 2.8 (memory layout) → needs 2.7 (memory mapping) first
- 2.9 (clocks) — self-contained
- 2.10 (toolchains) — independent of the rest

---

## Status

**Chapter 2 complete (2026-05-04)** — all 10 files written, format validated on the §2.6 pilot then scaled to the rest. See [`../RECAP.md`](../RECAP.md) for a one-glance summary.
