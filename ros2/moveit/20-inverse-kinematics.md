# Part 20 — Inverse Kinematics: From Pose to Joint Angles

## 20.1 — The Analogy

You point at a cup on a table and say "grab that." Your brain instantly computes what angles your shoulder, elbow, and wrist need — without thinking about matrices. That's **inverse kinematics** (IK).

Forward kinematics is easy: "given joint angles, where is the hand?" It's just multiplying matrices. Inverse kinematics is the hard direction: "given where I want the hand, what joint angles do I need?" It's a system of nonlinear equations that may have zero, one, or many solutions.

---

## 20.2 — Forward Kinematics: The Easy Direction

### Denavit-Hartenberg (DH) convention

Each joint in a serial chain is described by 4 parameters:

```
| Parameter | Symbol | Meaning |
|-----------|--------|---------|
| Joint angle | θᵢ | Rotation around Zᵢ₋₁ (variable for revolute) |
| Link offset | dᵢ | Translation along Zᵢ₋₁ |
| Link length | aᵢ | Translation along Xᵢ |
| Link twist | αᵢ | Rotation around Xᵢ |
```

The transformation from frame i-1 to frame i:

```
         [ cos θᵢ   -sin θᵢ cos αᵢ    sin θᵢ sin αᵢ    aᵢ cos θᵢ ]
T_i =    [ sin θᵢ    cos θᵢ cos αᵢ   -cos θᵢ sin αᵢ    aᵢ sin θᵢ ]
         [   0          sin αᵢ            cos αᵢ            dᵢ      ]
         [   0            0                  0                1       ]
```

### Forward kinematics of the full chain

The end-effector pose in world coordinates:

```
T_0^6 = T_0^1(q1) · T_1^2(q2) · T_2^3(q3) · T_3^4(q4) · T_4^5(q5) · T_5^6(q6)

       [ R₃ₓ₃   p₃ₓ₁ ]     R = orientation (rotation matrix)
T_0^6 = [               ]     p = position (translation vector)
       [ 0₁ₓ₃     1   ]
```

For the repo's 6-DOF arm, this is a product of six 4x4 matrices. The result is a 4x4 matrix encoding position (x, y, z) and orientation (3x3 rotation matrix) of `tool_link` in world coordinates.

### Example: 2-link planar arm

```
         (x, y)
          *
         / link2 (L2)
        /
       * joint2
      / link1 (L1)
     /
    O joint1 (base)

Forward kinematics:
  x = L1 cos(q1) + L2 cos(q1 + q2)
  y = L1 sin(q1) + L2 sin(q1 + q2)

Given q1 = 30°, q2 = 45°, L1 = 0.5m, L2 = 0.3m:
  x = 0.5 cos(30°) + 0.3 cos(75°) = 0.433 + 0.078 = 0.511m
  y = 0.5 sin(30°) + 0.3 sin(75°) = 0.250 + 0.290 = 0.540m
```

---

## 20.3 — The IK Problem

### Formal definition

Given a desired end-effector pose T_desired (position + orientation), find joint angles q such that:

```
FK(q) = T_desired

Where FK: ℝⁿ → SE(3)
  n = number of joints
  SE(3) = group of rigid body transformations (position + orientation)
```

### Why it's hard

1. **Nonlinear**: cos and sin are nonlinear functions — can't solve with linear algebra alone
2. **Multiple solutions**: a 6-DOF arm can often reach the same pose with different joint configurations
3. **No solution**: the target may be out of reach or require violating joint limits
4. **Singularities**: at certain configurations, the arm loses a degree of freedom

### Multiple solutions (2-link arm example)

```
For the 2-link arm, given target (x, y):

  q2 = ± arccos( (x² + y² - L1² - L2²) / (2·L1·L2) )

  Two solutions:
                            
  "Elbow up":   * target     "Elbow down":  * target
                /|                            |\
               / |                            | \
    link2 →  /  |                            |  \  ← link2
            * joint2                   joint2 *
           /                                    \
          / link1                        link1   \
         O                                        O

  Both reach the same (x, y) but with opposite elbow angle.
```

For a 6-DOF arm in 3D, there can be up to **16 solutions** for a generic pose.

---

## 20.4 — Analytical IK (Closed-Form)

For specific robot geometries (e.g., 6-DOF with spherical wrist), the IK equations can be solved algebraically.

### 2-link planar arm (full derivation)

```
Given: target (x, y), link lengths L1, L2
Find: q1, q2

Step 1 — Solve q2 (law of cosines):

  x² + y² = L1² + L2² + 2·L1·L2·cos(q2)

  cos(q2) = (x² + y² - L1² - L2²) / (2·L1·L2)

  q2 = ± arccos(c)    where c = (x² + y² - L1² - L2²) / (2·L1·L2)

  If |c| > 1: no solution (target unreachable)
  If |c| = 1: one solution (arm fully extended or folded)
  If |c| < 1: two solutions (elbow up/down)

Step 2 — Solve q1:

  q1 = atan2(y, x) - atan2(L2·sin(q2), L1 + L2·cos(q2))

Example: target (0.5, 0.4), L1 = 0.5, L2 = 0.3

  c = (0.25 + 0.16 - 0.25 - 0.09) / (2·0.5·0.3) = 0.07/0.3 = 0.233
  q2 = ± arccos(0.233) = ± 76.5°

  For q2 = +76.5° (elbow up):
    q1 = atan2(0.4, 0.5) - atan2(0.3·sin(76.5°), 0.5 + 0.3·cos(76.5°))
    q1 = 38.7° - atan2(0.292, 0.568)
    q1 = 38.7° - 27.2° = 11.5°

  Verification: FK(11.5°, 76.5°) ≈ (0.5, 0.4) ✓
```

### Pieper's method (6-DOF spherical wrist)

For a 6-DOF arm where the last 3 joints intersect at a point (spherical wrist), IK decouples into:

```
Step 1: Position (joints 1-3)
  Solve for the wrist center position using the first 3 joints.
  The wrist center is at: p_wrist = p_target - d6 · R_target · [0, 0, 1]ᵀ
  
  This gives 3 equations in 3 unknowns → solvable analytically.

Step 2: Orientation (joints 4-6)
  Given joints 1-3, compute R_0^3.
  Then: R_3^6 = (R_0^3)ᵀ · R_target
  
  Extract Euler angles from R_3^6 → gives q4, q5, q6.
```

### Limitations of analytical IK

- Only works for specific robot geometries (spherical wrist, known structure)
- Must be derived per robot — no generic analytical solver exists
- The repo's arm doesn't have a perfect spherical wrist, so analytical IK is approximate

---

## 20.5 — Numerical IK (Iterative)

For general robots, IK is solved **iteratively** using the **Jacobian matrix**.

### The Jacobian

The Jacobian J maps joint velocities to end-effector velocities:

```
ẋ = J(q) · q̇

Where:
  ẋ = [vx, vy, vz, ωx, ωy, ωz]ᵀ  (end-effector velocity, 6×1)
  q̇ = [q̇1, q̇2, ..., q̇6]ᵀ        (joint velocities, 6×1)
  J = 6×6 matrix (for a 6-DOF arm)
```

Each column of J represents how the end-effector moves when one joint moves:

```
        ∂x/∂q1  ∂x/∂q2  ...  ∂x/∂q6
        ∂y/∂q1  ∂y/∂q2  ...  ∂y/∂q6
J(q) =  ∂z/∂q1  ∂z/∂q2  ...  ∂z/∂q6
        ∂ωx/∂q1 ∂ωx/∂q2 ... ∂ωx/∂q6
        ∂ωy/∂q1 ∂ωy/∂q2 ... ∂ωy/∂q6
        ∂ωz/∂q1 ∂ωz/∂q2 ... ∂ωz/∂q6
```

### Computing the Jacobian for revolute joints

For revolute joint i with axis zᵢ₋₁ and origin oᵢ₋₁:

```
Column i of J:

  Jv_i = zᵢ₋₁ × (o_n - oᵢ₋₁)    (linear velocity contribution)
  Jω_i = zᵢ₋₁                      (angular velocity contribution)

Where:
  zᵢ₋₁ = rotation axis of joint i (in world frame)
  oᵢ₋₁ = origin of joint i (in world frame)
  o_n   = end-effector position (in world frame)
  ×     = cross product
```

### Newton-Raphson IK

Use the Jacobian to iteratively approach the solution:

```
Jacobian IK(q_initial, T_target, max_iter, tolerance):
    q = q_initial
    
    for i = 1 to max_iter:
        T_current = FK(q)
        
        // Error between current and target pose
        Δx = pose_error(T_current, T_target)    // 6×1 vector
        
        if ||Δx|| < tolerance:
            return q    // converged!
        
        // Compute Jacobian at current configuration
        J = jacobian(q)
        
        // Solve for joint correction
        Δq = J⁻¹ · Δx        // if J is square and non-singular
        
        // Update joints
        q = q + α · Δq        // α = step size (0.1-1.0)
    
    return FAILURE (did not converge)
```

### Pose error computation

```
Position error:
  Δp = p_target - p_current    (simple subtraction, 3×1)

Orientation error (using rotation matrices):
  R_error = R_target · R_current⁻¹
  Δω = angle_axis(R_error)     (convert to axis-angle, 3×1)

Combined:
  Δx = [Δp; Δω]    (6×1 vector)
```

### Example iteration (2-link arm)

```
Target: (0.5, 0.4), L1 = 0.5, L2 = 0.3
Initial guess: q = [0, 0] (arm straight along x-axis)

Iteration 1:
  FK([0, 0]) = (0.8, 0.0)     Current position
  Δx = (0.5 - 0.8, 0.4 - 0.0) = (-0.3, 0.4)
  
  J = [-L1 sin(q1) - L2 sin(q1+q2),  -L2 sin(q1+q2)]
      [ L1 cos(q1) + L2 cos(q1+q2),   L2 cos(q1+q2)]
  
  J([0,0]) = [0,    0  ]
             [0.8,  0.3]
  
  J is singular! (determinant = 0). Arm is fully extended along x-axis.
  → Use damped least squares instead (see 20.6).

  With damping: Δq = [0.38, 0.82]
  q = [0.38, 0.82]

Iteration 2:
  FK([0.38, 0.82]) = (0.507, 0.436)    Much closer!
  Δx = (-0.007, -0.036)
  Continue iterating...

Iteration 5:
  FK(q) ≈ (0.500, 0.400)    Converged ✓
```

---

## 20.6 — Jacobian Pseudoinverse and Damped Least Squares

### When J is not invertible

J⁻¹ doesn't always exist:
- **Singular** configurations (arm fully extended, joints aligned)
- **Redundant** robots (7+ DOF: J is not square, 6×7)
- **Under-actuated** robots (< 6 DOF: can't reach all poses)

### Moore-Penrose pseudoinverse

For a non-square or singular J:

```
J† = Jᵀ · (J · Jᵀ)⁻¹    (right pseudoinverse, for redundant arms)

or

J† = (Jᵀ · J)⁻¹ · Jᵀ    (left pseudoinverse, for under-actuated arms)

Δq = J† · Δx

This minimizes ||Δq|| (smallest joint motion to achieve Δx).
```

### Damped Least Squares (DLS)

Near singularities, J† amplifies noise. DLS adds damping:

```
Δq = Jᵀ · (J · Jᵀ + λ² · I)⁻¹ · Δx

Where:
  λ = damping factor (typically 0.01-0.1)
  I = identity matrix

As λ → 0: DLS → pseudoinverse (fast but unstable near singularities)
As λ → ∞: DLS → gradient descent (slow but always stable)
```

### Manipulability and singularity

The **manipulability** measures how "well-conditioned" the arm is at a given configuration:

```
w(q) = √( det(J · Jᵀ) )

w = 0:     Singular (the arm can't move in some direction)
w > 0:     Non-singular (the arm can move freely)
w >> 0:    High manipulability (well-conditioned, fast convergence)
```

### Physical examples of singularities

```
1. Fully extended arm:
   O========== * target (at max reach)
   → Can't move the end-effector further outward
   → Lost 1 DOF (radial direction)

2. Aligned joints:
       |
       |  joint4 axis
       |  ≡ joint6 axis     (both rotate around same axis)
       |
   → Two joints do the same thing
   → Lost 1 DOF (redundant rotation)

3. Wrist at shoulder:
   Joint1 axis ≡ Joint4 axis (when arm is folded back)
   → Same problem as aligned joints
```

---

## 20.7 — IK Solvers in MoveIt

### Solver hierarchy

```
MoveIt IK request
    |
    v
CachedSrvKinematicsPlugin (your config)
    |
    |-- cache hit? → return cached solution (microseconds)
    |
    |-- cache miss → delegate to underlying solver
                       |
                       v
                  KDL (default)  or  IKFast  or  TRAC-IK  or  BioIK
```

### Solver comparison

| Solver | Method | Speed | Success rate | Setup |
|--------|--------|-------|-------------|-------|
| **KDL** | Newton-Raphson + pseudoinverse | ~1ms | ~70% | Automatic from URDF |
| **IKFast** | Analytical (generated code) | ~1μs | ~99% (in workspace) | Requires offline codegen |
| **TRAC-IK** | KDL + SQP (parallel) | ~1ms | ~95% | Drop-in replacement |
| **BioIK** | Evolutionary algorithm | ~5ms | ~98% | Good for constrained IK |

### KDL (Kinematics and Dynamics Library)

The default solver. Uses the Jacobian method from 20.5:

```
Algorithm:
1. Start from random seed (or current joint state)
2. Newton-Raphson iterations with damped Jacobian
3. If stuck, restart from new random seed
4. Return first solution found within limits

Configured in kinematics.yaml:
  kinematics_solver_search_resolution: 0.005   (step size in joint space)
  kinematics_solver_timeout: 0.005              (5ms per attempt)
```

### IKFast

Uses **symbolic computation** (OpenRAVE) to derive closed-form IK for your specific robot, then generates C++ code:

```
Offline (once, takes minutes):
  openrave --ikfast robot.dae --iktype=transform6d
  → generates ikfast_output.cpp (thousands of lines of atan2, sqrt, etc.)

Online (per query):
  ikfast_solve(T_target) → up to 16 solutions in ~1 microsecond
```

1000x faster than numerical IK. But only works for 6-DOF arms with certain geometries.

---

## 20.8 — Redundancy Resolution (7+ DOF)

A 7-DOF arm (like a human arm) has **more joints than needed** for 6D pose control. The extra DOF creates a **null space** — motions that change joint angles without moving the end-effector.

```
Δq = J† · Δx + (I - J† · J) · q_null

Where:
  J† · Δx              = minimum-norm solution (moves end-effector)
  (I - J† · J) · q_null = null space projection (doesn't move end-effector)
  q_null               = arbitrary joint velocity (secondary objective)
```

### Secondary objectives

The null space can optimize a secondary goal while maintaining the end-effector pose:

| q_null | Effect |
|--------|--------|
| -∇(joint limit cost) | Stay away from joint limits |
| -∇(singularity cost) | Maximize manipulability |
| q_default - q | Stay close to a preferred configuration |
| -∇(collision cost) | Avoid self-collision |

```
Example: elbow positioning

  Both configurations reach the same cup:
  
  Config A:            Config B:
    /\                   __
   /  \   cup *         /  \   cup *
  /    --*             |    --*
  O                    O
  
  The null space lets you choose elbow up vs elbow down
  while the hand stays at the cup.
```

---

## 20.9 — Example: Solving IK and Moving to a Pose

Instead of using named poses (joint values from the SRDF), here we give MoveIt a **target pose** in Cartesian space. It must solve IK to find joint angles, then plan a path:

```python
#!/usr/bin/env python3
"""move_to_pose.py — Move the arm to a specific XYZ position and orientation."""

import rclpy
from moveit.planning import MoveItPy
from geometry_msgs.msg import PoseStamped

def main():
    rclpy.init()
    moveit = MoveItPy(node_name="ik_example")
    arm = moveit.get_planning_component("arm")

    # Define a target pose for tool_link
    target_pose = PoseStamped()
    target_pose.header.frame_id = "base_link"

    # Position: 40cm forward, 10cm left, 50cm up from the base
    target_pose.pose.position.x = 0.4
    target_pose.pose.position.y = 0.1
    target_pose.pose.position.z = 0.5

    # Orientation: tool pointing downward (quaternion for 180° rotation around Y)
    target_pose.pose.orientation.x = 0.0
    target_pose.pose.orientation.y = 1.0
    target_pose.pose.orientation.z = 0.0
    target_pose.pose.orientation.w = 0.0

    # Set the goal — MoveIt will solve IK internally
    arm.set_start_state_to_current_state()
    arm.set_goal_state(pose_stamped_msg=target_pose, pose_link="tool_link")

    plan_result = arm.plan()

    if plan_result:
        traj = plan_result.trajectory.joint_trajectory
        # The last waypoint contains the IK solution
        final_joints = traj.points[-1].positions
        print("IK solution (joint angles):")
        for name, val in zip(traj.joint_names, final_joints):
            print(f"  {name}: {val:+.3f} rad ({val * 57.3:+.1f} deg)")

        moveit.execute(plan_result.trajectory, controllers=[])
    else:
        print("Planning failed — pose may be unreachable or in collision")

    rclpy.shutdown()
```

### What happens step by step:

```
1. arm.set_goal_state(pose_stamped_msg=target_pose, pose_link="tool_link")
   
   MoveIt stores the Cartesian target. IK is NOT solved yet.

2. arm.plan()

   a. Get current joints from /joint_states → q_start
   
   b. Solve IK for the target pose (Part 20.5):
      - CachedSrvKinematicsPlugin checks cache → miss
      - KDL solver runs Newton-Raphson iterations:
        
        Iteration 1: FK(q_guess) = T_current
                     Δx = T_target - T_current   (6×1 pose error)
                     Δq = J†(q) · Δx             (pseudoinverse step)
                     q = q + Δq
        
        Iteration 2-10: repeat until ||Δx|| < 0.005
      
      - Returns q_goal = [0.24, -0.54, -0.42, 0.00, 0.96, 0.24]
   
   c. Plan path: RRTConnect from q_start to q_goal (Part 19)
   
   d. Time parameterization (Part 21)
   
   e. Return trajectory

3. moveit.execute() → sends to arm_controller
```

### When IK fails:

```python
# Target at 2 meters — far beyond the arm's ~1.7m reach
target_pose.pose.position.x = 2.0
target_pose.pose.position.z = 0.0

arm.set_goal_state(pose_stamped_msg=target_pose, pose_link="tool_link")
result = arm.plan()  # → None (IK has no solution, cos(q2) > 1)
```

---

## 20.10 — Example: Controlling the Gripper

The gripper is a separate planning group with 1 DOF. You control it by setting joint values directly (no IK needed):

```python
#!/usr/bin/env python3
"""gripper_control.py — Open and close the gripper."""

import rclpy
from moveit.planning import MoveItPy

def main():
    rclpy.init()
    moveit = MoveItPy(node_name="gripper_example")
    gripper = moveit.get_planning_component("gripper")

    # Open the gripper (named pose from SRDF)
    gripper.set_start_state_to_current_state()
    gripper.set_goal_state(configuration_name="gripper_open")
    result = gripper.plan()
    if result:
        moveit.execute(result.trajectory, controllers=[])
        print("Gripper opened")

    # Close the gripper
    gripper.set_start_state_to_current_state()
    gripper.set_goal_state(configuration_name="gripper_close")
    result = gripper.plan()
    if result:
        moveit.execute(result.trajectory, controllers=[])
        print("Gripper closed")

    # Half-close (custom joint value, not from SRDF)
    gripper.set_start_state_to_current_state()
    gripper.set_goal_state(
        joint_values={"gripper_left_finger_joint": 0.03}  # 3cm gap
    )
    result = gripper.plan()
    if result:
        moveit.execute(result.trajectory, controllers=[])
        print("Gripper at 3cm opening")

    rclpy.shutdown()
```

### Gripper state map (from the SRDF):

```
gripper_left_finger_joint value:
  0.00 → gripper_close (fingers touching)
  0.03 → gripper_half_close
  0.06 → gripper_open (fingers 6cm apart)

The right finger mirrors via the mimic joint (Part 17):
  right = -1 × left + 0

No IK, no complex planning — just a 1D trajectory from current to target.
```

---

## 20.11 — Summary: IK Decision Tree

```
Is the robot geometry known and standard (6-DOF, spherical wrist)?
  |
  +-- Yes → Use IKFast (analytical, fastest, most reliable)
  |
  +-- No → Is the robot < 7 DOF?
            |
            +-- Yes → Use TRAC-IK (numerical, good success rate)
            |
            +-- No → Is there a secondary objective (null space)?
                      |
                      +-- Yes → Use BioIK (handles constraints well)
                      |
                      +-- No → Use KDL (default, adequate for most cases)
```

The repo uses `CachedSrvKinematicsPlugin` which wraps another solver with a lookup cache — good for repeated IK queries at similar poses (common during motion planning where the planner tests thousands of nearby configurations).

---

## 20.12 — C++ Example: Pose Goal with IK

```cpp
#include <moveit/move_group_interface/move_group_interface.h>
#include <geometry_msgs/msg/pose.hpp>

// Set a Cartesian pose goal — MoveIt solves IK automatically
geometry_msgs::msg::Pose target_pose;
target_pose.position.x = 0.3;
target_pose.position.y = 0.0;
target_pose.position.z = 0.5;
target_pose.orientation.w = 1.0;  // facing forward (quaternion)

arm.setPoseTarget(target_pose);

// Plan and execute
moveit::planning_interface::MoveGroupInterface::Plan plan;
bool success = (arm.plan(plan) == moveit::core::MoveItErrorCode::SUCCESS);
if (success) arm.execute(plan);

// You can also compute IK directly without planning:
moveit::core::RobotStatePtr current_state = arm.getCurrentState();
bool ik_found = current_state->setFromIK(
    arm.getRobotModel()->getJointModelGroup("arm"),
    target_pose,
    0.1  // timeout in seconds
);
```

---

## 20.13 — Quick Reference

| Concept | Key Point |
|---|---|
| Forward kinematics (FK) | Joint angles → end-effector pose: `T = T1 * T2 * ... * Tn` |
| Inverse kinematics (IK) | End-effector pose → joint angles (may have 0, 1, or many solutions) |
| DH convention | 4 parameters per joint: a, d, α, θ — builds each Tᵢ matrix |
| Jacobian (J) | 6×n matrix: `ẋ = J * q̇` — maps joint velocities to Cartesian velocity |
| Numerical IK | Newton-Raphson: `q_new = q + J⁺ * (x_goal - FK(q))` — iterative |
| Pseudoinverse (J⁺) | `J⁺ = Jᵀ(JJᵀ)⁻¹` — least-norm solution for redundant robots |
| Damped least squares | `J⁺ = Jᵀ(JJᵀ + λ²I)⁻¹` — stable near singularities |
| Singularity | Jacobian loses rank → some Cartesian directions become impossible |
| KDL | Default numerical IK solver in MoveIt (Newton-Raphson based) |
| TRAC-IK | Better success rate than KDL (tries multiple seeds + SQP fallback) |
| IKFast | Analytical solver — fastest and most reliable, but requires code generation |
| C++ API | `arm.setPoseTarget(pose)` → MoveIt calls IK internally |

---

**Prev:** [Part 19 — Motion Planning Algorithms](19-motion-planning.md)
**Next:** [Part 21 — Trajectory Generation](21-trajectory-generation.md)
**See also:** [Mathematics — Linear Algebra](../../mathematics/01-linear-algebra/README.md) — matrix × matrix, matrix inverse, and the linear systems behind the DH transform, Jacobian, and pseudoinverse `J⁺ = Jᵀ(JJᵀ)⁻¹`
