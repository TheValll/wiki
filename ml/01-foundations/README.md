# 01 — Foundations

Concept pages (not models). The mental scaffolding everything else hangs from: how training works at the meta-level, how to split data, why models over- or under-fit, what loss and regularisation actually mean.

These follow the **atomic concept** format (like `mathematics/`) — no Hyperparameters / Evaluation / Variants sections, just intuition → mechanism → math → example. Reach for these whenever a model page says "see foundations".

## Sources

- **Deisenroth ch 8** — _When Models Meet Data_: ERM, parameter estimation (MLE / MAP), model selection
- **Géron — Hands-On ML** ch 2 + ch 4 — practical framing for splits, learning curves, regularisation
- **Bishop — PRML** ch 1 — decision theory, model complexity, the bias-variance derivation

## Pages

| §   | Page                                            | In one line                                                                                    |
| --- | ----------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| 1.1 | **Bias-Variance Tradeoff** _(planned)_          | The fundamental tension — simple models are stable but biased, complex ones flexible but noisy |
| 1.2 | **Train / Validation / Test Split** _(planned)_ | Three disjoint datasets, three different jobs — no leakage allowed                             |
| 1.3 | **Overfitting & Underfitting** _(planned)_      | Read the gap between training and validation error to diagnose model capacity                  |
| 1.4 | **Loss Functions Overview** _(planned)_         | Map of MSE / MAE / BCE / CCE / Hinge / Huber — when each is the right cost                     |
| 1.5 | **Empirical Risk Minimization** _(planned)_     | The formal frame for "training": pick parameters minimising average loss on training data      |
| 1.6 | **Regularization Overview** _(planned)_         | L1, L2, early stopping, dropout — why we deliberately handicap a model that fits too well      |

## Applied in

_(filled as cross-domain use cases land — most foundations pages will be heavily linked from every model in chapters 02-12)_
