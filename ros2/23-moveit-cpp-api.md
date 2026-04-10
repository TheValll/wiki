# Part 23 — MoveIt C++ API (MoveGroupInterface)

## 23.1 — The Analogy

Part 22 showed how to **start** the full MoveIt stack — robot_state_publisher, controllers, move_group, RViz. But clicking buttons in RViz is not how a real application controls a robot.

Think of Part 22 as learning to drive with an instructor (RViz) who moves the steering wheel for you. This part is about **taking the wheel yourself** — writing C++ code that commands the arm programmatically.

The `MoveGroupInterface` is your steering wheel, gear stick, and GPS all in one: you tell it where to go (named pose, joint values, or Cartesian pose) and it handles the planning, collision checking, and execution.

---

## 23.2 — Package Setup

### package.xml

The only MoveIt dependency needed for the C++ API:

```xml
<depend>rclcpp</depend>
<depend>moveit_ros_planning_interface</depend>
```

`moveit_ros_planning_interface` pulls in everything: `MoveGroupInterface`, planning pipeline, trajectory execution, IK solvers.

### CMakeLists.txt

```cmake
find_package(ament_cmake REQUIRED)
find_package(rclcpp REQUIRED)
find_package(moveit_ros_planning_interface REQUIRED)

add_executable(test_moveit src/test_moveit.cpp)
ament_target_dependencies(test_moveit rclcpp moveit_ros_planning_interface)

install(TARGETS test_moveit DESTINATION lib/${PROJECT_NAME}/)
```

No need to link individual MoveIt libraries — `ament_target_dependencies` resolves the full dependency chain.

### Header

```cpp
#include <moveit/move_group_interface/move_group_interface.hpp>
```

Note the `.hpp` extension — MoveIt2 uses `.hpp` headers (not `.h` like some older tutorials show).

---

## 23.3 — Creating a MoveGroupInterface

### The executor pattern

`MoveGroupInterface` needs a spinning node to receive feedback from action servers. If you create the interface in `main()` before spinning, you need a background executor:

```cpp
int main(int argc, char **argv)
{
  rclcpp::init(argc, argv);
  auto node = std::make_shared<rclcpp::Node>("test_moveit");

  // Spin in background — MoveGroupInterface needs callbacks to work
  rclcpp::executors::SingleThreadedExecutor executor;
  executor.add_node(node);
  auto spinner = std::thread([&executor]() { executor.spin(); });

  // Now safe to create the interface
  auto arm = moveit::planning_interface::MoveGroupInterface(node, "arm");

  // ... plan and execute ...

  rclcpp::shutdown();
  spinner.join();
  return 0;
}
```

### Why the background thread?

```
MoveGroupInterface::plan()
    |
    | Sends MoveGroup action goal
    | Waits for result
    |                          BUT action feedback/result arrive via callbacks
    |                          → node must be spinning to process them
    |
    v
Without spin → plan() hangs forever (deadlock)
With spin    → callbacks fire → plan() returns
```

### Alternative: class-based pattern

If the node is already spinning (e.g., from a launch file with `rclcpp::spin(node)`), you can create the interface directly in the constructor:

```cpp
class CommanderNode
{
public:
  CommanderNode(std::shared_ptr<rclcpp::Node> node)
  {
    arm_ = std::make_shared<MoveGroupInterface>(node, "arm");
    arm_->setMaxVelocityScalingFactor(1.0);
    arm_->setMaxAccelerationScalingFactor(1.0);
  }
private:
  std::shared_ptr<MoveGroupInterface> arm_;
};

int main(int argc, char **argv)
{
  rclcpp::init(argc, argv);
  auto node = std::make_shared<rclcpp::Node>("CommanderNode");
  auto commander = CommanderNode(node);
  rclcpp::spin(node);   // Spin handles callbacks — no extra thread needed
  rclcpp::shutdown();
}
```

This works because `spin(node)` runs the event loop, and the `MoveGroupInterface` methods are called from subscription callbacks (which are themselves dispatched by the executor).

---

## 23.4 — The Plan-Execute Pattern

Every movement follows the same three steps:

```cpp
// 1. Set start state (where the robot is NOW)
arm.setStartStateToCurrentState();

// 2. Set goal (where the robot should GO)
arm.setNamedTarget("home");     // one of several goal types

// 3. Plan and execute
moveit::planning_interface::MoveGroupInterface::Plan plan;
bool success = (arm.plan(plan) == moveit::core::MoveItErrorCode::SUCCESS);

if (success) {
  arm.execute(plan);
}
```

### What each step does

```
setStartStateToCurrentState()
  → Queries /joint_states
  → Sets q_start = current joint values
  → Without this: planner may use stale start state from last plan

setNamedTarget("home")
  → Looks up "home" in SRDF → q_goal = [0, 0, 0, 0, 0, 0]
  → Stored as the planning goal (no IK needed for named targets)

plan(plan)
  → Calls OMPL RRTConnect (default planner)
  → Checks collisions (FCL + ACM) at every sample
  → Smooths path (shortcutting + B-spline)
  → Time parameterization (TOPP-RA)
  → Returns SUCCESS or FAILURE

execute(plan)
  → Sends FollowJointTrajectory action to arm_controller
  → Waits for completion
  → Returns SUCCESS or FAILURE
```

### Reusable helper

```cpp
void planAndExecute(const std::shared_ptr<MoveGroupInterface> &interface)
{
  MoveGroupInterface::Plan plan;
  bool success = (interface->plan(plan) == moveit::core::MoveItErrorCode::SUCCESS);

  if (success) {
    interface->execute(plan);
  }
}
```

---

## 23.5 — Goal Types

MoveIt supports four ways to specify where the arm should go. Each has different trade-offs:

### A. Named target (from SRDF)

```cpp
arm.setStartStateToCurrentState();
arm.setNamedTarget("pose_1");
planAndExecute(arm);
```

Uses the joint values stored in the SRDF (see Part 22.4). **No IK solve needed** — fastest goal type.

Available named targets depend on the SRDF:
```
"home"         → all joints at 0 (arm straight up)
"pose_1"       → [0.21, -0.27, -0.74, -2.09, -0.34, 0.0]
"pose_2"       → [1.57, -0.27, -0.74, -2.09, -0.34, 0.0]
"gripper_open"  → gripper fully open
"gripper_close" → gripper fully closed
```

### B. Joint value target

```cpp
std::vector<double> joints = {1.5, 0.5, 0.0, 1.5, 0.0, -0.7};

arm.setStartStateToCurrentState();
arm.setJointValueTarget(joints);
planAndExecute(arm);
```

Directly specifies joint angles in radians. Like a named target but with arbitrary values. **No IK solve needed** — the goal is already in joint space.

### C. Pose target (Cartesian goal)

```cpp
tf2::Quaternion q;
q.setRPY(3.14, 0.0, 0.0);   // roll, pitch, yaw → quaternion
q = q.normalize();

geometry_msgs::msg::PoseStamped target_pose;
target_pose.header.frame_id = "base_link";
target_pose.pose.position.x = 0.0;
target_pose.pose.position.y = -0.7;
target_pose.pose.position.z = 0.4;
target_pose.pose.orientation.x = q.getX();
target_pose.pose.orientation.y = q.getY();
target_pose.pose.orientation.z = q.getZ();
target_pose.pose.orientation.w = q.getW();

arm.setStartStateToCurrentState();
arm.setPoseTarget(target_pose);
planAndExecute(arm);
```

Specifies the **end-effector pose** in Cartesian space. The planner must solve IK (Part 20) to convert this into joint values before planning the path.

#### Orientation with tf2::Quaternion

Orientations in ROS use quaternions (not Euler angles). `tf2::Quaternion::setRPY()` converts human-readable roll/pitch/yaw to quaternion form:

```
setRPY(3.14, 0.0, 0.0)
  → roll = 3.14 (π rad = 180°)  → end effector points DOWN
  → pitch = 0.0
  → yaw = 0.0
  → quaternion ≈ [x=1.0, y=0.0, z=0.0, w≈0.0]

Always normalize: q = q.normalize()
  → Avoids numerical drift causing invalid quaternions
```

#### PoseStamped vs Pose

```
PoseStamped = Pose + Header(frame_id, timestamp)
  - frame_id = "base_link" → target is relative to the robot base
  - Without frame_id, MoveIt doesn't know which coordinate system the pose is in
```

### D. Cartesian path (waypoint following)

```cpp
std::vector<geometry_msgs::msg::Pose> waypoints;

// Start from current end-effector position
geometry_msgs::msg::Pose pose1 = arm.getCurrentPose().pose;
pose1.position.z += -0.2;    // move 20cm down
waypoints.push_back(pose1);

geometry_msgs::msg::Pose pose2 = pose1;
pose2.position.y += 0.2;     // move 20cm to the side
waypoints.push_back(pose2);

geometry_msgs::msg::Pose pose3 = pose2;
pose3.position.y += -0.2;    // come back
pose3.position.z += 0.2;     // move back up
waypoints.push_back(pose3);

moveit_msgs::msg::RobotTrajectory trajectory;
double fraction = arm.computeCartesianPath(waypoints, 0.01, trajectory);

if (fraction == 1) {
  arm.execute(trajectory);
}
```

#### How computeCartesianPath works

```
computeCartesianPath(waypoints, eef_step, trajectory)
                         |          |         |
                         |          |         └─ Output: the computed trajectory
                         |          └─ Max step between interpolation points (meters)
                         └─ Ordered list of end-effector poses to follow

The method:
1. Interpolates between consecutive waypoints in Cartesian space
2. At each interpolation point (every 0.01m = 1cm):
   - Solves IK → joint values
   - Checks collision
   - If IK fails or collision → stops, returns partial fraction
3. Returns fraction of the path that succeeded (0.0 to 1.0)

fraction = 1.0 → entire path is valid, trajectory is complete
fraction < 1.0 → partial path (collision or IK failure midway)
```

#### Cartesian path vs pose target

| | `setPoseTarget` | `computeCartesianPath` |
|---|---|---|
| **Path shape** | Free (planner decides) | Straight lines in Cartesian space |
| **Waypoints** | Just start and goal | Multiple intermediate points |
| **Use case** | "Get there somehow" | "Follow this exact path" (welding, painting) |
| **IK needed** | Only at goal | At every interpolation point |
| **Can fail mid-path** | No (all-or-nothing) | Yes (returns fraction) |

---

## 23.6 — Velocity and Acceleration Scaling

```cpp
arm.setMaxVelocityScalingFactor(1.0);       // 100% of joint_limits.yaml values
arm.setMaxAccelerationScalingFactor(1.0);    // 100% of joint_limits.yaml values
```

The scaling factor multiplies the limits from `joint_limits.yaml`:

```
joint_limits.yaml:  max_velocity = 1.0 rad/s
scaling factor:     0.5
effective limit:    0.5 rad/s

Useful for:
  0.1 → slow, cautious movements (testing, near obstacles)
  0.5 → normal operation
  1.0 → full speed (production, no humans nearby)
```

---

## 23.7 — Multi-Group Management (Arm + Gripper)

A robot with an arm and a gripper has **two planning groups** (Part 17.5). Each gets its own `MoveGroupInterface`:

```cpp
auto arm = std::make_shared<MoveGroupInterface>(node, "arm");
auto gripper = std::make_shared<MoveGroupInterface>(node, "gripper");
```

### Gripper control

The gripper uses named targets because it only has two useful states:

```cpp
void openGripper()
{
  gripper->setStartStateToCurrentState();
  gripper->setNamedTarget("gripper_open");
  planAndExecute(gripper);
}

void closeGripper()
{
  gripper->setStartStateToCurrentState();
  gripper->setNamedTarget("gripper_close");
  planAndExecute(gripper);
}
```

### Typical pick sequence

```cpp
arm->setNamedTarget("pre_grasp");   planAndExecute(arm);      // 1. Move above object
openGripper();                                                  // 2. Open gripper
arm->setNamedTarget("grasp");       planAndExecute(arm);      // 3. Lower to object
closeGripper();                                                 // 4. Close gripper
arm->setNamedTarget("home");        planAndExecute(arm);      // 5. Lift and retreat
```

---

## 23.8 — Commander Node: Callback-Based Control

The project includes a full commander node that accepts movement commands via ROS2 topics. This is the pattern for integrating MoveIt into a larger application:

### Full C++ code — `commander_template.cpp` (complete, compilable):

```cpp
#include <rclcpp/rclcpp.hpp>
#include <moveit/move_group_interface/move_group_interface.hpp>
#include <example_interfaces/msg/bool.hpp>
#include <example_interfaces/msg/string.hpp>
#include <std_msgs/msg/float64_multi_array.hpp>

using MoveGroupInterface = moveit::planning_interface::MoveGroupInterface;
using Bool = example_interfaces::msg::Bool;
using String = example_interfaces::msg::String;
using Float64MultiArray = std_msgs::msg::Float64MultiArray;
using namespace std::placeholders;

class CommanderNode
{
public:
  CommanderNode(std::shared_ptr<rclcpp::Node> node)
  {
    node_ = node;
    arm_ = std::make_shared<MoveGroupInterface>(node_, "arm");
    arm_->setMaxVelocityScalingFactor(1.0);
    arm_->setMaxAccelerationScalingFactor(1.0);

    gripper_ = std::make_shared<MoveGroupInterface>(node_, "gripper");

    open_gripper_sub_ = node->create_subscription<Bool>(
      "open_gripper", 10,
      std::bind(&CommanderNode::openGripperCallback, this, _1));

    named_target_sub_ = node->create_subscription<String>(
      "named_target", 10,
      std::bind(&CommanderNode::namedTargetCallback, this, _1));

    joint_target_sub_ = node->create_subscription<Float64MultiArray>(
      "joint_target", 10,
      std::bind(&CommanderNode::jointTargetCallback, this, _1));
  }

  void goToNamedTarget(const std::string &name)
  {
    arm_->setStartStateToCurrentState();
    arm_->setNamedTarget(name);
    planAndExecute(arm_);
  }

  void goToJointTarget(const std::vector<double> &joints)
  {
    arm_->setStartStateToCurrentState();
    arm_->setJointValueTarget(joints);
    planAndExecute(arm_);
  }

  void goToPoseTarget(double x, double y, double z,
                      double roll, double pitch, double yaw,
                      bool cartesian_path = false)
  {
    tf2::Quaternion q;
    q.setRPY(roll, pitch, yaw);
    q = q.normalize();

    geometry_msgs::msg::PoseStamped target_pose;
    target_pose.header.frame_id = "base_link";
    target_pose.pose.position.x = x;
    target_pose.pose.position.y = y;
    target_pose.pose.position.z = z;
    target_pose.pose.orientation.x = q.getX();
    target_pose.pose.orientation.y = q.getY();
    target_pose.pose.orientation.z = q.getZ();
    target_pose.pose.orientation.w = q.getW();

    arm_->setStartStateToCurrentState();

    if (!cartesian_path) {
      arm_->setPoseTarget(target_pose);
      planAndExecute(arm_);
    } else {
      std::vector<geometry_msgs::msg::Pose> waypoints;
      waypoints.push_back(target_pose.pose);
      moveit_msgs::msg::RobotTrajectory trajectory;
      double fraction = arm_->computeCartesianPath(waypoints, 0.01, trajectory);

      if (fraction == 1) {
        arm_->execute(trajectory);
      }
    }
  }

  void openGripper()
  {
    gripper_->setStartStateToCurrentState();
    gripper_->setNamedTarget("gripper_open");
    planAndExecute(gripper_);
  }

  void closeGripper()
  {
    gripper_->setStartStateToCurrentState();
    gripper_->setNamedTarget("gripper_close");
    planAndExecute(gripper_);
  }

private:
  void planAndExecute(const std::shared_ptr<MoveGroupInterface> &interface)
  {
    MoveGroupInterface::Plan plan;
    bool success = (interface->plan(plan) == moveit::core::MoveItErrorCode::SUCCESS);

    if (success) {
      interface->execute(plan);
    }
  }

  void namedTargetCallback(const String &msg)
  {
    goToNamedTarget(msg.data);
  }

  void jointTargetCallback(const Float64MultiArray &msg)
  {
    goToJointTarget(msg.data);
  }

  void openGripperCallback(const Bool &msg)
  {
    if (msg.data) {
      openGripper();
    } else {
      closeGripper();
    }
  }

  std::shared_ptr<rclcpp::Node> node_;
  std::shared_ptr<MoveGroupInterface> arm_;
  std::shared_ptr<MoveGroupInterface> gripper_;

  rclcpp::Subscription<Bool>::SharedPtr open_gripper_sub_;
  rclcpp::Subscription<String>::SharedPtr named_target_sub_;
  rclcpp::Subscription<Float64MultiArray>::SharedPtr joint_target_sub_;
};

int main(int argc, char **argv)
{
  rclcpp::init(argc, argv);
  auto node = std::make_shared<rclcpp::Node>("CommanderNode");
  auto commander = CommanderNode(node);
  rclcpp::spin(node);
  rclcpp::shutdown();
  return 0;
}
```

### Architecture of the commander node

```
                    ROS2 Topics
                        |
        +---------------+---------------+
        |               |               |
  /named_target   /joint_target   /open_gripper
  (String)        (Float64Multi)  (Bool)
        |               |               |
        v               v               v
+-----------------------------------------------+
|              CommanderNode                     |
|                                                |
|  namedTargetCallback()  jointTargetCallback()  |
|       |                       |                |
|       v                       v                |
|  goToNamedTarget()     goToJointTarget()       |
|       |                       |                |
|       +-------+-------+-------+                |
|               |                                |
|        planAndExecute()                        |
|               |                                |
|    +----------+----------+                     |
|    |                     |                     |
|  arm_ (MoveGroupInterface)                     |
|  gripper_ (MoveGroupInterface)                 |
+-----------------------------------------------+
                |
                v
        FollowJointTrajectory action
                |
                v
         arm_controller / gripper_controller
```

### Commanding from the terminal

```bash
# Move to a named SRDF pose
ros2 topic pub --once /named_target example_interfaces/msg/String "{data: 'home'}"

# Move to specific joint values (6 joints, in radians)
ros2 topic pub --once /joint_target std_msgs/msg/Float64MultiArray \
  "{data: [1.5, 0.5, 0.0, 1.5, 0.0, -0.7]}"

# Open gripper
ros2 topic pub --once /open_gripper example_interfaces/msg/Bool "{data: true}"

# Close gripper
ros2 topic pub --once /open_gripper example_interfaces/msg/Bool "{data: false}"
```

---

## 23.9 — Common Pitfalls

| Problem | Cause | Fix |
|---------|-------|-----|
| `plan()` hangs forever | Node not spinning | Add background executor thread or use `rclcpp::spin()` |
| Planning fails (no path found) | Goal in collision or unreachable | Check SRDF named poses, check joint limits, visualize in RViz |
| `computeCartesianPath` returns fraction < 1 | IK failure or collision along the path | Increase eef_step, simplify waypoints, check workspace limits |
| Quaternion error | Non-normalized quaternion | Always call `q.normalize()` after `setRPY()` |
| Wrong frame_id | Pose target in wrong coordinate frame | Use `"base_link"` for fixed-base arms |
| Stale start state | Forgot `setStartStateToCurrentState()` | Always call it before setting a new goal |

---

## 23.10 — API Quick Reference

### MoveGroupInterface construction

| Method | What it does |
|---|---|
| `MoveGroupInterface(node, "arm")` | Connect to planning group "arm" |
| `setMaxVelocityScalingFactor(0.5)` | Scale velocity limits (0.0-1.0) |
| `setMaxAccelerationScalingFactor(0.5)` | Scale acceleration limits (0.0-1.0) |

### Setting goals

| Method | Goal type | IK needed? |
|---|---|---|
| `setNamedTarget("home")` | SRDF named pose | No |
| `setJointValueTarget({...})` | Joint angles (radians) | No |
| `setPoseTarget(pose_stamped)` | End-effector pose | Yes |
| `computeCartesianPath(waypoints, step, traj)` | Waypoint path | Yes (every step) |

### Planning and execution

| Method | What it does |
|---|---|
| `setStartStateToCurrentState()` | Sync start state from `/joint_states` |
| `plan(plan)` | Compute trajectory (returns `MoveItErrorCode`) |
| `execute(plan)` | Send trajectory to controller |
| `execute(trajectory)` | Send raw `RobotTrajectory` to controller |
| `getCurrentPose()` | Get current end-effector `PoseStamped` |

### Error codes

```cpp
moveit::core::MoveItErrorCode::SUCCESS           // Planning succeeded
moveit::core::MoveItErrorCode::FAILURE            // Generic failure
moveit::core::MoveItErrorCode::PLANNING_FAILED    // No valid path found
moveit::core::MoveItErrorCode::INVALID_GOAL       // Goal is in collision or out of bounds
moveit::core::MoveItErrorCode::TIMED_OUT          // Planning exceeded time limit
```

---

**Prev:** [Part 22 — MoveIt Bringup & Integration](22-moveit-bringup.md)
