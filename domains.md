# Domains — How to Write a Course

This file is the **playbook** for adding a new domain to the wiki (e.g. `embedded/`, `low-level/`, `electronics/`, `ml/`, `dl/`). It captures the conventions distilled from the 4 existing domains (`mathematics/`, `rust/`, `ros2/`, `research/`).

> Companion to [`CLAUDE.md`](CLAUDE.md) (agent rules) and [`me.md`](me.md) (user profile). When you add a new domain, **follow this file step by step**.

---

## 1. Two structural patterns

Before writing anything, pick which pattern fits the domain. The choice depends on the *natural granularity* of the source material.

### Pattern A — **Atomic concept** (math, rust, future ml/dl)

Each chapter folder, each file inside is **one concept**. A "concept" = something you could explain in 5-15 minutes standing at a whiteboard. Used when the source material is built from small, composable building blocks.

```
<domain>/
├── README.md           ← syllabus + reading order
├── RECAP.md            ← single-glance table of every concept
├── 01-<chapter-slug>/
│   ├── README.md       ← chapter TOC + Applied-in table
│   ├── 1.1-<slug>.md   ← one concept = one file
│   ├── 1.2-<slug>.md
│   └── …
├── 02-<chapter-slug>/
└── …
```

**Examples:**
- `mathematics/01-linear-algebra/` → 9 concept files (`1.1-norm.md`, `1.2-dot-product.md`, …)
- `rust/05-ownership/` → 10 concept files (`5.1-why-ownership.md` through `5.10-return-values.md`)

**Per-file structure** (atomic pattern):
```markdown
# X.Y — Concept Title

**What it does:**
1-2 sentences on purpose + a short mechanical analogy.

## Intuition — <subtitle>
Physical image → step-by-step decomposition → "what to remember" closure.
ASCII art frames preferred for transformations (one frame per step).

## Formula
Code-block math.

## Simple example
Minimal numeric case.

## Complex example
Realistic scenario from robotics / ML / physics.

## See also (optional)
Cross-domain links.
```

### Pattern B — **Layered topic** (ros2)

Each file = **one topic**, internally layered into ~6-8 facets (Analogy → Concepts → Code → Math → Flow). Used when the source material is built from larger units that are pedagogically inseparable (e.g. "Topics & Pub/Sub" needs all of: analogy + publisher code + subscriber code + QoS + serialization to be useful).

```
<domain>/
├── README.md           ← syllabus + reading order
├── RECAP.md            ← single-glance table (1 row per file)
├── <block-1>/          ← optional sub-folder grouping
│   ├── 00-<slug>.md
│   ├── 01-<slug>.md
│   └── …
└── <block-2>/
```

**Example:**
- `ros2/basics/02-topics-pub-sub.md` covers Analogy / Topics / Publisher (deep dive) / Subscriber (deep dive) / QoS / Serialization / Python comparison / Flow diagram — all internal `## 2.1 — …` to `## 2.8 — …` sections inside one file.

**Per-file structure** (layered pattern):
```markdown
# Part N — Topic Title

Intro: 1-3 sentences framing the topic, often with a small table.

## N.1 — The Analogy
Physical image / mechanical analogy. Often the best opener.

## N.2 — The Core Concept
The thing itself, plainly stated.

## N.3 — Code Deep-Dive
Fenced code blocks with language tag (```cpp / ```python / ```rust).

## N.4 — Comparison / Variant
e.g. C++ vs Python.

## N.5 — Math / Internals
Formulas, memory layouts, protocol details when applicable.

## N.6 — Flow / Sequence Diagram
ASCII art showing how things connect over time.

## N.X — Quick Reference (optional)
Table summarizing options, signatures, or "when to use what".

**Next:** [Part N+1 — …](next-file.md)
```

### Decision rule

| Source material… | Use Pattern |
|---|---|
| Textbook chapters with many small theorems / definitions | **A** (atomic) |
| Programming language with many small features | **A** (atomic) |
| Framework / system whose pieces only make sense together | **B** (layered) |
| Tutorial that teaches end-to-end builds | **B** (layered) |

---

## 2. Naming conventions

| Element | Convention | Example |
|---|---|---|
| Domain folder | lowercase, kebab-case | `mathematics/`, `ros2/`, `low-level/` |
| Chapter folder (Pattern A) | `NN-<slug>/` (zero-padded number) | `01-linear-algebra/`, `05-ownership/` |
| Concept file (Pattern A) | `X.Y-<slug>.md` | `1.1-norm.md`, `5.6-move-semantics.md` |
| Topic file (Pattern B) | `NN-<slug>.md` | `02-topics-pub-sub.md` |
| Block folder (Pattern B, optional) | lowercase, kebab-case | `setup/`, `basics/`, `ros2-control/`, `moveit/` |
| Slug | short, descriptive, kebab-case, no chapter prefix | `dot-product` not `linear-algebra-dot-product` |

**Numbering is stable** once published. If you insert a concept later, append rather than renumber. Exception: a deliberate refacto, validated explicitly with the user.

---

## 3. Per-file content rules

Inherited from `CLAUDE.md` §3, restated here for new domains:

- **Language:** English (user is French but wiki content stays English for consistency with sources).
- **Code blocks:** triple backticks + language tag.
- **Tables:** used heavily for case analysis, summary, comparison (3 columns max for "consolidation tables").
- **ASCII art:** rounded corners (`┌────┐`), step-by-step frames for transformations, sparingly for emphasis.
- **No prose-only pages:** always include tables, code, or diagrams.
- **No emojis** unless explicitly requested.
- **No HTML / `<br>`:** pure markdown only.
- **Cross-references:** standard markdown links with relative paths (`[text](../other-domain/file.md)`).

### Pedagogy (from `how-i-learn.md`)

- **Physical image first**, formula last (`how-i-learn.md` §2). Order the page accordingly: analogy → decomposition → mechanism → formula → examples.
- **Step-by-step ASCII frames** beat one dense diagram. One frame per transformation step.
- **Three-case tables** at the end of a section ("acute / perpendicular / obtuse", "min / max / saddle", "owns / borrows / clones") lock the concept.
- **Mechanical analogies** (cart, gear, rubber sheet, conveyor belt) work even for non-geometric concepts. Used in math and rust alike.

---

## 4. Per-chapter `README.md` (Pattern A only)

Every chapter folder has a `README.md` that is the chapter TOC. Format:

```markdown
# NN — Chapter Title

Each page covers **one concept**. Course content (formula, examples) and intuition merged.

| § | Page | In one line |
|---|------|-------------|
| X.1 | [Concept Name](./X.1-slug.md) | 1-line essence in plain English |
| X.2 | [Concept Name](./X.2-slug.md) | … |

## Applied in (optional)

| Concept | Used in |
|---------|---------|
| **Concept A** | [Other Domain — Page](../../other/page.md) — 1-line context |
```

The "Applied in" table makes cross-domain reuse visible. Add it whenever the chapter's concepts are referenced from another domain.

---

## 5. Domain `README.md` (always required)

Every domain has a top-level `README.md` with:

1. **One-line scope statement** at the top.
2. **At-a-glance pointer** to the `RECAP.md`.
3. **Companion pointer** if any (e.g. `rust-intuition.md`, future `*-intuition.md` files).
4. **Conventions section** — note the structural pattern used (A or B), naming rules.
5. **Syllabus** — chapter list with links to each chapter folder's README (Pattern A) or each topic file (Pattern B). Group into blocks if the domain has > 10 entries.
6. **Reading order & prerequisites** — for domains with non-linear dependencies (Pattern B especially), include an ASCII DAG and "skip rules" listing the minimal prereqs to jump to a specific topic. See `ros2/README.md` for the canonical example.

---

## 6. `RECAP.md` (always required)

Single-glance table of every concept in the domain. One row per concept-file (Pattern A) or per topic-file (Pattern B). Three columns:

```markdown
| Concept | What / for what (1-2 sentences) | Intuition (1 phrase) |
```

Group rows by chapter (Pattern A) or by block (Pattern B). The whole RECAP should be readable in under a minute.

Link the RECAP from the domain `README.md` with `> **At a glance:** [RECAP.md](./RECAP.md)`.

---

## 7. Cross-references

| Relationship | Convention |
|---|---|
| Same folder | `[text](./file.md)` |
| Same domain, different chapter | `[text](../other-chapter/file.md)` |
| Different domain | `[text](../../other-domain/chapter/file.md)` |
| Wiki root file (CLAUDE, README, me, how-i-learn) | `[text](../CLAUDE.md)` from `<domain>/`, or `[text](../../CLAUDE.md)` from a chapter folder |

When you add a new cross-domain link:
1. Add a `> See also: [other domain — concept](path)` line at the end of the relevant section.
2. Update the source chapter's `README.md` "Applied in" table.
3. Update the target chapter's `README.md` if appropriate.
4. Update the root `README.md` "Cross-domain connections" table for high-traffic links.
5. Update `CLAUDE.md` §6 cross-domain table if the link represents a recurring pattern.

---

## 8. Companion intuition file (optional)

Two flavors observed:

- **Inline (math):** intuition merged into each concept file under `## Intuition — <subtitle>`. Best when each concept has its own distinct mental image.
- **Global (rust):** one big `<domain>-intuition.md` at the domain root, hand-curated, train-readable, no formulas or exercises. Best when intuitions span multiple concepts and the prose flow matters.

Decide per domain. If unsure, start inline; promote to global only if you find yourself repeating yourself.

---

## 9. Launch checklist for a new domain

Use this when the user says *"I want to add a new domain X"*:

- [ ] **Validate scope with the user.** What's the source material? What's the goal? Is it linear or non-linear?
- [ ] **Pick the structural pattern** (A atomic or B layered). State your reasoning.
- [ ] **Propose folder + naming convention** before any file is written.
- [ ] **Wait for explicit validation** before mass file creation (CLAUDE.md §9: never auto-ingest).
- [ ] **Create the domain folder** with an empty `README.md` skeleton.
- [ ] **Write the first 1-2 chapters** as a pilot. Iterate the format with the user before scaling.
- [ ] **Build the chapter `README.md`** for each chapter as you go.
- [ ] **Build the domain `RECAP.md`** once the structure stabilizes.
- [ ] **Add the reading-order DAG** in the domain `README.md` if dependencies are non-linear.
- [ ] **Update root `README.md`** — add the domain to the Domains table + cross-domain connections.
- [ ] **Update `CLAUDE.md` §1** Current domains table.
- [ ] **Update `CLAUDE.md` §6** cross-domain table if applicable.
- [ ] **Consider an atlas companion** if the domain has flows, schemas, or notation worth capturing as a long-term artifact (e.g. `<domain>/<domain>-flows.md`, `<domain>/notation.md`). Optional — only when there's a clear visual/symbolic payload to preserve.

---

## 10. Anti-patterns (don't do this)

- **Pages with only prose** — always include tables, code, or diagrams.
- **Multiple concepts per file in Pattern A** — split, don't bundle. Exception: pedagogically tight bundles like Pattern B layered topics.
- **Re-explaining concepts already in the wiki** — link to the existing page instead.
- **Renumbering existing pages** without explicit user validation. Numbering is stable.
- **Auto-ingesting raw material** without the validation flow (`raw/` → ask questions → propose plan → wait → write).
- **Verbose multi-paragraph docstrings inside code blocks** — keep snippets focused on the concept being illustrated.
- **HTML, emojis (unless asked), or French in wiki content.**
- **Bypassing the user's validation** for non-trivial structural changes.

---

## 11. Maintenance scripts

Two recurring scripts proven useful during the math + rust + ros2 refactos:

### A. Split a multi-concept main file into per-concept files (Pattern A)

```bash
extract_section() {
  awk -v sec="^## $2 —" -v next_pat="^## " '
    $0 ~ sec { found=1; print; next }
    found && $0 ~ next_pat { exit }
    found { print }
  ' "$1" | sed '1s/^## /# /'
}

extract_section old-chapter.md "1.1" > new-chapter/1.1-slug.md
```

### B. Bulk-rewrite cross-references after a structural move

Use Python with `os.walk` + regex. The `ros2/` reorg used a Python one-shot script to:
1. Compute target folder for each chapter number.
2. Rewrite same-domain inter-file links to use the new sub-folder paths.
3. Bump `../mathematics/` → `../../mathematics/` for files moved one level deeper.

Pattern:
```python
def folder_for(n):
    n = int(n)
    if n == 0: return "setup"
    if 1 <= n <= 7: return "basics"
    # ...

# For each link, compute target folder and rewrite if cross-folder.
```

After any structural change: run `grep -rn "<old-pattern>" . --include="*.md"` to verify zero stale references.
