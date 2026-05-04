# Embedded — RECAP

Single-glance summary of every concept in `embedded/`, grouped by chapter.

---

## 02 — Microcontroller Concepts

| Concept | What / for what | Quick mental shortcut |
|---|---|---|
| [2.1 MCU vs MPU](./02-microcontroller-concepts/2.1-mcu-vs-mpu.md) | MPU = horsepower with external scaffolding; MCU = self-contained Swiss Army knife | External RAM + disk → MPU. Coin-battery one-chip → MCU. |
| [2.2 SoC](./02-microcontroller-concepts/2.2-soc.md) | "System on Chip" — a beefed-up MCU with optional accelerators (GPU, FPGA) | Marketing says SoC, treat it as MCU. ESP32 = SoC = MCU. |
| [2.3 Processor Architectures](./02-microcontroller-concepts/2.3-processor-architectures.md) | ISA = the language a CPU speaks (ARM, RISC-V, Xtensa). Memory architecture = single highway (Von-Neumann) vs two highways (Harvard) | Modern MCUs are Harvard for pipelining. ISA is not portable. |
| [2.4 Peripherals](./02-microcontroller-concepts/2.4-peripherals.md) | The senses + actuators of the MCU: GPIO, Timer/Counter, PWM, ADC/DAC, UART/SPI/I²C | UART = debug. SPI = fast multi-device. I²C = pin-light multi-device. |
| [2.5 Pin Interface](./02-microcontroller-concepts/2.5-pin-interface.md) | Pins are multiplexed — software picks which peripheral function drives each pin | Pin "doesn't work"? Check (1) clock enabled (2) mux setting (3) peripheral config. |
| [2.6 Polling vs Interrupts](./02-microcontroller-concepts/2.6-polling-interrupts.md) | Polling = pull (CPU asks). Interrupts = push (peripheral notifies). IVT = function-pointer table in Flash. | Rare or async events → interrupts. Streaming control loop → polling. |
| [2.7 Memory Mapping](./02-microcontroller-concepts/2.7-memory-mapping.md) | Every memory cell + every peripheral register has a unique address; CPU talks to all by reading / writing addresses | No special "talk to peripheral" opcodes — just LOAD/STORE on memory-mapped addresses. |
| [2.8 Application Memory Layout](./02-microcontroller-concepts/2.8-application-memory-layout.md) | Sections: `.text`, `.rodata`, `.data`, `.bss`, heap, stack, IVT. Code in Flash, mutable data in SRAM. | Code/constants → Flash. Mutable runtime → SRAM. Boot copies `.data` Flash→SRAM, zeroes `.bss`. |
| [2.9 Clocks](./02-microcontroller-concepts/2.9-clocks.md) | Clock source (XTAL, RC, PLL) → clock tree → block destinations. 3 knobs: division, gating, domains. | If a peripheral seems dead, check clock enabled + frequency. |
| [2.10 Toolchains](./02-microcontroller-concepts/2.10-toolchains.md) | Compile chain (compile → assemble → link). Debug chain (probe ↔ control software ↔ debugger). Bare-metal vs RTOS. | Cross-compile on host PC for target ISA. Probe + OpenOCD/probe-rs + GDB = debug. |

---

## 03 — Embedded Rust & ESP *(planned)*
## 04+ — Project setup & peripherals *(planned)*
