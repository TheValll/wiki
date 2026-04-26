# ROS2 — RECAP

Single-glance table of every page across the ROS2 domain. One row per file. Click through to the full page.

---

## `setup/` + `basics/` — Foundations (00 → 07)

| Concept | What / for what | Intuition |
|---|---|---|
| [00 Environment & Setup](./setup/00-environment-setup.md) | Workspace layout, `colcon`, sourcing, package anatomy | The toolchain you need before any line of ROS2 runs |
| [01 Nodes, DDS & Graph](./basics/01-nodes-dds-graph.md) | What a node is; DDS as the discovery + transport layer; the runtime graph | Each node is a process; DDS is the invisible postal service |
| [02 Topics & Pub/Sub](./basics/02-topics-pub-sub.md) | Asynchronous many-to-many channels with QoS | Newspaper subscription — publishers print, subscribers read |
| [03 Services](./basics/03-services.md) | Synchronous request/reply | Phone call — caller waits for an explicit answer |
| [04 Custom Interfaces](./basics/04-custom-interfaces.md) | `.msg` and `.srv` files, IDL pipeline, code generation | Define your own message types; rosidl compiles them to C++/Python |
| [05 Parameters](./basics/05-parameters.md) | Per-node configuration via YAML / runtime API | The dashboard knobs of a node |
| [06 Launch Files](./basics/06-launch-files.md) | Bring up multiple nodes at once with substitutions | Stage manager that starts the whole show |
| [07 URDF & Visualization](./basics/07-urdf-visualization.md) | XML model of links/joints, Xacro macros, TF2 tree, RViz | Blueprint of the robot's physical structure |

## `ros2-control/` — Robot Control (08 → 16)

| Concept | What / for what | Intuition |
|---|---|---|
| [08 ros2_control Architecture](./ros2-control/08-ros2-control-architecture.md) | Three-layer stack (controllers / manager / hardware) and the realtime loop | Fast, deterministic reflex loop on top of ROS2 |
| [09 Hardware Interface](./ros2-control/09-hardware-interface.md) | `SystemInterface` plugin, `read()`/`write()`, lifecycle | The bridge between ros2_control and physical hardware |
| [10 ros2_control URDF](./ros2-control/10-ros2-control-urdf.md) | `<ros2_control>` tags + `pluginlib` registration | Wiring diagram — which board drives which joint |
| [11 Controllers (DiffDrive)](./ros2-control/11-controllers-diffdrive.md) | JointStateBroadcaster + DiffDriveController + odometry | Steering + odometry for a two-wheel base |
| [12 LX-225 Driver](./ros2-control/12-lx225-driver.md) | Serial protocol implementation for LX-225 servos | Telephone operator on the UART line |
| [13 Custom Controller](./ros2-control/13-writing-custom-controller.md) | Writing your own `controller_interface::ControllerInterface` | When the stock controllers don't fit, roll your own |
| [14 Controller Manager Internals](./ros2-control/14-controller-manager-internals.md) | How the manager schedules controllers and exposes services | The orchestrator that activates / deactivates controllers |
| [15 Lifecycle & State Machines](./ros2-control/15-lifecycle-state-machines.md) | Unconfigured → Inactive → Active transitions | Strict state machine that disciplines bring-up and shutdown |
| [16 Transmissions, Sensors, GPIO](./ros2-control/16-transmissions-sensors-gpio.md) | Joint↔actuator coupling, sensor and GPIO interfaces | Beyond simple joints — pulleys, encoders, digital pins |

## `moveit/` — Manipulation (17 → 23)

| Concept | What / for what | Intuition |
|---|---|---|
| [17 MoveIt Architecture](./moveit/17-moveit-architecture.md) | Three layers: planning, kinematics, execution; `move_group` as conductor | Reflexes alone don't plan — MoveIt does the brain work |
| [18 Configuration Space](./moveit/18-configuration-space.md) | Robot state as a vector of joint values; obstacles in C-space | A car's pose = `(x, y, θ)` — the space of all possible states |
| [19 Motion Planning](./moveit/19-motion-planning.md) | RRT / PRM sampling-based planners; collision checking | Flashlight in a dark forest — sample, test, connect |
| [20 Inverse Kinematics](./moveit/20-inverse-kinematics.md) | Pose → joint angles; analytic, Jacobian, damped least squares | Brain reflex when you reach for a cup |
| [21 Trajectory Generation](./moveit/21-trajectory-generation.md) | Time-parameterize the path: cubic / quintic splines, TOTG | Path = where; trajectory = where + when |
| [22 MoveIt Bringup](./moveit/22-moveit-bringup.md) | Wiring `robot_state_publisher`, controllers, `move_group`, RViz | Full launch sequence — putting all the parts on stage |
| [23 MoveIt C++ API](./moveit/23-moveit-cpp-api.md) | `MoveGroupInterface` for programmatic control | Talk to MoveIt from your own application code |
