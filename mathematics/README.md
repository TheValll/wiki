# Mathematics Wiki

Formulas used across robotics, graphics, ML, and AI — with concrete analogies, simple examples, worked complex examples, and (where written) physical/intuition pages merged in the same file.

Each domain folder is a chapter; each `X.Y-...md` file inside is **one concept**. Course content (formula, examples) and intuition (physical image, step-by-step decomposition) live together.

> **At a glance:** [`RECAP.md`](./RECAP.md) — single-glance table of every math concept, grouped by chapter.
>
> **Notation atlas:** [`notation.md`](./notation.md) — bilingual FR/EN reference for Greek letters, set notation, quantifiers, operators, linear algebra, calculus, probability symbols. Lookup, not drill.

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
