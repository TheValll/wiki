# Review System

Personal spaced-repetition review agent. Runs **1h-2h evening sessions** that blend spaced repetition, original practice, and articulated recall — moving concepts from discovery to solid mastery across every domain of the wiki.

---

## Concept

The wiki contains **what you've studied on paper**. The review system tracks **what you've actually practiced and retained**. These are deliberately decoupled.

```
┌─────────────────────────────┐     ┌──────────────────────────┐
│  Wiki (reference)           │     │  Review (practice)        │
│                             │     │                           │
│  rust/, ros2/, mathematics/ │     │  checklists/  — scope     │
│  *-intuition.md companions  │     │  progress/    — state     │
│  = theory you wrote         │     │  external-tracking — ext. │
│                             │     │  AGENT.md     — rules     │
└─────────────────────────────┘     └──────────────────────────┘
        ↑                                       ↓
        └─ agent reads for theory               agent never quizzes beyond
           (incl. intuition companions          the active checklist
           in intuition mode)
```

---

## Folder layout

```
review/
├── AGENT.md                   ← framework prompt (session rules)
├── README.md                  ← this file
├── external-tracking.md       ← non-wiki state (book chapters, papers, projects)
├── checklists/                ← curriculum per domain (ordered concept list)
│   ├── rust.md
│   ├── math.md
│   └── <domain>.md
└── progress/                  ← state per domain (levels 0-4 + intuition drills)
    ├── rust.md
    ├── math.md
    └── <domain>.md
```

- **`checklists/<domain>.md`** — curriculum, manually curated in study order. Everything beyond the current position is locked.
- **`progress/<domain>.md`** — state, updated at the end of every session. Also logs intuition drills when the domain has an intuition companion.
- **`external-tracking.md`** — a single file for things outside the wiki (Rust book chapter, paper queue, DeepSight milestones, PhD deadlines). Read at the end of every session for the check-in.

---

## How to run a session

Say *"review"* — with or without a domain. Claude always runs the **opener** first:

### 1. Session opener (2-3 min)

Claude asks:
- How much time you have tonight (1h, 1h30, 2h)
- Which domains you want to cover (1, 2, or 3)
- For each domain that supports it: which **mode** (`practice` / `intuition` / `mix`)
- Any specific focus inside a domain ("lifetimes in Rust", "dot product under the hood")

Claude then proposes a **time allocation** (e.g., 2h = 70 min math-practice + 40 min rust-practice + 10 min check-in), you validate or adjust.

### 2. Per-domain block (6 steps, unchanged)

For each domain in the plan:

1. **Warm-up** — 1-2 flashcards on concepts in active review (occasionally pulls a Level-4 concept aged >3 weeks to cold-check long-term retention)
2. **Lesson** — theory + simple example + applied example on the next concept
3. **Challenge** — **3 original exercises** (practice mode) OR **1 articulation drill** (intuition mode, no formulas) OR both (mix)
4. ⏸ **Waits for your answers**
5. **Correction** — grading with reasoning feedback (or articulation-gap feedback in intuition mode)
6. **Bonus** — tech/science culture paragraph (or fresh analogy in intuition mode)
7. **Progress update** — writes to `progress/<domain>.md` + gives you a save code

### 3. Final check-in (~5 min, always runs)

Claude reads `external-tracking.md` and asks 2-3 short concrete questions about state outside the wiki: Rust book chapter, papers read, DeepSight milestones, PhD prep, etc. If you report an update, Claude edits `external-tracking.md` on the spot.

---

## Practice modes

| Mode | When to use | Challenge format |
|------|-------------|------------------|
| **practice** | You want to apply, compute, solve problems | 3 original exercises (easy / tricky / applied) |
| **intuition** | You want to consolidate *why* a concept works, no calculation | 1 drill: re-explain in your own words, with your own analogies/schemas, **no formulas allowed** |
| **mix** | You want both | 1 articulation drill + 2 original exercises |

Intuition mode is currently available for:
- `maths` — backed by `mathematics/01-linear-algebra-intuition.md`
- `rust` — backed by `rust/rust-intuition.md` (covers chapters 1-13)

It becomes automatically available for any domain that grows an intuition companion page.

---

## Mastery levels

| Level | Meaning |
|-------|---------|
| 0 | Just discovered, untested |
| 1 | First success (same day) |
| 2 | Second success (later session) |
| 3 | Third success (later session) |
| 4 | Mastered — archived, no longer actively quizzed (but may be cold-checked after 3+ weeks) |

Success: +1. Failure: −1 (floor at 0). Level 4 removes the concept from active review. Intuition-mode articulations are tracked separately in a dedicated table in `progress/<domain>.md`.

---

## Advancing the curriculum

Concepts don't unlock automatically. You advance manually:

- *"On passe au concept suivant"* → unlocks the next item in the checklist
- *"Ajoute le module proba à ma checklist math"* → moves concepts from "Not yet reached" into active scope
- *"Je pense que X est maîtrisé, passe-le en Niv 4"* → manual override

This is deliberate: even if your wiki contains the probability chapter, the agent won't quiz on it until you explicitly add it to the checklist.

---

## Current state

| Domain | Checklist | Progress | Intuition companion? |
|--------|-----------|----------|----------------------|
| `math` | 41 concepts across 7 modules | Stage 15/41 — in review: 11-15 | ✓ (linear algebra, growing) |
| `rust` | 37 concepts across 8 modules (book ch 1-11) | Stage 34/37 — ch 12-13 being read | ✓ (`rust-intuition.md`, ch 1-13) |
| `ros2` | *(not yet created)* | — | — |

---

## Adding a new domain

1. Create `checklists/<domain>.md` with the ordered concept list (see `math.md` as a template)
2. Create `progress/<domain>.md` with initial state (`Current position: 0/N`)
3. *(Optional)* Create `<domain>/*-intuition.md` companion pages to enable intuition mode
4. Start a session: *"review"* → pick the new domain in the opener menu
