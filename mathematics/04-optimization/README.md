# 04 — Optimization

Each page covers **one concept**.

| § | Page | What it does |
|---|------|--------------|
| 4.1 | [Gradient Descent](./4.1-gradient-descent.md) | Walk downhill on the loss landscape, one negative-gradient step at a time. |
| 4.2 | [Newton's Method (1D)](./4.2-newton-1d.md) | Use slope and curvature to jump straight at the minimum — quadratic convergence. |
| 4.3 | [Newton's Method (Multivariate)](./4.3-newton-multivariate.md) | Same trick with the Hessian — costly but fast when it converges. |
| 4.4 | [Classifying Stationary Points](./4.4-stationary-points.md) | Read the Hessian's eigenvalues to label min / max / saddle. |

## Applied in

| Concept | Used in |
|---------|---------|
| **Gradient descent** | [`05-ml-basics.md`](./05-ml-basics.md) — training linear regression and classification |
| **Newton's method** | [`ros2/20-inverse-kinematics.md`](../../ros2/moveit/20-inverse-kinematics.md) — numerical IK solver |
| **Hessian for extrema** | [`ros2/19-motion-planning.md`](../../ros2/moveit/19-motion-planning.md) — path-cost optimization (future work) |
| **Saddle avoidance** | Future `dl/` domain — loss landscape of neural networks, second-order methods (K-FAC, L-BFGS) |
