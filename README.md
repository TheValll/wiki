# Wiki

Personal learning wiki — structured notes across the technical domains I'm studying, synthesized from books, courses, videos, and articles.

Agent instructions: see [`CLAUDE.md`](CLAUDE.md).

---

## Domains

| Domain | Scope | Status |
|--------|-------|--------|
| [**Rust**](rust/README.md) | The Rust language, based on the official book | Ch 1-11 ✓ |
| [**ROS2**](ros2/README.md) | Robot Operating System 2 — nodes, control, MoveIt | 24 pages ✓ |
| [**Mathematics**](mathematics/README.md) | Math for robotics, ML, graphics | 3 pages, growing |
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

Spaced-repetition practice — see [`review/README.md`](review/README.md).

- `review/AGENT.md` — session rules (warm-up → lesson → challenge → correction → bonus → progress update)
- `review/checklists/<domain>.md` — curriculum (ordered concept list)
- `review/progress/<domain>.md` — current mastery levels (0 → 4)

Start a session with *"review <domain>"* (e.g., `review math`, `review rust`).

**Key principle:** the wiki holds theory you've *written down*; the review system tracks what you've actually *practiced*. The agent never quizzes on concepts not in the active checklist.

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
├── README.md           ← this file
├── raw/                ← unprocessed sources
├── rust/               ← domain
├── ros2/               ← domain
├── mathematics/        ← domain
└── …                   ← future domains
```

Each domain has its own `README.md` acting as a syllabus / table of contents.
