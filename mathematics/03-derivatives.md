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
Identifies where a derivative **does not exist**. A function must be *continuous* AND *smooth* to be differentiable at a point. The four classical failures are: **corners, cusps, jumps, and vertical tangents**.

Imagine driving a **car along the graph** of the function, with the steering wheel locked to the curve. The derivative is your heading (compass direction). If the road has a sharp corner, your compass flips — is it heading left or right? If the road breaks in two (jump), you teleport — no heading either. If the road becomes vertical, your compass spins to infinity. A function is non-differentiable exactly where the road stops being a smooth, gentle curve.

**The four cases:**

### 1) Corner (kink) — left and right slopes differ

```
f(x) = |x|

     \       /
      \     /
       \   /
        \ /
    -----•-----   corner at x = 0
        / \
```
```
f'(0⁻) = −1,   f'(0⁺) = +1   ⇒   f'(0) does not exist.
```

### 2) Cusp — both slopes become infinite in opposite signs

```
f(x) = x^(2/3)
f'(x) = (2/3) · x^(−1/3)

f'(0⁻) → −∞,   f'(0⁺) → +∞   ⇒   not differentiable at x = 0.
```
Looks like a sharp "beak" — the curve comes down vertically and leaves vertically.

### 3) Jump discontinuity — the function itself is discontinuous

```
         1  ─────────
            
   ────•                    (open circle at 0⁻, closed at 0⁺)
         0
```
```
f(x) = { 0   if x < 0
       { 1   if x ≥ 0

Not continuous at 0   ⇒   not differentiable at 0.
```
Rule: **no continuity ⇒ no derivative** (but the converse is false — continuous functions can still fail to be differentiable).

### 4) Vertical tangent — slope is infinite but function is continuous

```
f(x) = ∛x = x^(1/3)
f'(x) = (1/3) · x^(−2/3)

f'(0) → +∞   ⇒   not differentiable at x = 0,
but f is continuous at 0.
```
The curve passes smoothly through 0 but momentarily goes straight up.

---

**Simple example:**
`f(x) = |x − 2|` has a **corner at x = 2**:
```
Left slope  = −1
Right slope = +1
f'(2) does not exist.
```

**Complex example (piecewise function with three failures):**
```
        { x²        if x < 0
f(x) =  { √x        if 0 ≤ x < 4
        { 3         if x = 4
        { x − 1     if x > 4
```

- At `x = 0`:  `f(0⁻) = 0`, `f(0⁺) = 0` → continuous.
  - Left derivative: `(x²)' = 2x` → at 0⁻ = 0
  - Right derivative: `(√x)' = 1/(2√x)` → at 0⁺ → +∞
  - ⇒ **vertical tangent from the right**, non-differentiable at 0.

- At `x = 4`:  `f(4⁻) = √4 = 2`, `f(4) = 3`, `f(4⁺) = 3`.
  - Function value jumps from 2 to 3 → **jump discontinuity**, non-differentiable at 4.

- Everywhere else: differentiable.
