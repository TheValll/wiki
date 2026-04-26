# 07 — Statistics

Each page covers **one concept**.

| § | Page | What it does |
|---|------|--------------|
| 7.1 | [Expected Value (Mean)](./7.1-expected-value.md) | Probability-weighted average of a random variable. |
| 7.2 | [Variance and Standard Deviation](./7.2-variance.md) | Average squared distance from the mean — spread. |
| 7.3 | [Skewness](./7.3-skewness.md) | Asymmetry of a distribution around its mean. |
| 7.4 | [Kurtosis](./7.4-kurtosis.md) | Tail weight — how often extreme values occur. |
| 7.5 | [Quantiles, Percentiles, Boxplots](./7.5-quantiles-boxplots.md) | Order-based summaries that resist outliers. |
| 7.6 | [Covariance](./7.6-covariance.md) | Joint variability of two variables (raw, sign-only). |
| 7.7 | [Correlation](./7.7-correlation.md) | Normalized covariance in [-1, 1]. |
| 7.8 | [Multivariate Gaussian](./7.8-multivariate-gaussian.md) | The Gaussian generalized to vectors — covariance matrix. |
| 7.9 | [Population vs Sample (`n − 1`)](./7.9-population-vs-sample.md) | Why dividing by n−1 (Bessel's correction) makes the estimator unbiased. |
| 7.10 | [Central Limit Theorem](./7.10-clt.md) | Why averages of anything tend to be Gaussian. |
| 7.11 | [Maximum Likelihood Estimation](./7.11-mle.md) | Pick the parameters that make the observed data most plausible. |
| 7.12 | [Hypothesis Testing](./7.12-hypothesis-testing.md) | Reject the null only when the data would be very unlikely if it were true. |

## Applied in

| Concept | Used in |
|---------|---------|
| **Covariance / multivariate Gaussian** | Kalman filter, particle filter (probabilistic robotics) |
| **CLT** | [`05-ml-basics.md`](./05-ml-basics.md) (noise assumptions), Kalman filter, confidence intervals |
| **MLE** | [`05-ml-basics.md`](./05-ml-basics.md) (loss functions as MLE under Gaussian/Bernoulli noise) |
| **Hypothesis testing** | A/B testing, sensor anomaly detection, drug trials, model evaluation |
| **Quantiles / boxplot** | SLAM residual diagnostics, outlier detection |
| **Skewness / kurtosis** | Robust filtering, outlier rejection, non-Gaussian noise modeling |
