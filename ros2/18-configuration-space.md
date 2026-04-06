# Part 18 — Configuration Space: Where the Robot Lives

## 18.1 — The Analogy

Imagine you're parking a car. You don't think about every point on the car's body — you think about three numbers: **x position, y position, and heading angle**. These three numbers fully describe the car's state. The set of all possible (x, y, theta) values is the car's **configuration space**.

For a robot arm, the configuration is the vector of all joint angles. A 6-DOF arm lives in a **6-dimensional space** where each axis is a joint angle.

---

## 18.2 — Configuration Space (C-Space)

### Definition

The **configuration** of a robot is the minimal set of numbers that fully describes the position of every point on the robot. For a serial manipulator with n revolute joints:

```
q = [q1, q2, q3, ..., qn] ∈ ℝⁿ

Where:
  q1 = joint1 angle (radians)
  q2 = joint2 angle (radians)
  ...
  qn = jointn angle (radians)
```

The **configuration space** (C-space) is the set of all possible configurations:

```
C = { q ∈ ℝⁿ | q_i_min ≤ q_i ≤ q_i_max for all i }
```

For the 6-DOF arm in the repo:

```
C ⊂ ℝ⁶

Dimensions:
  q1 ∈ [-π, π]      (joint1, shoulder yaw)
  q2 ∈ [-π/2, π/2]  (joint2, shoulder pitch)
  q3 ∈ [-π/2, 0]    (joint3, elbow pitch)
  q4 ∈ [-π, π]      (joint4, forearm rotation)
  q5 ∈ [-π/2, π/2]  (joint5, wrist pitch)
  q6 ∈ (-∞, +∞)     (joint6, continuous — wrist roll)
```

### Why C-space matters

Motion planning is hard in **workspace** (3D physical space) because the robot is a complex shape. But in C-space, the robot is a **single point**:

```
Workspace:                    C-space:
  3D, robot = complex shape     6D, robot = single point
  Obstacles = tables, walls     Obstacles = forbidden regions
  Path = swept volume           Path = curve through 6D space

Planning in C-space reduces collision checking to:
  "Is this single point inside an obstacle region?"
```

---

## 18.3 — C-Space Obstacles

An obstacle in the workspace maps to a region in C-space called a **C-obstacle**:

```
C_obs = { q ∈ C | robot(q) ∩ obstacle ≠ ∅ }

In words: the set of all configurations where the robot collides with the obstacle.
```

The **free space** is what's left:

```
C_free = C \ C_obs
```

### Example: 2-DOF arm with a wall

```
Physical space:                   C-space (q1 vs q2):
                                   q2
  +----------+                      |
  |  wall    |                      |  ████████
  |          |    /--- link2        |  ████████  <- C_obs
  +----------+   /                  |  ████████     (wall)
               O--- link1          |
               |                    +------------- q1
             base                   
                                   The wall becomes a blob
                                   in the q1-q2 plane.
```

For a 6-DOF arm, C-obstacles are 6-dimensional blobs. You can't visualize them, but algorithms can test membership: "given q, does robot(q) collide with anything?"

### C-space topology

Some joints wrap around (continuous rotation). Joint6 in the repo is continuous, so q6 = -π and q6 = +π are the **same configuration**. This makes C-space topologically a product of circles and intervals:

```
C = S¹ × [-π/2, π/2] × [-π/2, 0] × S¹ × [-π/2, π/2] × S¹

Where:
  S¹ = circle (for joints that wrap: joint1, joint4, joint6)
  [a, b] = interval (for joints with hard limits: joint2, joint3, joint5)
```

This topology matters for distance metrics and interpolation (you don't want to take the long way around a circle).

---

## 18.4 — Dimensionality and Complexity

### The curse of dimensionality

A grid-based search with resolution Δ in n dimensions needs:

```
N_cells = ∏ᵢ (range_i / Δ)

For the 6-DOF arm with Δ = 0.1 rad:
  N = (2π/0.1) × (π/0.1) × (π/2/0.1) × (2π/0.1) × (π/0.1) × (2π/0.1)
  N ≈ 63 × 31 × 16 × 63 × 31 × 63
  N ≈ 60 billion cells
```

At 1 byte per cell, that's **60 GB** of memory — just for an empty grid. And each cell needs a collision check. **Grid search is impossible in 6D.**

This is why motion planning uses **sampling-based** methods (Part 19): instead of discretizing the entire space, randomly sample configurations and connect them.

### Degrees of freedom recap

| System | DOF | C-space |
|--------|-----|---------|
| Point in 2D | 2 | ℝ² |
| Car (x, y, θ) | 3 | ℝ² × S¹ |
| 6-DOF arm | 6 | (S¹)³ × [a,b]³ |
| Dual arm system | 12 | ℝ¹² |
| Humanoid (30+ joints) | 30+ | ℝ³⁰⁺ |

---

## 18.5 — Collision Checking

At every sampled configuration, MoveIt must answer: "Is q in C_free?"

### Forward kinematics

First, compute where every link is in 3D space. This uses the **product of homogeneous transformation matrices** along the kinematic chain:

```
T_0^n(q) = T_0^1(q1) · T_1^2(q2) · T_2^3(q3) · ... · T_{n-1}^n(qn)

Where each T_i^{i+1}(qi) is a 4x4 matrix:

T = [ R(qi)  d_i ]
    [   0     1  ]

R(qi) = rotation matrix for joint i at angle qi
d_i   = translation from link i to link i+1 (from URDF)
```

For a revolute joint around the Z axis:

```
        [ cos(qi)  -sin(qi)  0  0 ]
T_i =   [ sin(qi)   cos(qi)  0  0 ]  ·  T_offset
        [    0         0      1  0 ]
        [    0         0      0  1 ]

T_offset encodes the xyz and rpy from the URDF <origin> tag.
```

This gives the pose (position + orientation) of every link frame in world coordinates.

### Collision geometry

Each link has a collision shape (from the URDF). MoveIt uses the **FCL library** (Flexible Collision Library) to test geometric intersections:

| Shape | Collision test |
|-------|---------------|
| Box vs Box | Separating Axis Theorem (SAT) |
| Sphere vs Sphere | Distance between centers < sum of radii |
| Cylinder vs Box | GJK algorithm |
| Mesh vs Mesh | BVH (Bounding Volume Hierarchy) |

### The GJK algorithm (Gilbert-Johnson-Keerthi)

For convex shapes, GJK determines if two shapes overlap by computing their **Minkowski difference** and checking if it contains the origin:

```
Minkowski difference:
  A ⊖ B = { a - b | a ∈ A, b ∈ B }

If 0 ∈ (A ⊖ B), then A and B overlap.

GJK finds this efficiently without computing the full Minkowski difference.
It iteratively builds a simplex (point → line → triangle → tetrahedron)
inside A ⊖ B, searching for the origin.
```

```
Iteration 1: pick a support point → point
Iteration 2: pick another → line segment
  Does the line contain the origin? If not, discard the far point.
Iteration 3: pick another → triangle
  Does the triangle contain the origin? If not, keep closest edge.
Iteration 4: pick another → tetrahedron (3D)
  Does the tetrahedron contain the origin?
  Yes → collision. No → repeat.

Typical convergence: 3-5 iterations. Very fast.
```

### Self-collision

The robot can collide with itself. For the 6-DOF arm, link1 could hit link5 in certain configurations. MoveIt checks all link pairs **not disabled** in the ACM (Part 17).

### Collision check cost

A single collision check involves:
1. Forward kinematics: O(n) matrix multiplications (n = number of joints)
2. Shape-pair tests: O(p) where p = enabled link pairs (~8 for our arm)
3. FCL GJK/SAT: O(1) per pair for convex shapes

Total: ~microseconds per check. A typical plan does **thousands** of checks.

---

## 18.6 — Distance Metrics in C-Space

Planning algorithms need a notion of **distance** between configurations. The standard metric is weighted Euclidean:

```
d(q_a, q_b) = √( Σᵢ wᵢ · (q_a_i - q_b_i)² )

Where wᵢ = weight for joint i (usually 1.0 for all joints)
```

### Handling wrap-around joints

For continuous joints (joint6), the angle wraps:

```
Standard:   d(q6_a, q6_b) = |q6_a - q6_b|         WRONG for wrap-around

Correct:    d(q6_a, q6_b) = min(|Δ|, 2π - |Δ|)    where Δ = q6_a - q6_b
```

Example:
```
q6_a = -170° = -2.967 rad
q6_b = +170° = +2.967 rad

Wrong distance: |(-2.967) - (2.967)| = 5.934 rad  (almost full rotation)
Correct distance: 2π - 5.934 = 0.349 rad           (20° shortcut)
```

### Interpolation

To move from q_a to q_b, linear interpolation in C-space:

```
q(t) = q_a + t · (q_b - q_a)    for t ∈ [0, 1]

For wrap-around joints, interpolate along the shorter arc.
```

This gives a straight line in C-space, but a **curved** motion in workspace (the end-effector traces a non-straight path). Cartesian straight-line motion requires a different approach (Part 21).

---

## 18.7 — Task Space vs Joint Space

**Joint space** = C-space. Coordinates are joint angles.
**Task space** (workspace) = the space where the end-effector lives. Coordinates are (x, y, z, roll, pitch, yaw).

```
Joint space:  q = [q1, q2, q3, q4, q5, q6] ∈ ℝ⁶
Task space:   x = [x, y, z, rx, ry, rz] ∈ ℝ³ × SO(3)

Forward kinematics:  f(q) = x     (easy — matrix multiplications)
Inverse kinematics:  f⁻¹(x) = q   (hard — nonlinear, multiple solutions)
```

Planning can happen in either space:

| | Joint-space planning | Task-space planning |
|---|---|---|
| **Coordinates** | Joint angles | End-effector pose |
| **Path shape** | Curved in workspace | Straight line in workspace |
| **Use case** | General motion | Welding, painting, assembly |
| **Planner** | OMPL (RRT, PRM) | Pilz (LIN, CIRC commands) |
| **IK needed?** | At start/goal only | At every waypoint |

---

## 18.8 — Example: Adding an Obstacle to the Planning Scene

The theory says workspace obstacles become C-obstacles in C-space. Here's how to concretely add a table in front of the robot — the planner will have to route around it.

```python
#!/usr/bin/env python3
"""add_table_obstacle.py — Add a table to the scene, then plan around it."""

import rclpy
from moveit.planning import MoveItPy
from geometry_msgs.msg import PoseStamped
from moveit_msgs.msg import CollisionObject
from shape_msgs.msg import SolidPrimitive

def main():
    rclpy.init()
    moveit = MoveItPy(node_name="obstacle_example")
    arm = moveit.get_planning_component("arm")
    planning_scene_monitor = moveit.get_planning_scene_monitor()

    # --- 1. Define the obstacle: a 1m x 0.5m x 0.02m box (table top) ---
    with planning_scene_monitor.read_write() as scene:
        collision_obj = CollisionObject()
        collision_obj.id = "table"
        collision_obj.header.frame_id = "base_link"

        # Shape: box (type=1)
        box = SolidPrimitive()
        box.type = SolidPrimitive.BOX
        box.dimensions = [1.0, 0.5, 0.02]     # length, width, thickness

        # Position: 50cm in front of the robot, 40cm high
        box_pose = PoseStamped()
        box_pose.header.frame_id = "base_link"
        box_pose.pose.position.x = 0.5
        box_pose.pose.position.y = 0.0
        box_pose.pose.position.z = 0.4
        box_pose.pose.orientation.w = 1.0

        collision_obj.primitives.append(box)
        collision_obj.primitive_poses.append(box_pose.pose)
        collision_obj.operation = CollisionObject.ADD

        scene.apply_collision_object(collision_obj)
        print("Table added to the scene")

    # --- 2. Plan a motion (the planner routes around the table) ---
    arm.set_start_state_to_current_state()
    arm.set_goal_state(configuration_name="pose_1")

    plan_result = arm.plan()
    if plan_result:
        print(f"Plan found: {len(plan_result.trajectory.joint_trajectory.points)} waypoints")
        # Without the table: the arm would go in a straight line
        # With the table: the planner generates a detour in C-space
        moveit.execute(plan_result.trajectory, controllers=[])
    else:
        print("No collision-free path — the target may be blocked")

    rclpy.shutdown()
```

### What happens in C-space:

```
Without obstacle:                 With the table:
  C-space (q1 vs q2)               C-space (q1 vs q2)
                                   
  goal *............* start       goal *              * start
       (direct path)                    \    ████     |
                                         \   ████    |  <- C-obstacle
                                          \  ████   |     (the table)
                                           \       |
                                            +-----+
                                          (path goes around)
```

The table (a 3D box) becomes a blob in the 6-dimensional C-space. The RRT planner only sees C-space and samples configurations that avoid this blob (Part 19).

---

## 18.9 — Summary: The Planning Problem

Given:
- **Start**: current configuration q_start ∈ C_free
- **Goal**: target configuration q_goal ∈ C_free (or target pose in task space → solve IK)
- **Obstacles**: set of C-obstacles derived from world geometry + self-collision

Find:
- A **continuous path** τ: [0,1] → C_free with τ(0) = q_start and τ(1) = q_goal

Such that:
- τ(t) ∈ C_free for all t (no collision at any point along the path)
- The path is "good" (short, smooth, avoids unnecessary motion)

This is the **motion planning problem**. Part 19 covers the algorithms that solve it.

---

**Prev:** [Part 17 — MoveIt Architecture](17-moveit-architecture.md)
**Next:** [Part 19 — Motion Planning Algorithms](19-motion-planning.md)
