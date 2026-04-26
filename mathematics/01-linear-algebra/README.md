# 01 — Linear Algebra (Vectors & Matrices)

Each page covers **one concept**. Course content (formula, examples) and intuition (physical image, step-by-step decomposition) are merged in the same file.

| § | Page | In one line |
|---|------|-------------|
| 1.1 | [Vector Norm](./1.1-norm.md) | Pythagoras cascaded across perpendicular axes. |
| 1.2 | [Dot Product](./1.2-dot-product.md) | Cooperation score — signed shadow of one arrow onto another. |
| 1.3 | [Matrix × Vector](./1.3-matrix-vector.md) | Recipe for mixing the matrix's columns. |
| 1.4 | [Matrix × Matrix](./1.4-matrix-matrix.md) | Two machines in series, pre-collapsed into one. |
| 1.5 | [Matrix Inverse](./1.5-inverse.md) | The un-machine — exists iff no dimension was destroyed. |
| 1.6 | [Determinant](./1.6-determinant.md) | Signed volume factor of a machine. |
| 1.7 | [det(AB) = det(A)·det(B)](./1.7-det-product.md) | Stretches in series compound by multiplication. |
| 1.8 | [det(A⁻¹) = 1/det(A)](./1.8-det-inverse.md) | To undo a stretch, divide by it. |
| 1.9 | [Characteristic Equation](./1.9-characteristic-equation.md) | Finding the machine's direction-stable axes. |

## Applied in

| Concept | Used in |
|---------|---------|
| **Euclidean norm / dot product** | [ROS2 — Configuration Space](../../ros2/moveit/18-configuration-space.md) (weighted distance `d = √(Σ wᵢ(qᵢ−q'ᵢ)²)`) |
| **Matrix × matrix, matrix × vector** | [ROS2 — Inverse Kinematics](../../ros2/moveit/20-inverse-kinematics.md) (DH transforms, forward kinematics as a chain of 4×4 matrices) |
| **Matrix inverse, linear systems** | [ROS2 — Inverse Kinematics](../../ros2/moveit/20-inverse-kinematics.md) (Jacobian, pseudoinverse `J⁺ = Jᵀ(JJᵀ)⁻¹`, damped least squares) |
| **2×2 linear systems** | [ROS2 — DiffDrive Controller](../../ros2/ros2-control/11-controllers-diffdrive.md) (wheel velocity equations from `cmd_vel`) |
| **Distance metrics** | [ROS2 — Motion Planning](../../ros2/moveit/19-motion-planning.md) (nearest-neighbor lookups in RRT / PRM) |
| **Eigenvalues / eigenvectors** | Physics (principal moments of inertia), and future `ml/` domain (PCA, covariance decomposition) |
