# Derivatives

## 3.1 — Lagrange vs Leibniz Notation

**What it does:**
Two equivalent ways to write a derivative. Lagrange uses primes (`f'(x)`), Leibniz uses a ratio of differentials (`df/dx`). They mean the same thing — the instantaneous rate of change of `f` with respect to `x` — but emphasize different intuitions.

Imagine a **car's speedometer**. Lagrange notation says "at this moment, the speed is 60 km/h" — a single number at a given instant (`f'(t)`). Leibniz notation says "a tiny change in time `dt` produces a tiny change in position `dx`, and their ratio is the speed" — it tells you *what is divided by what*, which becomes critical in physics (units!) and in the chain rule (the fractions visually cancel).

**Notations:**
```
Lagrange:   f'(x),   f''(x),   f⁽ⁿ⁾(x)
Leibniz :  df/dx,  d²f/dx²,  dⁿf/dxⁿ
```

**Equivalence table:**

| Lagrange   | Leibniz         | Meaning                                 |
|------------|-----------------|-----------------------------------------|
| f'(x)      | df/dx           | 1st derivative                          |
| f''(x)     | d²f/dx²         | 2nd derivative                          |
| y'         | dy/dx           | when y = f(x)                           |
| (f∘g)'(x)  | df/dg · dg/dx   | chain rule (Leibniz makes it obvious)   |

**Simple example:**
For `f(x) = x²`:
```
Lagrange:  f'(x) = 2x
Leibniz :  df/dx = 2x
```
Same result, different notation.

**Complex example (why Leibniz helps in physics):**
A mass has position `x = sin(t²)`. Find velocity `dx/dt`.

With **chain rule in Leibniz** form:
```
Let u = t².  Then x = sin(u).
dx/dt = dx/du · du/dt = cos(u) · 2t = 2t · cos(t²)
```
The fractions "cancel" visually — a mnemonic Lagrange lacks. In Lagrange:
```
x'(t) = cos(t²) · 2t   (chain rule, less obvious at a glance)
```

---

## 3.2 — Derivatives of Common Functions

**What it does:**
A reference table of derivatives for the most common function families. Every complicated derivative reduces to these building blocks combined with the chain/product/quotient rules.

Imagine a **toolbox**. A carpenter does not reinvent the hammer every time — they grab the right tool from the box. Similarly, when differentiating `sin(3x² + 1)`, you grab `(sin u)' = u'·cos u` and `(xⁿ)' = n·xⁿ⁻¹` from the toolbox and combine them. Memorizing this list is the foundation of calculus.

**Core table (both basic form and composed form with `u = u(x)`):**

| Function               | Derivative (simple)       | Derivative with u = u(x)        |
|------------------------|---------------------------|---------------------------------|
| k (constant)           | 0                         | —                               |
| ax + b (linear)        | a                         | —                               |
| x²                     | 2x                        | —                               |
| ax² + bx + c (quadratic)| 2ax + b                  | —                               |
| xⁿ (power)             | n · xⁿ⁻¹                  | n · uⁿ⁻¹ · u'                   |
| 1/x (inverse)          | −1 / x²                   | −u' / u²                        |
| √x                     | 1 / (2√x)                 | u' / (2√u)                      |
| eˣ (exponential)       | eˣ                        | u' · eᵘ                         |
| aˣ                     | aˣ · ln(a)                | u' · aᵘ · ln(a)                 |
| ln(x) (natural log)    | 1 / x                     | u' / u                          |
| log_a(x)               | 1 / (x · ln a)            | u' / (u · ln a)                 |
| sin(x)                 | cos(x)                    | u' · cos(u)                     |
| cos(x)                 | −sin(x)                   | −u' · sin(u)                    |
| tan(x)                 | 1 + tan²(x) = 1 / cos²(x) | u' · (1 + tan²(u))              |

**Combination rules (detailed in sections 3.3 – 3.6):**
```
(k · u)'    = k · u'                            [scalar rule]
(u + v)'    = u' + v'                           [sum rule]
(u · v)'    = u' · v + u · v'                   [product rule]
(u / v)'    = (u' · v − u · v') / v²            [quotient rule, corollary of product rule]
(f ∘ g)'(x) = f'(g(x)) · g'(x)                  [chain rule]
```

**Simple example:**
`f(x) = 3x² + 2x − 5`
```
f'(x) = 2·3x + 2 = 6x + 2
```

**Complex example (combining rules):**

*(a) Product + chain + log:*  `f(x) = ln(x² + 1) · sin(x)`

Let `u = ln(x² + 1)` and `v = sin(x)`. Product rule:
```
f'(x) = u' · v + u · v'

u = ln(x² + 1):   u' = (x² + 1)' / (x² + 1) = 2x / (x² + 1)
v = sin(x):       v' = cos(x)

f'(x) = [2x / (x² + 1)] · sin(x) + ln(x² + 1) · cos(x)
```

*(b) Power + chain:*  `g(x) = (2x + 1)⁵`

Let `u = 2x + 1`, so `u' = 2`:
```
g'(x) = 5 · u⁴ · u' = 5 · (2x + 1)⁴ · 2 = 10 · (2x + 1)⁴
```

*(c) Quotient + trig:*  `h(x) = sin(x) / (x² + 1)`

Let `u = sin(x)`, `v = x² + 1`:
```
u' = cos(x),   v' = 2x

h'(x) = (u'·v − u·v') / v²
      = (cos(x)·(x² + 1) − sin(x)·2x) / (x² + 1)²
```

---

## 3.3 — Scalar Multiplication Rule

**What it does:**
Lets you pull a constant factor out of a derivative. If a function is scaled by `k`, its rate of change is scaled by the same `k`.

Imagine watching a recorded video of a car trip at **2× playback speed**. Every rate — velocity, acceleration, heart rate — is also doubled, uniformly. The constant `k` does not depend on `x`, so it just "rides along" untouched when you differentiate.

**Formula:**
```
(k · u)' = k · u'
```

**Simple example:**
`f(x) = 5x³`
```
f'(x) = 5 · (x³)' = 5 · 3x² = 15x²
```

**Complex example (scalar outside a composed function):**
`f(x) = 7 · sin(2x + 1)`
```
Let u = 2x + 1,   u' = 2.
(sin u)' = u' · cos(u) = 2·cos(2x + 1)

f'(x) = 7 · 2·cos(2x + 1) = 14·cos(2x + 1)
```

---

## 3.4 — Sum Rule

**What it does:**
The derivative of a sum is the sum of the derivatives. Lets you split a complicated expression into independent terms and differentiate each one separately.

Imagine a group of people walking on **parallel treadmills**, each at their own speed. The total distance of the group grows at a rate equal to the sum of the individual speeds — you can track each person independently and add the results. The sum rule says differentiation is linear: no cross-talk between terms.

**Formula:**
```
(u + v)' = u' + v'
(u − v)' = u' − v'
```

**Simple example:**
`f(x) = x² + 3x`
```
f'(x) = 2x + 3
```

**Complex example (polynomial + trig + log):**
`f(x) = x³ − 4x² + cos(x) − ln(x)`
```
f'(x) = 3x² − 8x − sin(x) − 1/x
```
Each term is differentiated in isolation, then summed.

---

## 3.5 — Product Rule

**What it does:**
Derivative of a product of two functions. **You cannot just multiply the derivatives** — you must account for the fact that both factors change at the same time, so the product grows by contributions from both sides.

Imagine a **rectangular garden** where both the length `u(t)` and the width `v(t)` grow over time. Between times `t` and `t + dt`, the area grows by three tiny pieces:
- a strip added on the right side (width grew by `dv`):   `u · dv`
- a strip added on the top (length grew by `du`):         `v · du`
- a tiny corner square (both grew at once):               `du · dv`  ← negligible (second-order)

So the rate of area growth is `u · (dv/dt) + v · (du/dt) = u'v + uv'`.

**Formula:**
```
(u · v)' = u' · v + u · v'
```
(Quotient rule is a corollary:  `(u / v)' = (u'·v − u·v') / v²`)

**Simple example:**
`f(x) = x² · sin(x)`
```
Let u = x²,      u' = 2x
Let v = sin(x),  v' = cos(x)

f'(x) = u'·v + u·v'
      = 2x · sin(x) + x² · cos(x)
```

**Complex example (polynomial × exponential):**
`f(x) = (3x² + 1) · eˣ`
```
u = 3x² + 1,   u' = 6x
v = eˣ,        v' = eˣ

f'(x) = 6x · eˣ + (3x² + 1) · eˣ
      = eˣ · (3x² + 6x + 1)
```

---

## 3.6 — Chain Rule

**What it does:**
Derivative of a **composition** `f(g(x))` — one function feeds its output into another. The total rate is the **product** of the outer rate (evaluated at the inner function) and the inner rate.

Imagine two **meshed gears**. Gear A is driven by a motor and turns at 3 rotations/second. Gear B is chained to A and turns 2 rotations for every 1 rotation of A. Then gear B turns at `3 × 2 = 6` rotations/second. The chain rule says: when functions are chained, their rates **multiply**.

**Formula:**
```
Lagrange:  (f ∘ g)'(x) = f'(g(x)) · g'(x)
Leibniz :       df/dx  = df/du · du/dx
```
Fast mnemonic for `y = (stuff)ⁿ`, `y = sin(stuff)`, `y = e^stuff`, …:
> "Derivative of the outer, **keep** the inside, **times** derivative of the inside."

**Simple example:**
`f(x) = (2x + 3)⁴`
```
Outer: y = u⁴,   dy/du = 4u³
Inner: u = 2x + 3,   du/dx = 2

f'(x) = 4u³ · 2 = 8 · (2x + 3)³
```

**Complex example (triple nesting):**
`f(x) = sin(ln(x² + 1))`

Three layers: `u = x² + 1`,  `v = ln(u)`,  `y = sin(v)`.
```
dy/dv = cos(v)
dv/du = 1/u
du/dx = 2x

df/dx = dy/dv · dv/du · du/dx
      = cos(v) · (1/u) · 2x
      = cos(ln(x² + 1)) · [2x / (x² + 1)]
```

---

## 3.7 — Non-Differentiable Functions

**What it does:**
Identifies where a derivative **does not exist**. Short section — the goal is just to *recognize* the two common patterns: **corners** and **jumps**.

### Recall — the derivative is a slope

At a smooth point, `f'(a)` is the slope of the **unique tangent line** to the graph at `x = a`. Zoom in on a smooth curve and it looks straight — that slope is `f'(a)`.

A function is **non-differentiable at `a`** when no single slope works at that point.

### Intuition — the car on the road

Drive a car along the graph, steering wheel locked to the curve. The derivative is your heading (compass direction).

| Graph shape | Heading | Differentiable? |
|-------------|---------|-----------------|
| Smooth curve | Well-defined | ✓ |
| Sharp corner | Flips abruptly | ✗ |
| Jump (broken road) | You'd have to teleport | ✗ |

---

### 1) Corner — two different slopes meet

The canonical example uses the **absolute value** function `f(x) = |x|`.

**Reminder — absolute value `|x|`** is the distance from `x` to 0 on the number line. Always ≥ 0.
```
|x| = {  x   if x ≥ 0
      { −x   if x < 0
```
Examples: `|3| = 3`, `|−5| = 5`, `|0| = 0`. It simply "strips the sign".

**Graph of `|x|`** — a "V" made of two straight half-lines meeting at the origin:
```
             y
             │
      \      │      /
       \     │     /
        \    │    /
         \   │   /
     -----•------------  x
             0
             ↑
         corner at x = 0
```

**Slope on each side:**
```
   Left side  (x < 0):  f(x) = −x    →   slope = −1
   Right side (x > 0):  f(x) = +x    →   slope = +1
```

Try to draw "the" tangent at `x = 0`:
```
       \              /
  slope \            /  slope
  = −1   \          /   = +1
          \        /
           \      /
    ────────•────────
           x = 0
```
Which line is the tangent? **Neither** — the two sides disagree. So `f'(0)` doesn't exist.

**Formal check with the limit definition** (`f'(a) = lim_{h→0} [f(a+h) − f(a)] / h`):
```
f'(0) = lim (h → 0)  |h| / h
```
- If `h → 0⁺` (from the right):  `|h|/h = h/h = +1`
- If `h → 0⁻` (from the left):   `|h|/h = (−h)/h = −1`

The two one-sided limits disagree → no overall limit → no derivative at 0.

---

### 2) Jump — the function itself breaks

The canonical example is a **step function** — it jumps from one value to another without passing through anything in between.

```
f(x) = { 0   if x < 0
       { 1   if x ≥ 0
```

**Graph:**
```
             y
             │
      1 ────────────●         ● = closed point (value is reached)
             │
             │                ○ = open point (value NOT reached)
             │
      0 ────○
             │
     ────────┼──────────  x
             0
```

Between the two pieces there is a **vertical gap of height 1** at `x = 0`. The function value "teleports" from 0 to 1 instantly.

**Why no derivative?** A tangent needs a smooth neighborhood to attach to. Here the function isn't even continuous at 0 — there's no nearby behavior to measure a slope against. It's like asking the speed of a car that teleports: undefined.

---

**Rule of thumb:** **not continuous ⇒ not differentiable.**

The reverse is **false** — a function can be perfectly continuous and still fail to be differentiable. The corner `|x|` proves it: no jump at 0, but still no derivative because the two slopes disagree.

**Short example:** `f(x) = |x − 2|` has a corner at `x = 2`. Left slope `−1`, right slope `+1`, so `f'(2)` does not exist.

---

## 3.8 — Partial Derivatives

**What it does:**
Extends the notion of derivative to functions of **several variables**. The partial derivative `∂f/∂x` measures how `f(x, y, ...)` changes when you vary only `x`, holding the other variables fixed. Core tool for gradient descent, backpropagation, physics of fields, and multivariate optimization.

Imagine a **topographic map**: altitude `f(x, y)` depends on east-west (`x`) and north-south (`y`) position. If you stand at one spot and walk **only east** for a tiny step, how fast does your altitude change? That slope is `∂f/∂x`. Walk **only north**? That's `∂f/∂y`. Each partial derivative is a slice of the landscape along one direction — you freeze everything else and do a regular 1D derivative.

**Notation:**
```
Lagrange-style :   f_x,  f_y,  f_xy (mixed)
Leibniz-style  :   ∂f/∂x,  ∂f/∂y,  ∂²f/(∂x∂y)
```

**How to compute `∂f/∂x`:**
Treat **every variable except `x`** as a constant, then differentiate normally with respect to `x`.

**Simple example:**
```
f(x, y) = 3x²y + 4y³ + 5x

∂f/∂x  =  6xy + 0 + 5   (y is frozen, so 4y³ → 0)
       =  6xy + 5

∂f/∂y  =  3x² + 12y² + 0   (x is frozen, so 5x → 0)
       =  3x² + 12y²
```

**Complex example (loss function in ML regression):**
Given prediction `ŷ = w·x + b` and squared loss `L = (y − ŷ)² = (y − wx − b)²`, compute how `L` responds to each parameter:
```
∂L/∂w  =  2·(y − wx − b)·(−x)  =  −2x·(y − ŷ)
∂L/∂b  =  2·(y − wx − b)·(−1)  =  −2·(y − ŷ)
```
→ These are the raw gradients used in **gradient descent**: each weight `w, b` is updated in the direction that reduces the loss. See [`04-optimization.md`](./04-optimization.md) for how this powers training.

---

## 3.9 — Gradient and Hessian

**What it does:**
Packages all partial derivatives of a multivariable function into structured objects.
- The **gradient** `∇f` is the vector of all first partial derivatives — it points toward the steepest ascent of `f`.
- The **Hessian** `H` is the matrix of all second partial derivatives — it describes local curvature (how the landscape bends).

Think of `∇f` as the **compass needle** on the topographic map: it always points uphill, and its length tells you how steep the climb is. Think of `H` as a **curvature sensor**: at a valley floor it reads "bowl" (positive curvature everywhere), at a ridge "dome" (negative curvature), at a pass "bowl in one direction, dome in another" (saddle).

**Formulas (for `f : ℝⁿ → ℝ`):**

Gradient (n × 1 column vector):
```
∇f = [ ∂f/∂x₁ ]
     [ ∂f/∂x₂ ]
     [   ...  ]
     [ ∂f/∂xₙ ]
```

Hessian (n × n symmetric matrix — by Schwarz's theorem `f_xy = f_yx` for smooth `f`):
```
H = [ f_xx   f_xy   ...   f_xn ]
    [ f_yx   f_yy   ...   f_yn ]
    [  ...    ...   ...    ... ]
    [ f_nx   f_ny   ...   f_nn ]
```

For `f(x, y)` (2 variables):
```
∇f = [ f_x ]        H = [ f_xx   f_xy ]
     [ f_y ]            [ f_yx   f_yy ]
```

**Simple example (2D):**
```
f(x, y) = x² + 3xy + 2y²

f_x = 2x + 3y                f_xx = 2     f_xy = 3
f_y = 3x + 4y                f_yy = 4     f_yx = 3

∇f = [ 2x + 3y ]        H = [ 2   3 ]
     [ 3x + 4y ]            [ 3   4 ]
```

**Complex example (Rosenbrock "banana" function — classic optimization benchmark):**
```
f(x, y) = (1 − x)² + 100·(y − x²)²

f_x = −2·(1 − x) + 100·2·(y − x²)·(−2x) = −2·(1−x) − 400·x·(y − x²)
f_y = 100·2·(y − x²) = 200·(y − x²)

f_xx = 2 − 400·(y − x²) + 800·x²  = 2 − 400y + 1200x²
f_xy = f_yx = −400x
f_yy = 200

At the minimum (1, 1):
∇f = [ −2·0 − 400·1·0 ]   = [ 0 ]     ← confirmed stationary point
     [       200·0    ]     [ 0 ]

H  = [ 2 − 400·1 + 1200   −400 ]   = [ 802   −400 ]
     [       −400          200 ]     [−400    200 ]

det(H) = 802·200 − (−400)² = 160 400 − 160 000 = 400 > 0
f_xx > 0  (and eigenvalues both positive)
```
→ At `(1, 1)`, `f` has a **local minimum**. The ill-conditioned Hessian (eigenvalues span two orders of magnitude) is exactly why naïve gradient descent struggles on Rosenbrock — motivating Newton's method and its variants (see [`04-optimization.md`](./04-optimization.md)).

---

## Applied in

Where derivatives show up across the wiki:

| Concept | Used in |
|---------|---------|
| **First derivative as velocity** | [ROS2 — Trajectory Generation](../ros2/21-trajectory-generation.md) — joint velocity `q̇(t) = dq/dt` |
| **Second derivative as acceleration** | [ROS2 — Trajectory Generation](../ros2/21-trajectory-generation.md) — joint acceleration `q̈(t) = d²q/dt²` |
| **Continuity classes (C¹, C², C⁴)** | [ROS2 — Trajectory Generation](../ros2/21-trajectory-generation.md) — cubic splines are C² (smooth velocity), quintic splines are C⁴ (smooth acceleration) |
| **Derivative as linear approximation** | [ROS2 — Inverse Kinematics](../ros2/20-inverse-kinematics.md) — the Jacobian is the derivative of forward kinematics: `ẋ = J · q̇` |
| **Gradients for optimization** | Future `ml/` and `dl/` domains — gradient descent, backpropagation |
