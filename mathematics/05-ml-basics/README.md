# 05 — Machine Learning Basics

Each page covers **one concept**.

| § | Page | What it does |
|---|------|--------------|
| 5.1 | [Min-Max Normalization](./5.1-min-max-normalization.md) | Rescale features into [0, 1] using observed min/max. |
| 5.2 | [Standardization (Z-score)](./5.2-standardization.md) | Center on the mean, scale by standard deviation. |
| 5.3 | [Linear Regression](./5.3-linear-regression.md) | Fit a line/hyperplane that minimizes squared error. |
| 5.4 | [RMSE](./5.4-rmse.md) | Average prediction error in the original units. |
| 5.5 | [Sigmoid Function](./5.5-sigmoid.md) | Squash any real number into a probability in (0, 1). |
| 5.6 | [Binary Cross-Entropy](./5.6-binary-cross-entropy.md) | Loss that penalizes confident wrong predictions much more than uncertain ones. |
| 5.7 | [Classification Metrics](./5.7-classification-metrics.md) | Accuracy, precision, recall, F1 — when to trust each. |

## Applied in

| Concept | Used in |
|---------|---------|
| **Linear regression** | Curve fitting, battery models, sensor calibration |
| **Sigmoid + cross-entropy** | Binary classifiers, logistic regression, neural network output layers |
| **RMSE** | [ROS2 — Inverse Kinematics](../../ros2/moveit/20-inverse-kinematics.md) (residual error), Kalman filter tuning, GPS accuracy |
| **Normalization** | Neural network preprocessing, sensor fusion |
| **Classification metrics** | Obstacle detection, terrain classification, anomaly detection |
| **Gradient descent** | [`04-optimization.md`](./04-optimization.md) — mechanics of training |
