# Wiki — Agent Instructions

This file is the **schema** for any LLM agent (Claude Code or other) working on this wiki. Read it first before making any changes.

> **Also read [`me.md`](me.md) and [`how-i-learn.md`](how-i-learn.md)** at the start of every new conversation.
> - `me.md` — who the user is, long-term goals (PhD EPFL 2028), active projects.
> - `how-i-learn.md` — pedagogical profile: formats that land (multi-frame schemas, physical analogies), articulation-based correction pattern, signals to watch. Maintained by the agent over time, the user can correct it.
> - **When adding a new domain, also read [`domains.md`](domains.md)** — playbook for structuring a new course (folder layout, file conventions, RECAP, reading-order DAG, launch checklist).

---

## 1. What this wiki is

A **personal learning wiki** — structured notes synthesizing books, courses, videos, and articles across multiple technical domains the user is studying. It is a **learning tool**, not a dump database: every page is manually validated by the user.

**Current domains:**

| Folder | Scope | Status |
|--------|-------|--------|
| [`rust/`](rust/README.md) | Intuition layer over the official 2024 Rust book (no per-chapter pages) | `rust-intuition.md` ✓ |
| [`ros2/`](ros2/README.md) | ROS2 — nodes, control, MoveIt, hardware drivers | 24 pages, 4 sub-folders |
| [`mathematics/`](mathematics/README.md) | Math used in robotics / ML / graphics | 7 chapters, ~50 concept files |
| `ml/` *(planned)* | Machine learning + deep learning models (classical → DL → generative → RL) | — (to be rebuilt) |
| [`research/`](research/README.md) | Meta-skills (reading, writing) for PhD prep | 2 pages |
| [`school/`](school/README.md) | M1/M2 Efrei course notes (parent domain, one sub-folder per course) | Big Data Framework ✓ |
| [`daily/`](daily/README.md) | Per-day learning journal (tracks log + articulation + gaps) — written in English | Started 2026-05-03 |
| [`embedded/`](embedded/README.md) | Embedded systems + embedded Rust on ESP32 (Hiari ESP Core Library, no_std, esp-hal) | Pilot 2026-05-04 — chapter 2 in progress |

**Planned domains** (folders to be created as work begins): `low-level/` (memory, pointers, OS), `electronics/`, plus master/doctorate course material.

---

## 2. Directory layout

```
wiki/
├── CLAUDE.md           ← this file (agent schema)
├── README.md           ← global index, cross-domain map
├── raw/                ← unprocessed sources (articles, transcripts, notes)
│   └── README.md       ← explains the ingest workflow
├── rust/
│   ├── README.md            ← short domain index
│   └── rust-intuition.md    ← bookmark layer (every concept of book ch.1-21 in 2-4 lines)
│                              ground truth = raw/rust-book/ (curl of doc.rust-lang.org/book)
├── ros2/
│   ├── README.md
│   ├── RECAP.md
│   ├── ros2-flows.md   ← visual atlas (multi-frame ASCII schemas of ROS2 flows)
│   ├── setup/          ← 00
│   ├── basics/         ← 01-07
│   ├── ros2-control/   ← 08-16
│   └── moveit/         ← 17-23
├── mathematics/
│   ├── README.md
│   ├── RECAP.md
│   ├── notation.md     ← bilingual FR/EN math notation atlas
│   └── 01-*/ … 07-*/   ← one folder per chapter, one file per concept
├── daily/
│   ├── README.md       ← convention + template + recent-entries index
│   └── YYYY-MM-DD.md   ← one file per day (tracks log + articulation + gaps), English
├── embedded/
│   ├── README.md       ← domain index, Pattern B (layered), book = Hiari ESP Core Library
│   └── 02-*/ … NN-*/   ← one folder per book chapter, files mirror book §X.Y numbering
├── anki/
│   ├── README.md       ← convention + per-chapter file index
│   ├── init.md         ← template / reference (working example)
│   └── NN-<chapter>.md ← one file per math chapter, all cards for that chapter
└── <future-domain>/
    ├── README.md
    └── …
```

**Note:** there is no `review/` system. The wiki is a learning reference; practice happens in books (Deisenroth for math, the official Rust book, etc.) and in real projects (DeepSight). Articulation drills are ad hoc on user demand — see §7.

Each domain folder has its **own `README.md`** = index + syllabus with links to every page in that domain.

---

## 3. Page format conventions

All domain pages follow the same visual grammar (look at any existing file for the pattern). Preserve it when editing or creating new pages.

| Element | Convention |
|---------|------------|
| **Title** | `# Part N — Topic Title` |
| **Intro** | 1-3 sentences framing the topic, often with a small table |
| **Sections** | `## N.1 — Subtitle`, `## N.2 — Subtitle`, … |
| **Code blocks** | Fenced with triple backticks + language tag (```rust, ```cpp, ```bash) |
| **Tables** | Used heavily for comparisons, options, summaries |
| **ASCII art boxes** | For key insights — `┌─────┐` rounded-corner style, used sparingly for emphasis |
| **Cross-references** | Standard markdown links `[text](path/to/page.md)` — obsidian renders both these and `[[wiki-links]]` but we use standard markdown for portability |
| **Summary section** | Most pages end with a `## N.X — Summary` table |
| **Language** | English for all wiki content (user is French but wants the wiki in English for consistency with source material) |

**Do NOT:**
- Add emojis unless the user asks
- Write multi-paragraph docstrings or verbose comments in code blocks
- Create pages with only prose — always include tables, code, or diagrams
- Use `<br>` or HTML — stick to pure markdown

---

## 4. Ingest workflow (from `raw/`)

When the user says *"ingest `raw/<file>`"*:

1. **Read** the raw file completely
2. **Ask questions before acting:**
   - Which **domain(s)** does this belong to? (rust, ros2, math, new domain?)
   - What's the **focus** — what should be emphasized?
   - **New page** or **update existing**? If update, which one(s)?
   - **Granularity** — one dense page or split into several?
3. **Propose** a plan (don't write files yet): list the pages you'd create/edit, with 1-line summaries
4. **Wait for validation**
5. **Then** write the pages following the format conventions
6. **Update indexes:** the relevant domain's `README.md`, and the root `README.md` if the domain changes or cross-references are added

```
┌──────────────────────────────────────────────────────────────────┐
│  Never auto-ingest without user validation. This is a            │
│  LEARNING wiki — the user wants to review before content lands.  │
└──────────────────────────────────────────────────────────────────┘
```

---

## 5. Updating existing pages

- Prefer `Edit` over `Write` (sends only the diff)
- Preserve the existing tone, structure, and level of detail
- If adding a section, place it in a logical spot and update the `Summary` table at the end
- If the change affects the domain index, update that domain's `README.md`
- If the change introduces cross-domain links, update the root `README.md`

---

## 6. Cross-references

When a concept spans domains, add a **"See also"** line at the end of the relevant section:

```markdown
> See also: [Linear Algebra — Matrix × Vector](../mathematics/01-linear-algebra/1.3-matrix-vector.md)
```

Known cross-domain connections to watch for:

| From | To | Why |
|------|----|----|
| `ros2/moveit/20-inverse-kinematics.md` | `mathematics/01-linear-algebra/` | Jacobian, rotation matrices |
| `ros2/moveit/19-motion-planning.md` | `mathematics/` | Distance metrics, sampling |
| `embedded/` | `rust/` (no_std patterns), `low-level/` (planned), `electronics/` (planned) | Registers, GPIO, protocols, ESP HAL |
| `ml/`, `dl/` (future) | `mathematics/03-derivatives/` | Gradients, backprop |

---

## 7. Articulation drills on demand

If the user explicitly asks for a drill / explainer / articulation on a concept (*"explique-moi X"*, *"drill-moi sur Y"*, *"fais-moi un schéma de Z"*), run a focused mini-session:

1. **Recap** in 1-2 paragraphs — visual / intuitive: physical image first, mechanism second, formula last (cf [`how-i-learn.md`](how-i-learn.md) §1.1-1.2). For ROS2 flows, prefer a multi-frame ASCII schema and update [`ros2/ros2-flows.md`](ros2/ros2-flows.md) if a new frame lands. For math, surface the relevant symbols and update [`mathematics/notation.md`](mathematics/notation.md) if symbols are new.
2. **Articulation** — ask the user to re-explain in his own words. **STOP. Wait.**
3. **Correction** — pinpoint the precise gap (inversion, missing piece, wrong direction). Don't silently rewrite his draft (cf `how-i-learn.md` §2).

No persistence, no levels, no progress tracking. This is **ad hoc**, not a system. Concept source = any wiki page, the atlases above, or off-wiki on user request.

---

## 8. Anki cards — markdown source → Anki

**Scope: math only.** Anki is used for declarative knowledge (formulas, definitions, identities, theorems). Rust / ROS2 / embedded use practice instead — no Anki for procedural knowledge.

**Authorship: self-made.** Valentin writes his own cards (generation effect). The agent does NOT propose, generate, or pre-fill Anki cards by default, even when atomic facts surface. If explicitly asked *"give me a card on X"*, comply.

**Language: English.** Front and back of every card are written in English — same convention as the rest of the wiki (cf. §3) and as `daily/` entries. Doubles as English practice.

### Workflow

1. **Tooling**: [Obsidian-to-Anki](https://github.com/Pseudonium/Obsidian_to_Anki) plugin in Obsidian + AnkiConnect addon (code `2055492159`) in Anki desktop.
2. **Source of truth**: markdown files in this wiki. Anki = read-only consumer that handles spaced-repetition scheduling.
3. **Direction**: Markdown → Anki, uni-directional. Modifications made in Anki do not propagate back to markdown.
4. **Trigger**: Obsidian command `Obsidian_to_Anki: Scan vault` pushes new/modified cards. Anki desktop must be running.

### Card placement

Cards live in a dedicated [`anki/`](anki/README.md) folder, **one file per chapter**, mirroring the `mathematics/<chapter>/` structure. Example: `anki/01-linear-algebra.md` ↔ `mathematics/01-linear-algebra/`. Each file contains all cards for concepts in that chapter, with a single `TARGET DECK: Mathematics::<chapter>` at the top. Confirmed 2026-05-04 (Option B over inline placement).

### Card format

- **Note type**: `Basic` only. The `Basic (and reversed card)` note type does **not** behave correctly in this setup (Card 2 ends up identical to Card 1 instead of swapping Front/Back), so reverse cards are written **manually** — one `START / Basic / END` block per direction.
- **Default deck**: `Mathematics`. Override per file via a `TARGET DECK: Mathematics::<chapter>` line at the top of the `## Anki` section to land cards in a sub-deck.
- **No per-card tags** for now — the deck hierarchy handles navigation.
- **LaTeX**: `$...$` inline, `$$...$$` display. Renders identically in Anki and Obsidian.

### Format spec — one concept, two manually-reversed cards

````markdown
## 1.1.X — Anki

TARGET DECK: Mathematics::01-linear-algebra

START
Basic
Front: What is the formula of the Euclidean norm in $\mathbb{R}^n$?
Back: $\|v\| = \sqrt{\sum_{i=1}^n v_i^2}$ — Pythagoras cascaded across $n$ perpendicular axes.
END

START
Basic
Front: $\|v\| = \sqrt{\sum_{i=1}^n v_i^2}$ — what concept does this formula represent?
Back: Euclidean norm in $\mathbb{R}^n$.
END
````

### Card-writing conventions

- **Front**: one line ending with `?`. A simple question on the concept (definition / formula / property).
- **Back**: one to two lines max. LaTeX formula + short intuition.
- **Atomic**: one fact per card. A page with 5 atomic concepts → 10 cards (5 Q→A + 5 A→Q).
- **Two directions per concept**: write the reverse manually. Reformulating the question forces a second pass of generation effect, which is in fact better for retention than mechanically auto-generated reverses.
- **Avoid open-ended questions** ("explain the dot product") — articulation happens in `daily/`, not in Anki.
- **Source lens**: the card reflects how the currently-read source frames the concept (typically MML for math).
- **English only**, both front and back. No mixed-language cards.

---

## 9. What NOT to do

- **Don't create documentation files beyond wiki pages** unless explicitly requested (no TODO.md, NOTES.md, status reports)
- **Don't merge domains** — if a topic straddles two domains, write in the primary domain and cross-reference
- **Don't auto-ingest** — always validate with the user first
- **Don't reorganize historical pages** without discussing — numbering is stable (e.g., `13-tests.md` stays `13-tests.md` even if inserting later)
- **Don't touch `.obsidian/`** — it's the Obsidian app config, managed by the app itself
- **Don't write user-facing content in French** — wiki content stays in English

---

## 10. Conversation style with the user

- The user is **French-speaking** — conversation in the chat is in French, wiki content in English
- He's building towards **robotics / low-level / ML expertise**
- He **validates every significant change** — propose before acting for non-trivial edits
- Responses should be **concise** — he reads diffs himself, no need to re-explain what changed
- When he asks a question about a concept, prefer citing the wiki page over re-explaining from scratch
