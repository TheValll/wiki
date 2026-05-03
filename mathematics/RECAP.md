# Mathematics — RECAP

Single-glance table of every concept across the math domain. One row per concept-file. Click through to the full page.

---

## 01 — Linear Algebra

| Concept | What / for what | Intuition |
|---|---|---|
| [1.1 Norm](./01-linear-algebra/1.1-norm.md) | Length of a vector — single scalar from a directional quantity | Pythagoras cascaded across perpendicular axes |
| [1.2 Dot Product](./01-linear-algebra/1.2-dot-product.md) | How much two arrows cooperate; angle, projection, orthogonality | Signed shadow of one arrow onto another |
| [1.3 Matrix × Vector](./01-linear-algebra/1.3-matrix-vector.md) | Apply a linear transform to a vector | Recipe for mixing the matrix's columns |
| [1.4 Matrix × Matrix](./01-linear-algebra/1.4-matrix-matrix.md) | Compose two transforms into one | Two machines in series, pre-collapsed into one |
| [1.5 Inverse](./01-linear-algebra/1.5-inverse.md) | Undo a transform; solve `Ax = b` | The un-machine — exists iff no dimension was destroyed |
| [1.6 Determinant](./01-linear-algebra/1.6-determinant.md) | Volume scaling, orientation flip, invertibility check | Signed area/volume factor of the machine |
| [1.7 det(AB)](./01-linear-algebra/1.7-det-product.md) | det distributes over product | Stretches in series compound by multiplication |
| [1.8 det(A⁻¹)](./01-linear-algebra/1.8-det-inverse.md) | det of inverse = reciprocal of det | To undo a stretch, divide by it |
| [1.9 Characteristic Eq.](./01-linear-algebra/1.9-characteristic-equation.md) | Find eigenvalues / direction-stable axes | Scale factors λ that make `A − λI` collapse a dimension |

## 02 — Algebra & Solving

| Concept | What / for what | Intuition |
|---|---|---|
| [2.1 Discriminant](./02-algebra-solving/2.1-discriminant.md) | Number of real roots of `ax²+bx+c` before solving | Does the parabola cross / kiss / miss the ground? |
| [2.2 Linear Systems (2×2 + general)](./02-algebra-solving/2.2-linear-system-2x2.md) | Solve 2 eqs in `(x, y)` (substitution / Cramer); generalizes to `Ax = b` for `m × n` systems | Two roads cross on a map; same idea scales to planes & hyperplanes |
| [2.3 Gaussian Elimination](./02-algebra-solving/2.3-gaussian-elimination.md) | Solve `n×n` linear systems via row-reduction + back-substitution | Reshape into a staircase, climb back up |
| [2.4 Remarkable Identities](./02-algebra-solving/2.4-remarkable-identities.md) | `(a+b)²`, `a²−b²`, `(a+b+c)²` — pre-cut algebraic blocks | Squared sums = pure squares + cross terms counted twice |

## 03 — Derivatives

| Concept | What / for what | Intuition |
|---|---|---|
| [3.1 Lagrange vs Leibniz](./03-derivatives/3.1-lagrange-vs-leibniz.md) | Two notations for the same derivative concept | Speedometer (`f'`) vs distance-over-time (`df/dx`) |
| [3.2 Common Derivatives](./03-derivatives/3.2-common-derivatives.md) | Reference table for `xⁿ, eˣ, ln, sin, cos, …` | Each entry has a physical reason for its shape |
| [3.3 Scalar Rule](./03-derivatives/3.3-scalar-rule.md) | `(k·u)' = k·u'` — pull constants out | Constant = playback-speed knob |
| [3.4 Sum Rule](./03-derivatives/3.4-sum-rule.md) | `(u+v)' = u'+v'` — differentiate term by term | Parallel treadmills, no cross-talk |
| [3.5 Product Rule](./03-derivatives/3.5-product-rule.md) | `(u·v)' = u'v + uv'` for two simultaneous factors | Growing rectangle: two strips + negligible corner |
| [3.6 Chain Rule](./03-derivatives/3.6-chain-rule.md) | `(f∘g)' = f'(g)·g'` for composed functions | Meshed gears — chained rates multiply |
| [3.7 Non-Differentiable](./03-derivatives/3.7-non-differentiable.md) | When the derivative does not exist (corners, jumps) | Car heading breaks — no single tangent direction |
| [3.8 Partial Derivatives](./03-derivatives/3.8-partial-derivatives.md) | One-axis slope on multi-variable function | East step on a topographic map; freeze the others |
| [3.9 Gradient & Hessian](./03-derivatives/3.9-gradient-hessian.md) | Vector of partials (uphill direction) + matrix of 2nd partials (curvature) | Compass + curvature sensor on the landscape |

## 04 — Optimization

| Concept | What / for what | Intuition |
|---|---|---|
| [4.1 Gradient Descent](./04-optimization/4.1-gradient-descent.md) | Iterate `θ ← θ − α∇L` to minimize loss | Foggy mountain: feel downhill, step, repeat |
| [4.2 Newton (1D)](./04-optimization/4.2-newton-1d.md) | `x ← x − f'/f''` — quadratic convergence near the optimum | Use slope AND curvature to jump to the minimum |
| [4.3 Newton (multivariate)](./04-optimization/4.3-newton-multivariate.md) | Same with Hessian: `θ ← θ − H⁻¹∇f` | Curvature-aware step in n-D |
| [4.4 Stationary Points](./04-optimization/4.4-stationary-points.md) | Classify min / max / saddle from Hessian eigenvalues | Bowl, dome, or pass — read the eigenvalues |

## 05 — Machine Learning Basics

| Concept | What / for what | Intuition |
|---|---|---|
| [5.1 Min-Max Normalization](./05-ml-basics/5.1-min-max-normalization.md) | Rescale features into `[0, 1]` | Stretch the data to fit a unit ruler |
| [5.2 Standardization](./05-ml-basics/5.2-standardization.md) | Center on mean, scale by σ → mean 0, variance 1 | Re-express each feature in "standard deviation units" |
| [5.3 Linear Regression](./05-ml-basics/5.3-linear-regression.md) | Fit `ŷ = w·x + b` minimizing squared error | The cleanest line through the cloud |
| [5.4 RMSE](./05-ml-basics/5.4-rmse.md) | Average error in original units | Typical distance from prediction to truth |
| [5.5 Sigmoid](./05-ml-basics/5.5-sigmoid.md) | Squash any real number into `(0, 1)` | Soft on/off switch — a probability dial |
| [5.6 Binary Cross-Entropy](./05-ml-basics/5.6-binary-cross-entropy.md) | Classification loss based on predicted probability | Penalize confident wrong calls disproportionately |
| [5.7 Classification Metrics](./05-ml-basics/5.7-classification-metrics.md) | Accuracy / precision / recall / F1 from confusion matrix | Pick the metric that matches the cost of mistakes |

## 06 — Probability

| Concept | What / for what | Intuition |
|---|---|---|
| [6.1 Basic Probability](./06-probability/6.1-basic-probability.md) | Sample space, events, P as a fraction | Favorable outcomes over total outcomes |
| [6.2 Union of Events](./06-probability/6.2-union-of-events.md) | `P(A ∪ B)` disjoint vs general (inclusion-exclusion) | Don't count the overlap twice |
| [6.3 Independence](./06-probability/6.3-independence.md) | `P(A ∩ B) = P(A)·P(B)` iff independent; else conditional | Does knowing A change the bet on B? |
| [6.4 Bayes' Theorem](./06-probability/6.4-bayes-theorem.md) | Flip a conditional: `P(A|B)` from `P(B|A)` | Update prior beliefs in light of evidence |
| [6.5 Bernoulli](./06-probability/6.5-bernoulli.md) | One coin flip with bias `p` | The atomic random event |
| [6.6 Binomial](./06-probability/6.6-binomial.md) | Count of successes in `n` Bernoulli trials | How many heads in 10 flips? |
| [6.7 PMF / PDF / CDF](./06-probability/6.7-pmf-pdf-cdf.md) | Three ways to describe a distribution | Probability per outcome, per length, accumulated |
| [6.8 Gaussian](./06-probability/6.8-gaussian.md) | Bell curve `N(μ, σ²)` | The universal limit of summed independent noise |

## 07 — Statistics

| Concept | What / for what | Intuition |
|---|---|---|
| [7.1 Expected Value](./07-statistics/7.1-expected-value.md) | Probability-weighted average | The long-run mean under the distribution |
| [7.2 Variance / σ](./07-statistics/7.2-variance.md) | Average squared distance from the mean | How wide is the cloud? |
| [7.3 Skewness](./07-statistics/7.3-skewness.md) | Asymmetry of the distribution | Tail sticking out left vs right |
| [7.4 Kurtosis](./07-statistics/7.4-kurtosis.md) | Tail weight relative to a Gaussian | How extreme are the extreme values? |
| [7.5 Quantiles / Boxplots](./07-statistics/7.5-quantiles-boxplots.md) | Order-based summaries, robust to outliers | Cut the sorted data at fixed percentages |
| [7.6 Covariance](./07-statistics/7.6-covariance.md) | Joint variability — sign tells direction | Do the two variables move together? |
| [7.7 Correlation](./07-statistics/7.7-correlation.md) | Covariance normalized into `[-1, 1]` | Strength of linear association, unit-free |
| [7.8 Multivariate Gaussian](./07-statistics/7.8-multivariate-gaussian.md) | Gaussian in n-D — covariance matrix encodes shape | Tilted ellipses of equal density |
| [7.9 Population vs Sample (n−1)](./07-statistics/7.9-population-vs-sample.md) | Bessel's correction for unbiased sample variance | We "spent" one DoF estimating the mean |
| [7.10 CLT](./07-statistics/7.10-clt.md) | Sums/averages tend to Gaussian regardless of source | Why the bell curve appears everywhere |
| [7.11 MLE](./07-statistics/7.11-mle.md) | Pick parameters that maximize the data likelihood | Choose the world in which our data is most plausible |
| [7.12 Hypothesis Testing](./07-statistics/7.12-hypothesis-testing.md) | Reject `H₀` when data would be implausible under it | Trial by improbability |
