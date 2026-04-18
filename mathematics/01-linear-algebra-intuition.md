# Linear Algebra — Under the Hood

A companion to [`01-linear-algebra.md`](./01-linear-algebra.md). That page is the reference — formulas, worked examples, cheat sheet. This page is the **pure intuition**: why each operation works the way it does, told in plain language, with no formulas and no exercises. Readable on a train.

Each section matches a section of the reference page.

---

## 1.1 — The Euclidean Norm: Pythagoras, Stacked

A vector is an arrow — it leaves one point and arrives at another. The norm is simply the length of that arrow: a single number describing how "long" it is, regardless of which direction it points.

**Where the idea comes from.** In two dimensions, draw an arrow from the origin to the point with horizontal offset 3 and vertical offset 4. You can see that arrow as the hypotenuse of a right triangle whose legs are 3 and 4. Pythagoras tells you the hypotenuse squared equals the sum of the two legs squared — so the length squared is 9 plus 16, which is 25, and the length itself is 5. That is the whole story in 2D.

**Why it still works in 3D, 4D, or 100D.** This is the elegant part. Take an arrow in three dimensions. First, look at its "shadow" on the floor — the projection onto the horizontal plane. That shadow is itself a 2D vector, so by Pythagoras you already know its length. Now your real arrow forms a new right triangle: the shadow lies flat, and the vertical height stands up perpendicular to it. Apply Pythagoras a second time, and you get the full 3D length. The squared contributions from each dimension simply add together under a single square root.

You can keep going. Four dimensions? The 3D sub-arrow becomes the "shadow" of the 4D arrow, and Pythagoras cascades one more time. Every new dimension contributes its own squared term, and the total stays a clean sum of squares.

**The deep intuition.** Each coordinate axis is perpendicular to every other axis — x is perpendicular to y, y is perpendicular to z, and so on. As long as the axes are perpendicular, you can cascade Pythagoras forever. That is the one quiet assumption behind the formula, and it is why the norm works in any number of dimensions, even in spaces you cannot visualize. Perpendicularity is what lets the squared contributions add up without interfering with each other.

---

## 1.2 — The Dot Product: A Score of Alignment

The dot product takes two arrows and returns a single number that tells you **how much they cooperate**. Arrows pointing the same way give a large positive number. Perpendicular arrows give zero. Arrows pointing against each other give a negative number. That is the whole semantic.

**The physical image.** Two people push a cart. If they push in the same direction, their forces fully add up. If one pushes forward and the other sideways, only the forward push moves the cart — the sideways one does nothing for *this* motion. If they push against each other, they cancel. The dot product measures exactly that cooperation.

**View 1 — The projection (shadow) picture.**

Take vector `a`, and look at it from `b`'s direction. How long is the **shadow** of `a` on the line carried by `b`? When the two arrows point the same way, the shadow of `a` is `a` itself. When `a` is perpendicular to `b`, the shadow collapses to zero. When `a` points against `b`, the shadow falls *behind* the origin — a "negative" length. The dot product is that shadow length, scaled by the length of `b`.

The cosine of the angle between them is simply the factor that encodes "how complete the shadow is": 1 when perfectly aligned, 0 when perpendicular, −1 when opposed.

**The decomposition trick.** A cleaner way to think about it: take `a` and break it into two pieces — one **parallel** to `b`, one **perpendicular** to `b`. Throw away the perpendicular piece (it does not help `b` at all). The parallel piece is what survives — its signed length, times the length of `b`, is the dot product.

The three cases below walk through this decomposition step by step.

---

### Case 1 — Acute angle (arrows cooperate, dot > 0)

**Step 1.** Start with `a` and `b`.
```
        a
       ╱
      ╱
     ╱
    ╱
   •──────────→ b
```

**Step 2.** Drop a perpendicular from the tip of `a` straight down onto `b`'s line.
```
        a
       ╱│
      ╱ │   ← perpendicular from a's tip
     ╱  │
    ╱   │
   •────┴─────→ b
```

**Step 3.** Split `a` into a piece **along b** (horizontal) and a piece **perpendicular to b** (vertical).
```
        a
       ╱│
      ╱ │  ← perpendicular piece (DISCARDED)
     ╱  │
    ╱   │
   •════╧═════→ b
   ←════→
   parallel piece (SURVIVES, along b)
```

**Step 4.** Result.
```
   parallel piece = positive length
   → dot(a, b) > 0
   "a contributes to pushing along b"
```

---

### Case 2 — Perpendicular (independent, dot = 0)

**Step 1.** `a` and `b` meet at 90°.
```
        ↑ a
        │
        │
        │
        •──────→ b
```

**Step 2.** Drop the perpendicular from `a`'s tip onto `b`'s line — it lands exactly on the origin.
```
        ↑ a
        │  ← a is ALREADY perpendicular to b
        │
        │
        •──────→ b
```

**Step 3.** The parallel piece has length **zero**. All of `a` sits in the perpendicular piece.
```
        ↑ a      ← a is entirely perpendicular → DISCARDED
        │
        │
        │
        •──────→ b
        ↑
    parallel piece = 0
```

**Step 4.** Result.
```
   parallel piece = 0
   → dot(a, b) = 0
   "a pushes neither with nor against b"
```

---

### Case 3 — Obtuse angle (opposed, dot < 0)

**Step 1.** `a` and `b` point in roughly opposite directions (angle > 90°).
```
    a
     ╲
      ╲
       ╲
        ╲
         •──────→ b
```

**Step 2.** Drop the perpendicular from `a`'s tip down to `b`'s line. It lands **behind** the origin (to the left of it).
```
    a
     ╲│
      ╲│
       ╲
        ╲
   ←─────•──────→ b's line
    ↑    ↑
    └─┬──┘
    a's shadow lands HERE,
    behind the origin
```

**Step 3.** Split `a`: the parallel piece points **backward** relative to `b` — its length counts as **negative**. The perpendicular piece is discarded as always.
```
    a
     ╲│   ← perpendicular piece (DISCARDED)
      ╲│
       ╲
        ╲
   ←═════╧══════→ b's line
   ←═════→
   parallel piece (SURVIVES, but backward)
   length counted as NEGATIVE
```

**Step 4.** Result.
```
   parallel piece = negative length
   → dot(a, b) < 0
   "a pushes AGAINST b"
```

---

### The three cases in one table

| Case | Parallel piece (what survives) | Sign of dot |
|---|---|---|
| Acute (< 90°) | positive length, along b | **+** |
| Perpendicular (= 90°) | zero length | **0** |
| Obtuse (> 90°) | negative length (opposite to b) | **−** |

The dot product is this signed length, scaled by the length of `b`. The perpendicular part of `a` is ignored because it contributes nothing in `b`'s direction.

---

**View 2 — Why the "multiply and add coordinates" formula also works.**

There is a second way to compute the dot product: take the coordinates two by two, multiply them, and sum. Why does this match the projection picture?

The reason is the same one that made the norm formula work: **the coordinate axes are perpendicular to each other**. When you "cross" the x-component of `a` with the y-component of `b`, the contribution is zero — the x-axis and y-axis do not cooperate, they are perpendicular by construction. Only the "matched" products (x with x, y with y, z with z) survive, because those components lie along the same axis and are perfectly aligned with one another.

So the component formula is silently doing the same decomposition as the projection picture, one axis at a time.

**The bridge between the two views.** Coordinates encode all the geometric information of a vector — its direction and its length, baked into the numbers. So any operation that depends only on directions and lengths (like the dot product) can be computed either geometrically (angle, cosine) or algebraically (coordinate by coordinate). Both routes arrive at the same number. It is not a coincidence; it is a consequence of working in a frame whose axes are perpendicular unit vectors.

---

**What to remember.**

The dot product is a **cooperation score** between two arrows. It strips away the perpendicular part of `a` (which helps nothing in `b`'s direction) and keeps the signed length of what remains along `b`. That is why it shows up everywhere: in physics (work = force · displacement — only the force along the motion counts), in angle computation, in orthogonality tests (dot = 0 means "independent directions"), and in machine learning (dot product as a similarity score between two signals).
