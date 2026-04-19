# Math — Review Progress

**Current position:** 17/41 — *3D and acceleration: partial derivatives and second derivative* (#16 downgraded to awareness-only, not in active spaced repetition — course scope is corners + jumps only, wiki §3.7 trimmed accordingly on 2026-04-18)
**Last session:** 2026-04-18 (3rd of the day — review-system test, #16 intro)
**Initialized from:** `[Sauvegarde | Étape de la checklist = 13/41 | Nouveau Concept = 13. Dérivées complexes (Niv 0) | Maîtrise validée = {Module 0, Module 1} | En cours de consolidation = {11. Dérivées simples: Niv 0, 12. Chain Rule: Niv 0, 13. Dérivées complexes: Niv 0} | Prochaine étape = 14. Les règles de calcul]`

---

## Mastered (Level 4) — archived, no longer quizzed

### Module 0 — Foundations
- 1. Powers and square roots
- 2. Fractions and basic algebraic manipulation
- 3. Isolating a variable (ax + b = c)
- 4. Remarkable identities and factoring
- 5. Laws of logarithms and exponentials

### Module 1 — Linear Algebra
- 6. Vectors: norm and dot product
- 7. Matrices: vector × matrix and matrix × matrix
- 8. Matrices: determinant and inverse
- 9. Systems of equations: 2 and 3 variables, discriminant
- 10. Advanced resolution: Gaussian elimination

---

## In review (active spaced repetition)

| # | Concept | Level | Last seen |
|---|---------|-------|-----------|
| 11 | Basic derivatives: linear, powers, inverse | 1 | 2026-04-18 |
| 12 | Chain rule (function composition) | 1 | 2026-04-18 |
| 13 | Complex derivatives: trig, exponential, logarithm | 1 | 2026-04-18 |
| 14 | Calculation rules: sum, product, scalar multiplication | 1 | 2026-04-18 |
| 15 | Derivatives of quadratics and curves | 1 | 2026-04-18 |
| 16 | Non-differentiable functions (corners + jumps, awareness only — not drilled) | — | 2026-04-18 |

---

## Not yet reached (locked — do not quiz)

### Module 2 — Analysis (remaining)
- 17. 3D and acceleration: partial derivatives and second derivative ← **next**

### Module 3 — Probability & Statistics
- 18-25 (locked)

### Module 4 — AI & Optimization
- 26-30 (locked)

### Module 5 — Spatial Geometry & Kinematics
- 31-34 (locked)

### Module 6 — State Estimation & Probabilistic Robotics
- 35-38 (locked)

### Module 7 — Dynamics & Control
- 39-41 (locked)

---

## Intuition drills (under-the-hood, no-formulas mode)

Concepts articulated in **intuition mode** — re-explained by the user in his own words, using his own analogies and schemas, **without formulas**. Source: `mathematics/*-intuition.md` companion pages.

Format: each successful articulation = "validated". A second successful articulation a few weeks later = "consolidated".

| Concept | Source | First validated | Consolidated | Notes |
|---------|--------|-----------------|--------------|-------|
| 1.1 — Euclidean Norm | [`01-linear-algebra-intuition.md`](../../mathematics/01-linear-algebra-intuition.md) | — | — | Explained by agent on 2026-04-18, not yet drilled as articulation |
| 1.2 — Dot Product | [`01-linear-algebra-intuition.md`](../../mathematics/01-linear-algebra-intuition.md) | 2026-04-18 | — | Articulated clean: projection / ombre / 3 cases (s'aident, orthogonaux, s'opposent). Gap closed after correction: `a·b` = ombre × `|b|` (not just ombre). |

---

## Session history

| Date | Session focus | Notes |
|------|---------------|-------|
| 2026-04-17 | Initialization from MathBot save code | State imported — concepts 11, 12, 13 discovered but untested in this system |
| 2026-04-17 | Warm-up #12 (chain rule) failed + lesson #14 (calculation rules) | #14 → Lv 1 (product rule OK, arithmetic slip + chain rule still weak). #12 failed twice — stays at Lv 0. |
| 2026-04-18 | Warm-up #11 + #12 + lesson #15 (derivatives of quadratics) | #12 OK (chain rule clean) → Lv 1. #11 failed (exponent decrement error on x⁻² → -14x⁻³) — stays Lv 0. #15 → Lv 1: principle grasped, but two sign slips (x = 2 written as -2; -b with b<0). Ex3 (peak-fitting 3×3 system) not attempted → walked through. |
| 2026-04-18 | Consolidation session — warm-up #13+#11 failed, drill + re-check resolved #11 and #14 | Warm-up failed (`2·ln(x)` treated as product, `(eˣ)' = x·eˣ`). Drill 4/5 (exo 4 was actually a misread of `2/√x` as `2·√x` due to cramped notation — saved feedback memory). Final re-check on `3·x^(-1/2)` clean, incl. fractional negative exponent decrement. Final: #11 → Lv 1, #14 → Lv 1. #13 untested properly (trig/exp/log not re-drilled) — stays Lv 0. #16 still deferred to next session. |
| 2026-04-18 | Review-system test — mix mode (1h) | Warm-up #13 clean (`2eˣ + cos(x) − 3ln(x)`) → Lv 1. Lesson #16 overshot course scope (4 cases including cusps/vert. tangents & fractional exponents, out of his actual study → coude + saut only). Drill 1.2 Dot Product validated (3 cases solid; correction: `a·b` = ombre × `|b|`, not just ombre). Exo 1 (cusp on `√|x|`) unfair (out of scope), exo 2 (rover) abandoned — both walked through. Saved memory `feedback_wiki_scope_vs_course.md`. #16 stays Lv 0, to be re-drilled next session at coude/saut level. |
