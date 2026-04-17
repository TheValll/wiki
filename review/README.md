# Review System

Personal spaced-repetition review agent. Run short, focused sessions (15-30 min) to move concepts from discovery to solid mastery across every domain of the wiki.

---

## Concept

The wiki contains **what you've studied on paper**. The review system tracks **what you've actually practiced and mastered**. These are deliberately decoupled.

```
┌─────────────────────┐     ┌──────────────────────┐
│  Wiki (reference)    │     │  Review (practice)    │
│                      │     │                       │
│  rust/, ros2/, …     │     │  checklists/ — scope  │
│  = theory you wrote  │     │  progress/   — state  │
│                      │     │  AGENT.md    — rules  │
└─────────────────────┘     └──────────────────────┘
     ↑                              ↓
     └─ agent reads for theory      agent never quizzes beyond
                                    the active checklist
```

---

## Folder layout

```
review/
├── AGENT.md                    ← framework prompt (session rules)
├── README.md                   ← this file
├── checklists/                 ← curriculum per domain (ordered concept list)
│   ├── rust.md
│   ├── math.md
│   └── <domain>.md
└── progress/                   ← state per domain (levels 0-4)
    ├── rust.md
    ├── math.md
    └── <domain>.md
```

- **`checklists/<domain>.md`** is the **curriculum**. Manually curated, in study order. The agent treats everything beyond the current position as locked.
- **`progress/<domain>.md`** is the **state**. The agent updates this at the end of every session. You never need to copy-paste save codes.

---

## How to run a session

Say *"review <domain>"* (e.g., `review math`, `review rust`). Claude will:

1. **Warm-up** — 1-2 flashcards on concepts you're currently consolidating
2. **Lesson** — theory + simple example + applied example on the next unlocked concept
3. **Challenge** — 3 original exercises (easy / tricky / applied)
4. ⏸ **Waits for your answers**
5. **Correction** — grading with reasoning feedback
6. **Bonus** — a short piece of tech/science culture
7. **Progress update** — writes to `progress/<domain>.md` + gives you a save code

---

## Mastery levels

| Level | Meaning |
|-------|---------|
| 0 | Just discovered, untested |
| 1 | First success (same day) |
| 2 | Second success (later session) |
| 3 | Third success (later session) |
| 4 | Mastered — archived, no longer quizzed |

Success: +1. Failure: −1 (floor at 0). Level 4 removes the concept from active review.

---

## Advancing the curriculum

Concepts don't unlock automatically. You advance manually:

- *"On passe au concept suivant"* → unlocks the next item in the checklist
- *"Ajoute le module proba à ma checklist math"* → moves concepts from "Not yet reached" into active scope
- *"Je pense que X est maîtrisé, passe-le en Niv 4"* → manual override

This is deliberate: even if your wiki contains the probability chapter, the agent won't quiz on it until you explicitly add it to the checklist.

---

## Initial state (as of 2026-04-17)

| Domain | Checklist | Progress | Started? |
|--------|-----------|----------|----------|
| `math` | 41 concepts across 7 modules | Stage 13/41 — in review: concepts 11, 12, 13 | Yes |
| `rust` | 13 chapters from the Rust book | Ch 3-11 covered in past quiz sessions | Pending reset to formal review system |
| `ros2` | *(to create when you want to review ROS2)* | — | No |

---

## Adding a new domain

1. Create `checklists/<domain>.md` with the ordered concept list (see `math.md` as a template)
2. Create `progress/<domain>.md` with initial state (`Current position: 0/N`)
3. Start a session: *"review <domain>"*
