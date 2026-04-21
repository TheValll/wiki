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
└── progress/                  ← state per domain (flow M: levels 0-4 · flow C: validated/in progress)
    ├── rust.md                   (flow C)
    ├── math.md                   (flow M)
    └── <domain>.md
```

- **`checklists/<domain>.md`** — curriculum, manually curated in study order. Everything beyond the current position is locked.
- **`progress/<domain>.md`** — state, updated at the end of every session. Declares which **review flow** the domain uses at the top (see "Two review flows" below). Also logs intuition drills when the domain has an intuition companion.
- **`external-tracking.md`** — a single file for things outside the wiki (Rust book chapter, paper queue, DeepSight milestones, PhD deadlines). Read at the end of every session for the check-in.

---

## Two review flows

Different domains use different learning models. Pick one per domain.

| Flow | Short name | Model | Domains |
|------|-----------|-------|---------|
| **Flow M** | Mastery (spaced repetition) | Multi-session consolidation, levels 0→4, warm-ups on old concepts | `maths`, `ros2` |
| **Flow C** | Competence validation | Single-session pass/fail, binary status, no warm-ups (retention driven by real-world use) | `rust` |

The flow affects the per-domain block structure and the progress file format. Full spec in [`AGENT.md`](./AGENT.md) §1.0, §3.M, §3.C.

---

## How to run a session

Say *"review"* — with or without a domain. Claude always runs the **opener** first:

### 1. Session opener (2-3 min)

Claude asks:
- How much time you have tonight (1h, 1h30, 2h)
- Which domains you want to cover (1, 2, or 3)
- For each domain, which **mode** (depends on the flow):
  - Flow M domains (`maths`): `practice` / `intuition` / `mix`
  - Flow C domains (`rust`): `validation` (default) or `intuition` (standalone drill)
- Any specific focus inside a domain ("lifetimes in Rust", "dot product under the hood")

Claude then proposes a **time allocation** (e.g., 2h = 70 min math-practice + 40 min rust-validation + 10 min check-in), you validate or adjust.

### 2. Per-domain block — structure depends on flow

**Flow M — Mastery (maths, ros2): 6 steps**

1. **Warm-up** — 1-2 flashcards on concepts in active review (occasionally pulls a Level-4 concept aged >3 weeks to cold-check long-term retention)
2. **Lesson** — theory + simple example + applied example on the next concept
3. **Challenge** — 3 original exercises (practice) OR 1 articulation drill (intuition, no formulas) OR both (mix)
4. ⏸ **Waits for your answers**
5. **Correction** — grading with reasoning feedback (or articulation-gap feedback in intuition mode)
6. **Bonus** — tech/science culture paragraph (or fresh analogy in intuition mode)
7. **Progress update** — writes to `progress/<domain>.md` + gives you a save code

**Flow C — Competence validation (rust): 5 steps**

1. **Theory check** — brief reformulation of the concept in your own words (quick, not a full articulation drill)
2. **Lesson** *(only if the concept is new, or theory was off)* — theory + simple + applied example
3. **Exercise battery** — **4 to 5 original exercises**, progressive difficulty
4. ⏸ **Waits for your answers**
5. **Correction & verdict** — theory OK + ≥4/5 exos correct → **Validated**; otherwise → **In progress** with gap notes
6. **Progress update** — moves concept to Validated or keeps it In progress with updated gaps

No warm-up. No spaced repetition. Retention post-validation is your job — you keep Rust alive by coding on real projects.

### 3. Final check-in (~5 min, always runs)

Claude reads `external-tracking.md` and asks 2-3 short concrete questions about state outside the wiki: Rust book chapter, papers read, DeepSight milestones, PhD prep, etc. If you report an update, Claude edits `external-tracking.md` on the spot.

---

## Practice modes

**Flow M (maths):**

| Mode | When to use | Challenge format |
|------|-------------|------------------|
| **practice** | You want to apply, compute, solve problems | 3 original exercises (easy / tricky / applied) |
| **intuition** | You want to consolidate *why* a concept works, no calculation | 1 drill: re-explain in your own words, with your own analogies/schemas, **no formulas allowed** |
| **mix** | You want both | 1 articulation drill + 2 original exercises |

**Flow C (rust):**

| Mode | When to use | Challenge format |
|------|-------------|------------------|
| **validation** | You want to pass a Rust concept to "Validated" (the default for Rust) | 4-5 original exercises (easy / intermediate / trap / applied / bonus) |
| **intuition** | Standalone articulation drill, orthogonal to validation | 1 drill: re-explain in your own words, no code/formulas |

Intuition mode is currently available for:
- `maths` — backed by `mathematics/intuition/*.md` (§§1.1-1.9, 2.1-2.4, 3.1-3.9 as of 2026-04-21)
- `rust` — backed by `rust/rust-intuition.md` (covers chapters 1-13)

It becomes automatically available for any domain that grows an intuition companion page.

---

## Progress states — per flow

**Flow M — mastery levels (0 → 4)** — used in `maths`, `ros2`:

| Level | Meaning |
|-------|---------|
| 0 | Just discovered, untested |
| 1 | First success (same day) |
| 2 | Second success (later session) |
| 3 | Third success (later session) |
| 4 | Mastered — archived, no longer actively quizzed (but may be cold-checked after 3+ weeks) |

Success: +1. Failure: −1 (floor at 0). Level 4 removes the concept from active review.

**Flow C — binary validation** — used in `rust`:

| State | Meaning |
|-------|---------|
| In progress | In the checklist, not yet validated. Gap notes may be attached from prior attempts. |
| Validated | Passed a full validation session (theory OK + ≥4/5 exos correct). Archived — retention is the user's job. |

No intermediate levels. No cold-checks. If the user reports a real-world gap on a validated concept, he explicitly requests to re-open it.

Intuition-mode articulations are tracked separately in a dedicated table in `progress/<domain>.md`, regardless of flow.

---

## Advancing the curriculum

Concepts don't unlock automatically. You advance manually:

- *"On passe au concept suivant"* → unlocks the next item in the checklist
- *"Ajoute le module proba à ma checklist math"* → moves concepts from "Not yet reached" into active scope
- *"Je pense que X est maîtrisé, passe-le en Niv 4"* → manual override

This is deliberate: even if your wiki contains the probability chapter, the agent won't quiz on it until you explicitly add it to the checklist.

---

## Current state

| Domain | Flow | Checklist | Progress | Intuition companion? |
|--------|------|-----------|----------|----------------------|
| `math` | M | 41 concepts across 7 modules | Stage 17/41 — in review: 11-15 | ✓ (`intuition/` — §§1-3 covered) |
| `rust` | C | 37 concepts across 8 modules (book ch 1-11) | 11 Validated · 4 In progress | ✓ (`rust-intuition.md`, ch 1-13) |
| `ros2` | *(not yet set up)* | — | — | — |

---

## Adding a new domain

1. Create `checklists/<domain>.md` with the ordered concept list (see `math.md` as a template)
2. Decide which **flow** fits the domain:
   - **Flow M** — when long-term retention comes from spaced repetition on the reviewed material itself (core theoretical domains, formal study)
   - **Flow C** — when long-term retention comes from *using* the material in real work (programming languages, tools, frameworks — things practiced daily)
3. Create `progress/<domain>.md` with initial state. Declare the flow at the top (`**Review flow:** Flow M` or `Flow C`). Format follows `math.md` (Flow M) or `rust.md` (Flow C) as a template.
4. *(Optional)* Create `<domain>/*-intuition.md` companion pages to enable intuition mode
5. Start a session: *"review"* → pick the new domain in the opener menu
