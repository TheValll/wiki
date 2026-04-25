# Wiki — TODO

Backlog of structural and content work for the wiki. Updated as items land or get re-prioritised. See [`me.md`](me.md) for personal goals, [`how-i-learn.md`](how-i-learn.md) for pedagogy, and [`research/README.md`](research/README.md) for PhD-prep work.

---

## 1. Refacto: 1 concept = 1 file (course + intuition merged)

Several pages currently bundle multiple concepts (e.g. `mathematics/01-linear-algebra.md` covers vectors, matrices, products on a single page). Goal: split so each file holds **one concept**, with the intuition section folded in (no more separate `*-intuition.md` companions).

- [ ] Decide naming convention before moving anything (flat `mathematics/01-vector.md` vs nested `mathematics/linear-algebra/01-vector.md`)
- [ ] Audit `mathematics/` — list current concepts × current files, identify the splits
- [ ] Audit `rust/` — same
- [ ] Audit `ros2/` — most pages already look 1-concept; confirm and flag exceptions
- [ ] Pilot the refacto on one domain first (suggestion: `mathematics/`, smallest)
- [ ] After the pilot, decide the fate of existing `*-intuition.md` companions (merge into main file vs. delete)
- [ ] Update each domain's `README.md` + cross-domain links in the root `README.md` after the move

> Note: this breaks the "numbering is stable" rule from CLAUDE.md §9. Explicit exception, validated 2026-04-25.

---

## 2. Open the Rust review block

Rust book deep-dive quiz is done (ch 3-11) but no review system in place yet for Rust.

- [ ] Create `review/checklists/rust.md` — ordered curriculum derived from current wiki pages
- [ ] Create `review/progress/rust.md` — initial mastery state (use quiz results as baseline)
- [ ] First weak spots to drill: module visibility, `?` vs `unwrap`, lifetime elision
- [ ] Run a first review session in `practice` mode and adjust the checklist

---

## 3. Per-domain RECAP page

A `RECAP.md` in each domain — single-glance table of all concepts in the domain.

Format (per row):

| Concept | What it does / what it's for (1-2 sentences) | Intuition (short) |

To create:
- [ ] `mathematics/RECAP.md`
- [ ] `rust/RECAP.md`
- [ ] `ros2/RECAP.md`
- [ ] `research/RECAP.md`

Link from each domain's `README.md` → its `RECAP.md`.

Best done **after** the refacto in §1 — the row count stabilises once each concept has its own file.

---

## 4. PhD prep — academic groundwork

- [ ] Make research about labs and university with a PhD
- [ ] Identify and study 3-5 EPFL robotics PhD profiles — undergrad → master → PhD trajectory, papers, current position
- [ ] Build a starter reading list (15-20 papers) in robotics — apply the [literature survey procedure](research/01-reading-papers.md) to each target lab's recent output
- [ ] First pass-1 sweep on recent ICRA / IROS proceedings to triage relevant work
