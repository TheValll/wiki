# 06 — Probability

Each page covers **one concept**.

| § | Page | What it does |
|---|------|--------------|
| 6.1 | [Basic Probability](./6.1-basic-probability.md) | Sample space, events, P as a fraction over outcomes. |
| 6.2 | [Union of Events](./6.2-union-of-events.md) | P(A ∪ B) — disjoint case and the inclusion-exclusion correction. |
| 6.3 | [Independence and Dependence](./6.3-independence.md) | When P(A ∩ B) factors, and when conditional probability is needed. |
| 6.4 | [Bayes' Theorem](./6.4-bayes-theorem.md) | Flip a conditional: from P(B|A) to P(A|B). |
| 6.5 | [Bernoulli Distribution](./6.5-bernoulli.md) | One coin flip with bias `p`. |
| 6.6 | [Binomial Distribution](./6.6-binomial.md) | Count of successes in `n` independent Bernoulli trials. |
| 6.7 | [PMF, PDF, CDF](./6.7-pmf-pdf-cdf.md) | The three ways to describe a distribution. |
| 6.8 | [Gaussian Distribution](./6.8-gaussian.md) | Bell curve — the universal limit of summed noise. |

## Applied in

| Concept | Used in |
|---------|---------|
| **Bayes' theorem** | Bayes filter, Kalman filter, particle filter, SLAM, classification |
| **Gaussian noise** | [ROS2 — DiffDrive Controller](../../ros2/ros2-control/11-controllers-diffdrive.md) (odometry covariance), sensor fusion |
| **Independence** | [`02-algebra-solving.md`](./02-algebra-solving.md) §2.4 (variance of sums) |
| **Binomial** | Reliability engineering, A/B testing, hypothesis testing |
| **PDF / CDF** | Monte Carlo simulation, rejection sampling, probabilistic planning |
