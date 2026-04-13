# Algebra — Discriminant & Linear Systems

## 2.1 — Discriminant of a Quadratic

**What it does:**
Tells you how many real roots a quadratic equation `ax² + bx + c = 0` has, *before* computing them. Δ > 0 → two real roots, Δ = 0 → one (double) root, Δ < 0 → no real roots (two complex conjugates).

Imagine throwing a **ball** and asking: "will it hit the ground twice, once, or never?" The trajectory is a parabola, and the ground is the x-axis. The discriminant is a **weather forecast** — before simulating the throw, it tells you if the ball crosses the ground (Δ > 0), just grazes it (Δ = 0), or never touches it because the parabola floats entirely above (or below) the x-axis (Δ < 0).

**Formula:**
```
Δ = b² − 4ac
```
Roots (when Δ ≥ 0):
```
x = (−b ± √Δ) / (2a)
```

**Simple example:**
`x² − 5x + 6 = 0`  (a=1, b=−5, c=6)
```
Δ = (−5)² − 4·1·6 = 25 − 24 = 1
x = (5 ± 1) / 2   ⇒   x = 3  or  x = 2
```

**Complex example (projectile motion):**
A ball is thrown upward from height 2 m at velocity 10 m/s. When does it hit the ground (`y = 0`)?
`y(t) = −4.9 t² + 10 t + 2 = 0`  (a=−4.9, b=10, c=2)
```
Δ = 10² − 4·(−4.9)·2 = 100 + 39.2 = 139.2
√Δ ≈ 11.80

t = (−10 ± 11.80) / (2·−4.9)
t = (−10 + 11.80) / −9.8 = −0.184   ← rejected (t < 0)
t = (−10 − 11.80) / −9.8 =  2.224 s
```
→ The ball hits the ground at t ≈ 2.22 s.

---

## 2.2 — Solving a 2-Variable Linear System

**What it does:**
Finds the unique `(x, y)` that satisfies two linear equations simultaneously. Geometrically, the intersection of two lines in the plane.

Imagine two **detectives** arriving at a crime scene, each with a partial testimony. Alone, each testimony describes an infinite number of possible suspects (a whole line of them). But the **one suspect** that matches *both* testimonies is the intersection of the two lines — a single point. Solving the system is cross-checking the two stories to narrow down the unique answer.

**Formulas — two methods:**

*Substitution:*
```
From eq₁:  x = (c₁ − b₁·y) / a₁
Substitute into eq₂, solve for y, back-substitute for x.
```

*Cramer's rule:* for
```
a₁·x + b₁·y = c₁
a₂·x + b₂·y = c₂
```
```
D  = | a₁  b₁ | = a₁b₂ − a₂b₁
     | a₂  b₂ |

x = (c₁·b₂ − c₂·b₁) / D
y = (a₁·c₂ − a₂·c₁) / D
```

**Simple example (substitution):**
```
2x + y = 7
 x − y = 2

From eq₂:  x = 2 + y
Into eq₁:  2(2 + y) + y = 7   ⇒   4 + 3y = 7   ⇒   y = 1
x = 2 + 1 = 3
```
→ `(x, y) = (3, 1)`.

**Complex example (Cramer's rule):**
```
3x + 2y = 16
5x − 4y = 10

D  = 3·(−4) − 5·2   = −22
Dx = 16·(−4) − 10·2 = −84
Dy = 3·10 − 5·16    = −50

x = −84 / −22 = 42/11 ≈ 3.818
y = −50 / −22 = 25/11 ≈ 2.273
```

---

## 2.3 — Solving a 3-Variable System (Gaussian Elimination + Matrices)

**What it does:**
Finds `(x, y, z)` satisfying three linear equations. Gaussian elimination transforms the augmented matrix into upper-triangular form via row operations, then back-substitutes from the bottom row upward.

Imagine you have a **stack of three tangled ropes** and you want to straighten them out. You pull one rope at a time: fix the first rope's end, then use it to untangle the second, then use both to untangle the third. By the end, each rope is independent. Gaussian elimination is exactly this: use each equation to kill one variable in every equation below, leaving a clean staircase (upper-triangular matrix) that you solve bottom-up.

**Formula (workflow):**
```
1. Write the augmented matrix [A | b].
2. Row-reduce to upper triangular using Rᵢ ← Rᵢ − k·Rⱼ.
3. Back-substitute: solve the last row first, feed the result up.
```

**Simple example:**
```
 x +  y +  z = 6
2x −  y + 3z = 14
 x + 2y −  z = 2
```

Augmented:
```
[1   1   1 |  6]
[2  −1   3 | 14]
[1   2  −1 |  2]
```
Row ops:
```
R₂ ← R₂ − 2R₁:   [1   1   1 |  6]
R₃ ← R₃ −  R₁:   [0  −3   1 |  2]
                  [0   1  −2 | −4]

R₃ ← R₃ + (1/3)R₂:
                  [1    1     1  |   6  ]
                  [0   −3     1  |   2  ]
                  [0    0   −5/3 | −10/3]
```
Back-substitute:
```
−5/3 · z = −10/3   ⇒   z = 2
−3y + z = 2         ⇒   −3y + 2 = 2   ⇒   y = 0
 x + y + z = 6      ⇒    x + 0 + 2 = 6   ⇒   x = 4
```
→ `(x, y, z) = (4, 0, 2)`.

**Complex example (simplified 3-joint robot inverse kinematics):**
Find joint positions `(q₁, q₂, q₃)` satisfying three sensor equations:
```
 2q₁ +  q₂ +  q₃ =  8
 4q₁ − 6q₂       = −2
−2q₁ + 7q₂ + 2q₃ =  9
```

Augmented:
```
[ 2   1   1 |  8]
[ 4  −6   0 | −2]
[−2   7   2 |  9]
```
Row ops:
```
R₂ ← R₂ − 2R₁:   [ 2   1   1 |   8]
R₃ ← R₃ +  R₁:   [ 0  −8  −2 | −18]
                  [ 0   8   3 |  17]

R₃ ← R₃ +  R₂:   [ 2   1   1 |   8]
                  [ 0  −8  −2 | −18]
                  [ 0   0   1 |  −1]
```
Back-substitute:
```
q₃ = −1
−8q₂ − 2·(−1) = −18   ⇒   −8q₂ = −20   ⇒   q₂ = 2.5
2q₁ + 2.5 + (−1) = 8  ⇒   2q₁ = 6.5    ⇒   q₁ = 3.25
```
→ `(q₁, q₂, q₃) = (3.25, 2.5, −1)`.
