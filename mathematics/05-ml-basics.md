# Machine Learning — Basics (Regression, Classification, Metrics)

## 5.1 — Min-Max Normalization

**What it does:**
Rescales a feature's values to the range `[0, 1]` (or any custom range). Used when features have different units/scales and you want them all to contribute equally to a model — otherwise one feature (e.g., income in dollars) can dominate another (e.g., age in years) by sheer magnitude.

Imagine a **recipe** calling for "one cup of flour and one pinch of salt." If you measured both in grams, the salt would be invisible next to the flour's bulk. Min-max scaling is equivalent to **rescaling each ingredient to the same range** before mixing — so the model "tastes" each feature equally regardless of its natural units.

**Formula:**
```
x_scaled = (x − x_min) / (x_max − x_min)
```
For a custom range `[a, b]`:
```
x_scaled = a + (b − a) · (x − x_min) / (x_max − x_min)
```

**Simple example:**
Heights (cm): `[150, 160, 170, 180, 190]`, rescale to `[0, 1]`:
```
x_min = 150, x_max = 190
150 → (150−150)/40 = 0.00
160 → (160−150)/40 = 0.25
170 → (170−150)/40 = 0.50
180 → (180−150)/40 = 0.75
190 → (190−150)/40 = 1.00
```

**Complex example (robot sensor fusion):**
A rover fuses a laser rangefinder (values in meters, 0 – 30) with a wheel encoder (ticks, 0 – 2 000 000). Before feeding both into a neural network for terrain classification, normalize each to `[0, 1]` independently. Without this, the encoder's huge magnitude would make gradients dominated by one input while the laser contributes ~0 to learning.

**Pitfalls:**
- Sensitive to **outliers** — one extreme value stretches the range and compresses everything else into a narrow band. Robust alternatives: min-max with percentile clipping, or standardization (§5.2).

---

## 5.2 — Standardization (Z-score)

**What it does:**
Transforms a feature so it has **mean 0** and **standard deviation 1**. Like min-max, it puts features on a comparable scale — but instead of bounding by the range, it centers on the mean and measures in units of σ (standard deviations).

Imagine judging **students' grades** across different classes. A raw score of 85 means nothing without context: is that class' average 70 (the student is great) or 90 (the student is struggling)? Standardizing turns every grade into "how many σ above or below average?" — suddenly scores are comparable across classes. Z-scores are this comparability in mathematical form.

**Formula:**
```
z = (x − μ) / σ
```

| Symbol | Meaning |
|--------|---------|
| `μ` | Mean of the feature |
| `σ` | Standard deviation |
| `z` | Z-score — "how many σ away from the mean" |

**Simple example:**
Temperatures (°C): `[20, 22, 24, 26, 28]`. Mean `μ = 24`, σ ≈ 2.83.
```
20 → (20 − 24) / 2.83 ≈ −1.41
22 → (22 − 24) / 2.83 ≈ −0.71
24 → (24 − 24) / 2.83 =   0
26 → (26 − 24) / 2.83 ≈  0.71
28 → (28 − 24) / 2.83 ≈  1.41
```

**Complex example (outlier detection via Z-score):**
Wheel velocity readings from a wheel encoder: `[1.0, 1.1, 1.0, 1.2, 8.5, 1.0, 1.1]`. The `8.5` reading (a transient electrical spike) has:
```
μ ≈ 2.13,  σ ≈ 2.79
z(8.5) = (8.5 − 2.13) / 2.79 ≈ 2.28
```
A common rule: **|z| > 3** → treat as outlier (depending on your tolerance). At |z| ≈ 2.28 this reading is suspicious but borderline; if we used stricter thresholds, we'd filter it. Used in real-time SLAM to reject sensor spikes.

### Min-max vs standardization

| | Min-Max | Standardization |
|-|---------|-----------------|
| Range | `[0, 1]` (bounded) | Unbounded (typically ~`[−3, 3]`) |
| Preserves shape | Yes | Yes |
| Outlier-sensitive | **Very** | Moderate |
| Required by | Neural nets with sigmoid/tanh outputs, image pixels | Linear/logistic regression, SVM, PCA |

---

## 5.3 — Linear Regression

**What it does:**
Fits a straight line (or hyperplane) to a cloud of data points. Given inputs `x` and outputs `y`, finds weights `(w, b)` such that `ŷ = w·x + b` is as close as possible to the true `y`. The simplest supervised ML model and the building block for everything else.

Imagine a **physicist measuring** the extension of a spring under different weights. She plots (weight, extension) pairs on graph paper, sees a roughly linear pattern, and draws the best-fit line by eye. Linear regression is the formal, optimal version of "by eye" — it finds the line that **minimizes the total squared vertical distance** from every data point to the line.

### Prediction (univariate)

```
ŷ = w·x + b
```

### Multivariate

For `n` features `(x₁, x₂, …, xₙ)`:
```
ŷ = b + w₁·x₁ + w₂·x₂ + … + wₙ·xₙ
  = b + wᵀx
```

### Loss function (Mean Squared Error)

For a training set of `n` points `(xᵢ, yᵢ)`:
```
L(w, b) = (1/n) · Σᵢ (yᵢ − ŷᵢ)²

           ┌─────────── per-sample squared residual ──────────┐
         = (1/n) · Σᵢ (yᵢ − (w·xᵢ + b))²
```

Some textbooks use `1/(2n)` instead — the extra factor of 1/2 makes the gradient cleaner (the 2 from differentiating the square cancels) without changing the optimum.

### Partial derivatives (for gradient descent)

Using `L = (1/n)·Σᵢ (yᵢ − ŷᵢ)²`:
```
∂L/∂w  = −(2/n) · Σᵢ xᵢ·(yᵢ − ŷᵢ)
∂L/∂b  = −(2/n) · Σᵢ     (yᵢ − ŷᵢ)
```
Dropping the constant `2/n` (absorbed into the learning rate), the update direction is proportional to `−xᵢ·(yᵢ − ŷᵢ)` and `−(yᵢ − ŷᵢ)`. This is why you often see the compact form `∂L/∂w ∝ −(y − ŷ)·x`.

**Simple example:**
Fit `y = w·x + b` to 3 points: `(1, 2), (2, 4), (3, 6)`. Clearly `w = 2, b = 0`. Starting at `w = 0, b = 0`, `α = 0.1`:
```
Predictions:  ŷ = [0, 0, 0],  residuals = [2, 4, 6]
∂L/∂w = −(2/3) · (1·2 + 2·4 + 3·6) = −(2/3)·28 ≈ −18.67
∂L/∂b = −(2/3) · (2 + 4 + 6)       = −(2/3)·12 =  −8.0

w₁ = 0 − 0.1·(−18.67) =  1.87
b₁ = 0 − 0.1·( −8.0 ) =  0.80
```
After hundreds of iterations, `(w, b)` converges to approximately `(2, 0)`.

**Complex example (rover battery consumption model):**
Predict battery drain (`y`, % per km) from speed (`x₁`, m/s), payload (`x₂`, kg), and slope (`x₃`, degrees):
```
ŷ = b + w₁·x₁ + w₂·x₂ + w₃·x₃
```
After fitting on historical telemetry: `b = 0.2, w₁ = 0.05, w₂ = 0.01, w₃ = 0.08`. The signs are interpretable — all three inputs increase consumption — and the magnitudes rank their relative impact (slope dominates per-unit). Used to plan mission energy budgets.

---

## 5.4 — Root Mean Squared Error (RMSE)

**What it does:**
Summarizes how far a model's predictions stray from the truth, in the **same units** as the target variable. Lower RMSE = tighter predictions. The "industry standard" regression metric.

Imagine the **predictions** are darts thrown at a bullseye (the true values). RMSE is the **typical distance** of each dart from the center — not the average raw distance, but the root of the average squared distance, which penalizes big misses more than small ones. If you predict temperatures in °C, an RMSE of 2.1 means "on average, our predictions are off by about 2.1 °C."

**Formula:**
```
RMSE = √( (1/n) · Σᵢ (ŷᵢ − yᵢ)² )
```

**Simple example:**
```
Predictions ŷ = [3, 5, 2, 7]
Truth       y = [2, 6, 2, 8]
Residuals     = [1, −1, 0, −1]
Squared       = [1,  1, 0,  1]
Mean          = 3/4 = 0.75
RMSE          = √0.75 ≈ 0.866
```
→ Predictions typically off by ~0.87 units.

**Complex example (GPS localization accuracy):**
A GPS receiver reports position; ground truth comes from a survey-grade RTK system. Over 1000 samples, compute RMSE of (x, y) position:
```
RMSE_x ≈ 1.4 m
RMSE_y ≈ 1.8 m
RMSE_combined = √(1.4² + 1.8²) ≈ 2.28 m
```
→ Expected GPS positional error in this environment is ~2.3 m. Used to set the process-noise covariance of a Kalman filter that fuses GPS with wheel odometry.

---

## 5.5 — The Sigmoid Function

**What it does:**
Squashes any real number into the interval `(0, 1)`. Interprets the output of a linear model as a **probability**, which is the gateway from regression to classification.

Imagine a **thermostat dial** that outputs "full off" (0) at extreme cold and "full on" (1) at extreme heat, with a smooth transition in between. Sigmoid is exactly this: for very negative input it outputs ≈ 0, for very positive ≈ 1, and for input = 0 it's exactly 0.5 (the "indecision threshold").

**Formula:**
```
σ(z) = 1 / (1 + e^(−z))  =  (1 + e^(−z))^(−1)
```

**Properties:**
- `σ(0) = 0.5`
- `σ(z) → 1` as `z → +∞`
- `σ(z) → 0` as `z → −∞`
- Monotonically increasing, smooth, differentiable everywhere

**Derivative — elegant and essential:**
```
σ'(z) = σ(z) · (1 − σ(z))
```
This closed form (expressed in terms of `σ(z)` itself) is why sigmoid was historically the default activation in neural nets — the backprop gradient is trivial to compute once `σ(z)` is known.

**Simple example:**
```
z =  0    →  σ(z) = 0.5
z =  2    →  σ(z) ≈ 0.881
z = −2    →  σ(z) ≈ 0.119
z =  5    →  σ(z) ≈ 0.993
```

**Complex example (logistic regression for obstacle detection):**
A rover's forward collision model takes features `[distance_to_obstacle, velocity, brake_force]` and outputs a probability of collision:
```
z = b + w₁·d + w₂·v + w₃·F_brake
p_collision = σ(z)
```
Decision threshold typically `p > 0.5`: emergency brake. In calibrated systems, thresholds are tuned based on the **ROC curve** (see precision/recall below).

---

## 5.6 — Binary Cross-Entropy Loss

**What it does:**
Measures how far a probability prediction is from the true class (0 or 1). The standard loss for binary classification. Penalizes **confident wrong predictions** heavily and **confident right predictions** very lightly.

Imagine a **weather forecaster**. Saying "100% chance of rain" when it's sunny is disastrous — the viewer cancelled their picnic. Saying "60% chance of rain" when it's sunny is a soft miss. Cross-entropy rewards humility: the more confidently you commit to a wrong answer, the more it hurts. Matches how we'd grade a forecaster's reputation.

**Formula:**
```
L(y, ŷ) = −y · ln(ŷ) − (1 − y) · ln(1 − ŷ)
```
Only one of the two terms is nonzero per sample:
- If `y = 1`: `L = −ln(ŷ)` — the more `ŷ → 1`, the smaller `L`
- If `y = 0`: `L = −ln(1 − ŷ)` — the more `ŷ → 0`, the smaller `L`

**Derivative:**
```
∂L/∂ŷ = (ŷ − y) / (ŷ · (1 − ŷ))
```

**Elegant chain combining sigmoid + cross-entropy:**
When `ŷ = σ(z)` and you compute `∂L/∂z`, the ugly denominator `ŷ(1 − ŷ)` cancels perfectly with `σ'(z)`, giving:
```
∂L/∂z = ŷ − y
```
This is why the **sigmoid + cross-entropy** pairing is ubiquitous — the gradient is just the residual, as clean as linear regression.

**Simple example:**
```
True y = 1,  Predicted ŷ = 0.9    →  L = −ln(0.9)    ≈ 0.105  (small — good)
True y = 1,  Predicted ŷ = 0.1    →  L = −ln(0.1)    ≈ 2.303  (big — bad)
True y = 0,  Predicted ŷ = 0.05   →  L = −ln(0.95)   ≈ 0.051  (small — good)
True y = 0,  Predicted ŷ = 0.99   →  L = −ln(0.01)   ≈ 4.605  (very bad)
```

**Complex example (training a face-detection classifier):**
For each sliding window of an image, the model outputs `ŷ ∈ [0, 1]` ("face-probability"). Train with cross-entropy over thousands of labeled examples. The loss gradient pushes weights to increase `ŷ` on faces and decrease it on non-faces, **scaling up** the push when the model is confidently wrong — which accelerates learning.

---

## 5.7 — Classification Metrics (Confusion Matrix)

**What it does:**
Once you have predictions, you need to **grade** them. A confusion matrix counts the four possible outcomes of a binary classifier's predictions, and derived metrics (accuracy, precision, recall, F1) summarize different notions of "correctness."

Imagine a **spam filter**. "How good is it?" has no single answer — depends what you care about. Missing a spam email (false negative) annoys the user slightly; flagging a legitimate email as spam (false positive) might make them miss a job offer. Different metrics weight these errors differently. The confusion matrix is the raw 2×2 tally; precision/recall are perspectives on it.

### The 2×2 confusion matrix

|                   | **Predicted: 1**   | **Predicted: 0**   |
|-------------------|--------------------|--------------------|
| **Actual: 1**     | TP (True Positive) | FN (False Negative)|
| **Actual: 0**     | FP (False Positive)| TN (True Negative) |

### Derived metrics

```
Accuracy    = (TP + TN) / (TP + TN + FP + FN)
Precision   = TP / (TP + FP)                    — "of what I predicted positive, how much was right?"
Recall      = TP / (TP + FN)                    — "of what was actually positive, how much did I catch?"
Specificity = TN / (TN + FP)                    — "of what was actually negative, how much did I correctly reject?"
F1-score    = 2 · (Precision · Recall) / (Precision + Recall)
```

| Metric | When to prioritize |
|--------|--------------------|
| **Accuracy** | Classes are balanced and errors are symmetric |
| **Precision** | Cost of a false positive is high (e.g., a medication trial: don't falsely claim a drug works) |
| **Recall** | Cost of a false negative is high (e.g., cancer screening: catch every positive, false alarms are OK) |
| **Specificity** | You care about correctly excluding the negative class (e.g., medical tests, safety systems) |
| **F1-score** | Imbalanced classes or you want a single-number compromise between precision and recall |

### Accuracy can be deceptive

If 99% of emails are not spam and your model predicts "not spam" for everything:
```
Accuracy = 99%     (looks great!)
Recall   = 0%      (catches no spam — useless)
```
→ Always check **at least** precision + recall, not accuracy alone, on imbalanced data.

**Simple example:**
A spam filter tested on 1000 emails (100 spam, 900 ham). Results:
- TP = 80 (correctly flagged spam)
- FN = 20 (missed spam)
- FP = 30 (ham marked as spam)
- TN = 870 (correctly let ham through)

```
Accuracy    = (80 + 870) / 1000 = 0.95   → 95%
Precision   = 80 / (80 + 30)   = 0.73    → of flagged emails, 73% really were spam
Recall      = 80 / (80 + 20)   = 0.80    → caught 80% of actual spam
Specificity = 870 / (870 + 30) = 0.967
F1          = 2·(0.73·0.80) / (0.73 + 0.80) ≈ 0.76
```

**Complex example (rover terrain classifier):**
A rover classifies terrain as "safe" (1) vs "unsafe" (0) from camera images:
- False positive (predicts safe, actually unsafe) → rover drives into a hole. Disaster.
- False negative (predicts unsafe, actually safe) → rover gets cautious and takes a detour. Annoying but safe.

→ The cost of FP >>> cost of FN. **Maximize specificity (or precision on the "safe" class)** at the expense of recall. You'd rather call occasional safe terrain unsafe than the reverse.

---

## Applied in

| Concept | Used in |
|---------|---------|
| **Linear regression** | Curve fitting, battery models, sensor calibration |
| **Sigmoid + cross-entropy** | Binary classifiers, logistic regression, neural network output layers |
| **RMSE** | [ROS2 — Inverse Kinematics](../ros2/20-inverse-kinematics.md) (residual error), Kalman filter tuning, GPS accuracy |
| **Normalization** | Neural network preprocessing, sensor fusion |
| **Classification metrics** | Obstacle detection, terrain classification, anomaly detection |
| **Gradient descent** | [`04-optimization.md`](./04-optimization.md) — mechanics of training |
