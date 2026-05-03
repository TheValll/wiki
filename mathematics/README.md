# Mathematics Wiki

Formulas used across robotics, graphics, ML, and AI — with concrete analogies, simple examples, worked complex examples, and (where written) physical/intuition pages merged in the same file.

Each domain folder is a chapter; each `X.Y-...md` file inside is **one concept**. Course content (formula, examples) and intuition (physical image, step-by-step decomposition) live together.

> **At a glance:** [`RECAP.md`](./RECAP.md) — single-glance table of every math concept, grouped by chapter.
>
> **Notation atlas:** [`notation.md`](./notation.md) — bilingual FR/EN reference for Greek letters, set notation, quantifiers, operators, linear algebra, calculus, probability symbols. Lookup, not drill.

---

## The MML lens — why this domain exists

This wiki's math content is largely structured around **Mathematics for Machine Learning** (Deisenroth, Faisal, Ong). The book's Chapter 1 sets up the bridge between math foundations and ML problems — captured here once so chapter pages don't have to repeat it.

### The three components of machine learning

Machine learning is about designing algorithms that **automatically extract valuable patterns from data**. Three concepts sit at the core:

| Component | One sentence | Wiki angle |
|---|---|---|
| **Data** | The fuel — patterns to extract, ideally without domain expertise | Represented as vectors |
| **Model** | A simplified version of the unknown data-generating process | Probabilistic or optimization view |
| **Learning** | Optimize model parameters so the model generalizes to *unseen* data | Numerical optimization |

A **predictor** is the trained model used to make predictions; **training** is the act of fitting it (adapting parameters so it performs well on future unseen data). The phrase "ML algorithm" is overloaded — it can mean either the predictor or the training procedure. Context disambiguates.

> Performing well on data already seen (training data) only proves we found a good way to memorize. Generalization to *unseen* data is the actual goal.

### Three ways to read a vector

Same object, three lenses — each useful in different chapters:

| View | A vector is... | Useful when |
|---|---|---|
| **Computer science** | An array of numbers | Implementing algorithms (NumPy, arrays in memory) |
| **Physics** | An arrow with direction + magnitude | Analytic geometry, building intuition |
| **Mathematical** | An object that supports addition + scaling | Abstract reasoning, vector spaces |

> Watch out: array operations don't automatically implement vector operations. Element-wise multiplication is not the dot product.

### Two strategies to read the math

| Strategy | How it goes | Pro | Con |
|---|---|---|---|
| **Bottom-up** | Foundations → applications | Each step rests on the prior one | Foundations feel unmotivated and get forgotten |
| **Top-down** | Applications → foundations | Clear "why I need this" pull | Foundations stay shaky; vocab without grip |

MML splits Part I (foundations) and Part II (problems) so it can be read either way. This wiki mirrors that split: each `mathematics/` chapter is a building block; each `ml/` page reaches back when it needs one. Most readers (Valentin included) end up combining both — bottom-up where the gap is too wide, top-down where motivation is needed.

### The four pillars — what math supports what ML problem

The MML book's Figure 1.1 shows ML as a Greek temple: four ML problems (the pillars) standing on six math foundations (the base rows).

```
   ┌────────────────────────────────────────────────────────────┐
   │                     Machine Learning                       │   ← roof
   └────────────────────────────────────────────────────────────┘
       │              │              │              │
    ┌──┴──┐        ┌──┴──┐        ┌──┴──┐        ┌──┴──┐
    │ Reg-│        │ Dim │        │ Den-│        │Class│
    │ res-│        │ Red.│        │ sity│        │ ifi-│         ← 4 pillars
    │ sion│        │     │        │ Est.│        │ cat.│
    │ MML9│        │MML10│        │MML11│        │MML12│
    └─────┘        └─────┘        └─────┘        └─────┘
   ────────────────────────────────────────────────────────────
   │  Vector Calculus    │   Probability    │  Optimization   │   ← row 2
   │      (MML 5)        │     (MML 6)      │     (MML 7)     │
   ────────────────────────────────────────────────────────────
   │  Linear Algebra     │ Analytic Geom.   │ Matrix Decomp.  │   ← row 1
   │      (MML 2)        │     (MML 3)      │     (MML 4)     │
   ────────────────────────────────────────────────────────────
```

The four ML problems in plain English:

| Pillar | Goal | Example |
|---|---|---|
| **Regression** | Map inputs `x ∈ ℝᴰ` to real-valued outputs `y ∈ ℝ` | Predict a house price from features |
| **Dimensionality Reduction** | Find a compact lower-dim representation of high-dim data | PCA on a 1000-feature dataset down to 2D |
| **Density Estimation** | Find a probability distribution that describes a dataset | Fit a Gaussian mixture to clustered points |
| **Classification** | Map inputs `x` to *integer* labels `y` (special care vs regression) | SVM separating spam from ham |

### Loose mapping MML chapters → this wiki

The wiki doesn't follow MML's chapter numbering 1-to-1 (it merges some, defers others, and keeps room for non-MML sources). Cross-reference table:

| MML chapter | Wiki location |
|---|---|
| 2 — Linear Algebra | [`01-linear-algebra/`](./01-linear-algebra/README.md) |
| 3 — Analytic Geometry | partially inside [`01-linear-algebra/`](./01-linear-algebra/README.md) (norms, dot product) |
| 4 — Matrix Decompositions | not yet covered |
| 5 — Vector Calculus | [`03-derivatives/`](./03-derivatives/README.md) |
| 6 — Probability & Distributions | [`06-probability/`](./06-probability/README.md) + [`07-statistics/`](./07-statistics/README.md) |
| 7 — Continuous Optimization | [`04-optimization/`](./04-optimization/README.md) |
| 8–12 — Models & ML problems | *(planned — `ml/` domain to be rebuilt)* |

> The exercises and tutorials of MML can be found at <https://mml-book.com>.

---

## Chapters

### [01 — Linear Algebra (Vectors & Matrices)](./01-linear-algebra/README.md)
Vector norm, dot product, matrix × vector / matrix, inverse, determinant, characteristic equation.

### [02 — Algebra (Discriminant, Linear Systems, Identities)](./02-algebra-solving/README.md)
Discriminant of a quadratic, 2-variable systems, Gaussian elimination, remarkable identities.

### [03 — Derivatives](./03-derivatives/README.md)
Notation, common derivatives, scalar / sum / product / chain rules, non-differentiable points, partial derivatives, gradient, Hessian.

### [04 — Optimization](./04-optimization/README.md)
Gradient descent, Newton's method (1D and multivariate), classifying stationary points.

### [05 — Machine Learning Basics](./05-ml-basics/README.md)
Min-max normalization, standardization, linear regression, RMSE, sigmoid, binary cross-entropy, classification metrics.

### [06 — Probability](./06-probability/README.md)
Basic probability, union, independence, Bayes, Bernoulli, binomial, PMF/PDF/CDF, Gaussian.

### [07 — Statistics](./07-statistics/README.md)
Mean, variance, skewness, kurtosis, quantiles, covariance, correlation, multivariate Gaussian, n−1 correction, CLT, MLE, hypothesis testing.

---

## Conventions

- **One concept = one file.** File name = `<chapter>-<concept-slug>/<X.Y-slug>.md`.
- **Cross-references** use relative paths: `[ROS2 — IK](../../ros2/moveit/20-inverse-kinematics.md)`.
- **Intuition** sections live inside each concept file (no separate companion folder).

For the broader wiki structure and agent rules, see [`../CLAUDE.md`](../CLAUDE.md).
