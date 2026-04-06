# Part 17 — MoveIt: The Big Picture

## 17.1 — The Analogy

ros2_control (Parts 8-16) gives your robot **reflexes** — a fast, deterministic loop that reads sensors and writes commands 50-100 times per second. But reflexes alone don't solve the problem: "move the arm from here to there without hitting the table."

**MoveIt** is the **brain**. It answers three questions:
1. **Where can the arm go?** (configuration space, collision checking)
2. **How does it get there?** (motion planning, path search)
3. **When does each joint move?** (trajectory generation, time parameterization)

Think of it like a GPS navigator:
- The **map** is the configuration space (all possible joint positions)
- The **obstacles** are collisions (self-collision + environment)
- The **route planner** is the motion planning algorithm (RRT, PRM, etc.)
- The **speed profile** is the trajectory (how fast to drive each segment)

---

## 17.2 — Architecture Overview

```
+---------------------------------------------------------------+
|                      YOUR APPLICATION                          |
|  Python: MoveGroupInterface    C++: MoveGroupInterface        |
|  RViz: MotionPlanning plugin   CLI: ros2 action send_goal     |
+-------------------------------+-------------------------------+
                                |
                     MoveGroup Action Server
                     (plan, execute, or both)
                                |
                                v
+---------------------------------------------------------------+
|                       move_group NODE                          |
|                                                                |
|  +------------------+  +------------------+  +-------------+  |
|  | Planning Pipeline |  | Planning Scene   |  | Trajectory  |  |
|  |                   |  |                  |  | Execution   |  |
|  | 1. OMPL           |  | - Robot state    |  |             |  |
|  | 2. Pilz           |  | - World geometry |  | Sends to    |  |
|  | 3. CHOMP          |  | - ACM            |  | controllers |  |
|  +------------------+  +------------------+  +-------------+  |
|           |                     |                    |         |
+---------------------------------------------------------------+
            |                     |                    |
            v                     v                    v
     Collision checking     /robot_description   FollowJointTrajectory
     (FCL library)          /planning_scene        action to
                                                   arm_controller
                                                        |
                                                        v
                                              +-------------------+
                                              | ros2_control      |
                                              | (Parts 8-16)      |
                                              +-------------------+
```

---

## 17.3 — MoveIt vs ros2_control

| | ros2_control | MoveIt |
|---|---|---|
| **Question answered** | "How to talk to hardware" | "Where to move" |
| **Loop rate** | 50-1000 Hz (real-time) | On-demand (seconds) |
| **Input** | Velocity/position commands | Goal pose |
| **Output** | Electrical signals | Joint trajectory |
| **Deterministic?** | Yes (fixed time budget) | No (planning can take variable time) |
| **Collision aware?** | No | Yes |

They are complementary: MoveIt **plans** a trajectory, then hands it to ros2_control to **execute** it.

```
MoveIt:       "Go from home to target in 2.5 seconds"
                |
                v
              [joint1: 0.0 → 0.5 → 0.8 → 1.0]
              [joint2: 0.0 → -0.1 → -0.2 → -0.3]
              [time:   0.0 → 0.8 → 1.6 → 2.5]
                |
                v
ros2_control: interpolates at 100 Hz, sends to motors
```

---

## 17.4 — The Planning Scene

The Planning Scene is MoveIt's **model of reality**. It contains:

### 1. Robot state
The current joint positions of every joint in the robot.

### 2. World geometry
Objects in the environment that the robot must avoid:
- **Collision objects**: boxes, cylinders, meshes added by the user
- **Octomap**: 3D voxel grid built from depth cameras
- **Attached objects**: objects the robot is holding (move with the gripper)

### 3. Allowed Collision Matrix (ACM)
A boolean matrix that says which link pairs **don't need** collision checking:

```
              base  shoulder  arm  elbow  forearm  wrist  hand  gripper_L  gripper_R
base           -      ✗       ✓     ✓      ✓       ✓      ✓      ✓          ✓
shoulder       ✗      -       ✗     ✓      ✓       ✓      ✓      ✓          ✓
arm            ✓      ✗       -     ✗      ✓       ✓      ✓      ✓          ✓
elbow          ✓      ✓       ✗     -      ✗       ✓      ✓      ✓          ✓
forearm        ✓      ✓       ✓     ✗      -       ✗      ✓      ✓          ✓
wrist          ✓      ✓       ✓     ✓      ✗       -      ✗      ✓          ✓
hand           ✓      ✓       ✓     ✓      ✓       ✗      -      ✗          ✗

✗ = collision check disabled (adjacent or never collide)
✓ = collision check enabled (could collide in some config)
```

This is generated by the **MoveIt Setup Assistant**, which samples thousands of random joint configurations, tests all link pairs for collision, and disables pairs that never collide.

The ACM is critical for performance: a 7-link arm has 21 link pairs. Without the ACM, every planning sample would check all 21. With it, only ~8 need checking.

---

## 17.5 — The SRDF (Semantic Robot Description)

The URDF describes **geometry**. The SRDF adds **meaning**:

| URDF | SRDF |
|------|------|
| Link shapes and positions | Which joints form the "arm" group |
| Joint limits | Named poses ("home", "ready") |
| Materials | End effector definition |
| Mass and inertia | Collision disable rules |

### Planning groups

A group is a kinematic chain that MoveIt plans together:

```
"arm" group:  joint1 → joint2 → joint3 → joint4 → joint5 → joint6
              (6 DOF serial chain — base_link to tool_link)

"gripper" group:  gripper_left_finger_joint + gripper_right_finger_joint
                  (parallel jaw — planned separately from arm)
```

Why separate? The arm has 6 DOF and needs complex motion planning. The gripper has 1 DOF (open/close) — no planning needed, just a direct command.

### Virtual joint

Anchors the robot to the world:

```xml
<virtual_joint name="virtual_joint" type="fixed" parent_frame="world" child_link="base_link"/>
```

For a fixed-base arm: `type="fixed"` (the arm doesn't move through space).
For a mobile manipulator: `type="floating"` (the arm rides on a mobile base).

### End effector

```xml
<end_effector name="gripper" parent_link="tool_link" group="gripper" parent_group="arm"/>
```

This tells MoveIt: "When I plan for the arm, the gripper is the tool at the tip. When I add an attached collision object, attach it to tool_link."

---

## 17.6 — The move_group Node

The `move_group` node is the central process. It provides:

### Action servers

| Action | Purpose |
|--------|---------|
| `/move_action` (MoveGroup) | Plan + execute in one call |
| `/arm_controller/follow_joint_trajectory` | Execute a pre-computed trajectory |

### Services

| Service | Purpose |
|---------|---------|
| `/compute_ik` | Solve inverse kinematics |
| `/compute_fk` | Solve forward kinematics |
| `/get_planning_scene` | Get current scene state |
| `/apply_planning_scene` | Add/remove collision objects |

### The MoveGroup action flow

```
1. Client sends goal:
   - target pose (or joint values)
   - planning group ("arm")
   - planner ("RRTConnect")
   - planning time (5.0s)

2. move_group:
   a. Gets current robot state from /joint_states
   b. Gets current planning scene (obstacles)
   c. Calls the planning pipeline:
      - Planner finds a path in C-space
      - Time parameterization adds velocity/acceleration profile
   d. Validates: no collisions along entire trajectory
   e. Sends trajectory to arm_controller via FollowJointTrajectory

3. arm_controller:
   - Interpolates waypoints at 100Hz
   - Writes position commands to hardware interfaces

4. move_group monitors execution:
   - Feedback: current progress
   - Result: success / failure / preempted
```

---

## 17.7 — Example: First MoveIt Python Script

The minimal script to connect to MoveIt and move the arm. It ties together everything above: the move_group node, planning groups, and the plan → execute flow.

```python
#!/usr/bin/env python3
"""move_to_home.py — Send the arm to the 'home' pose defined in the SRDF."""

import rclpy
from moveit.planning import MoveItPy

def main():
    rclpy.init()

    # 1. Connect to MoveIt (waits for move_group to be ready)
    moveit = MoveItPy(node_name="example_node")

    # 2. Get the "arm" planning group (defined in the SRDF)
    arm = moveit.get_planning_component("arm")

    # 3. Set the target: the named pose "home" (all joints at 0)
    arm.set_start_state_to_current_state()
    arm.set_goal_state(configuration_name="home")

    # 4. Plan (RRTConnect by default)
    plan_result = arm.plan()

    if plan_result:
        # 5. Execute the trajectory on the real robot (or mock hardware)
        robot_trajectory = plan_result.trajectory
        moveit.execute(robot_trajectory, controllers=[])
        print("Motion complete!")
    else:
        print("Planning failed")

    rclpy.shutdown()

if __name__ == "__main__":
    main()
```

### What happens under the hood:

```
arm.set_goal_state("home")
  → Reads the SRDF: home = [0, 0, 0, 0, 0, 0]
  → No IK needed (joint values are stored directly in the SRDF)

arm.plan()
  → Gets q_start from /joint_states
  → Calls OMPL RRTConnect in C-space (Part 18-19)
  → Smoothing + time parameterization (Part 21)
  → Returns a trajectory with timings

moveit.execute(trajectory)
  → Sends FollowJointTrajectory to arm_controller
  → Controller interpolates at 100Hz and writes to command interfaces
```

---

## 17.8 — What's Coming Next

The following parts dive into the math and algorithms behind each component:

| Part | Topic | Core math |
|------|-------|-----------|
| 18 | Configuration Space | C-space topology, dimensionality, obstacle mapping |
| 19 | Motion Planning Algorithms | RRT, PRM, graph search, probabilistic completeness |
| 20 | Inverse Kinematics | Jacobian, numerical IK, singularities |
| 21 | Trajectory Generation | Time parameterization, TOPP-RA, velocity profiles |
| 22 | Bringup & Integration | Launching the full system, controller wiring |

---

## 17.9 — C++ Equivalent: MoveGroupInterface

The Python example above can also be written in C++:

```cpp
#include <rclcpp/rclcpp.hpp>
#include <moveit/move_group_interface/move_group_interface.h>

int main(int argc, char** argv)
{
    rclcpp::init(argc, argv);
    auto node = rclcpp::Node::make_shared("example_node");

    // 1. Connect to MoveIt (planning group "arm" from SRDF)
    moveit::planning_interface::MoveGroupInterface arm(node, "arm");

    // 2. Set target: named pose "home"
    arm.setNamedTarget("home");

    // 3. Plan
    moveit::planning_interface::MoveGroupInterface::Plan plan;
    bool success = (arm.plan(plan) == moveit::core::MoveItErrorCode::SUCCESS);

    if (success) {
        // 4. Execute
        arm.execute(plan);
        RCLCPP_INFO(node->get_logger(), "Motion complete!");
    } else {
        RCLCPP_ERROR(node->get_logger(), "Planning failed");
    }

    rclcpp::shutdown();
    return 0;
}
```

CMakeLists.txt additions:
```cmake
find_package(moveit_ros_planning_interface REQUIRED)
add_executable(move_to_home src/move_to_home.cpp)
ament_target_dependencies(move_to_home rclcpp moveit_ros_planning_interface)
```

---

## 17.10 — Quick Reference

| Concept | Key Point |
|---|---|
| MoveIt2 | Motion planning framework — sits above ros2_control |
| `move_group` node | Central orchestrator: planning + execution |
| Planning group | Set of joints that move together (defined in SRDF) |
| SRDF | Semantic info: groups, named poses, end effectors, disabled collisions |
| ACM | Allowed Collision Matrix — skip known-safe collision pairs |
| Planning scene | World model = URDF + SRDF + obstacles + ACM |
| Plan flow | `set_goal → plan() → execute()` |
| Action interface | `FollowJointTrajectory` — controller executes the trajectory |
| C++ API | `MoveGroupInterface arm(node, "group")` → `setNamedTarget()` → `plan()` → `execute()` |
| Python API | `MoveItPy(node_name="...")` → `get_planning_component("group")` |

---

**Prev:** [Part 16 — Transmissions, Sensors & GPIO](16-transmissions-sensors-gpio.md)
**Next:** [Part 18 — Configuration Space](18-configuration-space.md)
