# Part 21 — Trajectory Generation: From Path to Motion

## 21.1 — The Analogy

A **path** is a route on a map — it says *where* to go but not *when*. A **trajectory** is a driving plan — it adds speed: "be at this point at this time."

The motion planner (Part 19) finds a path: a sequence of joint configurations. But the robot can't execute a path — it needs to know the position, velocity, and acceleration at every instant. Trajectory generation adds the time dimension.

---

## 21.2 — Path vs Trajectory

```
Path:       τ: [0, 1] → C-space
            τ(s) = joint positions at path parameter s
            No time information. s is just a progress variable.

Trajectory: q: [0, T] → C-space
            q(t) = joint positions at time t
            Includes timing: velocity q̇(t) and acceleration q̈(t)
```

### Example

```
Path (from planner):
  s=0.0:  q = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0]   (home)
  s=0.5:  q = [0.4, -0.2, -0.3, 0.1, 0.2, 0.0]   (midpoint)
  s=1.0:  q = [0.8, -0.3, -0.5, 0.2, 0.3, 0.0]   (goal)

Trajectory (after time parameterization):
  t=0.0s: q = [0.0, ...], q̇ = [0.0, ...], q̈ = [0.5, ...]
  t=0.5s: q = [0.2, ...], q̇ = [0.8, ...], q̈ = [0.0, ...]
  t=1.0s: q = [0.4, ...], q̇ = [0.8, ...], q̈ = [0.0, ...]   (cruising)
  t=1.5s: q = [0.6, ...], q̇ = [0.8, ...], q̈ = [-0.5, ...]
  t=2.0s: q = [0.8, ...], q̇ = [0.0, ...], q̈ = [-0.5, ...]   (stopped)
```

---

## 21.3 — Constraints

The trajectory must respect physical limits at every instant, for every joint:

```
Joint limits:        q_min ≤ q(t) ≤ q_max
Velocity limits:     |q̇(t)| ≤ q̇_max
Acceleration limits: |q̈(t)| ≤ q̈_max
```

From the repo's `joint_limits.yaml`:

```yaml
joint1:
  has_velocity_limits: true
  max_velocity: 1.0            # rad/s
  has_acceleration_limits: true
  max_acceleration: 1.0        # rad/s²
```

The trajectory must also be **smooth** — joints can't teleport (continuous position) and motors can't produce infinite torque (continuous velocity).

```
Required continuity:
  C⁰: position is continuous         (no teleportation)
  C¹: velocity is continuous          (no instantaneous speed changes)
  C²: acceleration is continuous      (no infinite jerk — smooth motion)
```

---

## 21.4 — Trapezoidal Velocity Profile

The simplest time parameterization. Each joint follows an **accelerate → cruise → decelerate** pattern:

```
Velocity:
  q̇_max  ___________________
         /                   \
        /                     \
  0 ___/                       \___
      |   |        |        |   |
      t0  t1       t2       t3  t4
      
      accel  cruise    decel

Position:
     /‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾/
    /                  /
   /                  /
  /                  /
  parabola  line   parabola

Acceleration:
  a_max  ___
        |   |
        |   |                    ___
  0 ____|   |___________________|   |____
                                | -a_max|
```

### Math

**Phase 1 — Acceleration** (t ∈ [0, t_accel]):
```
q̈(t) = a_max
q̇(t) = a_max · t
q(t)  = q_start + ½ · a_max · t²
```

**Phase 2 — Cruise** (t ∈ [t_accel, t_cruise]):
```
q̈(t) = 0
q̇(t) = v_max
q(t)  = q(t_accel) + v_max · (t - t_accel)
```

**Phase 3 — Deceleration** (t ∈ [t_cruise, T]):
```
q̈(t) = -a_max
q̇(t) = v_max - a_max · (t - t_cruise)
q(t)  = q(t_cruise) + v_max · (t - t_cruise) - ½ · a_max · (t - t_cruise)²
```

### Timing computation

```
Given: distance Δq, max velocity v_max, max acceleration a_max

Time to accelerate to v_max:
  t_accel = v_max / a_max

Distance during accel/decel:
  d_accel = 2 · ½ · v_max² / a_max = v_max² / a_max

If d_accel > Δq:
  → No cruise phase (triangular profile)
  → v_peak = √(a_max · Δq)
  → T = 2 · v_peak / a_max

If d_accel ≤ Δq:
  → Has cruise phase
  → d_cruise = Δq - v_max² / a_max
  → t_cruise = d_cruise / v_max
  → T = 2 · v_max / a_max + t_cruise
```

### Example

```
Joint1 needs to move: Δq = 0.8 rad
v_max = 1.0 rad/s, a_max = 1.0 rad/s²

d_accel = 1.0² / 1.0 = 1.0 rad

Since 1.0 > 0.8: triangular profile (can't reach full speed)

v_peak = √(1.0 · 0.8) = 0.894 rad/s
T = 2 · 0.894 / 1.0 = 1.789 s

Profile:
  t=0.000s: q̇=0.000    ↗ accelerating
  t=0.447s: q̇=0.447    ↗
  t=0.894s: q̇=0.894    ← peak (start decelerating)
  t=1.342s: q̇=0.447    ↘
  t=1.789s: q̇=0.000    ← stopped at goal
```

### Multi-joint synchronization

Each joint has a different distance to travel. The trajectory uses the **slowest joint's timing** so all joints start and stop together:

```
Joint1: Δq = 0.8 rad → T = 1.789 s
Joint2: Δq = 0.3 rad → T = 1.095 s
Joint3: Δq = 0.5 rad → T = 1.414 s

Synchronized: T = max(1.789, 1.095, 1.414) = 1.789 s

Joint2 and Joint3 are slowed down to match Joint1.
Their v_max is reduced: v_new = Δq / (T - T_accel_new)
```

---

## 21.5 — Time-Optimal Path Parameterization (TOPP)

Trapezoidal profiles are simple but **not optimal** — they treat each waypoint independently. **TOPP** finds the time-optimal parameterization for a given path, respecting all joint velocity and acceleration limits simultaneously.

### The idea

Instead of parameterizing each joint separately, treat the path as a 1D curve parameterized by arc length s:

```
q(s) = path(s)    s ∈ [0, 1]    (given by the planner)

s(t) = how fast we traverse the path at time t

q(t) = path(s(t))
q̇(t) = path'(s) · ṡ(t)
q̈(t) = path''(s) · ṡ(t)² + path'(s) · s̈(t)
```

The problem reduces to finding **ṡ(t)** (the speed along the path) that minimizes total time T while respecting:

```
For every joint i, at every path parameter s:

  |path'ᵢ(s) · ṡ|  ≤  v_max_i          (velocity limit)
  |path''ᵢ(s) · ṡ² + path'ᵢ(s) · s̈|  ≤  a_max_i   (acceleration limit)
```

### Phase-plane analysis

The constraints define a feasible region in the **(s, ṡ)** phase plane:

```
ṡ (speed along path)
 ^
 |     ___________
 |    /           \        velocity limit ceiling
 |   /   feasible  \
 |  /    region     \
 | /                 \
 |/___________________\___________> s (progress along path)
 0                     1

The optimal trajectory follows:
  - Maximum acceleration until hitting the ceiling
  - Cruise along the ceiling
  - Maximum deceleration to reach ṡ=0 at s=1
```

### TOPP-RA (Reachability Analysis)

MoveIt uses **TOPP-RA** (Time-Optimal Path Parameterization via Reachability Analysis), a modern algorithm:

```
1. Discretize path: s_0 = 0, s_1, s_2, ..., s_N = 1

2. Backward pass: for each s_i from N to 0:
   Compute maximum ṡ(s_i) such that the robot can decelerate to stop.
   This gives the "maximum reachable velocity" curve.

3. Forward pass: for each s_i from 0 to N:
   Accelerate as fast as possible without exceeding:
   - Joint velocity limits
   - Joint acceleration limits  
   - The maximum reachable velocity from the backward pass

4. Result: optimal ṡ(s) profile → integrate to get timing s(t)
```

### Why TOPP-RA is better than trapezoidal

```
Trapezoidal:
  Joint1: ___/‾‾‾‾\___     (independent profiles)
  Joint2: __/‾‾‾‾‾‾\__
  Joint3: _/‾\__________    Synchronized but not optimal.
                            Total time: T_trap

TOPP-RA:
  All joints: coordinated profile that pushes each limit simultaneously
  
  Total time: T_topp ≤ T_trap (often 20-40% faster)

  The savings come from exploiting coupling: when joint1 is far
  from its limit, joint2 can go faster.
```

---

## 21.6 — Spline-Based Trajectories

The `JointTrajectoryController` (ros2_control) receives waypoints and interpolates between them using **cubic or quintic splines**.

### Cubic spline (C¹ continuous)

Between waypoints q_i and q_{i+1} at times t_i and t_{i+1}:

```
q(t) = a₀ + a₁·(t-tᵢ) + a₂·(t-tᵢ)² + a₃·(t-tᵢ)³

Constraints:
  q(tᵢ)   = qᵢ         (position at start)
  q(tᵢ₊₁) = qᵢ₊₁       (position at end)
  q̇(tᵢ)   = q̇ᵢ         (velocity at start)
  q̇(tᵢ₊₁) = q̇ᵢ₊₁       (velocity at end)

4 constraints, 4 coefficients → unique solution.
```

### Quintic spline (C² continuous)

```
q(t) = a₀ + a₁·(t-tᵢ) + a₂·(t-tᵢ)² + a₃·(t-tᵢ)³ + a₄·(t-tᵢ)⁴ + a₅·(t-tᵢ)⁵

Additional constraints:
  q̈(tᵢ)   = q̈ᵢ         (acceleration at start)
  q̈(tᵢ₊₁) = q̈ᵢ₊₁       (acceleration at end)

6 constraints, 6 coefficients → unique solution.
```

Quintic splines ensure smooth acceleration (no torque discontinuities), important for real hardware.

### Controller interpolation

At 100 Hz, the `arm_controller` receives waypoints and interpolates:

```
MoveIt sends:           Controller does:
  t=0.0s: q=[0.0, ...]    t=0.00s: q=[0.000, ...]  (interpolated)
                            t=0.01s: q=[0.002, ...]
                            t=0.02s: q=[0.008, ...]
                            ...
  t=0.5s: q=[0.4, ...]    t=0.50s: q=[0.400, ...]
                            t=0.51s: q=[0.408, ...]
                            ...
  t=1.0s: q=[0.8, ...]    t=1.00s: q=[0.800, ...]

Between waypoints: cubic spline interpolation at 100 Hz
```

---

## 21.7 — Cartesian Trajectory Generation

For straight-line motion in workspace (Pilz LIN), the trajectory is defined in Cartesian coordinates, not joint space:

### Position interpolation (linear)

```
p(t) = p_start + (t/T) · (p_goal - p_start)
```

### Orientation interpolation (SLERP)

Rotations are interpolated using **Spherical Linear Interpolation** on quaternions:

```
Given: quaternions q_start and q_goal

SLERP(q_start, q_goal, t/T):
  θ = arccos(q_start · q_goal)                    (angle between quaternions)
  
  q(t) = [sin((1-t/T)·θ) / sin(θ)] · q_start 
       + [sin((t/T)·θ) / sin(θ)] · q_goal

Properties:
  - Constant angular velocity (uniform rotation)
  - Shortest path on the rotation sphere (great arc)
  - t=0 → q_start, t=T → q_goal
```

### Why not linear interpolation for rotations?

```
Linear interpolation of rotation matrices:
  R(t) = (1-t)·R_start + t·R_goal    WRONG!

  The result is NOT a valid rotation matrix.
  Rotation matrices must satisfy: R·Rᵀ = I, det(R) = 1
  A linear combination of two rotation matrices violates both.

SLERP on quaternions:
  q(t) = SLERP(q_start, q_goal, t)    CORRECT!

  The result IS a valid unit quaternion (valid rotation).
  Interpolation follows the shortest arc on the 4D unit sphere.
```

### IK at every point

Cartesian trajectories require IK at every interpolation step:

```
for t = 0, Δt, 2Δt, ..., T:
    p(t) = linear_interpolation(t)
    R(t) = SLERP(t)
    q(t) = IK(p(t), R(t))
    
    if IK fails or collision(q(t)):
        return INFEASIBLE
```

This is why Cartesian trajectories are more expensive and can fail (the straight line in workspace may pass through a singularity or collision).

---

## 21.8 — The FollowJointTrajectory Action

MoveIt sends the computed trajectory to ros2_control via the **FollowJointTrajectory** action:

```
# control_msgs/action/FollowJointTrajectory

Goal:
  trajectory:
    joint_names: [joint1, joint2, joint3, joint4, joint5, joint6]
    points:
      - positions: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        velocities: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        time_from_start: {sec: 0, nanosec: 0}
      - positions: [0.2, -0.1, -0.15, 0.05, 0.1, 0.0]
        velocities: [0.4, -0.2, -0.3, 0.1, 0.2, 0.0]
        time_from_start: {sec: 0, nanosec: 500000000}
      - positions: [0.8, -0.3, -0.5, 0.2, 0.3, 0.0]
        velocities: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
        time_from_start: {sec: 2, nanosec: 0}
  path_tolerance: [...]      # max deviation during execution
  goal_tolerance: [...]      # max deviation at final position

Feedback:
  actual: {positions, velocities}     # where the robot IS
  desired: {positions, velocities}    # where it SHOULD BE
  error: {positions, velocities}      # difference

Result:
  error_code: SUCCESSFUL | PATH_TOLERANCE_VIOLATED | GOAL_TOLERANCE_VIOLATED
```

### Execution monitoring

```
At each control cycle (100 Hz):

  q_desired = interpolate(trajectory, t_current)
  q_actual  = state_interfaces  (from hardware)
  q_error   = q_desired - q_actual

  if |q_error| > path_tolerance:
      ABORT (robot deviated too far — something went wrong)
  
  if t_current > T + goal_time_tolerance:
      if |q_error| > goal_tolerance:
          ABORT (didn't reach goal in time)
      else:
          SUCCESS
```

---

## 21.9 — Example: Controlling Velocity and Acceleration

MoveIt's time parameterization respects joint limits. You can scale these limits down for safer, slower motion — or push them up for faster execution:

```python
#!/usr/bin/env python3
"""velocity_scaling.py — Same motion at different speeds."""

import time
import rclpy
from moveit.planning import MoveItPy

def move_at_speed(arm, moveit, vel_scale, acc_scale, label):
    """Plan and execute at the given velocity/acceleration scaling."""
    arm.set_start_state_to_current_state()
    arm.set_goal_state(configuration_name="pose_1")

    # Scale factors: 0.0 to 1.0
    # 1.0 = use full joint_limits.yaml values (1.0 rad/s, 1.0 rad/s²)
    # 0.1 = use 10% of limits (0.1 rad/s, 0.1 rad/s²)
    arm.set_max_velocity_scaling_factor(vel_scale)
    arm.set_max_acceleration_scaling_factor(acc_scale)

    result = arm.plan()
    if result:
        traj = result.trajectory.joint_trajectory
        duration = traj.points[-1].time_from_start.sec \
                 + traj.points[-1].time_from_start.nanosec * 1e-9
        print(f"{label}: {duration:.2f}s, {len(traj.points)} waypoints")
        moveit.execute(result.trajectory, controllers=[])

        # Return to home for next test
        arm.set_start_state_to_current_state()
        arm.set_goal_state(configuration_name="home")
        arm.set_max_velocity_scaling_factor(1.0)
        arm.set_max_acceleration_scaling_factor(1.0)
        r = arm.plan()
        if r:
            moveit.execute(r.trajectory, controllers=[])

def main():
    rclpy.init()
    moveit = MoveItPy(node_name="speed_example")
    arm = moveit.get_planning_component("arm")

    move_at_speed(arm, moveit, 1.0, 1.0, "Full speed   ")
    move_at_speed(arm, moveit, 0.5, 0.5, "Half speed   ")
    move_at_speed(arm, moveit, 0.1, 0.1, "Slow (10%)   ")

    rclpy.shutdown()
```

### Typical output:

```
Full speed   : 1.79s, 8 waypoints
Half speed   : 3.12s, 12 waypoints
Slow (10%)   : 12.45s, 35 waypoints
```

### What changes with scaling:

```
From joint_limits.yaml:
  max_velocity:     1.0 rad/s
  max_acceleration: 1.0 rad/s²

With vel_scale=0.5, acc_scale=0.5:
  effective v_max = 0.5 rad/s
  effective a_max = 0.5 rad/s²

The path through C-space is the SAME — only the time parameterization
changes (Part 21.4). Lower limits → longer trapezoidal profile:

Full speed:                Half speed:
  v_max ____                 v_max/2 ________
       /    \                       /        \
      /      \                     /          \
  ___/        \___             ___/            \___
  |--- 1.79s ---|              |---- 3.12s ------|
```

---

## 21.10 — Example: Pick and Place Sequence

A complete pick-and-place operation combines everything: arm motion, gripper control, and attached collision objects:

```python
#!/usr/bin/env python3
"""pick_and_place.py — Pick an object from one location, place it at another."""

import rclpy
from moveit.planning import MoveItPy
from geometry_msgs.msg import PoseStamped
from moveit_msgs.msg import CollisionObject
from shape_msgs.msg import SolidPrimitive

def add_box(scene, name, x, y, z, sx, sy, sz):
    """Add a collision box to the planning scene."""
    obj = CollisionObject()
    obj.id = name
    obj.header.frame_id = "base_link"
    box = SolidPrimitive()
    box.type = SolidPrimitive.BOX
    box.dimensions = [sx, sy, sz]
    pose = PoseStamped()
    pose.header.frame_id = "base_link"
    pose.pose.position.x = x
    pose.pose.position.y = y
    pose.pose.position.z = z
    pose.pose.orientation.w = 1.0
    obj.primitives.append(box)
    obj.primitive_poses.append(pose.pose)
    obj.operation = CollisionObject.ADD
    scene.apply_collision_object(obj)

def main():
    rclpy.init()
    moveit = MoveItPy(node_name="pick_place_example")
    arm = moveit.get_planning_component("arm")
    gripper = moveit.get_planning_component("gripper")
    psm = moveit.get_planning_scene_monitor()

    # --- Setup: add a table and a small box (the object to pick) ---
    with psm.read_write() as scene:
        add_box(scene, "table",  0.5, 0.0, 0.2,  0.8, 0.6, 0.02)
        add_box(scene, "target", 0.5, 0.0, 0.23, 0.04, 0.04, 0.04)  # 4cm cube on table
    print("Scene ready: table + target object")

    # --- Step 1: Move above the object ---
    above_pick = PoseStamped()
    above_pick.header.frame_id = "base_link"
    above_pick.pose.position.x = 0.5
    above_pick.pose.position.y = 0.0
    above_pick.pose.position.z = 0.40      # 15cm above the object
    above_pick.pose.orientation.y = 1.0     # tool pointing down
    above_pick.pose.orientation.w = 0.0

    arm.set_start_state_to_current_state()
    arm.set_goal_state(pose_stamped_msg=above_pick, pose_link="tool_link")
    result = arm.plan()
    if result:
        moveit.execute(result.trajectory, controllers=[])
    print("Step 1: above pick position")

    # --- Step 2: Open gripper ---
    gripper.set_start_state_to_current_state()
    gripper.set_goal_state(configuration_name="gripper_open")
    result = gripper.plan()
    if result:
        moveit.execute(result.trajectory, controllers=[])
    print("Step 2: gripper opened")

    # --- Step 3: Lower to the object (Cartesian down movement) ---
    pick_pose = PoseStamped()
    pick_pose.header.frame_id = "base_link"
    pick_pose.pose.position.x = 0.5
    pick_pose.pose.position.y = 0.0
    pick_pose.pose.position.z = 0.25       # at object height
    pick_pose.pose.orientation.y = 1.0
    pick_pose.pose.orientation.w = 0.0

    arm.set_start_state_to_current_state()
    arm.set_goal_state(pose_stamped_msg=pick_pose, pose_link="tool_link")
    arm.set_max_velocity_scaling_factor(0.1)   # slow approach
    result = arm.plan()
    if result:
        moveit.execute(result.trajectory, controllers=[])
    print("Step 3: at pick position")

    # --- Step 4: Close gripper (grasp the object) ---
    gripper.set_start_state_to_current_state()
    gripper.set_goal_state(configuration_name="gripper_close")
    result = gripper.plan()
    if result:
        moveit.execute(result.trajectory, controllers=[])
    print("Step 4: object grasped")

    # --- Step 5: Attach the object to the gripper ---
    # This tells MoveIt: "the object now moves with the gripper"
    # Without this, the planner would avoid collisions between the
    # gripper and the object — making any motion impossible.
    with psm.read_write() as scene:
        scene.attach_object("target", "tool_link")
    print("Step 5: object attached to gripper")

    # --- Step 6: Lift and move to place position ---
    arm.set_max_velocity_scaling_factor(0.5)
    place_pose = PoseStamped()
    place_pose.header.frame_id = "base_link"
    place_pose.pose.position.x = 0.3
    place_pose.pose.position.y = 0.3        # 30cm to the side
    place_pose.pose.position.z = 0.40
    place_pose.pose.orientation.y = 1.0
    place_pose.pose.orientation.w = 0.0

    arm.set_start_state_to_current_state()
    arm.set_goal_state(pose_stamped_msg=place_pose, pose_link="tool_link")
    result = arm.plan()
    if result:
        moveit.execute(result.trajectory, controllers=[])
    print("Step 6: at place position")

    # --- Step 7: Open gripper (release) ---
    gripper.set_start_state_to_current_state()
    gripper.set_goal_state(configuration_name="gripper_open")
    result = gripper.plan()
    if result:
        moveit.execute(result.trajectory, controllers=[])
    print("Step 7: object released")

    # --- Step 8: Detach object and go home ---
    with psm.read_write() as scene:
        scene.detach_object("target")

    arm.set_start_state_to_current_state()
    arm.set_goal_state(configuration_name="home")
    arm.set_max_velocity_scaling_factor(1.0)
    result = arm.plan()
    if result:
        moveit.execute(result.trajectory, controllers=[])
    print("Step 8: back home — pick and place complete!")

    rclpy.shutdown()
```

### The sequence, mapped to theory:

```
Step 1 (move above):   IK (Part 20) → RRTConnect in C-space (Part 19) → TOPP-RA (Part 21)
Step 2 (open gripper): 1-DOF trajectory, no planning needed
Step 3 (lower):        IK → planning with slow velocity scaling (Part 21.4)
Step 4 (close gripper): 1-DOF trajectory
Step 5 (attach):       Modifies planning scene — object is now part of the robot
                       ACM updated: gripper↔target collisions disabled (Part 17.4)
Step 6 (move to place): IK → RRTConnect (now carries the object — collision shape included)
Step 7 (release):      1-DOF trajectory
Step 8 (home):         Named pose → direct joint goal → RRTConnect
```

---

## 21.11 — Summary: The Full Pipeline

```
1. Planner (Part 19):
   Input:  q_start, q_goal (or pose_goal → IK)
   Output: path [q₀, q₁, ..., qₙ] (no timing)
   
2. Smoothing (Part 19.9):
   Input:  raw path
   Output: smoothed path (shortcutting + B-spline)

3. Time parameterization (this part):
   Input:  smoothed path + velocity/acceleration limits
   Output: trajectory [q(t), q̇(t), q̈(t)] at each waypoint
   Method: TOPP-RA or trapezoidal

4. Execution (ros2_control):
   Input:  trajectory (via FollowJointTrajectory action)
   Output: motor commands at 100 Hz
   Method: spline interpolation between waypoints

Timeline:
  [plan: 0.1-5s] → [smooth: 0.01s] → [time param: 0.01s] → [execute: 1-10s]
```

---

## 21.12 — C++ Example: Velocity Scaling and Cartesian Path

```cpp
#include <moveit/move_group_interface/move_group_interface.h>

// Slow down execution (useful for approach/retreat)
arm.setMaxVelocityScalingFactor(0.3);   // 30% of max joint velocity
arm.setMaxAccelerationScalingFactor(0.3);

// Cartesian path (straight line in task space)
std::vector<geometry_msgs::msg::Pose> waypoints;
waypoints.push_back(current_pose);

auto target = current_pose;
target.position.z -= 0.1;  // Move 10cm down
waypoints.push_back(target);

moveit_msgs::msg::RobotTrajectory trajectory;
double fraction = arm.computeCartesianPath(
    waypoints,
    0.01,       // eef_step: max 1cm between interpolated points
    0.0,        // jump_threshold: 0 = disable jump detection
    trajectory
);

if (fraction > 0.99) {
    arm.execute(trajectory);
}
```

---

## 21.13 — Quick Reference

| Concept | Key Point |
|---|---|
| Path | Geometric: sequence of configs [q₀, q₁, ..., qₙ] — no timing |
| Trajectory | Path + timing: q(t), q̇(t), q̈(t) at each point |
| Trapezoidal profile | Accel → cruise → decel — simple, C¹ continuous |
| TOPP-RA | Time-Optimal Path Parameterization — fastest legal timing for a given path |
| Cubic spline | C² continuity — smooth position AND velocity transitions |
| Quintic spline | C⁴ continuity — also smooth acceleration (used by controllers) |
| SLERP | Spherical Linear Interpolation — smooth rotation between quaternions |
| Velocity scaling | `setMaxVelocityScalingFactor(0.3)` — 30% of joint limits |
| Cartesian path | `computeCartesianPath(waypoints, step)` — straight line in task space |
| `eef_step` | Max distance between interpolated Cartesian points (meters) |
| FollowJointTrajectory | Action: controller interpolates waypoints at 100Hz |
| Full pipeline | Plan → smooth → time parameterize → execute |

---

**Prev:** [Part 20 — Inverse Kinematics](20-inverse-kinematics.md)
**Next:** [Part 22 — MoveIt Bringup & Integration](22-moveit-bringup.md)
**See also:** [Mathematics — Derivatives](../../mathematics/03-derivatives/README.md) — velocity `q̇(t)` and acceleration `q̈(t)` are derivatives of `q(t)`; spline continuity classes C¹ / C² / C⁴ are about derivative continuity
