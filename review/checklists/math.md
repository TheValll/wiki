# Math — Curriculum Checklist

Ordered list of math concepts for review, grouped by module. Total: **41 concepts**.

Targeted applications: **Data Science, AI, spatial robotics, computer vision, navigation**.

Linked wiki pages (when they exist) are in [`mathematics/`](../../mathematics/). The agent reads them for theory only — exercises are always generated fresh.

---

## Module 0 — Foundations (middle school / high school level)

| # | Concept | Wiki ref |
|---|---------|----------|
| 1 | Powers and square roots | — |
| 2 | Fractions and basic algebraic manipulation | — |
| 3 | Isolating a variable (ax + b = c) | [`02-algebra-solving.md`](../../mathematics/02-algebra-solving.md) |
| 4 | Remarkable identities and factoring | [`02-algebra-solving.md`](../../mathematics/02-algebra-solving.md) §2.4 |
| 5 | Laws of logarithms and exponentials | — |

## Module 1 — Linear Algebra (the spatial world)

| # | Concept | Wiki ref |
|---|---------|----------|
| 6 | Vectors: norm and dot product | [`01-linear-algebra.md`](../../mathematics/01-linear-algebra.md) §1.1-1.2 |
| 7 | Matrices: vector × matrix and matrix × matrix | [`01-linear-algebra.md`](../../mathematics/01-linear-algebra.md) §1.3-1.4 |
| 8 | Matrices: determinant and inverse | [`01-linear-algebra.md`](../../mathematics/01-linear-algebra.md) §1.5-1.6 |
| 9 | Systems of equations: 2 and 3 variables, discriminant | [`02-algebra-solving.md`](../../mathematics/02-algebra-solving.md) §2.1-2.2 |
| 10 | Advanced resolution: Gaussian elimination | [`02-algebra-solving.md`](../../mathematics/02-algebra-solving.md) §2.3 |

## Module 2 — Analysis (dynamics and slopes)

| # | Concept | Wiki ref |
|---|---------|----------|
| 11 | Basic derivatives: linear, powers, inverse | [`03-derivatives.md`](../../mathematics/03-derivatives.md) §3.2 |
| 12 | Chain rule (function composition) | [`03-derivatives.md`](../../mathematics/03-derivatives.md) §3.6 |
| 13 | Complex derivatives: trig, exponential, logarithm | [`03-derivatives.md`](../../mathematics/03-derivatives.md) §3.2 |
| 14 | Calculation rules: sum, product, scalar multiplication | [`03-derivatives.md`](../../mathematics/03-derivatives.md) §3.3-3.5 |
| 15 | Derivatives of quadratics and curves | — |
| 16 | Non-differentiable functions: corners and jumps (awareness only — not drilled) | [`03-derivatives.md`](../../mathematics/03-derivatives.md) §3.7 |
| 17 | 3D and acceleration: partial derivatives and second derivative | [`03-derivatives.md`](../../mathematics/03-derivatives.md) §3.8-3.9 |

## Module 3 — Probability & Statistics (uncertainty)

| # | Concept | Wiki ref |
|---|---------|----------|
| 18 | Forms of randomness: PMF, PDF, CDF | [`06-probability.md`](../../mathematics/06-probability.md) §6.7 |
| 19 | Center and spread: expected value, variance, standard deviation | [`07-statistics.md`](../../mathematics/07-statistics.md) §7.1-7.2 |
| 20 | Shape of data: skewness, kurtosis, Z-score, IQR | [`07-statistics.md`](../../mathematics/07-statistics.md) §7.3-7.5 + [`05-ml-basics.md`](../../mathematics/05-ml-basics.md) §5.2 |
| 21 | Relationships: covariance and correlation | [`07-statistics.md`](../../mathematics/07-statistics.md) §7.6-7.7 |
| 22 | The king of distributions: Gaussian | [`06-probability.md`](../../mathematics/06-probability.md) §6.8 + [`07-statistics.md`](../../mathematics/07-statistics.md) §7.8 |
| 23 | Moving to the real world: population vs sample (n−1) | [`07-statistics.md`](../../mathematics/07-statistics.md) §7.9 |
| 24 | Key theorems: CLT and MLE | [`07-statistics.md`](../../mathematics/07-statistics.md) §7.10-7.11 |
| 25 | The courtroom: hypothesis tests (H0, H1), Type I and II errors | [`07-statistics.md`](../../mathematics/07-statistics.md) §7.12 |

## Module 4 — AI & Optimization (the machine learns)

| # | Concept | Wiki ref |
|---|---------|----------|
| 26 | Preparation: min-max scaling and standardization | [`05-ml-basics.md`](../../mathematics/05-ml-basics.md) §5.1-5.2 |
| 27 | Baseline models: linear regression and classification | [`05-ml-basics.md`](../../mathematics/05-ml-basics.md) §5.3, 5.5-5.6 |
| 28 | Measuring error: loss function and RMSE | [`05-ml-basics.md`](../../mathematics/05-ml-basics.md) §5.4, 5.6 |
| 29 | Finding the minimum (Lv 1): gradient descent | [`04-optimization.md`](../../mathematics/04-optimization.md) §4.1 |
| 30 | Finding the minimum (Lv 2): Hessian matrix and Newton's method | [`04-optimization.md`](../../mathematics/04-optimization.md) §4.2-4.4 |

## Module 5 — Spatial Geometry & Kinematics (3D and motion)

| # | Concept | Wiki ref |
|---|---------|----------|
| 31 | 3D representation: Euler angles and the "Gimbal Lock" problem | — |
| 32 | The robust alternative: quaternions | — |
| 33 | Spatial transformations: homogeneous matrices (SO(3) and SE(3)) | partial: [`ros2/20-inverse-kinematics.md`](../../ros2/20-inverse-kinematics.md) |
| 34 | Basic kinematics: rover motion (differential and Ackermann models) | [`ros2/11-controllers-diffdrive.md`](../../ros2/11-controllers-diffdrive.md) |

## Module 6 — State Estimation & Probabilistic Robotics (navigating under uncertainty)

| # | Concept | Wiki ref |
|---|---------|----------|
| 35 | Reasoning under uncertainty: the Bayes filter | — |
| 36 | Robotics' go-to tool: the Kalman filter (basics) | — |
| 37 | Nonlinear systems: Extended Kalman Filter (EKF) | — |
| 38 | Mapping and localization: particle filters (SLAM basics) | — |

## Module 7 — Dynamics & Control (acting physically)

| # | Concept | Wiki ref |
|---|---------|----------|
| 39 | Physical modeling: Ordinary Differential Equations (ODEs) | — |
| 40 | State-space representation: the mathematical formalism | — |
| 41 | Classical control: PID controller and error reduction over time | — |

---

## Notes

- "Wiki ref" columns show which wiki pages already cover the theory. Blank entries mean no wiki page yet — the agent can still quiz you, but theory comes from general knowledge (or you should add a wiki page as you study it).
- The order is deliberate — don't skip ahead. Each module builds on the previous.
- Cross-domain use cases (e.g., Jacobians in ROS2 IK, splines in trajectory generation) are documented in the respective wiki pages' "Applied in" sections.
