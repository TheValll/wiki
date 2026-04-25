# Wiki

Personal learning wiki — structured notes across the technical domains I'm studying, synthesized from books, courses, videos, and articles.

Agent instructions: see [`CLAUDE.md`](CLAUDE.md). Personal profile: [`me.md`](me.md). Pedagogical profile (agent-maintained): [`how-i-learn.md`](how-i-learn.md).

---

## Domains

| Domain | Scope | Status |
|--------|-------|--------|
| [**Rust**](rust/README.md) | The Rust language, based on the official book | Ch 1-15 ✓ |
| [**ROS2**](ros2/README.md) | Robot Operating System 2 — nodes, control, MoveIt | 24 pages ✓ |
| [**Mathematics**](mathematics/README.md) | Math for robotics, ML, graphics | 3 pages, growing |
| [**Research**](research/README.md) | Meta-skills (reading, writing, speaking) + PhD-prep field navigation | 4 pages, growing |
| **Embedded** *(planned)* | Rust embedded, no_std, peripherals | — |
| **Low-level** *(planned)* | Memory, pointers, OS internals | — |
| **Electronics** *(planned)* | Circuits, protocols, signals | — |
| **ML / DL** *(planned)* | Machine learning, deep learning | — |

---

## Cross-domain connections

Where concepts link across domains — follow these to get the full picture.

| Topic | Primary page | Related |
|-------|--------------|---------|
| Rotation matrices, Jacobian | [`ros2/20-inverse-kinematics.md`](ros2/20-inverse-kinematics.md) | [`mathematics/01-linear-algebra.md`](mathematics/01-linear-algebra.md) |
| Motion planning, distance metrics | [`ros2/19-motion-planning.md`](ros2/19-motion-planning.md) | [`mathematics/01-linear-algebra.md`](mathematics/01-linear-algebra.md) |
| Trajectory generation (splines, derivatives) | [`ros2/21-trajectory-generation.md`](ros2/21-trajectory-generation.md) | [`mathematics/03-derivatives.md`](mathematics/03-derivatives.md) |
| DiffDrive kinematics | [`ros2/11-controllers-diffdrive.md`](ros2/11-controllers-diffdrive.md) | [`mathematics/01-linear-algebra.md`](mathematics/01-linear-algebra.md) |

*(More links will appear here as domains are added — Rust ↔ Embedded, Embedded ↔ Electronics, ML ↔ Math, etc.)*

---

## Review system

1h-2h evening sessions blending spaced repetition, original practice, and articulated recall — see [`review/README.md`](review/README.md).

- `review/AGENT.md` — session rules: opener (time + mode) → per-domain 6 steps → final check-in
- `review/checklists/<domain>.md` — curriculum (ordered concept list)
- `review/progress/<domain>.md` — current mastery levels (0 → 4) + intuition drills log
- `review/external-tracking.md` — non-wiki state (book chapters, papers, projects, PhD prep)

Start a session with *"review"* — Claude runs an **opener menu** (time available, which domains, which mode: `practice` / `intuition` / `mix`) before starting any block. Every session ends with a ~5 min check-in on external state.

**Practice modes** (per domain, when supported):
- `practice` — original exercises, formulas, computation
- `intuition` — articulate the concept under-the-hood in your own words (no formulas), using the `*-intuition.md` companion
- `mix` — both

**Key principle:** the wiki holds theory you've *written down*; the review system tracks what you've actually *practiced and internalized*. The agent never quizzes on concepts outside the active checklist.

---

## Workflow

### Adding a new source

1. Drop raw material (article, transcript, notes, PDF) into [`raw/`](raw/README.md)
2. Ask Claude: *"Ingest `raw/<filename>` into the wiki"*
3. Claude asks clarifying questions (domain? focus? new page or update?)
4. Claude proposes a plan → I validate → Claude writes

### Querying

- "What does X mean?" → Claude cites the relevant wiki page
- "Give me a summary of domain Y" → Claude reads the domain's README
- "Generate Anki cards for chapter N of domain Y" → Claude produces CSV

### Linting (optional, periodic)

Ask Claude to: *"Lint the wiki — check for broken links, orphan pages, inconsistent formatting, contradictions between pages."*

---

## Layout

```
wiki/
├── CLAUDE.md           ← agent schema + conventions
├── me.md               ← personal profile (identity, goals, roadmap)
├── how-i-learn.md      ← pedagogical profile (agent-maintained)
├── README.md           ← this file
├── raw/                ← unprocessed sources
├── review/             ← spaced-repetition system
├── rust/               ← domain
├── ros2/               ← domain
├── mathematics/        ← domain (+ *-intuition.md companions)
└── …                   ← future domains
```

Each domain has its own `README.md` acting as a syllabus / table of contents.
