# 02 — Supervised Classical

Each page covers **one model**. The classical supervised toolkit — fit a function from labeled data, no neural networks yet. Reach for these first: they're fast, interpretable, and the strong baseline before any neural detour.

## Sources

- **Deisenroth ch 9** (linear regression — Bayesian view included), **ch 12** (SVM)
- **Géron — Hands-On ML** ch 4 (linear / logistic) for hands-on framing
- **Bishop — PRML** ch 4 (logistic), ch 6 (kernels) when extra rigor is wanted

## Pages

| § | Page | In one line |
|---|------|-------------|
| 2.1 | [Linear Regression](./2.1-linear-regression.md) · [story](./2.1-linear-regression-intuition.md) | Fit `ŷ = w·x + b` that minimizes squared error — the cleanest line through the cloud |
| 2.2 | **Logistic Regression** *(planned)* | Linear regression squashed through sigmoid for binary classification |
| 2.3 | **k-Nearest Neighbors** *(planned)* | Vote of the k closest training points — no training phase, pure lazy learner |
| 2.4 | **Naive Bayes** *(planned)* | Bayes' rule + feature-independence assumption — fast, surprisingly strong on text |
| 2.5 | **Support Vector Machine** *(planned)* | Maximum-margin separator + kernel trick |

## Applied in

*(filled as cross-domain use cases land)*
