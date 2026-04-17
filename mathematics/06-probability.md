# Probability — From Events to Distributions

## 6.1 — Basic Probability

**What it does:**
Assigns a number between 0 and 1 to an event, measuring how likely it is. The foundation of all reasoning under uncertainty — from weighing dice rolls to training Bayesian robots.

Imagine a **bag with 10 balls**, 3 red and 7 blue. The probability of drawing a red ball is 3/10 = 0.3 — literally the fraction of favorable outcomes among all possible outcomes. Probability is "counting favorable cases divided by counting all cases," extended to situations where counting is more subtle (continuous outcomes, infinite sample spaces).

**Formula:**
```
P(A) = (number of favorable outcomes) / (total number of outcomes)
     = |A| / |Ω|
```
where `Ω` is the sample space (all possible outcomes) and `A ⊂ Ω` is the event.

**Properties (axioms of probability):**
```
0 ≤ P(A) ≤ 1
P(Ω) = 1           (something in the sample space will happen)
P(∅) = 0           (nothing impossible happens)
```

**Complement:**
```
P(A') = 1 − P(A)         — "not A"
```

**Simple example (dice):**
Roll a fair six-sided die. Sample space: `Ω = {1, 2, 3, 4, 5, 6}`.
```
P(rolling a 3)       = 1/6 ≈ 0.167
P(rolling an even)   = 3/6 = 0.5       (events {2, 4, 6})
P(not rolling a 6)   = 1 − 1/6 = 5/6   (complement)
```

**Complex example (robot sensor failure rate):**
A LiDAR has a failure rate of 0.003 per scan. Over a mission of 1000 scans, what's the probability the sensor never fails?
```
P(one scan OK)       = 1 − 0.003 = 0.997
P(1000 scans all OK) = 0.997^1000 ≈ 0.0497
```
→ Only ~5% chance the LiDAR survives 1000 scans without a single failure. Drives redundancy design (adding a second sensor).

---

## 6.2 — Union of Events (Disjoint and General)

**What it does:**
Computes `P(A ∪ B)` — the probability that **at least one** of two events happens. Two flavors depending on whether the events can happen together.

Imagine two **overlapping circles** on a Venn diagram. The union is the total area covered by either circle. If they don't overlap (disjoint), you just add the two areas. If they overlap, you add them but then **subtract the overlap once** — otherwise you'd count it twice.

### Disjoint (mutually exclusive) events

Two events are **disjoint** when they cannot both happen: `A ∩ B = ∅`.
```
P(A ∪ B) = P(A) + P(B)
```

### General case (inclusion-exclusion)

```
P(A ∪ B) = P(A) + P(B) − P(A ∩ B)
```

For three events:
```
P(A ∪ B ∪ C) = P(A) + P(B) + P(C)
             − P(A ∩ B) − P(B ∩ C) − P(A ∩ C)
             + P(A ∩ B ∩ C)
```

**Simple example (dice, disjoint):**
Roll one die. Events: `A = {rolling 1}`, `B = {rolling 6}`. Disjoint (can't roll both at once).
```
P(A ∪ B) = 1/6 + 1/6 = 2/6 ≈ 0.333
```

**Simple example (dice, general):**
Events: `A = {rolling even}`, `B = {rolling ≥ 4}`. Overlap `A ∩ B = {4, 6}`.
```
P(A)     = 3/6 = 0.5
P(B)     = 3/6 = 0.5
P(A ∩ B) = 2/6 ≈ 0.333
P(A ∪ B) = 0.5 + 0.5 − 0.333 = 0.667
```

**Complex example (multi-sensor obstacle detection):**
Two independent sensors flag an obstacle. Sensor A has `P(detect) = 0.9`, Sensor B has `P(detect) = 0.85`. Probability that **at least one** flags the obstacle:
```
P(A ∪ B) = 0.9 + 0.85 − 0.9 · 0.85   (assuming independence — §6.3)
        = 1.75 − 0.765
        = 0.985
```
→ Redundant detection reaches 98.5%, much better than either sensor alone. Foundation of sensor-fusion reliability.

---

## 6.3 — Independence and Dependence

**What it does:**
Characterizes whether one event **influences** another. If independent, knowing A happened tells you nothing about B. If dependent, it does.

Imagine two **coin flips** vs drawing two cards without replacement. Coin flips are **independent** — the second flip has no memory of the first. Drawing cards is **dependent** — drawing the ace of spades first changes what's in the deck for the second draw. Independence is about whether events have "memory" of each other.

### Independent events

```
P(A ∩ B) = P(A) · P(B)       — both happen: probabilities multiply
```

Equivalently:
```
P(A | B) = P(A)              — B happening doesn't change A's probability
```

### Dependent events

```
P(A ∩ B) = P(A) · P(B | A)   — chain rule of probability
```

### Conditional probability (the general definition)

```
P(A | B) = P(A ∩ B) / P(B)     ,  provided P(B) > 0
```
Read as: "given that B happened, what's the probability A also happened?" Narrows the sample space to "cases where B is true."

**Simple example (independent — two dice):**
```
P(first = 3  AND  second = 5) = (1/6) · (1/6) = 1/36 ≈ 0.028
```

**Simple example (dependent — cards):**
From a 52-card deck, draw two cards without replacement. Probability both are aces:
```
P(first ace)              = 4/52
P(second ace | first ace) = 3/51        (only 3 aces left among 51 cards)
P(both aces)              = (4/52)·(3/51) ≈ 0.0045
```

**Complex example (rover pose and GPS):**
- `A` = "rover is within 2 m of target"
- `B` = "GPS signal is strong"

If GPS is strong, the rover's self-reported position is reliable → knowing `B` changes `P(A)`. These events are **dependent**. Conditional probabilities like `P(A | B)` vs `P(A | B')` drive whether the robot trusts its GPS or switches to wheel-odometry.

```
┌──────────────────────────────────────────────────────────────────┐
│  Independent  →  P(A ∩ B) = P(A) · P(B)                           │
│  Dependent    →  P(A ∩ B) = P(A) · P(B | A)                       │
│                                                                   │
│  Same formula — independence just means P(B | A) = P(B).          │
└──────────────────────────────────────────────────────────────────┘
```

---

## 6.4 — Bayes' Theorem

**What it does:**
Reverses the direction of conditioning: given `P(B | A)`, compute `P(A | B)`. This is the **single most important formula in probabilistic robotics, medical diagnostics, machine learning, and reasoning under uncertainty**.

Imagine a **detective**. She knows `P(fingerprint pattern | suspect)` — how likely each suspect leaves that pattern. But what she wants is `P(suspect | fingerprint pattern)` — given the pattern found, which suspect is it? Bayes' theorem inverts forward reasoning ("given a cause, what are the clues?") into reverse reasoning ("given the clues, what's the cause?").

**Formula:**
```
            P(B | A) · P(A)
P(A | B) = ─────────────────
                P(B)
```

Semantic names:
```
P(A | B)  =  posterior   (what we want to know — A given the evidence B)
P(B | A)  =  likelihood  (how well A explains B)
P(A)      =  prior       (what we believed about A before seeing B)
P(B)      =  evidence    (normalization — how probable B is overall)
```

### Law of total probability (to compute P(B))

If events `A₁, A₂, …, Aₙ` partition the sample space:
```
P(B) = Σᵢ P(B | Aᵢ) · P(Aᵢ)
```

**Simple example (classic medical test):**
A disease affects 1% of the population. A test is 99% accurate (99% sensitivity, 99% specificity). You test positive. What's the probability you actually have the disease?

```
Let A = "has disease",  B = "tests positive"

P(A)          = 0.01         (prior — disease prevalence)
P(B | A)      = 0.99         (likelihood — test sensitivity)
P(B | A')     = 0.01         (false-positive rate)
P(A')         = 0.99

P(B) = P(B | A)·P(A) + P(B | A')·P(A')
     = 0.99·0.01 + 0.01·0.99
     = 0.0099 + 0.0099
     = 0.0198

P(A | B) = (0.99 · 0.01) / 0.0198  ≈  0.5
```
→ Only **50%** probability you actually have the disease, despite a "99% accurate" test. The prior (`0.01`) dominates — this is the counterintuitive result that prevents reasoning errors in diagnostics. Doctors who don't grasp Bayes overestimate positive test meaning.

**Complex example (robot localization — the Bayes filter):**
A rover's position `x` is the hypothesis. A LiDAR scan `z` is the evidence.
```
P(x | z) = P(z | x) · P(x) / P(z)

P(x)    = prior belief about location (e.g., from last timestep)
P(z | x)= observation model (how likely this scan is, given we're at x)
P(x | z)= updated (posterior) belief
```
Applied iteratively every sensor tick, this is the **Bayes filter** — the mathematical skeleton of Kalman filters, particle filters, and SLAM. Each new measurement refines the belief via Bayes' rule.

---

## 6.5 — Bernoulli Distribution

**What it does:**
Models **one experiment with two possible outcomes**: success (1) or failure (0), with probability `p` of success. The simplest non-trivial probability distribution — the atom from which binomial, geometric, and more complex distributions are built.

Imagine flipping a **single biased coin** once. The coin has probability `p` of landing heads and `1 − p` of landing tails. Bernoulli codifies this single flip.

**Formula (PMF):**
```
P(X = k) = p^k · (1 − p)^(1 − k)       for k ∈ {0, 1}
```
Equivalently:
```
P(X = 1) = p       (success)
P(X = 0) = 1 − p   (failure)
```

**Moments:**
```
E[X]    = p
Var(X)  = p · (1 − p)
```

**Simple example (biased coin, `p = 0.7`):**
```
P(X = 1) = 0.7^1 · 0.3^0 = 0.7
P(X = 0) = 0.7^0 · 0.3^1 = 0.3
E[X]     = 0.7
Var(X)   = 0.7 · 0.3 = 0.21
```

**Complex example (single sensor reading):**
A contact sensor on a gripper returns 1 if an object is detected, 0 otherwise. For a given approach trajectory, contact occurs with probability `p = 0.92`. Modeling contact as a Bernoulli trial lets us:
```
- Compute expected success rate of grasping.
- Plan retry policies (if failure, try again).
- Stack Bernoulli trials into Binomial (§6.6) for the full pick-attempt statistics.
```

---

## 6.6 — Binomial Distribution

**What it does:**
Counts the number of successes in `n` **independent** Bernoulli trials with the same success probability `p`. Extends one-coin flips into "how many heads in 10 flips?"

Imagine flipping the same **biased coin 10 times** and counting heads. The outcome can be `0, 1, 2, …, 10`. Each count has its own probability — the Binomial distribution gives you the whole probability table. It's just Bernoulli, multiplied `n` times.

**Formula (PMF):**
```
P(X = k) = C(n, k) · p^k · (1 − p)^(n − k)

where  C(n, k) = n! / (k! · (n − k)!)    is the binomial coefficient
```

Semantic breakdown:
- `p^k` → probability of `k` specific successes
- `(1 − p)^(n − k)` → probability of the remaining `n − k` specific failures
- `C(n, k)` → number of ways to choose which `k` of the `n` trials are the successes

**Moments:**
```
E[X]    = n · p
Var(X)  = n · p · (1 − p)
```

**Simple example (10 coin flips, `p = 0.5`):**
Probability of exactly 4 heads:
```
P(X = 4) = C(10, 4) · 0.5^4 · 0.5^6
         = 210 · 0.0625 · 0.015625
         ≈ 0.205  ≈ 20.5%
```

**Complex example (drone package-delivery reliability):**
A delivery drone has `p = 0.95` success rate per package. Over a route of 20 packages, what's the probability of **≥ 18 successes**?
```
P(X ≥ 18) = P(X = 18) + P(X = 19) + P(X = 20)

P(X = 18) = C(20, 18) · 0.95^18 · 0.05^2  ≈ 190 · 0.3972 · 0.0025 ≈ 0.189
P(X = 19) = C(20, 19) · 0.95^19 · 0.05^1  ≈  20 · 0.3774 · 0.05   ≈ 0.377
P(X = 20) = C(20, 20) · 0.95^20 · 0.05^0  =   1 · 0.3585 · 1      ≈ 0.358

P(X ≥ 18) ≈ 0.189 + 0.377 + 0.358 ≈ 0.925
```
→ 92.5% chance of 18+ successful deliveries. Quantifies reliability for SLA guarantees.

---

## 6.7 — PMF, PDF, CDF

**What it does:**
Three complementary ways to describe a probability distribution. PMF / PDF give the **density** at a single point (or interval); CDF gives the **cumulative** probability up to a point.

Imagine a **histogram of exam grades**. The bar heights correspond to the PMF (discrete) or PDF (continuous) — density at each value. The running total of the bars from left to right is the CDF — "how many students scored below this threshold." CDF is always non-decreasing from 0 to 1; PDF is the **derivative** of the CDF.

### Discrete vs continuous

|                  | Discrete                     | Continuous                              |
|------------------|------------------------------|-----------------------------------------|
| Sample space     | Countable (e.g., integers)   | Uncountable (e.g., reals)               |
| Density notation | PMF `f(x) = P(X = x)`        | PDF `f(x)`, with `P(a ≤ X ≤ b) = ∫ₐᵇ f` |
| Total density    | `Σₓ f(x) = 1`                | `∫ f(x) dx = 1`                         |
| `P(X = x)`       | Can be > 0                   | **Always 0** (single points have no area)|

### Cumulative Distribution Function (CDF)

Same definition for both cases:
```
F(x) = P(X ≤ x)
```
Discrete:
```
F(x) = Σ_{k ≤ x} P(X = k)
```
Continuous:
```
F(x) = ∫_{−∞}^{x} f(t) dt           and    f(x) = F'(x)
```

**Properties of CDF:**
```
F(−∞) = 0,   F(+∞) = 1,   F is non-decreasing, right-continuous.
```

**Simple example (PMF of a die):**
`X = result of a fair die roll`:
```
f(1) = f(2) = ... = f(6) = 1/6       (PMF)
F(3) = P(X ≤ 3) = 3/6 = 0.5          (CDF at 3)
F(5) = 5/6 ≈ 0.833
F(6) = 1
```

**Simple example (PDF — uniform on [0, 1]):**
```
f(x) = 1    for 0 ≤ x ≤ 1;   else 0
F(x) = x    for 0 ≤ x ≤ 1;   F(x) = 0 for x < 0;   F(x) = 1 for x > 1
P(X = 0.3)     = 0        (point probability in continuous case)
P(0.2 ≤ X ≤ 0.5) = 0.3    (integrate density over the interval)
```

**Complex example (wait time between buses):**
Bus arrival intervals follow an exponential distribution with rate `λ = 1/10` (mean 10 min):
```
f(t) = λ · e^(−λt)                  for t ≥ 0
F(t) = 1 − e^(−λt)
```
Probability of waiting more than 15 min:
```
P(T > 15) = 1 − F(15) = e^(−15/10) = e^(−1.5) ≈ 0.223
```
→ 22% chance of a wait longer than 15 minutes. Used in queueing theory, reliability engineering, and sensor-event timing.

---

## 6.8 — The Gaussian (Normal) Distribution

**What it does:**
The most important continuous distribution in probability and statistics. Describes any quantity that arises as the **sum of many small independent random contributions** (thanks to the Central Limit Theorem — see [`07-statistics.md`](./07-statistics.md)). Foundation of linear regression noise, Kalman filters, neural network initialization, and countless natural phenomena (heights, measurement errors, thermal noise).

Imagine a **plinko machine** at a carnival: drop a ball into a pegboard, it bounces left or right at each peg, and finally lands in a bin at the bottom. The distribution of landing positions approximates a bell curve — most balls pile up in the middle, few reach the extreme bins. The Gaussian is the continuous, mathematical version of this phenomenon.

**Formula (PDF — univariate):**
```
                 1
f(x)  =  ─────────────  ·  exp( −½ · ((x − μ)/σ)² )
          σ · √(2π)
```

| Parameter | Meaning |
|-----------|---------|
| `μ` | Mean — center of the bell |
| `σ` | Standard deviation — spread (width) |
| `(x − μ)/σ` | **Z-score** — signed distance from mean in σ-units |

**Key properties:**
```
E[X]       = μ
Var(X)     = σ²
Mode       = μ  (peak is at the mean)
Symmetric about μ
```

### Empirical rule (68-95-99.7)

For a Gaussian, the probability of falling within `k · σ` of the mean:
```
P(|X − μ| ≤ 1σ) ≈ 68.2%
P(|X − μ| ≤ 2σ) ≈ 95.4%
P(|X − μ| ≤ 3σ) ≈ 99.7%
```

**Simple example (IQ scores):**
IQ is designed to be Gaussian with `μ = 100, σ = 15`.
```
P(85 ≤ IQ ≤ 115)  ≈ 68%      (within 1σ)
P(IQ > 130)       ≈ 2.3%     (more than 2σ above)
P(IQ > 145)       ≈ 0.13%    (more than 3σ above — "genius" threshold)
```

**Complex example (GPS noise model in sensor fusion):**
A GPS receiver reports position with Gaussian noise of `σ = 2.5 m` on each axis. To use GPS in a Kalman filter, we model:
```
z = x_true + w,    w ∼ N(0, σ² · I)
```
The noise covariance matrix `R = diag(σ_x², σ_y², σ_z²)` feeds directly into the Kalman update step. 95% of measurements fall within ~5 m of truth (2σ); the filter weights them accordingly.

```
┌──────────────────────────────────────────────────────────────────┐
│  Why the Gaussian is everywhere:                                  │
│                                                                   │
│  Central Limit Theorem — sums of many independent random          │
│  variables tend toward Gaussian, regardless of their individual   │
│  distributions. See 07-statistics.md §7.9.                        │
│                                                                   │
│  Result: measurement noise, biological traits, financial          │
│  returns, and countless other quantities are approximately        │
│  Gaussian. It's the "default" distribution when you don't         │
│  know better — and often it's enough.                             │
└──────────────────────────────────────────────────────────────────┘
```

Multivariate Gaussians (2D, nD, with correlations) live in [`07-statistics.md`](./07-statistics.md) §7.7.

---

## Applied in

| Concept | Used in |
|---------|---------|
| **Bayes' theorem** | Bayes filter, Kalman filter, particle filter, SLAM, classification |
| **Gaussian noise** | [ROS2 — DiffDrive Controller](../ros2/11-controllers-diffdrive.md) (odometry covariance), sensor fusion |
| **Independence** | [`02-algebra-solving.md`](./02-algebra-solving.md) §2.4 (variance of sums) |
| **Binomial** | Reliability engineering, A/B testing, hypothesis testing |
| **PDF / CDF** | Monte Carlo simulation, rejection sampling, probabilistic planning |
