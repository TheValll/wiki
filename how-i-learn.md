# How Valentin Learns — Pedagogical Notes

This file is maintained by the agent. It captures observations about **how** Valentin absorbs, articulates, and retains information, so sessions can adapt automatically. Valentin can read and correct it at any time; the agent updates it at the end of sessions whenever a new pattern is observed.

> Companion to [`me.md`](me.md). Where `me.md` says *who* and *what*, this file says *how*.

---

## 1. Formats that work

### Step-by-step visual decomposition
Multi-frame ASCII schemas (one frame per step) land far better than a single dense diagram. Each frame isolates one transformation — e.g., for the dot product: (1) the two vectors, (2) drop the perpendicular, (3) split into parallel + perpendicular pieces, (4) read the result. Breaking the motion into discrete frames lets him mentally replay the operation later.

### Physical / mental analogies (scales beyond geometry)
Mechanical images stick: a cart being pushed, a rope being pulled, a rubber sheet being deformed, a shadow on the floor. Once the physical image is in place, the formal mechanism slots in cleanly. Favor analogies involving forces, shadows, projections, and motion — mechanical, not abstract.

This pattern works **even for non-geometric concepts**. Validated on Rust (2026-04-18): ownership = holding a package, borrowing = lending a book, lifetimes = expiration dates on loans, iterators = a conveyor belt, closures = a chef who grabs from the pantry. The more mechanical the image, the better — even when the underlying concept has no visual form at all.

### Simple sentences and conversational tone
No academic register. Short sentences. Plain words. Meta-comments like "this is the elegant part" or "c'est ça la magie" serve as attention cues. Dense, jargon-heavy paragraphs cause drift.

### Tables for consolidation
End-of-section tables (3 columns max) summarize the case analysis and serve as locking mechanisms. The "three cases" table for the dot product was immediately read as a recap that closes the concept.

### Pure-intuition companion pages
Separate from reference pages, without formulas or exercises. Used for offline review (e.g., on a train). Pattern: `<topic>.md` = reference, `<topic>-intuition.md` = under-the-hood.

---

## 2. How he checks his own understanding

### Articulation pattern: "Si je comprends bien..."
After an explanation, Valentin reformulates the concept in his own words — sometimes with slight errors or inversions. This is his verification loop: he states a rough model, the agent corrects the misalignment, the final form locks in. **Do not rewrite his draft silently.** Pinpoint exactly which word, direction, or piece was off, and why. The precise correction step is load-bearing for his retention.

### Mental image before mechanism
If the mental image is clear (the cart, the rubber sheet, the shadow), he retains the mechanism long-term. If the mechanism is presented first without an image, retention is fragile. Order to follow: **physical image → decomposition → mechanism → formula** (formula last, or not at all in intuition mode).

### Under-the-hood reasoning is primary
He retains a concept better once he understands *why* it works, not just how to apply it. Derivations are welcome, but only after the intuition is in place. Premature abstraction loses him.

---

## 3. Signals to watch

| Signal | Meaning | Response |
|---|---|---|
| "j'ai compris", "c'est super utile" | Solid, ready to advance | Move on; save / consolidate |
| "si je comprends bien..." | Articulating a draft model | Listen for inversions or mismatches; correct precisely, not globally |
| Short neutral reply, no follow-up | Not fully locked | Offer a complementary angle (schema or analogy) before moving on |
| Explicit request for a schema | Words alone are insufficient for this concept | Always comply; flag the pattern for similar concepts in the future |
| "je marche comme ça" | He is naming a personal learning mechanism | Note it here for future sessions |

---

## 4. Session pacing

- Short preambles, long demonstrations. He asks for the meat early.
- One concept at a time, layered in depth rather than breadth per session.
- When he says "on s'arrête là" / "on va s'arreter", save state immediately — he values clean closure.
- He validates substantial changes before implementation ("propose before acting") — always give a recap for non-trivial work.

---

## 5. What not to do

- Do not explain a geometric or algebraic concept formula-first — the formula is the last step, not the first.
- Do not bundle multiple concepts into one session block without his agreement — one concept, layered, beats three concepts, shallow.
- Do not rephrase his articulation draft as if you never heard it — name the delta, don't overwrite it.

---

*Last updated by the agent: 2026-04-18 — initialized after multi-session linear algebra intuition work (sections 1.1 Norm, 1.2 Dot Product). Patterns observed over ~4 sessions.*
