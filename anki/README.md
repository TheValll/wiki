# Anki — Wiki Folder

Self-made Anki cards, written in markdown and synced to Anki via the [Obsidian-to-Anki](https://github.com/Pseudonium/Obsidian_to_Anki) plugin. The wiki is the source of truth; Anki handles the spaced-repetition scheduling.

> **Format & conventions:** see [`init.md`](./init.md) (template + working example) and [`../CLAUDE.md`](../CLAUDE.md) §8 (formal spec).

---

## Convention

- **Scope:** math only — declarative knowledge (formulas, definitions, identities). Rust / ROS2 / embedded use practice instead.
- **One file per chapter**, mirroring `mathematics/<chapter>/`. Example: `anki/01-linear-algebra.md` ↔ `mathematics/01-linear-algebra/`.
- **Note type:** `Basic` only (auto-reverses don't behave correctly here, so the two directions are written manually).
- **Two cards per concept**: one Q→A, one A→Q. Doubles the generation effect.
- **Language:** English on both Front and Back.
- **Sync:** `Ctrl+P → Obsidian_to_Anki: Scan Vault` with Anki desktop running.

---

## Files

| File | Covers | Status |
|---|---|---|
| [`init.md`](./init.md) | Template + working example (Euclidean norm, 2 reverse cards) | reference, scanned to deck `Mathematics::init` |
| `01-linear-algebra.md` | Vector norm, dot product, matrix × vector, inverse, det, eigenvalues | *(to be filled as MML §2 is read)* |
| `02-algebra-solving.md` | Discriminant, linear systems, Gaussian elimination, identities | *(planned)* |
| `03-derivatives.md` | Notation, common derivatives, chain rule, gradient, Hessian | *(planned)* |
| `04-optimization.md` | Gradient descent, Newton's method | *(planned)* |
| `05-ml-basics.md` | Normalization, linear regression, RMSE, sigmoid, BCE | *(planned)* |
| `06-probability.md` | Bayes, distributions, PMF/PDF/CDF | *(planned)* |
| `07-statistics.md` | Mean, variance, MLE, hypothesis testing | *(planned)* |

When MML introduces a new chapter (e.g. Matrix Decompositions, MML §4), create the matching `anki/<chapter>.md` file.

---

## Card format (recap)

````markdown
TARGET DECK: Mathematics::01-linear-algebra

## 1.1 — Vector norm

> Source: [1.1 — Vector Norm (Euclidean)](../mathematics/01-linear-algebra/1.1-norm.md)

START
Basic
Front: What is the formula of the Euclidean norm in $\mathbb{R}^n$?
Back: $\|v\| = \sqrt{\sum_{i=1}^n v_i^2}$ — Pythagoras cascaded across $n$ perpendicular axes.
<!--ID: 1777974570537-->
END

START
Basic
Front: $\|v\| = \sqrt{\sum_{i=1}^n v_i^2}$ — what concept does this formula represent?
Back: Euclidean norm in $\mathbb{R}^n$.
<!--ID: 1777974494404-->
END
````

Each `## X.Y — Concept` heading is followed by a `> Source:` line linking to the matching wiki page. One source line per concept block, before the `START` markers.

**Zero invention rule:** the Front and Back of every card must come *directly* from the linked source page — same definition, same formula, same intuition phrasing where possible. If a fact isn't on the source page, don't write a card for it; extend the source page first, then card it.

The plugin injects a `<!--ID: ...-->` comment inside each `START / END` block once the card is created in Anki. **Don't delete those** — they're how the plugin knows which markdown card maps to which Anki note for incremental sync.

---

## Workflow

1. Read a math concept (e.g. MML §3.1 on norms).
2. Open `anki/<matching-chapter>.md`.
3. Add a `## X.Y — Concept name` heading + a `> Source:` line linking the matching wiki page + 2 `START / Basic / END` blocks (Q→A and A→Q).
4. Save → `Ctrl+P → Scan Vault`.
5. Review the cards in Anki desktop / AnkiDroid / AnkiMobile via Anki sync.
