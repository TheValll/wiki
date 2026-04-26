# 03 — Derivatives

Each page covers **one concept**. Course content (formula, examples) and intuition merged in the same file.

| § | Page | In one line |
|---|------|-------------|
| 3.1 | [Lagrange vs Leibniz](./3.1-lagrange-vs-leibniz.md) | Two gauges on the same dashboard — speedometer vs distance-over-time. |
| 3.2 | [Common Derivatives](./3.2-common-derivatives.md) | Why each entry of the toolbox has the shape it does. |
| 3.3 | [Scalar Rule](./3.3-scalar-rule.md) | A constant is a playback-speed knob. |
| 3.4 | [Sum Rule](./3.4-sum-rule.md) | Parallel treadmills — no cross-talk. |
| 3.5 | [Product Rule](./3.5-product-rule.md) | The growing rectangle — two strips plus a negligible corner. |
| 3.6 | [Chain Rule](./3.6-chain-rule.md) | Meshed gears — chained rates multiply. |
| 3.7 | [Non-Differentiable Points](./3.7-non-differentiable.md) | Where the car's heading breaks down (corner or jump). |
| 3.8 | [Partial Derivatives](./3.8-partial-derivatives.md) | One-axis slope on a multi-axis landscape. |
| 3.9 | [Gradient & Hessian](./3.9-gradient-hessian.md) | A compass and a curvature sensor. |

## Applied in

| Concept | Used in |
|---------|---------|
| **First derivative as velocity** | [ROS2 — Trajectory Generation](../../ros2/moveit/21-trajectory-generation.md) — joint velocity `q̇(t) = dq/dt` |
| **Second derivative as acceleration** | [ROS2 — Trajectory Generation](../../ros2/moveit/21-trajectory-generation.md) — joint acceleration `q̈(t) = d²q/dt²` |
| **Continuity classes (C¹, C², C⁴)** | [ROS2 — Trajectory Generation](../../ros2/moveit/21-trajectory-generation.md) — cubic splines are C² (smooth velocity), quintic splines are C⁴ (smooth acceleration) |
| **Derivative as linear approximation** | [ROS2 — Inverse Kinematics](../../ros2/moveit/20-inverse-kinematics.md) — the Jacobian is the derivative of forward kinematics: `ẋ = J · q̇` |
| **Gradients for optimization** | Future `ml/` and `dl/` domains — gradient descent, backpropagation |
