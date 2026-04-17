# Statistics — Moments, Distributions, Inference

## 7.1 — Expected Value (Mean)

**What it does:**
The "balance point" of a probability distribution — the value you'd get on average if you sampled from it an infinite number of times. Shows up everywhere: long-run profit, typical sensor reading, loss function evaluated on a dataset.

Imagine balancing a **ruler with weights** placed at various marks. The position where the ruler balances on your fingertip is the expected value. If most weight is on the left, the balance point shifts left. If symmetric, it sits right in the middle.

**Formula:**
```
Discrete :  E[X] = Σₓ x · P(X = x)
Continuous: E[X] = ∫ x · f(x) dx
```

**Linearity of expectation** (crucial — holds even for dependent variables):
```
E[a·X + b]       = a · E[X] + b
E[X₁ + X₂ + …]  = E[X₁] + E[X₂] + …
```

**Simple example (fair die):**
```
E[X] = 1·(1/6) + 2·(1/6) + 3·(1/6) + 4·(1/6) + 5·(1/6) + 6·(1/6)
     = 21/6 = 3.5
```
Note: `E[X]` does not have to be a possible value of `X`.

**Complex example (expected mission time):**
A rover's mission has three segments with random durations `T₁, T₂, T₃`. Means: `E[T₁] = 20 min, E[T₂] = 35 min, E[T₃] = 15 min`. Total expected time:
```
E[T_total] = E[T₁] + E[T₂] + E[T₃] = 70 min
```
Holds **even if** the segments' durations are correlated (bad weather slowing all of them). That's the magic of linearity.

---

## 7.2 — Variance and Standard Deviation

**What it does:**
Measures the **spread** of a distribution around its mean. Variance uses squared deviations (good mathematical properties, bad units). Standard deviation is the square root of variance — same units as `X`, directly interpretable.

Imagine two **archers**. Both hit the bullseye on average (same mean), but one groups their arrows tightly (low variance) and the other scatters them (high variance). Variance captures "consistency vs chaos" — not where the center is, but how wide the spread is around it.

**Formulas:**
```
Var(X) = E[(X − μ)²]  =  E[X²] − (E[X])²
σ      = √Var(X)              (standard deviation)
```

**Rules:**
```
Var(a·X + b)   = a² · Var(X)                 (constant shift doesn't affect variance)
Var(X₁ + X₂)   = Var(X₁) + Var(X₂) + 2·Cov(X₁, X₂)
              = Var(X₁) + Var(X₂)            if independent
```

**Simple example:**
A random variable takes values `[2, 4, 4, 4, 5, 5, 7, 9]`:
```
Mean     μ    = 40 / 8 = 5
Squared deviations (xᵢ − μ)²:  [9, 1, 1, 1, 0, 0, 4, 16]
Variance σ²   = 32 / 8 = 4
Std dev  σ    = 2
```

**Complex example (wheel-odometry uncertainty):**
A wheel-encoder counts ticks per revolution with `σ_tick = 0.8 ticks`. Over one revolution (converting ticks to distance), the position estimate's variance accumulates:
```
Var(position) ≈ Var(tick) · (2π·r / ticks_per_rev)²
```
Propagating this over many revolutions uses `Var(X₁ + X₂ + …) = Σ Var(Xᵢ)` for independent noise. This is how a Kalman filter grows its covariance matrix through the motion model.

---

## 7.3 — Skewness (Asymmetry)

**What it does:**
Measures whether a distribution leans to the left or right. Positive skew = long tail on the right; negative skew = long tail on the left; zero = symmetric.

Imagine **salaries** in a company. A handful of executives earn vastly more than everyone else — the distribution has a long right tail. Median ≪ mean. That's **positive skewness**. Conversely, imagine **age at retirement**: most people retire around 65, few retire very young — the tail is on the left (negative skewness).

**Formula (3rd standardized moment):**
```
Skewness = E[ ((X − μ) / σ)³ ]
```

| Skew value | Shape |
|------------|-------|
| **> 0** | Long tail to the right (positive skew) |
| **= 0** | Symmetric (e.g., Gaussian) |
| **< 0** | Long tail to the left (negative skew) |

**Rule of thumb:**
- `|skew| < 0.5` → approximately symmetric
- `0.5 < |skew| < 1` → moderately skewed
- `|skew| > 1` → highly skewed

**Simple example (dataset):**
```
Incomes (k€): [30, 32, 35, 40, 500]
Mean        : 127.4
Median      :  35
```
Mean ≫ median → strong **positive skewness**. The outlier pulls the mean far from the bulk of the data.

**Complex example (GPS noise asymmetry):**
In open sky, GPS error is roughly Gaussian (symmetric, skew ≈ 0). In urban canyons with multipath, the error distribution becomes **positively skewed** — occasional large errors pull the tail right. Kalman filters that assume Gaussian noise can mishandle this; robust filters (Huber loss, iSAM) handle non-Gaussian residuals better.

---

## 7.4 — Kurtosis (Tail Weight)

**What it does:**
Measures how heavy the **tails** of a distribution are — or equivalently, how "peaked" the center is. Distinguishes "mostly typical values + occasional extreme outliers" (heavy tails) from "well-behaved around the mean" (light tails).

Imagine a **sniper vs a shotgun**. Both aim at the center. A sniper's shots cluster tightly near the bullseye with the occasional wild miss — **leptokurtic** (peaked center, heavy tails). A shotgun scatters shots broadly but never produces true outliers — **platykurtic** (flatter center, light tails). A normal rifle is in between — "mesokurtic" (normal-like).

**Formula (4th standardized moment):**
```
Kurtosis = E[ ((X − μ) / σ)⁴ ]
```

| Distribution | Kurtosis | Nickname |
|--------------|----------|----------|
| Gaussian (normal) | 3.0 | Mesokurtic (reference) |
| **Higher** (> 3) | — | **Leptokurtic** — peaked, heavy tails (e.g., Student-t, Laplace, most financial returns) |
| **Lower** (< 3) | — | **Platykurtic** — flatter, light tails (e.g., uniform) |

**Excess kurtosis** is often quoted instead: `excess = kurtosis − 3` (so the Gaussian sits at 0).

**Simple example (comparison):**
```
Uniform distribution on [0, 1]        :  kurtosis = 1.8    (platykurtic)
Standard Gaussian                      :  kurtosis = 3.0
Laplace (double-exponential)          :  kurtosis = 6.0    (leptokurtic)
Cauchy                                 :  kurtosis = ∞     (pathological — no defined mean or variance)
```

**Complex example (sensor outlier rejection):**
A LiDAR returning mostly good readings but occasional spurious max-range reflections produces a **leptokurtic** error distribution. A naive Kalman filter (assumes Gaussian → kurtosis 3) treats each spike as a high-information event and incorrectly shifts the state estimate. A **chi-square gating** step or robust filter first detects the high-kurtosis events and rejects them.

---

## 7.5 — Quantiles, Percentiles, and Boxplots

**What it does:**
Summarizes a distribution using **rank** rather than raw value. The k-th percentile is the value below which k% of the data falls. Particularly useful when distributions are skewed or have outliers — percentiles are more robust than mean ± σ.

Imagine **runners crossing a finish line**. The median runner (50th percentile) is the one with exactly half the field ahead and half behind. The 25th percentile is the one with 75% of the runners behind them (a slow runner). The 90th percentile is near the front (a fast runner). Percentiles work regardless of whether runners cluster tight or spread out.

**Definitions:**
```
q_p = value such that  P(X ≤ q_p) = p

Median      = q_{0.5}             = Q₂
Quartiles   = q_{0.25}, q_{0.5}, q_{0.75}  =  Q₁, Q₂, Q₃
Percentiles = q_{p}  for p ∈ [0, 1]
```

### Interquartile range (IQR)

```
IQR = Q₃ − Q₁       (spread of the middle 50% of the data)
```

### The boxplot

```
   x_min          Q₁      Q₂     Q₃          x_max        outliers
     │             │      │       │            │             •
     │             │      │       │            │             •
     ├─ ─ ─ ─ ─ ─ ─┤      │       ├─ ─ ─ ─ ─ ─ ┤
     │  whisker    │ box  │       │  whisker   │
                    ├──────────────┤
                    │     median   │
                    └──────────────┘
```

**Whisker rule (Tukey):**
```
Upper whisker = min(x_max, Q₃ + 1.5 · IQR)
Lower whisker = max(x_min, Q₁ − 1.5 · IQR)
Points beyond the whiskers are plotted individually — "outliers."
```

### QQ-plot (Quantile-Quantile)

Plots sample quantiles against the theoretical quantiles of a reference distribution (usually Gaussian). A straight diagonal line → sample matches the reference. Curved or S-shaped → deviates (skewed, heavy-tailed, etc.).

**Simple example:**
Data: `[1, 3, 4, 5, 7, 8, 9, 10, 14, 25]`
```
Q₁  = 4     (25% below)
Q₂  = 7.5   (median)
Q₃  = 10    (75% below)
IQR = 10 − 4 = 6
Upper whisker = 10 + 1.5·6 = 19   → point 25 is an outlier
Lower whisker = 4  − 1.5·6 = −5   → no low outliers
```

**Complex example (SLAM residual diagnostics):**
After running SLAM, plot the boxplot of residual errors (distance between observed and predicted landmark positions). If you see:
- Long whiskers, many outliers → sensor model is underestimating noise variance
- Strongly skewed box → systematic bias in motion model
- Clean box, few outliers → residuals are well-modeled

A QQ-plot against Gaussian checks if residuals are normally distributed (assumed by most estimators). Fat-tailed residuals argue for a robust estimator.

---

## 7.6 — Covariance

**What it does:**
Measures how two random variables **vary together**. Positive = they tend to increase together; negative = one increases when the other decreases; zero = no linear relationship (but may still be related nonlinearly).

Imagine **temperature and ice-cream sales**. On hot days, sales go up — they vary together positively (positive covariance). Now consider **rainfall and outdoor seating occupancy** — more rain, less seating used (negative covariance). Covariance formalizes this co-movement, in the raw (unnormalized) form.

**Formula:**
```
Cov(X, Y) = E[ (X − μₓ) · (Y − μᵧ) ]
         = E[X·Y] − E[X]·E[Y]
```

**Properties:**
```
Cov(X, X)        = Var(X)
Cov(X, Y)        = Cov(Y, X)           (symmetric)
Cov(aX + b, cY + d) = ac · Cov(X, Y)  (scales with both)
Cov independent  = 0                  (but the converse is false!)
```

### Covariance matrix (multivariate)

For `n` variables, the covariance matrix is `n × n`:
```
         [ Var(X₁)       Cov(X₁, X₂)   ...   Cov(X₁, Xₙ) ]
  Σ   =  [ Cov(X₂, X₁)   Var(X₂)       ...   Cov(X₂, Xₙ) ]
         [    ...            ...       ...        ...    ]
         [ Cov(Xₙ, X₁)   Cov(Xₙ, X₂)   ...   Var(Xₙ)     ]
```
Symmetric and positive semi-definite. Diagonal holds individual variances.

**Simple example:**
```
X = [1, 2, 3, 4, 5],  μₓ = 3
Y = [2, 4, 6, 8, 10], μᵧ = 6

Cov(X, Y) = (1/5) · Σ (xᵢ − 3)(yᵢ − 6)
          = (1/5) · [(-2)(-4) + (-1)(-2) + 0 + 1·2 + 2·4]
          = (1/5) · [8 + 2 + 0 + 2 + 8]
          = 20/5 = 4
```
Perfect positive co-movement (`Y = 2X`).

**Complex example (Kalman filter state covariance):**
A robot's state `[x, y, θ]` has an uncertainty matrix:
```
      [ σ_x²      ρ·σ_x·σ_y   0  ]
Σ  =  [ ρ·σ_x·σ_y  σ_y²        0  ]
      [ 0          0          σ_θ² ]
```
The off-diagonal `ρ·σ_x·σ_y` term captures that **x and y uncertainties are correlated** (if the robot is lost along the track, both x and y errors grow together). The filter propagates and updates this whole matrix every timestep — tracking not just "how uncertain" but "how the uncertainties relate."

---

## 7.7 — Correlation

**What it does:**
A **normalized** version of covariance — bounded in `[−1, 1]`, scale-free. Measures the **strength** of a linear relationship regardless of units.

Imagine **temperature measured in °C vs °F**. The two measurements have the same relationship (they're just a linear rescaling), but their covariance would be wildly different numerically depending on units. Correlation divides covariance by the standard deviations — suddenly the numerical answer is the same (`+1`, perfect linear) regardless of units. It answers "how tightly linear is this relationship?" rather than "how big."

**Formula:**
```
               Cov(X, Y)
ρ(X, Y)  =  ──────────────────
             σₓ · σᵧ

         =  Cov(X, Y) / √(Var(X) · Var(Y))
```

**Interpretation:**
| ρ value | Meaning |
|---------|---------|
| `+1` | Perfect positive linear relationship |
| `0.7 to 1` | Strong positive linear |
| `0.3 to 0.7` | Moderate positive |
| `0` | No linear relationship (may still be nonlinearly related!) |
| `−0.3 to −0.7` | Moderate negative |
| `−1` | Perfect negative linear |

**Simple example (same as covariance §7.6):**
```
X = [1, 2, 3, 4, 5],   σₓ = √2 ≈ 1.414
Y = [2, 4, 6, 8, 10],  σᵧ = √8 ≈ 2.828
Cov(X, Y) = 4

ρ = 4 / (√2 · √8) = 4 / 4 = 1.0
```
→ Perfect linear correlation, as expected for `Y = 2X`.

**Complex example (anomaly detection via correlation breakdown):**
In a healthy electric motor, current draw and rotational speed are strongly correlated (`ρ ≈ 0.95`). If bearings start failing, friction introduces unpredictable load and the correlation **drops** to `ρ ≈ 0.7`. Monitoring correlation over a sliding window is a cheap anomaly detector — no physics model needed. Used in predictive maintenance for industrial robots.

```
┌──────────────────────────────────────────────────────────────────┐
│  Remember: ρ = 0 means "no LINEAR relationship."                  │
│  Variables can still be perfectly dependent nonlinearly.          │
│                                                                   │
│  Example: Y = X²  with X ∈ [−1, 1] symmetric                      │
│   → Cov(X, Y) = 0, ρ = 0, yet Y is completely determined by X.    │
└──────────────────────────────────────────────────────────────────┘
```

---

## 7.8 — Multivariate Gaussian

**What it does:**
Extends the univariate Gaussian to multiple correlated variables. Encodes both **mean vector** (where the distribution is centered) and **covariance matrix** (how the variables spread and correlate). The workhorse of multivariate noise modeling, Kalman filters, PCA, and Gaussian mixture models.

Imagine a **2D scatterplot**. A univariate Gaussian describes a cloud along one axis. The multivariate version describes a 2D cloud shaped like an **ellipse** — centered at the mean, oriented along the directions of maximum variance (eigenvectors of the covariance), with axes proportional to √(eigenvalues). Uncorrelated variables give axis-aligned ellipses; correlated ones give tilted ellipses.

### 2D Gaussian — independent case (ρ = 0)

When `X` and `Y` are independent, the joint PDF factors as the product of the two marginals:
```
                       1
f(x, y)  =  ─────────────────────  · exp( −½ · [((x − μₓ)/σₓ)² + ((y − μᵧ)/σᵧ)²] )
             2π · σₓ · σᵧ
```

### 2D Gaussian — general case (dependent, with correlation ρ)

```
                            1
f(x, y) = ──────────────────────────────  · exp(−Q/2)
          2π · σₓ · σᵧ · √(1 − ρ²)

Q = 1/(1 − ρ²)  ·  [ ((x − μₓ)/σₓ)²
                   − 2ρ · ((x − μₓ)/σₓ)·((y − μᵧ)/σᵧ)
                   +     ((y − μᵧ)/σᵧ)² ]
```
When `ρ = 0`, the cross-term vanishes and this reduces to the independent form.

### n-dimensional form (vector / matrix notation)

Elegantly:
```
                    1
f(x)  =  ───────────────────────── · exp( −½ · (x − μ)ᵀ · Σ⁻¹ · (x − μ) )
          (2π)^(n/2) · √|Σ|
```
where `x, μ ∈ ℝⁿ` and `Σ` is the `n × n` covariance matrix.

**Simple example (axis-aligned 2D):**
```
μ = (0, 0),  σₓ = 1,  σᵧ = 2,  ρ = 0

Contours are ellipses:  (x/1)² + (y/2)² = constant
  → wider along the y-axis (σᵧ = 2) than along the x-axis (σₓ = 1)
```

**Complex example (Kalman filter position belief):**
A rover's estimated position `(x, y)` has a covariance matrix that evolves each timestep. After 10 seconds of driving east with noisy motion:
```
Σ_after_10s  =  [ 4.0   2.1 ]
                [ 2.1   1.5 ]
```
`ρ = 2.1 / √(4.0·1.5) ≈ 0.86` → heavily correlated. The uncertainty ellipse is **long and tilted northeast** — the robot is much more uncertain about how far east it went than how far north. This kind of shape drives decisions like "turn on GPS now" vs "keep dead reckoning."

---

## 7.9 — Population vs Sample (Why `n − 1`?)

**What it does:**
Distinguishes the distribution you'd observe if you had the **entire population** (everyone, every measurement forever) from what you observe in a **finite sample**. Statistics is about estimating population quantities from samples — and the estimators differ subtly (the "Bessel correction" `n − 1`).

Imagine wanting to know the **true mean height of all adults in a country** (population parameter `μ`). Measuring every adult is impossible, so you measure 1000 of them (a sample). Your sample mean `x̄` is your best guess at `μ`, but it's a *guess* — it carries error from having measured only 1000 instead of all 50 million. Sample statistics are always estimators of population parameters.

### Population (θ) vs sample (θ̂) notation

| Quantity | Population | Sample |
|----------|------------|--------|
| Mean | `μ` | `x̄` |
| Variance | `σ²` | `s²` |
| Std dev | `σ` | `s` |
| Proportion | `p` | `p̂` |

### Formulas

**Population (when you truly have all N members):**
```
μ    = (1/N) · Σᵢ xᵢ
σ²   = (1/N) · Σᵢ (xᵢ − μ)²
p    = k / N                   (proportion = fraction with property)
```

**Sample (n < population):**
```
x̄    = (1/n) · Σᵢ xᵢ
s²   = 1/(n − 1) · Σᵢ (xᵢ − x̄)²     ← note n − 1, not n
p̂    = k / n
```

### Why `n − 1` (Bessel's correction)?

The sample mean `x̄` is computed from the same data you're checking deviations against. That reuse makes the sum of squared deviations **systematically smaller** than it would be if you compared to the true population mean `μ`. Dividing by `n − 1` (one less) corrects this bias — resulting in an **unbiased estimator** of `σ²`.

Intuitively: you've "used up one degree of freedom" computing `x̄`, so there are only `n − 1` truly independent pieces of information left for estimating the spread.

**Simple example:**
Sample data: `[4, 6, 8]`.
```
x̄      = 6
∑(xᵢ − x̄)² = 4 + 0 + 4 = 8

Sample variance  s²  = 8 / (3 − 1) = 4.0     ← unbiased estimate
"Naive" variance     = 8 / 3       ≈ 2.67    ← biased (underestimates σ²)
```

**Complex example (reporting measurement accuracy):**
A lab measures the radius of 20 machined parts to certify manufacturing precision. They compute `s` (sample standard deviation with `n − 1`) and report it as "standard deviation of the production process." Using `n` would underestimate variability and falsely inflate the reported precision — regulators specifically require the Bessel-corrected estimator.

---

## 7.10 — Central Limit Theorem (CLT)

**What it does:**
States that the **sum (or mean) of many independent random variables** tends to a Gaussian distribution, regardless of the shape of the individual variables' distributions. **The single most important theorem in statistics** — it's why the Gaussian is everywhere.

Imagine a **Plinko board** with 10 rows of pegs. Dropping a single ball: the outcome is a fairly random Bernoulli walk. Dropping 10 000 balls: the landing positions form a nearly perfect bell curve, regardless of whether pegs are biased or not. The more rows, the tighter the bell. CLT says that sums of *anything* — as long as they're independent and not pathological — average out to Gaussian.

**Formula (informally):**
If `X₁, X₂, …, Xₙ` are independent identically distributed with mean `μ` and variance `σ²`, then as `n → ∞`:
```
X̄ₙ = (1/n) · Σᵢ Xᵢ   →   N(μ, σ²/n)
```
The sample mean has its own (Gaussian) distribution around `μ`, with shrinking variance `σ²/n`. Equivalently, the **standardized sum**:
```
(X̄ₙ − μ) / (σ / √n)  →  N(0, 1)
```

**Rate of convergence:**
For reasonably nice distributions, `n = 30` already gives a very good Gaussian approximation.

**Simple example (rolling dice):**
Roll a single die: distribution is uniform on `{1, …, 6}`, very non-Gaussian. Roll and sum 30 dice: the distribution of the sum is strikingly Gaussian, with mean `30·3.5 = 105` and variance `30·(35/12) ≈ 87.5`.

**Complex example (sensor noise averaging):**
An IMU reports gyroscope readings with non-Gaussian noise (e.g., slight skew from temperature effects). Individual readings are not Gaussian. **But** the average of 100 consecutive readings is nearly perfectly Gaussian (CLT), with `σ_avg = σ_single / 10`. This is why:
- Filters can assume Gaussian noise when averaging (Kalman filter)
- Calibration procedures compute means — smoother and well-understood
- Bias tests use t-tests against the mean — valid thanks to CLT

```
┌──────────────────────────────────────────────────────────────────┐
│  CLT is WHY Gaussians dominate applied probability:               │
│                                                                   │
│  Measurement errors → sum of tiny independent factors → Gaussian  │
│  Biological traits  → sum of many genetic effects   → Gaussian    │
│  Financial returns  → sum of many market moves      → Gaussian    │
│                      (approximately — heavy tails in crises)      │
│                                                                   │
│  When you don't know the true distribution, assume Gaussian and   │
│  justify it with CLT. You're right ~80% of the time.              │
└──────────────────────────────────────────────────────────────────┘
```

---

## 7.11 — Maximum Likelihood Estimation (MLE)

**What it does:**
A principled way to estimate distribution parameters from data: pick the parameters that make the **observed data most probable**. If the data already happened, the best guess for the distribution that produced them is the one under which the data is most likely.

Imagine a **detective** finding a bullet at a crime scene and comparing it to three suspects' guns. Suspect A's gun could have produced this bullet with probability 0.8. Suspect B's with 0.05. Suspect C's with 0.0001. The MLE choice is suspect A — the hypothesis under which the evidence is most plausible. MLE formalizes "who makes this observed data most likely?"

**Formula:**
Given data `x = (x₁, x₂, …, xₙ)` and a parameterized distribution `f(x; θ)`:
```
Likelihood :       L(θ; x) = ∏ᵢ f(xᵢ; θ)         (joint probability of the data, as a function of θ)
Log-likelihood:    ℓ(θ) = Σᵢ log f(xᵢ; θ)        (usually easier to work with)

MLE:   θ̂ = argmaxₜₕₑₜₐ  ℓ(θ)
```

The argmax is found by setting `dℓ/dθ = 0` and solving, or numerically via gradient methods.

**Simple example (MLE of a Gaussian's mean):**
Data `x₁, …, xₙ` sampled from `N(μ, σ²)` with known `σ`. Likelihood:
```
L(μ) = ∏ᵢ  (1 / (σ√(2π))) · exp(−(xᵢ − μ)² / (2σ²))
```
Log-likelihood:
```
ℓ(μ) = −(1/(2σ²)) · Σ (xᵢ − μ)²  +  constants
```
Maximizing `ℓ` is equivalent to minimizing `Σ (xᵢ − μ)²` — which happens at `μ = x̄` (sample mean). **MLE of a Gaussian mean is the sample mean** — a deeply intuitive result.

**Complex example (fitting a sensor bias):**
A gyroscope measures `z_i = ω_true + b + ε_i` where `b` is the bias and `ε_i ∼ N(0, σ²)`. Stationary measurements (where `ω_true = 0`) give:
```
z_i = b + ε_i

MLE of b:  b̂ = (1/n) · Σᵢ z_i
```
→ Average the readings over a stationary period. The bias estimate is literally the sample mean of the stationary readings. Used in **IMU calibration at startup** — a robot sits still for 5 seconds, accumulates readings, takes the mean as the bias correction.

### Connection to other estimators
- MLE with Gaussian noise → least-squares regression
- MLE with Bernoulli → logistic regression
- MLE with Poisson counts → Poisson regression

Most "standard" ML losses are secretly MLE under some noise assumption.

---

## 7.12 — Hypothesis Testing

**What it does:**
A formal framework for deciding whether observed data supports or contradicts a specific claim. Used everywhere experiments meet decisions — drug trials, A/B testing, sensor anomaly detection, scientific publication.

Imagine a **courtroom**. The null hypothesis `H₀` is the default: "innocent until proven guilty." You need overwhelming evidence (small p-value) to **reject** H₀ in favor of the alternative `H₁` ("guilty"). Hypothesis testing formalizes "how much evidence is enough?" and quantifies the risks of the two possible mistakes (wrongful conviction vs acquittal of the guilty).

### The four actors

| Symbol | Name | Meaning |
|--------|------|---------|
| `H₀` | **Null hypothesis** | The default claim (often "no effect / no difference") |
| `H₁` (or `Hₐ`) | **Alternative hypothesis** | The claim that contradicts `H₀` |
| `α` | **Significance level** | Acceptable risk of false rejection of H₀ (typically 0.05) |
| **p-value** | Probability of observing data at least this extreme *if H₀ were true* |

### The decision rule

```
If  p-value < α   →  reject H₀  (data is statistically inconsistent with H₀)
If  p-value ≥ α   →  fail to reject H₀  (data is compatible with H₀)
```

> ⚠️ We say **"fail to reject H₀"**, not "accept H₀." Failing to reject doesn't mean H₀ is true — it just means we don't have enough evidence to rule it out. The distinction is subtle but important.

### The two types of error

|                   | H₀ is true          | H₀ is false         |
|-------------------|---------------------|---------------------|
| **Reject H₀**     | **Type I error** (α — false positive) | Correct decision (power = 1 − β) |
| **Fail to reject H₀** | Correct decision | **Type II error** (β — false negative) |

```
Type I error (α) — rejecting H₀ when it's actually true     (false alarm)
Type II error (β) — failing to reject H₀ when it's false   (missed detection)
```

There's a fundamental tension: reducing α typically increases β, and vice versa. You can't have both zero; you balance them by choosing α (e.g., 0.05) and getting enough sample size to keep β small.

**Simple example (coin fairness test):**
You suspect a coin is biased. Flip it 100 times and get 60 heads.
```
H₀ : p = 0.5           (coin is fair)
H₁ : p ≠ 0.5           (coin is biased)
α  = 0.05

Under H₀, X ∼ Binomial(100, 0.5). E[X] = 50, σ = √(100·0.25) = 5.
z = (60 − 50) / 5 = 2.0
p-value (two-sided) ≈ 0.046

0.046 < 0.05  →  reject H₀.  The coin appears biased.
```

**Complex example (sensor calibration A/B test):**
You want to prove that a new IMU firmware reduces drift vs the old one.
```
H₀ : μ_new = μ_old             (no improvement)
H₁ : μ_new <  μ_old             (firmware reduces drift — one-sided)

Collect drift measurements from each: n_new = 30, n_old = 30.
Compute t-statistic, get p-value = 0.02.

0.02 < 0.05  →  reject H₀  →  firmware change is statistically significant.
```
Practical caveats:
- A statistically significant improvement may still be **practically negligible** (small effect size). Always check magnitude.
- With huge sample sizes, almost anything becomes "significant" — statistical vs practical significance are different.
- Running many tests inflates false-positive rate (Bonferroni, FDR corrections).

---

## Applied in

| Concept | Used in |
|---------|---------|
| **Covariance / multivariate Gaussian** | Kalman filter, particle filter (probabilistic robotics) |
| **CLT** | [`05-ml-basics.md`](./05-ml-basics.md) (noise assumptions), Kalman filter, confidence intervals |
| **MLE** | [`05-ml-basics.md`](./05-ml-basics.md) (loss functions as MLE under Gaussian/Bernoulli noise) |
| **Hypothesis testing** | A/B testing, sensor anomaly detection, drug trials, model evaluation |
| **Quantiles / boxplot** | SLAM residual diagnostics, outlier detection |
| **Skewness / kurtosis** | Robust filtering, outlier rejection, non-Gaussian noise modeling |
