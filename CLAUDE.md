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
| [`ml/`](ml/README.md) | Machine learning + deep learning models (classical → DL → generative → RL) | Skeleton, growing |
| [`research/`](research/README.md) | Meta-skills (reading, writing) for PhD prep | 2 pages |
| [`school/`](school/README.md) | M1/M2 Efrei course notes (parent domain, one sub-folder per course) | Big Data Framework ✓ |

**Planned domains** (folders to be created as work begins): `embedded/` (Rust embedded, no_std), `low-level/` (memory, pointers, OS), `electronics/`, plus master/doctorate course material.

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
├── ml/
│   ├── README.md       ← domain index + per-page schema for model files
│   ├── RECAP.md
│   └── 01-*/ … 13-*/   ← one folder per family, one file per model
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
| `rust/` (embedded future) | `low-level/`, `electronics/` | Registers, GPIO, protocols |
| `ml/`, `dl/` (future) | `mathematics/03-derivatives/` | Gradients, backprop |

---

## 7. Articulation drills on demand

If the user explicitly asks for a drill / explainer / articulation on a concept (*"explique-moi X"*, *"drill-moi sur Y"*, *"fais-moi un schéma de Z"*), run a focused mini-session:

1. **Recap** in 1-2 paragraphs — visual / intuitive: physical image first, mechanism second, formula last (cf [`how-i-learn.md`](how-i-learn.md) §1.1-1.2). For ROS2 flows, prefer a multi-frame ASCII schema and update [`ros2/ros2-flows.md`](ros2/ros2-flows.md) if a new frame lands. For math, surface the relevant symbols and update [`mathematics/notation.md`](mathematics/notation.md) if symbols are new.
2. **Articulation** — ask the user to re-explain in his own words. **STOP. Wait.**
3. **Correction** — pinpoint the precise gap (inversion, missing piece, wrong direction). Don't silently rewrite his draft (cf `how-i-learn.md` §2).

No persistence, no levels, no progress tracking. This is **ad hoc**, not a system. Concept source = any wiki page, the atlases above, or off-wiki on user request.

---

## 8. Anki card generation (future)

The user plans to generate Anki decks from wiki pages. When asked, output a CSV with this format:

```
front,back,tags
"What does `wrapping_add` do on overflow?","Wraps around (255u8.wrapping_add(1) = 0). Explicit, works in both debug and release.",rust::ch3::overflow
```

Keep cards **atomic** (one concept per card), include code snippets in the back when relevant, and tag by `domain::chapter::concept`.

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
