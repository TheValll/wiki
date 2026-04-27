# Wiki

Personal learning wiki — structured notes across the technical domains I'm studying, synthesized from books, courses, videos, and articles.

Agent instructions: see [`CLAUDE.md`](CLAUDE.md). Personal profile: [`me.md`](me.md). Pedagogical profile (agent-maintained): [`how-i-learn.md`](how-i-learn.md). New-domain playbook: [`domains.md`](domains.md).

---

## Domains

| Domain | Scope | Status |
|--------|-------|--------|
| [**Rust**](rust/README.md) | The Rust language, based on the official book | Ch 1-15 ✓ |
| [**ROS2**](ros2/README.md) | Robot Operating System 2 — nodes, control, MoveIt | 24 pages ✓ |
| [**Mathematics**](mathematics/README.md) | Math for robotics, ML, graphics | 3 pages, growing |
| [**ML / DL**](ml/README.md) | Machine learning + deep learning, classical → transformers → generative → RL | Skeleton, growing |
| [**Research**](research/README.md) | Meta-skills (reading, writing, speaking) + PhD-prep field navigation | 2 pages, growing |
| [**School**](school/README.md) | M1/M2 Efrei courses (course-bound, exam-oriented) | Big Data Framework ✓ |
| **Embedded** *(planned)* | Rust embedded, no_std, peripherals | — |
| **Low-level** *(planned)* | Memory, pointers, OS internals | — |
| **Electronics** *(planned)* | Circuits, protocols, signals | — |

---

## Cross-domain connections

Where concepts link across domains — follow these to get the full picture.

| Topic | Primary page | Related |
|-------|--------------|---------|
| Rotation matrices, Jacobian | [`ros2/moveit/20-inverse-kinematics.md`](ros2/moveit/20-inverse-kinematics.md) | [`mathematics/01-linear-algebra/`](mathematics/01-linear-algebra/README.md) |
| Motion planning, distance metrics | [`ros2/moveit/19-motion-planning.md`](ros2/moveit/19-motion-planning.md) | [`mathematics/01-linear-algebra/`](mathematics/01-linear-algebra/README.md) |
| Trajectory generation (splines, derivatives) | [`ros2/moveit/21-trajectory-generation.md`](ros2/moveit/21-trajectory-generation.md) | [`mathematics/03-derivatives/`](mathematics/03-derivatives/README.md) |
| DiffDrive kinematics | [`ros2/ros2-control/11-controllers-diffdrive.md`](ros2/ros2-control/11-controllers-diffdrive.md) | [`mathematics/01-linear-algebra/`](mathematics/01-linear-algebra/README.md) |

*(More links will appear here as domains are added — Rust ↔ Embedded, Embedded ↔ Electronics, ML ↔ Math, etc.)*

---

## How practice happens

There is **no review system in the wiki**. The wiki is a **reference**; practice happens elsewhere:

| Domain | Source of practice | Source of retention |
|---|---|---|
| **Maths** | [Mathematics for Machine Learning](https://mml-book.github.io) (Deisenroth, Faisal, Ong) — book + exercises | Re-reading + use in real ML projects |
| **Rust** | The official [Rust book](https://doc.rust-lang.org/book/), then Rust embedded on DeepSight | Real project code = the best retention |
| **ROS2** | No active practice for now | Re-reading wiki pages + the [`ros2/ros2-flows.md`](ros2/ros2-flows.md) atlas when needed |

**Articulation drills on demand.** If I want to lock in a concept, I can ask Claude *"explique-moi X"* / *"drill-moi sur Y"*. Claude runs a focused mini-session (recap → I re-explain in my own words → Claude pinpoints the gap). No persistence, no levels, no progress tracking — ad hoc, see [`CLAUDE.md`](CLAUDE.md) §7.

**Atlases** (built passively over time):
- [`mathematics/notation.md`](mathematics/notation.md) — bilingual FR/EN math notation reference
- [`ros2/ros2-flows.md`](ros2/ros2-flows.md) — multi-frame ASCII schemas of ROS2 mechanisms

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
├── domains.md          ← playbook for adding a new domain
├── README.md           ← this file
├── raw/                ← unprocessed sources
├── rust/               ← domain (incl. rust-intuition.md companion)
├── ros2/               ← domain (incl. ros2-flows.md visual atlas)
├── mathematics/        ← domain (incl. notation.md atlas)
├── research/           ← meta-skills (reading/writing papers)
├── school/             ← M1/M2 course notes (parent domain, sub-folders per course)
└── …                   ← future domains
```

Each domain has its own `README.md` acting as a syllabus / table of contents.
