# Mathematics Wiki

Formulas used across robotics, graphics, ML, and AI — with concrete analogies, simple examples, and worked complex examples.

Each entry follows the same format:
1. **What it does** — the purpose, plus a plain-language analogy.
2. **Formula** — the mathematical expression (and its composed form with `u, v` when relevant).
3. **Simple example** — minimal numeric case.
4. **Complex example** — realistic scenario, often drawn from robotics / ML / physics.

---

## Table of Contents

### [01 — Linear Algebra (Vectors & Matrices)](./01-linear-algebra.md)
- 1.1  Vector Norm (Euclidean)
- 1.2  Dot Product
- 1.3  Matrix × Vector Multiplication
- 1.4  Matrix × Matrix Multiplication
- 1.5  Matrix Inverse
- 1.6  Determinant (2×2 and 3×3)
- 1.7  det(AB) = det(A)·det(B)
- 1.8  det(A⁻¹) = 1 / det(A)
- 1.9  Characteristic Equation  det(A − λI) = 0

### [02 — Algebra (Discriminant, Linear Systems, Identities)](./02-algebra-solving.md)
- 2.1  Discriminant of a Quadratic
- 2.2  Solving a 2-Variable Linear System (substitution & Cramer)
- 2.3  Solving a 3-Variable System (Gaussian Elimination + Matrices)
- 2.4  Remarkable Identities — `(a±b)², (a+b)(a−b), (a+b+c)²`

### [03 — Derivatives](./03-derivatives.md)
- 3.1  Lagrange vs Leibniz Notation
- 3.2  Derivatives of Common Functions (with `u, v` composed forms)
- 3.3  Scalar Multiplication Rule
- 3.4  Sum Rule
- 3.5  Product Rule
- 3.6  Chain Rule
- 3.7  Non-Differentiable Functions (corners, cusps, jumps, vertical tangents)
- 3.8  Partial Derivatives
- 3.9  Gradient and Hessian

### [04 — Optimization](./04-optimization.md)
- 4.1  Gradient Descent
- 4.2  Newton's Method (1D)
- 4.3  Newton's Method (Multivariate)
- 4.4  Classifying Stationary Points (second-order test, eigenvalues of the Hessian)

### [05 — Machine Learning Basics](./05-ml-basics.md)
- 5.1  Min-Max Normalization
- 5.2  Standardization (Z-score)
- 5.3  Linear Regression
- 5.4  Root Mean Squared Error (RMSE)
- 5.5  The Sigmoid Function
- 5.6  Binary Cross-Entropy Loss
- 5.7  Classification Metrics (confusion matrix, accuracy, precision, recall, specificity, F1)

### [06 — Probability](./06-probability.md)
- 6.1  Basic Probability
- 6.2  Union of Events (disjoint and general)
- 6.3  Independence and Dependence
- 6.4  Bayes' Theorem
- 6.5  Bernoulli Distribution
- 6.6  Binomial Distribution
- 6.7  PMF, PDF, CDF
- 6.8  The Gaussian (Normal) Distribution

### [07 — Statistics](./07-statistics.md)
- 7.1  Expected Value (Mean)
- 7.2  Variance and Standard Deviation
- 7.3  Skewness (Asymmetry)
- 7.4  Kurtosis (Tail Weight)
- 7.5  Quantiles, Percentiles, and Boxplots
- 7.6  Covariance
- 7.7  Correlation
- 7.8  Multivariate Gaussian (2D independent, 2D dependent, n-D)
- 7.9  Population vs Sample (Bessel's `n − 1`)
- 7.10  Central Limit Theorem (CLT)
- 7.11  Maximum Likelihood Estimation (MLE)
- 7.12  Hypothesis Testing
