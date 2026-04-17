# Wiki — Agent Instructions

This file is the **schema** for any LLM agent (Claude Code or other) working on this wiki. Read it first before making any changes.

---

## 1. What this wiki is

A **personal learning wiki** — structured notes synthesizing books, courses, videos, and articles across multiple technical domains the user is studying. It is a **learning tool**, not a dump database: every page is manually validated by the user.

**Current domains:**

| Folder | Scope | Status |
|--------|-------|--------|
| [`rust/`](rust/README.md) | Rust language — based on the official Rust book | ch 1-11 done |
| [`ros2/`](ros2/README.md) | ROS2 — nodes, control, MoveIt, hardware drivers | 24 pages |
| [`mathematics/`](mathematics/README.md) | Math used in robotics / ML / graphics | linear algebra, derivatives |

**Planned domains** (folders to be created as work begins): `embedded/` (Rust embedded, no_std), `low-level/` (memory, pointers, OS), `electronics/`, `ml/`, `dl/`, plus master/doctorate course material.

---

## 2. Directory layout

```
wiki/
├── CLAUDE.md           ← this file (agent schema)
├── README.md           ← global index, cross-domain map
├── raw/                ← unprocessed sources (articles, transcripts, notes)
│   └── README.md       ← explains the ingest workflow
├── review/             ← spaced-repetition practice system
│   ├── AGENT.md        ← session framework (pedagogy rules)
│   ├── README.md       ← user-facing doc
│   ├── checklists/     ← curriculum per domain
│   └── progress/       ← mastery state per domain
├── rust/
│   ├── README.md       ← domain index
│   └── 01-*.md … 13-*.md
├── ros2/
│   ├── README.md
│   └── 00-*.md … 23-*.md
├── mathematics/
│   ├── README.md
│   └── 01-*.md … 03-*.md
└── <future-domain>/
    ├── README.md
    └── …
```

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
> See also: [Linear Algebra — Matrix × Vector](../mathematics/01-linear-algebra.md#13-matrix--vector-multiplication)
```

Known cross-domain connections to watch for:

| From | To | Why |
|------|----|----|
| `ros2/20-inverse-kinematics.md` | `mathematics/01-linear-algebra.md` | Jacobian, rotation matrices |
| `ros2/19-motion-planning.md` | `mathematics/` | Distance metrics, sampling |
| `rust/` (embedded future) | `low-level/`, `electronics/` | Registers, GPIO, protocols |
| `ml/`, `dl/` (future) | `mathematics/03-derivatives.md` | Gradients, backprop |

---

## 7. Review sessions

When the user says *"review <domain>"* (or equivalent in French), switch to **review mode**:

1. Read `review/AGENT.md` — follow the 6-step session flow strictly (warm-up → lesson → challenge → STOP → correction → bonus → update)
2. Read `review/checklists/<domain>.md` for the curriculum
3. Read `review/progress/<domain>.md` for the current state
4. Never quiz on concepts marked "Not yet reached"
5. Generate **original** exercises — never copy wiki examples
6. At end of session, update `review/progress/<domain>.md` directly

The review system is separate from the wiki: the wiki is theory (what the user wrote), the checklist is practice (what the user is drilling).

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
