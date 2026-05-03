# Mathematical Notation — Atlas

A bilingual (FR / EN) reference for mathematical symbols encountered in the wiki and review sessions. Used as a **lookup**, not a drill.

**Role:**

- A passive reference grown alongside reading (Mathematics for Machine Learning, …) and articulation drills (see [`CLAUDE.md`](../CLAUDE.md) §7).
- Each new symbol encountered in a chapter or drill is added here with FR + EN reading.
- Used as a lookup, not a flashcard deck. Familiarisation comes from repeated exposure in real exercises.

**Conventions:**

- One row per symbol
- "Premier vu dans" tracks the first lesson / chapter where the symbol appeared in this curriculum
- Symbols are grouped by category, not by order of appearance — easier lookup

---

## 1. Lettres grecques

| Symbole | Nom               | Lecture FR | Lecture EN        | Usage typique                  | Premier vu dans              |
| ------- | ----------------- | ---------- | ----------------- | ------------------------------ | ---------------------------- |
| α       | alpha (minuscule) | "alpha"    | "alpha"           | angle, paramètre, taux         | —                            |
| β       | bêta              | "bêta"     | "beta"            | angle, coefficient             | —                            |
| γ       | gamma             | "gamma"    | "gamma"           | angle, fonction Γ              | —                            |
| δ       | delta (min.)      | "delta"    | "delta"           | petite variation               | —                            |
| Δ       | Delta (maj.)      | "Delta"    | "Delta"           | grande variation, discriminant | Module 1, discriminant       |
| ε       | epsilon           | "epsilon"  | "epsilon"         | petit nombre, erreur           | —                            |
| θ       | thêta             | "thêta"    | "theta"           | angle                          | —                            |
| λ       | lambda            | "lambda"   | "lambda"          | valeur propre, taux            | Module 1, valeur propre      |
| μ       | mu                | "mu"       | "mu"              | moyenne, paramètre             | Module 3 (futur), moyenne    |
| π       | pi                | "pi"       | "pi"              | constante 3.14159…             | —                            |
| Π       | Pi (maj.)         | "Pi"       | "Pi (capital)"    | produit                        | —                            |
| σ       | sigma (min.)      | "sigma"    | "sigma"           | écart-type                     | Module 3 (futur), écart-type |
| Σ       | Sigma (maj.)      | "Sigma"    | "Sigma (capital)" | somme                          | —                            |
| τ       | tau               | "tau"      | "tau"             | temps caractéristique          | —                            |
| φ       | phi               | "phi"      | "phi"             | angle, phase                   | —                            |
| ψ       | psi               | "psi"      | "psi"             | fonction, état                 | —                            |
| ω       | oméga (min.)      | "oméga"    | "omega"           | pulsation, vitesse angulaire   | ROS2 (DiffDrive `ω`)         |
| Ω       | Oméga (maj.)      | "Oméga"    | "Omega (capital)" | univers (probas)               | Module 3 (futur)             |

---

## 2. Ensembles et appartenance

| Symbole | Lecture FR              | Lecture EN              | Sens                                  | Premier vu dans |
| ------- | ----------------------- | ----------------------- | ------------------------------------- | --------------- |
| ℕ       | "N"                     | "the natural numbers"   | entiers naturels {0, 1, 2, …}         | —               |
| ℤ       | "Z"                     | "the integers"          | entiers relatifs                      | —               |
| ℚ       | "Q"                     | "the rationals"         | rationnels                            | —               |
| ℝ       | "R"                     | "the reals"             | réels                                 | Module 1        |
| ℂ       | "C"                     | "the complex numbers"   | complexes                             | —               |
| ∅       | "ensemble vide"         | "the empty set"         | ensemble vide                         | —               |
| ∈       | "appartient à" / "dans" | "in" / "belongs to"     | `x ∈ A` : x est dans A                | Module 1        |
| ∉       | "n'appartient pas à"    | "not in"                | x n'est pas dans A                    | —               |
| ⊂       | "inclus dans" (strict)  | "is a strict subset of" | A est strictement inclus dans B       | —               |
| ⊆       | "inclus dans" (large)   | "is a subset of"        | A inclus ou égal à B                  | —               |
| ∪       | "union"                 | "union"                 | A ∪ B = "tout ce qui est dans A ou B" | —               |
| ∩       | "intersection"          | "intersection"          | A ∩ B = "ce qui est dans A et B"      | —               |
| ×       | "produit cartésien"     | "cartesian product"     | ℝ × ℝ = ℝ²                            | —               |

---

## 3. Quantificateurs et logique

| Symbole | Lecture FR                          | Lecture EN               | Sens               |
| ------- | ----------------------------------- | ------------------------ | ------------------ |
| ∀       | "pour tout"                         | "for all" / "for every"  | `∀x ∈ ℝ, …`        |
| ∃       | "il existe"                         | "there exists"           | `∃x ∈ ℝ tel que …` |
| ∃!      | "il existe un unique"               | "there exists a unique"  | unicité            |
| ⇒       | "implique"                          | "implies"                | A ⇒ B              |
| ⇔       | "équivaut à" / "si et seulement si" | "if and only if" / "iff" | A ⇔ B              |
| ∧       | "et"                                | "and"                    | logique            |
| ∨       | "ou"                                | "or"                     | logique            |
| ¬       | "non"                               | "not"                    | négation           |
| ≠       | "différent de"                      | "not equal to"           | —                  |
| :=      | "est défini par"                    | "is defined as"          | définition         |

---

## 4. Comparaison et ordre

| Symbole | Lecture FR                 | Lecture EN                 | Sens              |
| ------- | -------------------------- | -------------------------- | ----------------- |
| ≤       | "inférieur ou égal à"      | "less than or equal to"    | —                 |
| ≥       | "supérieur ou égal à"      | "greater than or equal to" | —                 |
| ≪       | "très petit devant"        | "much less than"           | ordre de grandeur |
| ≫       | "très grand devant"        | "much greater than"        | ordre de grandeur |
| ≈       | "approximativement égal à" | "approximately equal to"   | —                 |
| ∝       | "proportionnel à"          | "proportional to"          | y ∝ x             |
| ∞       | "infini"                   | "infinity"                 | —                 |

---

## 5. Opérations

| Symbole | Lecture FR         | Lecture EN            | Sens             |
| ------- | ------------------ | --------------------- | ---------------- |
| Σ       | "somme de … à …"   | "sum from … to …"     | `Σ_{i=1}^{n} aᵢ` |
| Π       | "produit de … à …" | "product from … to …" | `Π_{i=1}^{n} aᵢ` |
| √       | "racine carrée de" | "square root of"      | √x               |
| ⁿ√      | "racine nième de"  | "nth root of"         | ³√x              |
| ±       | "plus ou moins"    | "plus or minus"       | discriminant     |
| !       | "factorielle"      | "factorial"           | n!               |

---

## 6. Algèbre linéaire

| Symbole  | Lecture FR                    | Lecture EN            | Sens                     | Premier vu dans |
| -------- | ----------------------------- | --------------------- | ------------------------ | --------------- |
| `‖x‖`    | "norme de x"                  | "norm of x"           | longueur d'un vecteur    | Module 1 §1.1   |
| `u·v`    | "u scalaire v" / "u dot v"    | "u dot v"             | produit scalaire         | Module 1 §1.2   |
| `u × v`  | "u vectoriel v" / "u cross v" | "u cross v"           | produit vectoriel (3D)   | —               |
| `Aᵀ`     | "A transposée"                | "A transpose"         | transposée d'une matrice | Module 1 §1.3   |
| `A⁻¹`    | "A inverse"                   | "A inverse"           | matrice inverse          | Module 1 §1.6   |
| `det(A)` | "déterminant de A"            | "determinant of A"    | scalaire associé         | Module 1 §1.5   |
| `tr(A)`  | "trace de A"                  | "trace of A"          | somme des diagonaux      | —               |
| `rg(A)`  | "rang de A"                   | "rank of A"           | dimension de l'image     | —               |
| `Iₙ`     | "matrice identité n × n"      | "n×n identity matrix" | matrice identité         | Module 1        |

---

## 7. Calcul différentiel

| Symbole  | Lecture FR                               | Lecture EN                  | Sens                       | Premier vu dans  |
| -------- | ---------------------------------------- | --------------------------- | -------------------------- | ---------------- |
| `f'(x)`  | "f prime de x"                           | "f prime of x"              | dérivée première           | Module 2 §3.1    |
| `f''(x)` | "f seconde de x"                         | "f double prime of x"       | dérivée seconde            | Module 2 §3.9    |
| `df/dx`  | "dé f sur dé x"                          | "d f d x"                   | dérivée (notation Leibniz) | Module 2         |
| `∂f/∂x`  | "dérivée partielle de f par rapport à x" | "partial f over partial x"  | dérivée partielle          | Module 2 §3.8    |
| `∇f`     | "nabla f" / "gradient de f"              | "nabla f" / "gradient of f" | vecteur gradient           | Module 4 (futur) |
| `Δx`     | "delta x"                                | "delta x"                   | variation finie            | Module 2         |
| `lim`    | "limite"                                 | "limit"                     | `lim_{x→a} f(x)`           | Module 2         |
| `→`      | "tend vers"                              | "tends to" / "approaches"   | `x → 0`                    | Module 2         |
| `∫`      | "intégrale de"                           | "integral of"               | `∫ f(x) dx`                | —                |

---

## 8. Probabilités et statistiques (à venir — Module 3)

| Symbole     | Lecture FR                                    | Lecture EN                                        | Sens                 | Premier vu dans  |
| ----------- | --------------------------------------------- | ------------------------------------------------- | -------------------- | ---------------- |
| `P(A)`      | "probabilité de A"                            | "probability of A"                                | proba d'un événement | Module 3 (futur) |
| `P(A \| B)` | "probabilité de A sachant B"                  | "probability of A given B"                        | conditionnelle       | Module 3 (futur) |
| `𝔼[X]`      | "espérance de X"                              | "expected value of X"                             | moyenne théorique    | Module 3 §7.1    |
| `Var(X)`    | "variance de X"                               | "variance of X"                                   | dispersion           | Module 3 §7.2    |
| `σ(X)`      | "écart-type de X"                             | "standard deviation of X"                         | √Var(X)              | Module 3 §7.2    |
| `Cov(X,Y)`  | "covariance de X et Y"                        | "covariance of X and Y"                           | corrélation linéaire | Module 3 §7.6    |
| `𝒩(μ, σ²)`  | "loi normale de paramètres mu et sigma carré" | "normal distribution with mean μ and variance σ²" | gaussienne           | Module 3 §7.8    |
| `H₀, H₁`    | "H zéro, H un"                                | "H naught, H one"                                 | hypothèses test      | Module 3 §7.12   |

---

## 9. Robotique / spatial (3D et transformations)

| Symbole          | Lecture FR     | Lecture EN     | Sens                                  | Premier vu dans  |
| ---------------- | -------------- | -------------- | ------------------------------------- | ---------------- |
| `SO(3)`          | "S O trois"    | "S O three"    | groupe des rotations 3D               | Module 5 (futur) |
| `SE(3)`          | "S E trois"    | "S E three"    | groupe des transformations rigides 3D | Module 5 (futur) |
| `q` (quaternion) | "quaternion q" | "quaternion q" | rotation sans gimbal lock             | Module 5 (futur) |

---

_This atlas grows as new symbols are introduced. Add a row whenever a fresh symbol appears in a lesson or exercise. Never duplicate — check first._
