# Optimization — Gradient Descent, Newton, Extrema

## 4.1 — Gradient Descent

**What it does:**
Finds a (local) minimum of a differentiable function by repeatedly stepping in the direction of the negative gradient. The foundation of almost every ML training loop: given a loss function `L(θ)`, find the parameters `θ` that minimize it.

Imagine standing on a **foggy mountain** with only an altimeter and a compass. You can't see the summit or the valley, but you can feel which way is downhill. So you take one step in the steepest descending direction, check again, take another step, and repeat. As long as each step is small enough, you eventually land in a valley. Gradient descent is exactly this walk — the gradient `∇L` is the "uphill" compass, so `−∇L` is the downhill direction.

**Formula (update rule):**
```
θ_{k+1} = θ_k − α · ∇L(θ_k)
```

| Symbol | Meaning |
|--------|---------|
| `θ_k`  | Parameter vector at iteration `k` (e.g., weights of a model) |
| `α`    | Learning rate — step size |
| `∇L`   | Gradient of the loss at `θ_k` |

**Simple example (1D):**
Minimize `f(x) = x²`. Gradient: `f'(x) = 2x`. Learning rate `α = 0.1`, starting at `x₀ = 5`.
```
x₁ = 5 − 0.1 · 2·5   = 5 − 1.0  = 4.0
x₂ = 4 − 0.1 · 2·4   = 4 − 0.8  = 3.2
x₃ = 3.2 − 0.1·2·3.2 = 3.2 − 0.64 = 2.56
...
```
→ The sequence converges to 0 (the true minimum) at a geometric rate: `x_{k+1} = 0.8·x_k`.

**Complex example (linear regression training):**
Fit `ŷ = w·x + b` on `n` data points with mean squared error:
```
L(w, b) = (1/n) · Σᵢ (yᵢ − w·xᵢ − b)²

∂L/∂w  = −(2/n) · Σᵢ xᵢ·(yᵢ − ŷᵢ)
∂L/∂b  = −(2/n) · Σᵢ     (yᵢ − ŷᵢ)
```
Gradient descent update:
```
w ← w − α · (∂L/∂w)
b ← b − α · (∂L/∂b)
```
→ After thousands of iterations, `(w, b)` lands close to the least-squares solution. In practice, we use **mini-batch** variants: compute the gradient on a random subset of the data per step for speed (SGD, Adam, etc.).

**Practical notes:**
- `α` too large → oscillation or divergence
- `α` too small → glacial convergence
- Non-convex loss (deep networks) → you land in *some* local minimum, not necessarily the global one

---

## 4.2 — Newton's Method (1D)

**What it does:**
Finds roots or extrema **faster than gradient descent** by using second-order information (the curvature). Where gradient descent takes many cautious steps, Newton's method takes fewer but smarter steps — extrapolating from the current slope AND curvature.

Imagine you're trying to land a plane on a runway (find `x*` where `f(x*) = 0`). Gradient descent says: "go in this direction at constant speed." Newton's method says: "given your current height AND your current descent rate, you should aim for *this specific point* — which is where a straight glide would touch the runway." Each iteration jumps to the zero of the local **linear approximation**. When the function is well-behaved, Newton converges quadratically — the error *squares* with each step.

### Root-finding: solve `f(x) = 0`

**Derivation sketch:**
Near `x_k`, approximate `f` linearly:
```
f(x) ≈ f(x_k) + f'(x_k)·(x − x_k)
```
Set the approximation equal to 0 and solve for `x`:
```
f(x_k) + f'(x_k)·(x_{k+1} − x_k) = 0
⇒  x_{k+1} = x_k − f(x_k) / f'(x_k)
```

### Minimization: find `x*` where `f'(x*) = 0`

Apply Newton's method to the **derivative** instead of `f` itself:
```
x_{k+1} = x_k − f'(x_k) / f''(x_k)
```

**Simple example (root-finding, √2):**
Solve `f(x) = x² − 2 = 0`. Derivative: `f'(x) = 2x`. Start at `x₀ = 1`.
```
x₁ = 1 − (1 − 2) / (2·1)       = 1 − (−1/2)   = 1.5
x₂ = 1.5 − (2.25 − 2) / (3)    = 1.5 − 0.0833 = 1.4167
x₃ = 1.4167 − (0.0069) / 2.833 = 1.41422
```
→ Correct to 4 decimals in 3 steps. Compare with gradient descent, which would need hundreds of steps for the same accuracy.

**Complex example (robot end-effector tracking):**
A 1-DOF robot joint has a nonlinear position error `e(q) = sin(q) − 0.3`. Use Newton's method to find `q*` such that `e(q*) = 0` (i.e., the joint angle that hits the target):
```
e'(q) = cos(q)
q_{k+1} = q_k − (sin(q_k) − 0.3) / cos(q_k)

q₀ = 0.5
q₁ = 0.5 − (0.4794 − 0.3) / 0.8776 = 0.5 − 0.2044 = 0.2956
q₂ = 0.2956 − (0.2911 − 0.3) / 0.9565 = 0.2956 + 0.0093 = 0.3049
q₃ ≈ 0.30469  (arcsin(0.3) = 0.30469…)
```
→ Quadratic convergence. This pattern is the core of **numerical inverse kinematics** (see [`ros2/20-inverse-kinematics.md`](../ros2/20-inverse-kinematics.md)).

---

## 4.3 — Newton's Method (Multivariate)

**What it does:**
Generalizes Newton's method to functions `f : ℝⁿ → ℝ`. Replaces `f''` with the **Hessian** and `f'` with the **gradient**. Finds minima by solving a linear system at each step.

Imagine the foggy mountain analogy from gradient descent, but now your **topographic sensors** can also read the **curvature in every direction** (not just the slope). With that richer info, you can extrapolate where the valley floor is and step directly there — no more cautious constant-size steps.

**Formula:**
```
θ_{k+1} = θ_k − H⁻¹(θ_k) · ∇f(θ_k)
```
where `H` is the Hessian and `∇f` is the gradient of `f`.

**For 2 variables `(x, y)`:**
```
[ x_{k+1} ]   [ x_k ]             [ f_x(x_k, y_k) ]
[ y_{k+1} ] = [ y_k ] − H⁻¹(x_k, y_k) · [ f_y(x_k, y_k) ]
```

**In practice:** you rarely invert `H` explicitly — instead solve the linear system `H · Δθ = −∇f` for the step `Δθ`, then update `θ_{k+1} = θ_k + Δθ`.

**Simple example (quadratic `f(x, y) = x² + 3xy + 2y²`):**
```
∇f = [ 2x + 3y ]        H = [ 2   3 ]
     [ 3x + 4y ]            [ 3   4 ]

det(H) = 8 − 9 = −1
H⁻¹ = (1/−1) · [ 4  −3] = [−4   3 ]
               [−3   2]   [ 3  −2 ]
```
Starting at `(1, 1)`:
```
∇f(1,1) = [5, 7]ᵀ
Δθ = −H⁻¹ · ∇f = −[−4·5 + 3·7, 3·5 − 2·7]ᵀ = −[1, 1]ᵀ
(x_1, y_1) = (1, 1) + (−1, −1) = (0, 0)
```
→ Converges to the true minimum `(0, 0)` in **one step** because `f` is quadratic. Newton is exact for quadratics.

**Complex example (logistic regression training):**
For logistic loss `L(w) = −Σ [yᵢ log σ(wᵀxᵢ) + (1 − yᵢ) log(1 − σ(wᵀxᵢ))]`, the Hessian involves `σ(1−σ)` weighted terms. Newton's method (also called IRLS — Iteratively Reweighted Least Squares) converges in typically **5-15 iterations** where gradient descent takes thousands. The cost per step is higher (inverting an n×n Hessian), so Newton wins when `n` is small; gradient descent wins when `n` is huge (deep networks).

```
┌──────────────────────────────────────────────────────────────────┐
│  Gradient descent  → fast per step, many steps, scales to ML     │
│  Newton            → expensive per step, few steps, quadratic    │
│                      convergence, best for small/medium problems │
└──────────────────────────────────────────────────────────────────┘
```

---

## 4.4 — Classifying Stationary Points (Second-Order Test)

**What it does:**
Once you find a point `θ*` where `∇f(θ*) = 0` (stationary), you need to know **what kind** of point it is: a minimum, a maximum, a saddle, or something subtler. The Hessian's eigenvalues answer this.

Imagine feeling a **bowl vs a dome vs a pringle-chip shape vs a flat floor** with your hand, blindfolded. A bowl curves up in every direction (positive curvature). A dome curves down everywhere. A pringle chip curves up one way and down the other — that's a saddle. If the floor is flat in some direction, you can't tell which shape you're on without looking further.

### The classification — precise version

Let `λ₁, λ₂, …, λₙ` be the eigenvalues of the Hessian at `θ*`.

| Condition | Type |
|-----------|------|
| **All `λᵢ > 0`** (Hessian *positive definite*) | **Local minimum** — curves up in every direction |
| **All `λᵢ < 0`** (Hessian *negative definite*) | **Local maximum** — curves down in every direction |
| **Mixed signs** (some `λᵢ > 0`, some `λᵢ < 0`) | **Saddle point** — up in some directions, down in others |
| **At least one `λᵢ = 0`** | **Test inconclusive** — need higher-order analysis (flat direction) |

> ⚠️ Subtle but important: a **saddle point** requires strict opposite signs. If any eigenvalue is zero, the second-order test fails — the point could still be a minimum, maximum, or something exotic (monkey saddle, degenerate). Don't confuse "saddle" with "degenerate."

### Shortcut for 2D

For `f(x, y)` with Hessian `H = [[f_xx, f_xy], [f_xy, f_yy]]`, let:
```
D = det(H) = f_xx · f_yy − f_xy²
```

| Condition | Type |
|-----------|------|
| `D > 0` and `f_xx > 0` | Local **minimum** |
| `D > 0` and `f_xx < 0` | Local **maximum** |
| `D < 0` | **Saddle point** |
| `D = 0` | Inconclusive |

This is algebraically equivalent to the eigenvalue test — `D > 0` means the two eigenvalues share a sign (and `f_xx` tells you which), `D < 0` means they have opposite signs, `D = 0` means at least one is zero.

**Simple example (clear minimum):**
```
f(x, y) = x² + y²

∇f = [ 2x, 2y ]ᵀ  = 0  at (0, 0)
H  = [ 2  0 ]       eigenvalues: 2, 2  (both > 0)
     [ 0  2 ]
```
→ `(0, 0)` is a **local min** (actually the global min). `f_xx = 2 > 0`, `D = 4 > 0`, classic bowl.

**Complex example (saddle point in a loss landscape):**
```
f(x, y) = x² − y²

∇f = [ 2x, −2y ]ᵀ  = 0  at (0, 0)
H  = [ 2   0 ]       eigenvalues: 2, −2  (opposite signs)
     [ 0  −2 ]

D = 2·(−2) − 0 = −4 < 0
```
→ `(0, 0)` is a **saddle point**. Along the x-axis it's a minimum (curves up), along the y-axis it's a maximum (curves down). Classic pringle chip. In ML, saddle points are the main obstacle for high-dimensional optimization — gradient descent stalls there because the gradient is zero, even though it's not a true minimum.

**Degenerate example (test fails):**
```
f(x, y) = x² + y⁴

∇f = [ 2x, 4y³ ]ᵀ  = 0  at (0, 0)
H  = [ 2   0 ]       eigenvalues: 2, 0
     [ 0   0 ]
```
→ `D = 0`, test **inconclusive**. Higher-order check reveals it's actually a minimum (since `y⁴ ≥ 0`), but the Hessian alone can't tell.

---

## Applied in

| Concept | Used in |
|---------|---------|
| **Gradient descent** | [`05-ml-basics.md`](./05-ml-basics.md) — training linear regression and classification |
| **Newton's method** | [`ros2/20-inverse-kinematics.md`](../ros2/20-inverse-kinematics.md) — numerical IK solver |
| **Hessian for extrema** | [`ros2/19-motion-planning.md`](../ros2/19-motion-planning.md) — path-cost optimization (future work) |
| **Saddle avoidance** | Future `dl/` domain — loss landscape of neural networks, second-order methods (K-FAC, L-BFGS) |
