# Linear Algebra — Vectors & Matrices

## 1.1 — Vector Norm (Euclidean)

**What it does:**
Measures the length (magnitude) of a vector. In robotics, used to compute speed from a velocity vector, or distance from a position vector. The norm reduces a directional quantity to a single scalar.

Imagine you are pulling on a rope attached to a cart. The rope has a direction (which way you pull) and a strength (how hard you pull). The **norm** is just the strength — how many Newtons are in the rope — regardless of whether you pull east, north, or up. It is the "size" of the arrow, ignoring where it points.

**Formula:**
```
||v|| = √( v₁² + v₂² + ... + vₙ² )
```

**Simple example (2D):**
`v = (3, 4)`
```
||v|| = √(3² + 4²) = √(9 + 16) = √25 = 5
```

**Complex example (drone velocity):**
A drone has velocity `v = (1.2, -0.8, 2.5)` m/s (east, north, up):
```
||v|| = √(1.2² + (-0.8)² + 2.5²)
      = √(1.44 + 0.64 + 6.25)
      = √8.33
      ≈ 2.89 m/s
```
→ The drone moves at ~2.89 m/s overall, regardless of direction. Used for speed limits, energy budgets, etc.

---

## 1.2 — Dot Product

**What it does:**
Measures how much two vectors "point in the same direction". Returns a scalar. Used to compute angles between vectors, project one vector onto another, check orthogonality (dot = 0), and compute work in physics (W = F · d).

Imagine two people pushing a shopping cart. If they push in the **same direction**, their forces add up fully — the dot product is big and positive. If they push **perpendicular** to each other, one pushes forward and one sideways, and the sideways push does nothing to move the cart — the dot product is zero. If they push **against** each other, it is negative.

**Formula:**
```
a · b = a₁b₁ + a₂b₂ + ... + aₙbₙ
      = ||a|| · ||b|| · cos(θ)
```

**Simple example (2D):**
`a = (1, 2)`, `b = (3, 4)`:
```
a · b = 1·3 + 2·4 = 3 + 8 = 11
```

**Complex example (angle between robot arm and gravity):**
Robot end-effector direction `a = (0.6, 0, 0.8)` (unit vector), gravity direction `g = (0, 0, -1)`:
```
a · g = 0·0 + 0·0 + 0.8·(-1) = -0.8
cos(θ) = -0.8 / (1 · 1) = -0.8
θ = arccos(-0.8) ≈ 143°
```
→ The arm points 143° from gravity (i.e., 37° above horizontal). Used to compute gravity compensation torque.

---

## 1.3 — Matrix × Vector Multiplication

**What it does:**
Applies a linear transformation (rotation, scaling, shear, projection) encoded in a matrix to a vector. This is the workhorse of 3D graphics, robotics (forward kinematics), and neural networks.

Imagine a **machine** with several dials (the matrix rows) and a row of ingredients (the input vector). Each dial reads how much of each ingredient it wants, mixes them according to its own recipe, and outputs one number. Put all outputs together and you get the new vector. Change the matrix → change the recipe; change the vector → change the ingredients.

**Formula:**
For `A` of shape (m, n) and `v` of shape (n):
```
(Av)ᵢ = Σⱼ Aᵢⱼ · vⱼ
```

**Simple example (2×2 · 2):**
```
A = [2  0]     v = [3]
    [1  3]         [4]

Av = [2·3 + 0·4]   = [ 6]
     [1·3 + 3·4]     [15]
```

**Complex example (2D rotation by 90°):**
Rotation matrix `R` by 90° counter-clockwise applied to point `p = (1, 0)`:
```
R = [cos90  -sin90] = [0  -1]     p = [1]
    [sin90   cos90]   [1   0]         [0]

Rp = [0·1 + (-1)·0] = [0]
     [1·1 +   0·0]    [1]
```
→ The point (1, 0) rotated by 90° becomes (0, 1). Core of every robot's TF transform.

---

## 1.4 — Matrix × Matrix Multiplication

**What it does:**
Composes two linear transformations. Multiplying `B` then `A` is the same as applying a single combined matrix `AB`. Essential whenever you chain operations (rotate, then translate, then scale).

Imagine two **assembly lines** in sequence. The first line (matrix B) turns raw ingredients into subassemblies. The second line (matrix A) turns subassemblies into finished products. Instead of running both lines every time, you can build a **super-machine** `AB` that does both steps in one pass. Matrix multiplication *builds* that super-machine.

**Formula:**
For `A` of shape (m, n) and `B` of shape (n, p):
```
(AB)ᵢⱼ = Σₖ Aᵢₖ · Bₖⱼ
```

**Simple example (2×2 · 2×2):**
```
A = [1  2]    B = [5  6]
    [3  4]        [7  8]

AB = [1·5 + 2·7   1·6 + 2·8]   = [19  22]
     [3·5 + 4·7   3·6 + 4·8]     [43  50]
```

**Complex example (3×3 · 3×3):**
```
A = [1  0  2]    B = [2  1  0]
    [0  3  1]        [1  0  1]
    [4  1  0]        [0  2  3]

AB[0,0] = 1·2 + 0·1 + 2·0 = 2
AB[0,1] = 1·1 + 0·0 + 2·2 = 5
AB[0,2] = 1·0 + 0·1 + 2·3 = 6
AB[1,0] = 0·2 + 3·1 + 1·0 = 3
AB[1,1] = 0·1 + 3·0 + 1·2 = 2
AB[1,2] = 0·0 + 3·1 + 1·3 = 6
AB[2,0] = 4·2 + 1·1 + 0·0 = 9
AB[2,1] = 4·1 + 1·0 + 0·2 = 4
AB[2,2] = 4·0 + 1·1 + 0·3 = 1

AB = [2  5  6]
     [3  2  6]
     [9  4  1]
```
→ Used in URDF chains: base_link → shoulder → elbow → wrist collapsed into a single transform.

---

## 1.5 — Matrix Inverse

**What it does:**
The inverse `A⁻¹` "undoes" the transformation of `A`: `A⁻¹A = I`. Used to solve linear systems (`Ax = b` → `x = A⁻¹b`), invert kinematics, and reverse coordinate transforms.

Imagine a **gearbox** that converts engine rotations to wheel rotations at a 3:1 ratio. The inverse is another gearbox at 1:3 that converts wheel rotations back to engine rotations. If you run both in sequence you get back exactly what you started with — the identity. Some transformations (like crushing something flat) cannot be undone — those matrices are **singular** and have no inverse.

**Formula:**
```
A · A⁻¹ = A⁻¹ · A = I
```
For a 2×2 matrix:
```
A = [a  b]    A⁻¹ = 1/det(A) · [ d  -b]
    [c  d]                      [-c   a]
```
where `det(A) = ad - bc ≠ 0`.

**Simple example (2×2):**
```
A = [4  7]     det(A) = 4·6 - 7·2 = 10
    [2  6]

A⁻¹ = 1/10 · [ 6  -7] = [ 0.6  -0.7]
             [-2   4]   [-0.2   0.4]

Check:  A · A⁻¹ = [4·0.6 + 7·(-0.2)   4·(-0.7) + 7·0.4]  = [1  0] ✓
                  [2·0.6 + 6·(-0.2)   2·(-0.7) + 6·0.4]    [0  1]
```

**Complex example (solve `Ax = b` by inversion):**
```
A = [2  1]    b = [ 5]
    [1  3]        [10]

det(A) = 2·3 - 1·1 = 5
A⁻¹ = 1/5 · [ 3  -1] = [ 0.6  -0.2]
            [-1   2]   [-0.2   0.4]

x = A⁻¹ b = [ 0.6·5  + (-0.2)·10] = [ 3 - 2] = [1]
            [-0.2·5  +   0.4·10 ]   [-1 + 4]   [3]
```
→ Solution: `x = 1, y = 3`. Check: `2·1 + 1·3 = 5 ✓` and `1·1 + 3·3 = 10 ✓`.

---

## 1.6 — Determinant

**What it does:**
A scalar summary of a square matrix that tells you (1) whether the matrix is invertible (det ≠ 0), (2) by how much the transformation scales area/volume, and (3) whether it flips orientation (det < 0 means mirror-reflection).

Imagine a **unit square** painted on a rubber sheet. Apply the transformation `A` to the sheet. The determinant is the **signed area** of the resulting parallelogram. det = 2 → square doubled in size. det = 1 → area preserved. det = 0 → the square got squashed into a line (no inverse possible). det < 0 → the square was flipped like a pancake.

**Formula:**
For 2×2:
```
det[a  b] = ad - bc
   [c  d]
```
For 3×3 (Sarrus rule):
```
det[a  b  c]
   [d  e  f]  =  aei + bfg + cdh  -  ceg - bdi - afh
   [g  h  i]
```

**Simple example (2×2):**
```
A = [3  8]
    [4  6]
det(A) = 3·6 - 8·4 = 18 - 32 = -14
```
→ Area scaled by 14, orientation flipped.

**Complex example (3×3 via Sarrus):**
```
A = [1  2  3]
    [4  5  6]
    [7  8  10]

det(A) = 1·5·10 + 2·6·7 + 3·4·8
       - 3·5·7 - 2·4·10 - 1·6·8
       = 50 + 84 + 96 - 105 - 80 - 48
       = 230 - 233 = -3
```
→ det ≠ 0, so `A` is invertible.

---

## 1.7 — Determinant of a Product:  det(AB) = det(A)·det(B)

**What it does:**
The determinant distributes over matrix multiplication. Chaining two transformations multiplies their area-scaling factors.

Imagine you run a rubber sheet through **two presses in sequence**. The first press doubles the area (det = 2). The second press triples it (det = 3). The final area is 6× the original — you *multiply* the scalings. The determinant captures exactly this cumulative area change.

**Formula:**
```
det(A · B) = det(A) · det(B)
```

**Simple example:**
```
A = [2  0]    det(A) = 6
    [0  3]
B = [1  1]    det(B) = 1
    [0  1]

AB = [2  2]    det(AB) = 2·3 - 2·0 = 6
     [0  3]                          = det(A)·det(B) = 6·1 = 6 ✓
```

**Complex example (2×2 verification):**
```
A = [3  1]    det(A) = 3·2 - 1·4 = 2
    [4  2]
B = [5  6]    det(B) = 5·8 - 6·7 = -2
    [7  8]

AB = [3·5+1·7   3·6+1·8] = [22  26]
     [4·5+2·7   4·6+2·8]   [34  40]

det(AB) = 22·40 - 26·34 = 880 - 884 = -4
det(A)·det(B) = 2·(-2) = -4 ✓
```

---

## 1.8 — Determinant of an Inverse:  det(A⁻¹) = 1/det(A)

**What it does:**
If a transformation scales area by `k`, its inverse scales area by `1/k`. Follows directly from `det(AB) = det(A)·det(B)` applied to `A · A⁻¹ = I`.

Back to the **rubber press**: if the press doubles the area, the inverse press must halve it to return to the original. The inverse's scaling factor is the reciprocal of the forward scaling factor.

**Formula:**
```
det(A⁻¹) = 1 / det(A)
```

**Proof sketch:**
```
det(A · A⁻¹) = det(I) = 1
det(A) · det(A⁻¹) = 1
⇒ det(A⁻¹) = 1/det(A)
```

**Simple example (2×2):**
```
A = [4  7]    det(A) = 10
    [2  6]
A⁻¹ = [ 0.6  -0.7]   det(A⁻¹) = 0.6·0.4 - (-0.7)·(-0.2)
      [-0.2   0.4]              = 0.24 - 0.14 = 0.10 = 1/10 ✓
```

**Complex example (3×3 diagonal):**
```
A = [2  0  0]    det(A) = 2·3·4 = 24
    [0  3  0]
    [0  0  4]
A⁻¹ = [1/2   0    0 ]    det(A⁻¹) = (1/2)·(1/3)·(1/4) = 1/24 ✓
      [ 0   1/3   0 ]
      [ 0    0   1/4]
```

---

## 1.9 — Characteristic Equation:  det(A − λI) = 0

**What it does:**
Finds the **eigenvalues** `λ` of a matrix `A`. Eigenvalues are the scalar amounts by which eigenvectors are stretched — the "natural frequencies" of a transformation. Core of PCA, mechanical vibration analysis, moments of inertia, and robot stability.

Imagine the **rubber sheet** again. Most arrows on the sheet get rotated *and* stretched by `A`. But a few special arrows (eigenvectors) only get **stretched** — they keep pointing in the same direction. The stretching factor for each such arrow is an **eigenvalue**. To find them, we ask: "for what `λ` does `A` behave as a pure scaling by `λ`?" That happens exactly when `A − λI` crushes its eigenvector to zero, i.e. when `det(A − λI) = 0`.

**Formula:**
```
det(A − λ·I) = 0
```

For a 2×2 matrix `A = [[a, b], [c, d]]`:
```
A − λI = [a − λ    b  ]       det(A − λI) = (a − λ)(d − λ) − b·c
         [  c    d − λ]
```

For a 3×3 matrix `A = [[a, b, c], [d, e, f], [g, h, i]]`:
```
A − λI = [a − λ    b      c  ]
         [  d    e − λ    f  ]
         [  g      h    i − λ]
```

The equation `det(A − λI) = 0` yields the characteristic polynomial in `λ`. Its roots are the eigenvalues.

**Simple example (2×2):**
```
A = [4  1]
    [2  3]

A − λI = [4−λ   1 ]
         [ 2   3−λ]

det(A − λI) = (4−λ)(3−λ) − 1·2
            = λ² − 7λ + 12 − 2
            = λ² − 7λ + 10
            = (λ−2)(λ−5) = 0

⇒ λ₁ = 2,  λ₂ = 5
```

**Complex example (3×3 — inertia tensor of a robot link):**
```
I = [5  0  0]
    [0  3  1]
    [0  1  3]

I − λ·Id = [5−λ   0    0 ]
           [ 0   3−λ   1 ]
           [ 0    1   3−λ]

det = (5−λ) · [(3−λ)² − 1]
    = (5−λ) · (λ² − 6λ + 8)
    = (5−λ)(λ−2)(λ−4) = 0

⇒ λ₁ = 5,  λ₂ = 2,  λ₃ = 4
```
→ Principal moments of inertia are 2, 4, and 5 — the rotation axes for which torque aligns with angular acceleration.
