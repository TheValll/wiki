# ROS2 Learning Wiki

A structured deep-dive into ROS2, from basic concepts to ros2_control internals and MoveIt motion planning.

---

## Syllabus

### Block 0: Setup

| # | Part | Key Concepts |
|---|------|-------------|
| 0 | [Environment, Workspace & Essential Commands](00-environment-setup.md) | Workspace structure, colcon build, source, CLI commands, CMakeLists, package.xml |

### Block A: ROS2 Basics

| # | Part | Key Concepts | Files |
|---|------|-------------|-------|
| 1 | [Nodes, DDS & the Graph](01-nodes-dds-graph.md) | Processes, DDS discovery, spin loop, memory layout | `minimal_node.cpp`, `minimal_node.py` |
| 2 | [Topics & Pub/Sub](02-topics-pub-sub.md) | Async messaging, queues, QoS, serialization | `publisher_node`, `subscriber_node` |
| 3 | [Services: Request/Reply](03-services.md) | Sync communication, client-server, executors | `server_node`, `client_node` |
| 4 | [Custom Interfaces](04-custom-interfaces.md) | .msg/.srv files, IDL, CDR serialization, codegen | `MinimalInterface.msg`, `MinimalService.srv` |
| 5 | [Parameters](05-parameters.md) | Parameter server, YAML config, dynamic reconfigure | `parameters_node`, `minimal_params.yaml` |
| 6 | [Launch Files](06-launch-files.md) | Process orchestration, arguments, composition | `simple_app.launch.xml`, `display.launch.xml` |
| 7 | [URDF & Visualization](07-urdf-visualization.md) | Xacro, links, joints, inertia matrices, TF2 | `basic_urdf.urdf.xacro`, `mobile_base.xacro` |

### Block B: ROS2 Control

| # | Part | Key Concepts | Files |
|---|------|-------------|-------|
| 8 | [ros2_control Architecture](08-ros2-control-architecture.md) | Controller Manager, real-time loop, read/update/write | `template_controllers.yaml` |
| 9 | [Hardware Interface](09-hardware-interface.md) | SystemInterface, lifecycle, state/command interfaces | `mobile_base_hardware_interface.hpp/.cpp` |
| 10 | [ros2_control URDF](10-ros2-control-urdf.md) | `<ros2_control>` tags, plugins, pluginlib | `mobile_base.ros2_control.xacro` |
| 11 | [Controllers: DiffDrive](11-controllers-diffdrive.md) | Differential drive kinematics, odometry math | `template_controllers.yaml` |
| 12 | [Hardware Driver: LX-225](12-lx225-driver.md) | UART, serial protocol, baudrate, servo commands | `LX225Driver.hpp`, `lx225_test.cpp` |

### Block C: ros2_control Deep Dive

| # | Part | Key Concepts | Files |
|---|------|-------------|-------|
| 13 | [Writing a Custom Controller](13-writing-custom-controller.md) | Controller plugin, interface config, update loop, pluginlib | `basic_controller.hpp/.cpp`, `basic_controller_plugin.xml` |
| 14 | [Controller Manager Internals](14-controller-manager-internals.md) | ResourceManager, interface claiming, URDF parsing, memory layout | — |
| 15 | [Lifecycle & State Machines](15-lifecycle-state-machines.md) | rclcpp_lifecycle, transitions, error handling, startup sequence | — |
| 16 | [Transmissions, Sensors & GPIO](16-transmissions-sensors-gpio.md) | Gear ratios, SensorInterface, GPIO interfaces, combined systems | — |

### Block D: MoveIt

| # | Part | Key Concepts | Files |
|---|------|-------------|-------|
| 17 | [MoveIt Architecture](17-moveit-architecture.md) | move_group node, planning scene, ACM, SRDF semantics, MoveIt vs ros2_control | `basic_robot.srdf`, `moveit_controllers.yaml` |
| 18 | [Configuration Space](18-configuration-space.md) | C-space, C-obstacles, free space, dimensionality, collision checking (FCL, GJK), distance metrics | — |
| 19 | [Motion Planning Algorithms](19-motion-planning.md) | RRT, RRT-Connect, RRT*, PRM, OMPL, narrow passages, path smoothing, Pilz planner | — |
| 20 | [Inverse Kinematics](20-inverse-kinematics.md) | DH convention, Jacobian, numerical IK, pseudoinverse, DLS, singularities, redundancy | `kinematics.yaml` |
| 21 | [Trajectory Generation](21-trajectory-generation.md) | Path vs trajectory, trapezoidal profile, TOPP-RA, splines, SLERP, FollowJointTrajectory | `joint_limits.yaml`, `pilz_cartesian_limits.yaml` |
| 22 | [MoveIt Bringup & Integration](22-moveit-bringup.md) | Three-package pattern, launch sequence, controller wiring, execution flow, full stack | `display.launch.xml`, `ros2_controllers.yaml` |
| 23 | [MoveIt C++ API (MoveGroupInterface)](23-moveit-cpp-api.md) | MoveGroupInterface, named/joint/pose/Cartesian goals, plan-execute pattern, multi-group, commander node | `test_moveit.cpp`, `commander_template.cpp` |
