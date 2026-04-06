# Part 22 — MoveIt Bringup & Integration

## 22.1 — The Analogy

Parts 17-21 covered the theory: configuration space, planning algorithms, inverse kinematics, trajectory generation. This part connects everything to the **actual code** in the repo — how the three MoveIt packages work together and how to launch the full system.

Think of it as the difference between knowing how a car engine works (theory) and actually turning the key (integration).

---

## 22.2 — The Three-Package Pattern

The repo follows the standard MoveIt convention:

```
ros2_moveit_description_template    WHAT the robot IS
├── urdf/arm.xacro                    6-DOF kinematic chain
├── urdf/gripper.xacro                Parallel-jaw gripper
├── urdf/basic_robot.ros2_control.xacro  Hardware interface
└── urdf/arm.urdf.xacro              Top-level assembly

ros2_moveit_config_template         HOW to USE the robot
├── config/basic_robot.srdf           Planning groups, collisions
├── config/kinematics.yaml            IK solver config
├── config/joint_limits.yaml          Velocity/acceleration limits
├── config/moveit_controllers.yaml    MoveIt → controller mapping
├── config/ros2_controllers.yaml      ros2_control definitions
└── launch/*.launch.py                Launch building blocks

ros2_moveit_bringup                 START everything
├── launch/display.launch.xml         Single integrated launch
└── config/ros2_controllers.yaml      Controller config
```

### Why separate?

| Swap this... | Change only... | Rest stays the same |
|---|---|---|
| Real robot for simulation | description package (hardware plugin) | Config + bringup |
| IK solver | config package (kinematics.yaml) | Description + bringup |
| Planner | config package (planning pipeline) | Description + bringup |
| Launch strategy | bringup package | Description + config |

---

## 22.3 — The Robot Description

### Kinematic chain (arm.xacro)

A 6-DOF serial manipulator:

```
base_link → [joint1 Z] → shoulder_link → [joint2 Y] → arm_link
  → [joint3 Y] → elbow_link → [joint4 Z] → forearm_link
  → [joint5 Y] → wrist_link → [joint6 Z] → hand_link → tool_link
```

Joint types follow a standard industrial pattern: **Z-Y-Y-Z-Y-Z** (alternating yaw and pitch), giving full 6-DOF reach.

### Gripper (gripper.xacro)

A parallel-jaw gripper with a **mimic joint**:

```xml
<joint name="gripper_right_finger_joint" type="prismatic">
    <mimic joint="gripper_left_finger_joint" multiplier="-1" offset="0"/>
</joint>
```

Command one finger → the other mirrors it. This reduces the gripper from 2 DOF to 1 DOF for planning purposes.

### Hardware interface (basic_robot.ros2_control.xacro)

```xml
<hardware>
    <plugin>mock_components/GenericSystem</plugin>
</hardware>
```

`mock_components/GenericSystem` loops back commands as state — no real hardware needed. The robot "moves" perfectly in simulation. For a real robot, replace with your own `SystemInterface` plugin (Part 9).

All joints use **position** command/state interfaces. This means MoveIt sends position targets and the hardware reports position feedback.

---

## 22.4 — The SRDF Configuration

The SRDF (Part 17) defines what the planner needs to know:

### Planning groups

```xml
<group name="arm">     <!-- joints 1-6: planned by OMPL/Pilz -->
<group name="gripper">  <!-- finger joints: open/close only -->
```

### Key SRDF elements and their role in planning

| Element | Planning role |
|---------|-------------|
| **Groups** | Defines which joints the planner controls |
| **Named poses** | Precomputed IK solutions ("home", "pose_1", "gripper_open") |
| **End effector** | The gripper is attached to `tool_link` — IK target frame |
| **Virtual joint** | Connects robot to world frame (fixed = stationary arm) |
| **Collision disable** | Speeds up collision checking by skipping impossible collisions |

### Named poses in practice

```
"home":    all joints at 0 — arm straight up
"pose_1":  [0.21, -0.27, -0.74, -2.09, -0.34, 0.0] — a pre-planned reach pose
"pose_2":  [1.57, -0.27, -0.74, -2.09, -0.34, 0.0] — rotated variant

These are cached IK solutions. Using them avoids an IK solve:
  move_group.set_named_target("home")  → directly uses stored joint values
  move_group.set_pose_target(pose)     → must solve IK first
```

---

## 22.5 — Controller Wiring

Two configuration files handle the controller stack:

### ros2_controllers.yaml (ros2_control perspective)

Defines the controller plugins running inside the Controller Manager:

```yaml
controller_manager:
  ros__parameters:
    update_rate: 100    # Hz — 10ms per cycle

arm_controller:
  ros__parameters:
    type: joint_trajectory_controller/JointTrajectoryController
    joints: [joint1, joint2, joint3, joint4, joint5, joint6]
    command_interfaces: [position]
    state_interfaces: [position]

gripper_controller:
  ros__parameters:
    type: joint_trajectory_controller/JointTrajectoryController
    joints: [gripper_left_finger_joint]
    command_interfaces: [position]
    state_interfaces: [position]
```

### moveit_controllers.yaml (MoveIt perspective)

Tells MoveIt which controllers to send trajectories to:

```yaml
arm_controller:
  type: FollowJointTrajectory
  action_ns: follow_joint_trajectory
  joints: [joint1, joint2, joint3, joint4, joint5, joint6]

gripper_controller:
  type: FollowJointTrajectory
  action_ns: follow_joint_trajectory
  joints: [gripper_left_finger_joint]
```

### How they connect

```
MoveIt move_group
    |
    | "Execute trajectory for arm group"
    |
    | Looks up moveit_controllers.yaml:
    |   arm → arm_controller, type FollowJointTrajectory
    |
    | Sends action goal to:
    |   /arm_controller/follow_joint_trajectory
    |
    v
arm_controller (JointTrajectoryController, ros2_control)
    |
    | Interpolates waypoints at 100Hz
    | Writes to command interfaces
    |
    v
mock_components/GenericSystem
    |
    | command → state (loopback)
```

---

## 22.6 — The Launch Sequence

`ros2_moveit_bringup/launch/display.launch.xml` starts the full system:

```
ros2 launch ros2_moveit_bringup display.launch.xml

t=0.0s ─── robot_state_publisher ─────────────────────────────
            │ Processes arm.urdf.xacro → URDF string
            │ Publishes /robot_description (latched)
            │ Broadcasts TF for fixed joints (base→tool_link chain)
            
t=0.0s ─── ros2_control_node ────────────────────────────────
            │ Reads /robot_description
            │ Finds <ros2_control> tag → loads mock_components/GenericSystem
            │ on_init(): parses joint definitions (7 joints)
            │ on_configure(): initializes mock hardware (all zeros)
            │ Starts 100Hz loop: read() → update() → write()

t=0.5s ─── spawner: joint_state_broadcaster ──────────────────
            │ Loaded into Controller Manager → on_activate()
            │ Publishes /joint_states at 100Hz

t=0.5s ─── spawner: arm_controller ──────────────────────────
            │ JointTrajectoryController for joint1-6
            │ Exposes /arm_controller/follow_joint_trajectory action
            │ Waits for trajectory goals

t=0.5s ─── spawner: gripper_controller ───────────────────────
            │ JointTrajectoryController for gripper_left_finger_joint
            │ Exposes /gripper_controller/follow_joint_trajectory action

t=1.0s ─── move_group (from config_template) ─────────────────
            │ Loads: SRDF, kinematics, joint limits, planner config
            │ Initializes OMPL + Pilz planning pipelines
            │ Starts MoveGroup action server (/move_action)
            │ Connects to arm_controller + gripper_controller
            │ Ready to accept planning requests

t=1.0s ─── rviz2 ────────────────────────────────────────────
            │ Loads MoveIt RViz config
            │ Subscribes to /tf, /joint_states, /robot_description
            │ MotionPlanning plugin connects to move_group
            │ Interactive markers for drag-and-plan
```

---

## 22.7 — The Execution Flow (End to End)

A complete "plan and move" cycle:

```
Step 1: User drags interactive marker in RViz to target pose
         target = [x, y, z, qx, qy, qz, qw]

Step 2: User clicks "Plan"
         RViz → MoveGroup action: plan_only=true

Step 3: move_group processes the request:
         a. Get current joint state from /joint_states
            q_start = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
         
         b. Solve IK for the target pose (Part 20)
            q_goal = IK(target) = [0.8, -0.3, -0.5, 0.2, 0.3, 0.0]
            Solver: CachedSrvKinematicsPlugin → KDL
            Time: ~5ms
         
         c. Plan path in C-space (Part 19)
            Planner: RRTConnect
            Collision checking: FCL with ACM
            Time: ~50ms
            Result: path = [q₀, q₁, ..., qₙ] (20 waypoints)
         
         d. Smooth path (Part 19.9)
            Shortcutting + B-spline
            Result: path = [q₀, q₃, q₇, qₙ] (4 waypoints)
         
         e. Time parameterization (Part 21)
            Method: TOPP-RA
            Constraints: v_max = 1.0 rad/s, a_max = 1.0 rad/s²
            Result: trajectory with timing for each waypoint

Step 4: RViz visualizes planned trajectory (ghost arm follows path)

Step 5: User clicks "Execute"
         RViz → MoveGroup action: plan_and_execute=true
         (or reuses the previously planned trajectory)

Step 6: move_group sends trajectory to arm_controller
         FollowJointTrajectory action goal with N waypoints

Step 7: arm_controller executes at 100Hz:
         - Spline interpolation between waypoints
         - Writes position commands to hardware interfaces
         - Hardware (mock) updates state immediately
         - Feedback published: actual vs desired positions

Step 8: Execution completes
         arm_controller → move_group: SUCCESSFUL
         move_group → RViz: action result = success
```

---

## 22.8 — ROS2 Topics and Actions at Runtime

```bash
# Topics
/robot_description          # URDF string (latched)
/joint_states               # Current joint positions (100Hz)
/tf                         # Transform tree (100Hz)
/tf_static                  # Static transforms (once)
/planning_scene             # Current obstacles + robot state
/display_planned_path       # Trajectory for RViz visualization

# Actions
/move_action                         # MoveGroup plan+execute
/arm_controller/follow_joint_trajectory     # Arm trajectory execution
/gripper_controller/follow_joint_trajectory  # Gripper trajectory execution

# Services
/compute_ik                 # Solve inverse kinematics
/compute_fk                 # Solve forward kinematics
/get_planning_scene         # Query current scene
/apply_planning_scene       # Add/remove collision objects
```

---

## 22.9 — Useful Commands

```bash
# Launch the full system
ros2 launch ros2_moveit_bringup display.launch.xml

# Check controller status
ros2 control list_controllers
# Expected:
#   joint_state_broadcaster  [active]
#   arm_controller           [active]
#   gripper_controller       [active]

# Check hardware interfaces
ros2 control list_hardware_interfaces
# Expected:
#   command interfaces:
#     joint1/position [claimed]
#     joint2/position [claimed]
#     ...
#   state interfaces:
#     joint1/position
#     joint2/position
#     ...

# Move to a named pose (from SRDF)
ros2 action send_goal /move_action moveit_msgs/action/MoveGroup \
  "{request: {group_name: 'arm', named_target: 'home'}}"

# Add a collision box to the scene
ros2 service call /apply_planning_scene moveit_msgs/srv/ApplyPlanningScene \
  "{scene: {world: {collision_objects: [{id: 'table', 
    primitives: [{type: 1, dimensions: [1.0, 0.5, 0.02]}],
    primitive_poses: [{position: {x: 0.5, y: 0.0, z: 0.4}}],
    operation: 0}]}}}"
```

---

## 22.10 — Summary: The Full Stack

```
Layer 5: Application
  ├── RViz MotionPlanning plugin
  ├── Python MoveGroupInterface
  └── Custom nodes

Layer 4: MoveIt (move_group)
  ├── Planning pipeline (OMPL / Pilz)
  ├── IK solver (KDL / IKFast / TRAC-IK)
  ├── Collision checking (FCL + ACM)
  ├── Time parameterization (TOPP-RA)
  └── Planning scene management

Layer 3: Controller interface
  ├── FollowJointTrajectory action
  ├── JointTrajectoryController (arm + gripper)
  └── JointStateBroadcaster

Layer 2: ros2_control
  ├── Controller Manager (100Hz loop)
  ├── ResourceManager (interface maps)
  └── Hardware interface (mock or real)

Layer 1: Hardware
  └── mock_components/GenericSystem (or real servos)
```

Each layer only talks to its neighbors. Swap any layer without touching the others — that's the power of the ROS2 + MoveIt + ros2_control architecture.

---

**Prev:** [Part 21 — Trajectory Generation](21-trajectory-generation.md)
