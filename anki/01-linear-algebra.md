TARGET DECK: Mathematics::01-linear-algebra

## 1.1.a — Vector norm — definition

> Source: [1.1 — Vector Norm (Euclidean)](../mathematics/01-linear-algebra/1.1-norm.md)

START
Basic
Front: What is the Euclidean norm of a vector?
Back: The length (magnitude) of a vector — reduces a directional quantity to a single scalar.
<!--ID: 1777974617930-->
END

START
Basic
Front: What concept measures the length (magnitude) of a vector and reduces a directional quantity to a single scalar?
Back: The Euclidean norm.
<!--ID: 1777974617932-->
END

## 1.1.b — Vector norm — formula

> Source: [1.1 — Vector Norm (Euclidean)](../mathematics/01-linear-algebra/1.1-norm.md)

START
Basic
Front: What is the formula of the Euclidean norm in $\mathbb{R}^n$?
Back: $\|v\| = \sqrt{v_1^2 + v_2^2 + \cdots + v_n^2}$ — Pythagoras cascaded across $n$ perpendicular axes.
<!--ID: 1777974617934-->
END

START
Basic
Front: $\|v\| = \sqrt{v_1^2 + v_2^2 + \cdots + v_n^2}$ — what concept does this formula represent?
Back: The Euclidean norm in $\mathbb{R}^n$.
<!--ID: 1777974617935-->
END

## 1.2.a — Dot product — definition

> Source: [1.2 — Dot Product](../mathematics/01-linear-algebra/1.2-dot-product.md)

START
Basic
Front: What is the dot product of two vectors, and what does it measure?
Back: A scalar that measures how much two vectors point in the same direction — a cooperation score between two arrows.
```
    a              ↑ a               a
   ╱               │                  ╲
  ╱                │                   ╲
 •──────→ b        •──────→ b           •──────→ b
   dot > 0           dot = 0              dot < 0
   (acute)           (perp.)              (obtuse)
```
| Case | Parallel piece | Sign |
|---|---|---|
| Acute (< 90°) | positive length, along b | **+** |
| Perpendicular (= 90°) | zero length | **0** |
| Obtuse (> 90°) | negative length, opposite to b | **−** |
<!--ID: 1777974617937-->
END

START
Basic
Front: Which vector operation returns a scalar that measures how much two vectors point in the same direction — a cooperation score between two arrows?
Back: The dot product.
<!--ID: 1777974617938-->
END

## 1.2.b — Dot product — formula

> Source: [1.2 — Dot Product](../mathematics/01-linear-algebra/1.2-dot-product.md)

START
Basic
Front: What are the two formulas of the dot product of $a, b \in \mathbb{R}^n$?
Back: $a \cdot b = a_1 b_1 + a_2 b_2 + \cdots + a_n b_n = \|a\| \cdot \|b\| \cdot \cos(\theta)$ — coordinate form (component-by-component) and geometric form (lengths × cosine of the angle).
<!--ID: 1777974617940-->
END

START
Basic
Front: $a \cdot b = a_1 b_1 + a_2 b_2 + \cdots + a_n b_n = \|a\| \|b\| \cos(\theta)$ — what concept does this formula represent?
Back: The dot product of two vectors in $\mathbb{R}^n$.
<!--ID: 1777974617942-->
END
