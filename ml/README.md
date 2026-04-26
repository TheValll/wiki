# Machine Learning Wiki

Machine learning models — from classical (regression, trees) to deep (CNN, Transformers), generative (Diffusion), and reinforcement (PPO). Each page covers **one model**: what it does, how it works, the math, a schema, where it shines, and how to evaluate it.

> **At a glance:** [`RECAP.md`](./RECAP.md) — single-glance table of every model, grouped by chapter.

---

## Conventions

Each chapter is a **folder** grouping a model family; each `X.Y-...md` file inside is **one model**. Cross-references to math use relative paths (e.g. `[Linear Regression — math](../mathematics/05-ml-basics/5.3-linear-regression.md)`) so the same concept is never re-derived.

A few chapters (`01-foundations/`, `13-evaluation/`) hold **concept** pages rather than model pages — they use the lighter atomic format from `mathematics/` instead of the schema below.

---

## Page schema (every model file)

Every model page follows the same skeleton — **physical image first, formula last** (per [`../how-i-learn.md`](../how-i-learn.md)). Optional sections are dropped, not left blank.

| Section | Purpose |
|---|---|
| **Header** — `**What it does:**`, `**Family:**`, TL;DR table (Input · Output · Trains via · Cost) | One-glance identity card |
| **X.Y.1 — Intuition** | Physical analogy + multi-frame ASCII schema. No formulas yet. |
| **X.Y.2 — Mechanism** | The algorithm step-by-step in plain English. Pseudocode (≤ 15 lines) if it helps. |
| **X.Y.3 — Math** | Loss / objective, optimization, key equations. Cross-ref `mathematics/` rather than re-derive. |
| **X.Y.4 — Schema** | ASCII diagram: data flow, decision boundary, or layer stack. |
| **X.Y.5 — Use cases** | ✓ where it shines · ✗ where it breaks · canonical datasets (MNIST, ImageNet, IMDB…). |
| **X.Y.6 — Hyperparameters** | Table: name · effect · typical range · symptom if mistuned. |
| **X.Y.7 — Evaluation** | Model-specific metrics (log-loss, IoU, BLEU, FID…) + generic metrics + diagnostic curves + failure modes. |
| **X.Y.8 — Variants** *(optional)* | 3-col compare with siblings: variant · key diff · when to prefer. |
| **X.Y.9 — Implementation pointer** *(optional)* | sklearn / PyTorch / JAX in 5-10 lines max. |
| **See also** | Cross-refs (math, other `ml/` pages). |

---

## Sources per block

Every chapter states its primary book(s) at the top. Default mapping:

| Block | Primary source(s) |
|---|---|
| Foundations, classical supervised, unsupervised, dimensionality | **Deisenroth ch 8-12** + Hastie & Tibshirani (ESL) for depth |
| Trees & ensembles | **Géron — Hands-On ML** + original papers (XGBoost, LightGBM) |
| Neural nets, CNN, RNN | **Goodfellow — Deep Learning** + Géron for hands-on |
| Transformers | Original papers (Vaswani 2017, BERT, GPT, ViT) + Hugging Face docs |
| Generative | Papers (Kingma VAE, Goodfellow GAN, Ho DDPM, Stable Diffusion) |
| Reinforcement | **Sutton & Barto** + Spinning Up (OpenAI) |
| Evaluation, MLOps | Géron + scikit-learn user guide |

---

## Syllabus

### Block A — Foundations

| # | Chapter | Status |
|---|---------|--------|
| 01 | **Foundations** *(planned)* — bias-variance, train/val/test, loss, gradient descent context | — |

### Block B — Classical ML

| # | Chapter | Status |
|---|---------|--------|
| 02 | **Supervised Classical** *(planned)* — linear regression, logistic, k-NN, naive Bayes, SVM | — |
| 03 | **Trees & Ensembles** *(planned)* — decision tree, random forest, gradient boosting (XGBoost / LightGBM / CatBoost) | — |
| 04 | **Unsupervised** *(planned)* — K-Means, DBSCAN, GMM, hierarchical, isolation forest | — |
| 05 | **Dimensionality** *(planned)* — PCA, t-SNE, UMAP | — |

### Block C — Deep Learning

| # | Chapter | Status |
|---|---------|--------|
| 06 | **Neural Nets** *(planned)* — MLP, activations, backprop, regularization, optimizers | — |
| 07 | **CNN** *(planned)* — LeNet, AlexNet, VGG, ResNet, EfficientNet | — |
| 08 | **RNN & Sequences** *(planned)* — RNN, LSTM, GRU, seq2seq | — |
| 09 | **Transformers** *(planned)* — attention, BERT, GPT, ViT, CLIP, MoE | — |

### Block D — Generative & RL

| # | Chapter | Status |
|---|---------|--------|
| 10 | **Generative** *(planned)* — Autoencoder, VAE, GAN, Diffusion, NeRF / Gaussian Splatting | — |
| 11 | **Reinforcement** *(planned)* — Q-learning, DQN, PG, PPO, SAC, AlphaZero, RLHF | — |

### Block E — Other & Meta

| # | Chapter | Status |
|---|---------|--------|
| 12 | **Graph & Other** *(planned)* — GNN, Mamba / State-Space Models | — |
| 13 | **Evaluation** *(planned)* — CV, calibration, ROC/PR, fairness, MLOps basics | — |

---

For the broader wiki structure and agent rules, see [`../CLAUDE.md`](../CLAUDE.md). For the new-domain playbook used to build this folder, see [`../domains.md`](../domains.md).
