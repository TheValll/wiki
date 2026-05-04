# Anki — Init / Template

Reference and starter file for Anki cards in this wiki.

---

## Convention (set up 2026-05-04)

- **Note type**: `Basic` only. The `Basic (and reversed card)` note type does not behave correctly in this setup (Card 2 ends up identical to Card 1 instead of swapping Front/Back), so we write the two directions **manually**.
- **One concept = two cards** — one Q→A and one A→Q. Doubles the generation effect: writing the second direction forces a reformulation.
- **Format**: explicit `Front:` and `Back:` field names.
- **Language**: English on both Front and Back. Same convention as the rest of the wiki and `daily/`.
- **Sync trigger**: `Ctrl+P → Obsidian_to_Anki: Scan Vault`. Anki desktop must be running.
- **Deck target**: `TARGET DECK:` directive at the top of the file or above each card group.

---

## Example — one concept, two manually-reversed cards

TARGET DECK: Mathematics::init

START
Basic
Front: What is the formula of the Euclidean norm in $\mathbb{R}^n$?
Back: $\|v\| = \sqrt{\sum_{i=1}^n v_i^2}$ — Pythagoras cascaded across $n$ perpendicular axes.
<!--ID: 1777929842640-->
END

START
Basic
Front: $\|v\| = \sqrt{\sum_{i=1}^n v_i^2}$ — what concept does this formula represent?
Back: Euclidean norm in $\mathbb{R}^n$.
<!--ID: 1777929842644-->
END

---

## How to use

When you study a concept and want to memorize a fact:

1. Open the matching chapter file in this folder (e.g. `anki/01-linear-algebra.md` for concepts in `mathematics/01-linear-algebra/`). Create the file if it doesn't exist yet.
2. Add a heading `## X.Y — Concept name` (optional, for grouping).
3. Write **two** `START / Basic / END` blocks per atomic fact: one Q→A, one A→Q.
4. Save.
5. `Ctrl+P → Scan Vault` → cards appear in Anki under the deck specified by `TARGET DECK:` at the top of the file.

---

## Card-writing conventions

- **Front**: one question per card, ends with `?`. Single concept (definition, formula, property).
- **Back**: 1–2 lines max. Formula in LaTeX (`$...$` inline, `$$...$$` display) + short intuition.
- **Atomic**: one fact per card. A page with 5 atomic concepts → 10 cards (5 Q→A + 5 A→Q).
- **Avoid open-ended questions** ("explain the dot product") — articulation happens in `daily/`, not in Anki.
- **English only**, both Front and Back.

---

## Placement convention

**One file per chapter, all cards for that chapter inside.** Confirmed 2026-05-04.

```
anki/
├── README.md                   ← convention + index
├── init.md                     ← this file (template / reference)
├── 01-linear-algebra.md        ← cards for math chapter 01
├── 02-algebra-solving.md       ← cards for math chapter 02
├── 03-derivatives.md
├── 04-optimization.md
├── 05-ml-basics.md
├── 06-probability.md
├── 07-statistics.md
└── …                            ← one file per future chapter
```

Each file mirrors the `mathematics/<chapter>/` folder it covers. The mapping is direct: `anki/01-linear-algebra.md` ↔ `mathematics/01-linear-algebra/`. New MML chapters get new files (e.g. `anki/04-matrix-decomp.md` when MML §4 is reached).

Cards inside each file are organized by concept, optionally with `## X.Y — Concept` headings. A single `TARGET DECK: Mathematics::<chapter>` directive at the top of the file applies to every card in it.
