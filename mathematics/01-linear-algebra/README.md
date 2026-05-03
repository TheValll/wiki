# 01 — Linear Algebra (Vectors & Matrices)

Each page covers **one concept**. Course content (formula, examples) and intuition (physical image, step-by-step decomposition) are merged in the same file.

---

## What is a vector?

Linear algebra is the **algebra of vectors**. An *algebra* in the formal sense is a set of objects together with rules for combining them — here, two rules:

1. **Addition** — two vectors add to give a vector: `x + y = z`.
2. **Scaling** — a vector multiplied by a real number gives a vector: `λ·x` with `λ ∈ ℝ`.

These two operations must stay *inside* the set of vectors (the result is again a vector). This property is called **closure**, and the set itself is a **vector space** (formal definition deferred).

### Four instances of "vector"

The same algebraic skeleton appears on very different objects:

| # | Object | Why it's a vector |
|---|---|---|
| 1 | **Geometric vectors** `x⃗, y⃗` (arrows in 2D / 3D) | `x⃗ + y⃗` is another arrow (parallelogram rule); `λ·x⃗` is a stretched arrow |
| 2 | **Polynomials** `p(t), q(t)` | Two polynomials add to a polynomial; scaled polynomial is a polynomial |
| 3 | **Audio signals** (sequences of samples) | Mixing two signals gives a signal; amplifying a signal is still a signal |
| 4 | **Tuples in ℝⁿ**, e.g. `a = [1, 2, 3]ᵀ ∈ ℝ³` | Component-wise addition + scalar multiplication |

The book — and this wiki — focuses on **case 4**, vectors in `ℝⁿ`. Most algorithms are formulated there because arrays of numbers map cleanly to computer memory.

### Notation conventions

| Symbol | Meaning |
|---|---|
| `x⃗`, `y⃗` (arrow) | Geometric vector (school / physics convention) |
| `x`, `y` (**bold**) | General vector (book convention, used in this wiki) |
| `ℝⁿ` | Set of `n`-tuples of real numbers — the vector space of length-`n` real arrays |
| `xᵢ` or `x[i]` | The `i`-th component of `x` |

> **Caveat.** Array operations on a computer don't always implement vector operations. Element-wise multiplication `x * y` in NumPy is *not* the dot product — it's a Hadamard product. Always check what your library does.

### What this chapter builds

Once vectors and their two operations are in place, everything else in linear algebra falls out as natural consequences:

```
              vector  ──── closure ───→  vector space
                │                              │
       length, angle                  linear independence
       (Chapter 3 — Analytic Geometry)        │
                                              ↓
                                            basis
                                          (max. independent set)
                                              │
              ┌───────────────────────────────┤
              ↓                               ↓
           matrix                       linear mapping
        (= list of column vectors)   (rotation, projection, system)
              │
              ├──→ system of linear equations  → Ch. 2 of this wiki
              ├──→ matrix inverse              → 1.5
              └──→ determinant, eigenvalues    → 1.6 – 1.9
```

This wiki covers the operational side (norm, dot product, matrix multiplication, inverse, determinant, eigenvalues). The deeper structural concepts (vector space, basis, linear independence, rank) are deferred — they are useful but not yet needed for the current ML / robotics work.

---

## Pages

| § | Page | In one line |
|---|------|-------------|
| 1.1 | [Vector Norm](./1.1-norm.md) | Pythagoras cascaded across perpendicular axes. |
| 1.2 | [Dot Product](./1.2-dot-product.md) | Cooperation score — signed shadow of one arrow onto another. |
| 1.3 | [Matrix × Vector](./1.3-matrix-vector.md) | Recipe for mixing the matrix's columns. |
| 1.4 | [Matrix × Matrix](./1.4-matrix-matrix.md) | Two machines in series, pre-collapsed into one. |
| 1.5 | [Matrix Inverse](./1.5-inverse.md) | The un-machine — exists iff no dimension was destroyed. |
| 1.6 | [Determinant](./1.6-determinant.md) | Signed volume factor of a machine. |
| 1.7 | [det(AB) = det(A)·det(B)](./1.7-det-product.md) | Stretches in series compound by multiplication. |
| 1.8 | [det(A⁻¹) = 1/det(A)](./1.8-det-inverse.md) | To undo a stretch, divide by it. |
| 1.9 | [Characteristic Equation](./1.9-characteristic-equation.md) | Finding the machine's direction-stable axes. |

## Applied in

| Concept | Used in |
|---------|---------|
| **Euclidean norm / dot product** | [ROS2 — Configuration Space](../../ros2/moveit/18-configuration-space.md) (weighted distance `d = √(Σ wᵢ(qᵢ−q'ᵢ)²)`) |
| **Matrix × matrix, matrix × vector** | [ROS2 — Inverse Kinematics](../../ros2/moveit/20-inverse-kinematics.md) (DH transforms, forward kinematics as a chain of 4×4 matrices) |
| **Matrix inverse, linear systems** | [ROS2 — Inverse Kinematics](../../ros2/moveit/20-inverse-kinematics.md) (Jacobian, pseudoinverse `J⁺ = Jᵀ(JJᵀ)⁻¹`, damped least squares) |
| **2×2 linear systems** | [ROS2 — DiffDrive Controller](../../ros2/ros2-control/11-controllers-diffdrive.md) (wheel velocity equations from `cmd_vel`) |
| **Distance metrics** | [ROS2 — Motion Planning](../../ros2/moveit/19-motion-planning.md) (nearest-neighbor lookups in RRT / PRM) |
| **Eigenvalues / eigenvectors** | Physics (principal moments of inertia), and future `ml/` domain (PCA, covariance decomposition) |
