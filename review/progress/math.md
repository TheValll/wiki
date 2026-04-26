# Math — Review Progress

**Current position:** 17/41 — *3D and acceleration: partial derivatives and second derivative* (#16 downgraded to awareness-only, not in active spaced repetition — course scope is corners + jumps only, wiki §3.7 trimmed accordingly on 2026-04-18). **#17 gated** on consolidation of two residual Module 2 gaps (see session 2026-04-24): (1) distribution of minus on polynomial expansion, (2) "max vs extremum" justification (nature of critical point, not just existence).
**Last session:** 2026-04-24 (consolidation session — chain rule re-drilled + fil rouge on Module 2 integrative)
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
| 11 | Basic derivatives: linear, powers, inverse | 1 | 2026-04-24 |
| 12 | Chain rule (function composition) | 1 | 2026-04-24 |
| 13 | Complex derivatives: trig, exponential, logarithm | 2 | 2026-04-24 |
| 14 | Calculation rules: sum, product, scalar multiplication | 1 | 2026-04-24 |
| 15 | Derivatives of quadratics and curves | 0 | 2026-04-24 |
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
| 1.1 — Euclidean Norm | [`01-linear-algebra-intuition.md`](../../mathematics/01-linear-algebra/README.md) | — | — | Explained by agent on 2026-04-18, not yet drilled as articulation |
| 1.2 — Dot Product | [`01-linear-algebra-intuition.md`](../../mathematics/01-linear-algebra/README.md) | 2026-04-18 | — | Articulated clean: projection / ombre / 3 cases (s'aident, orthogonaux, s'opposent). Gap closed after correction: `a·b` = ombre × `|b|` (not just ombre). |
| 3.6 — Chain Rule | [`intuition/3.6-chain-rule.md`](../../mathematics/intuition/3.6-chain-rule.md) | 2026-04-24 | — | Articulated on 2nd try: landed on the **dimensional argument** (`du/dx · df/du` must yield `df/dx`, addition breaks units). First try gave the correct core ("série vs parallèle → ×") but without an owned image — engrenages and amplis didn't stick. Finally anchored via *taux de change en cascade* / *réductions en cascade* (-20% puis -10% = -28%, pas -30%). Positive side of the argument (cascade of factors) still to lock in — currently relies on the negative side (addition violates units). |

---

## Session history

| Date | Session focus | Notes |
|------|---------------|-------|
| 2026-04-17 | Initialization from MathBot save code | State imported — concepts 11, 12, 13 discovered but untested in this system |
| 2026-04-17 | Warm-up #12 (chain rule) failed + lesson #14 (calculation rules) | #14 → Lv 1 (product rule OK, arithmetic slip + chain rule still weak). #12 failed twice — stays at Lv 0. |
| 2026-04-18 | Warm-up #11 + #12 + lesson #15 (derivatives of quadratics) | #12 OK (chain rule clean) → Lv 1. #11 failed (exponent decrement error on x⁻² → -14x⁻³) — stays Lv 0. #15 → Lv 1: principle grasped, but two sign slips (x = 2 written as -2; -b with b<0). Ex3 (peak-fitting 3×3 system) not attempted → walked through. |
| 2026-04-18 | Consolidation session — warm-up #13+#11 failed, drill + re-check resolved #11 and #14 | Warm-up failed (`2·ln(x)` treated as product, `(eˣ)' = x·eˣ`). Drill 4/5 (exo 4 was actually a misread of `2/√x` as `2·√x` due to cramped notation — saved feedback memory). Final re-check on `3·x^(-1/2)` clean, incl. fractional negative exponent decrement. Final: #11 → Lv 1, #14 → Lv 1. #13 untested properly (trig/exp/log not re-drilled) — stays Lv 0. #16 still deferred to next session. |
| 2026-04-18 | Review-system test — mix mode (1h) | Warm-up #13 clean (`2eˣ + cos(x) − 3ln(x)`) → Lv 1. Lesson #16 overshot course scope (4 cases including cusps/vert. tangents & fractional exponents, out of his actual study → coude + saut only). Drill 1.2 Dot Product validated (3 cases solid; correction: `a·b` = ombre × `|b|`, not just ombre). Exo 1 (cusp on `√|x|`) unfair (out of scope), exo 2 (rover) abandoned — both walked through. Saved memory `feedback_wiki_scope_vs_course.md`. #16 stays Lv 0, to be re-drilled next session at coude/saut level. |
| 2026-04-21 | Fil rouge exercise — Math 1 × Math 2 (matrix `A = [[3,1],[2,2]]`) | Covered: det 2×2, matrix×vector, norm, dot product, characteristic equation, discriminant, quadratic factoring, `det(A) = λ₁·λ₂`, 2×2 system, normalization. All calculations clean end-to-end, eigenvector verification matched. Two formulation slips flagged: (1) `u·v > 0` ≠ "même direction" — only acute angle (same-direction would be `Au = k·u`, i.e. eigenvector); (2) writing "y = -2" instead of "y = -2x, je fixe x = 1" in eigenvector resolution. Normalization introduced as new micro-skill (dividing by norm to get unit vector). Forward pointer mentioned: eigenvectors orthogonal iff `A` symmetric → Dunod Ch 14 (espaces euclidiens). |
| 2026-04-24 | Mix 1h30 — pivoted from #17 lesson to Module 2 consolidation at user's request | **Warm-up failed 2/2**: #12 chain rule wrote `+` instead of `×` (structural); #15 apogée used discriminant (roots tool) instead of `g'=0` (critical point tool) — conceptual confusion between roots and extrema. **Chain rule re-lesson via taux de change cascade + 3 exos**: Exo1 `√(x²+7)` ✓, Exo2 `1/(3x²+2)³` ✗ (didn't rewrite as `u^(-3)` — fell into fake rule `(1/h)' = 1/h'`), Exo3 rover `√(16t²+9)` structure ✓ but arithmetic bug on `inner'` (wrote 256 instead of 32t). **Fil rouge Module 2 integrative**: Q1(a) ✓ chain rule on `-(t-3)²`, Q1(b) distribute-minus slip `-(t²-6t+9) = -t²+6t+9` (missed sign on constant, derivative right by luck); Q2 apogée t*=3, h=9 ✓ numerics but justification incomplete ("=0 = sommet" doesn't distinguish min/max); Q3 `ln(t²+1)` ✓ (typo on exponent notation, numerics right); Q4 product rule on `t²·√(t+1)` structure ✓ but forced an invalid factoring in final step. Q5 meta: user self-flagged product rule as weakness ✓ accurate. **Level changes**: #12 1→0→1 net (warm-up fail recovered via drill, intuition validated); #13 1→2 (clean independent success 6 days after last seen); #15 1→0 (warm-up fail, nature-of-critical-point gap to drill next); #11, #14 stay at 1. **Chain rule intuition validated** (dimensional argument, taux de change image). **#17 partial derivatives lesson postponed** — gated on next-session consolidation of: (1) distribute-minus reflex on polynomial expansion, (2) "it's a max because..." full justification (3 possible arguments: sign of `a`, sign change of f', or f''<0). |
