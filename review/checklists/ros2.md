# ROS2 — Maintenance Checklist

**Mode:** maintenance, not curriculum advancement. The 24 concepts below are already covered in the wiki — this checklist exists to **keep them alive** through warm-up drills during review sessions.

**Warm-up draw bias:**

| Block | Weight |
|---|---|
| `setup/` (1 concept) | 5% |
| `basics/` (7 concepts) | 25% |
| **`ros2-control/` (9 concepts)** | **50%** ← Valentin's flagged weakness, priority drilling |
| `moveit/` (7 concepts) | 20% |

The agent reads `progress/ros2.md` for current state, picks 1-2 concepts per session warm-up biased by the table above, then uses the **rappel + quick questions** below as drill material.

---

## `setup/` — Foundations

### 0. Environment, Workspace & Essential CLI

**Wiki:** [`setup/00-environment-setup.md`](../../ros2/setup/00-environment-setup.md)

**Rappel:** A ROS2 workspace is a directory with a fixed structure: `src/` for source packages, `build/` and `install/` produced by `colcon build`. After every build you must `source install/setup.bash` to make the binaries discoverable. Every C++ package has a `CMakeLists.txt` (build rules) + `package.xml` (metadata + dependencies). Daily CLI verbs: `ros2 run`, `ros2 node list`, `ros2 topic list/echo`, `ros2 launch`.

**Quick questions:**
- Which two files define a ROS2 C++ package?
- Why must you `source install/setup.bash` after every build?
- Difference between `colcon build` and `colcon build --symlink-install`?

**Mini-kata:** List the 8 ROS2 CLI commands you reach for most often when debugging.

---

## `basics/` — ROS2 Basics

### 1. Nodes, DDS & the Graph

**Wiki:** [`basics/01-nodes-dds-graph.md`](../../ros2/basics/01-nodes-dds-graph.md)

**Rappel:** A node is just a process running an `rclcpp::Node` (or `rclpy.Node`) instance, joining a DDS network. DDS handles discovery (no central master) and transport. The "graph" = nodes + topics + services + actions, all dynamic. `rclcpp::spin()` blocks the thread to process callbacks.

**Quick questions:**
- Does ROS2 have a central master like ROS1? Why or why not?
- What does `rclcpp::spin()` actually do?
- What is DDS for in ROS2?

**Mini-kata:** Write a minimal C++ node that takes its name as a parameter and logs "hello" every second (10 lines max, no CMake boilerplate).

---

### 2. Topics & Pub/Sub

**Wiki:** [`basics/02-topics-pub-sub.md`](../../ros2/basics/02-topics-pub-sub.md)

**Rappel:** Async many-to-many channels. A publisher writes to a typed topic, anyone can subscribe. QoS defines: reliability (`RELIABLE` vs `BEST_EFFORT`), durability (`VOLATILE` vs `TRANSIENT_LOCAL`), history (`KEEP_LAST` size N vs `KEEP_ALL`). Messages are serialized with CDR before being sent over the DDS bus.

**Quick questions:**
- Name two QoS parameters and their possible values.
- What happens if publisher and subscriber QoS don't match?
- Serialization: what's CDR and why not JSON?

**Mini-kata:** Write a C++ publisher that publishes a `geometry_msgs::msg::Twist` at 10 Hz with reliable QoS depth 10. Just members + timer skeleton (15 lines max).

---

### 3. Services (Request/Reply)

**Wiki:** [`basics/03-services.md`](../../ros2/basics/03-services.md)

**Rappel:** Synchronous request→reply communication. A client calls, waits for the reply. The server exposes a callback. Service = topic but bidirectional and blocking on the client side (`async_send_request` returns a future). Use for one-shot actions (calibrate, reset, query state) and not for streaming.

**Quick questions:**
- When prefer a service over a topic?
- What does `async_send_request` return and what do you do with it?
- What's the structure of a `.srv` file?

**Mini-kata:** Sketch an `AddTwoInts` C++ service: `.srv` header + signature of the server callback (5-8 lines).

---

### 4. Custom Interfaces (`.msg`, `.srv`)

**Wiki:** [`basics/04-custom-interfaces.md`](../../ros2/basics/04-custom-interfaces.md)

**Rappel:** For non-standard types, define `.msg`/`.srv` files that go through `rosidl` → IDL → generated C++/Python. Any package defining custom interfaces must depend on `rosidl_default_generators` (build) and `rosidl_default_runtime` (exec) in its `package.xml`. Everything is serialized through CDR with alignment (8-byte padding for doubles).

**Quick questions:**
- Which two `package.xml` dependencies are required for an interface package?
- What does `rosidl_generate_interfaces` do in the `CMakeLists.txt`?
- Difference between `.msg` and `.srv` (beyond the fact that `.srv` has two sections)?

**Mini-kata:** Write a `.msg` for a wheel state (position in rad, velocity in rad/s, current in A), then a `.srv` that asks to calibrate a wheel given by its ID.

---

### 5. Parameters

**Wiki:** [`basics/05-parameters.md`](../../ros2/basics/05-parameters.md)

**Rappel:** Runtime configuration of a node. Each node holds a `std::string → ParameterValue` map. Set/get programmatically or via YAML loaded at launch. Modifiable on the fly via `ros2 param set` (with validation callback `add_on_set_parameters_callback`). Used for PID gains, frequencies, operational modes.

**Quick questions:**
- How do you declare a parameter with type+default in C++?
- What is `add_on_set_parameters_callback` for?
- YAML format: where do the node name and namespace go?

**Mini-kata:** Write 4-5 lines of C++ to declare an `update_rate` parameter (double, default 50.0) and read it inside a timer callback.

---

### 6. Launch Files

**Wiki:** [`basics/06-launch-files.md`](../../ros2/basics/06-launch-files.md)

**Rappel:** Orchestration of a multi-node system. Format Python (most expressive) or XML/YAML. Defines: which nodes to start, their parameters, namespaces, remappings, conditions. `LaunchDescription`s can include each other → composition. Substitutions `LaunchConfiguration`, `PathJoinSubstitution`, `FindPackageShare` allow dynamic parameterization.

**Quick questions:**
- When prefer Python launch over XML?
- What does `LaunchConfiguration` do?
- How do you pass a YAML parameter file to a launched node?

**Mini-kata:** Sketch a `launch.py` that starts 2 nodes (a publisher + a subscriber) sharing a common parameter `topic_name`.

---

### 7. URDF & Visualization

**Wiki:** [`basics/07-urdf-visualization.md`](../../ros2/basics/07-urdf-visualization.md)

**Rappel:** Describes the physical structure of a robot in XML. `<link>` = rigid body (visual + collision + inertial). `<joint>` = link between two links with a type (revolute, prismatic, fixed, continuous). Xacro = macros + variables on top of URDF (preprocessor). TF2 publishes the transform tree between all frames. RViz visualizes everything.

**Quick questions:**
- Name 4 URDF joint types.
- What does TF2 do that URDF doesn't?
- Why use Xacro rather than raw URDF?

**Mini-kata:** Write a Xacro macro `<xacro:wheel name="..." parent="..."/>` that creates a cylindrical link + a revolute joint attaching it to the parent (8-10 lines).

---

## `ros2-control/` — Robot Control (priority drill)

### 8. ros2_control Architecture

**Wiki:** [`ros2-control/08-ros2-control-architecture.md`](../../ros2/ros2-control/08-ros2-control-architecture.md)

**Rappel:** Three layers: controllers (top, compute commands) → controller_manager (middle, orchestrates) → hardware_interface (bottom, talks to hardware). Deterministic loop at 50-100 Hz. Each cycle: `read()` (state from hardware) → `update()` (each active controller computes) → `write()` (commands to hardware). The manager runs in a dedicated real-time thread.

**Quick questions:**
- Give the exact order of the 3 phases of one cycle.
- Which thread does the controller_manager run in?
- What distinguishes ros2_control from plain pub/sub?

**Mini-kata:** Pseudocode (8-10 lines) of the controller_manager's main loop.

---

### 9. Hardware Interface

**Wiki:** [`ros2-control/09-hardware-interface.md`](../../ros2/ros2-control/09-hardware-interface.md)

**Rappel:** A plugin inheriting `hardware_interface::SystemInterface`. Lifecycle methods: `on_init`, `on_configure`, `on_activate`, `on_deactivate`, then in the loop: `read(time, period)` and `write(time, period)`. Exposes `state_interfaces` (read-only, what the hardware measures) and `command_interfaces` (write, what the controllers want to impose). Values are shared `double*`s.

**Quick questions:**
- Name the 4 main lifecycle methods of a SystemInterface.
- state_interface vs command_interface: who claims what?
- Why are interfaces `double` rather than typed messages?

**Mini-kata:** Write the header skeleton of `MyHardwareInterface : public SystemInterface` with the virtual methods to override (10-15 lines).

---

### 10. ros2_control URDF Tags

**Wiki:** [`ros2-control/10-ros2-control-urdf.md`](../../ros2/ros2-control/10-ros2-control-urdf.md)

**Rappel:** The `<ros2_control>` block in the URDF declares which hardware plugin to load for each resource (joint/sensor/gpio). Pluginlib finds the plugin via the pair `(library_path, class_name)` exported in an XML descriptor of the package. The CMakeLists exports that XML through `pluginlib_export_plugin_description_file`. Mock components let you test without real hardware.

**Quick questions:**
- Which XML tag attaches a joint to its hardware plugin?
- How does pluginlib discover that a plugin exists?
- What are mock components for?

**Mini-kata:** Write the minimal `<ros2_control>` block for a robot with two joints `wheel_left` and `wheel_right` using a custom plugin `my_robot/MyHardware`.

---

### 11. Controllers — DiffDrive & JointStateBroadcaster

**Wiki:** [`ros2-control/11-controllers-diffdrive.md`](../../ros2/ros2-control/11-controllers-diffdrive.md)

**Rappel:** Two stock controllers for a diffdrive base. **JointStateBroadcaster**: reads state_interfaces from all joints and publishes on `/joint_states` (read-only). **DiffDriveController**: takes `cmd_vel` (Twist), converts to wheel speeds via inverse kinematics `wL = (v − ω·L/2)/r`, `wR = (v + ω·L/2)/r`, and publishes odometry on `/odom`. L = wheel separation, r = wheel radius.

**Quick questions:**
- What does JointStateBroadcaster publish, and what does it claim?
- Give the diffdrive inverse kinematics formula.
- Where does the accumulating odometry error come from?

**Mini-kata:** By hand: robot with L=0.4 m, r=0.05 m, command `v=0.5 m/s, ω=1.0 rad/s`. What angular speed at each wheel?

---

### 12. LX-225 Driver

**Wiki:** [`ros2-control/12-lx225-driver.md`](../../ros2/ros2-control/12-lx225-driver.md)

**Rappel:** Serial driver for LX-225 servomotors over UART. Pattern: open serial port → write packet (header + ID + length + command + data + checksum) → read response → parse. Physical layer = USB-to-UART with a fixed baudrate (115200). The driver is fully decoupled from the ROS2 hardware_interface — it's just used by the latter via composition.

**Quick questions:**
- Default baudrate for the LX-225?
- What's in a packet (at least 5 fields)?
- Why is the driver decoupled from the SystemInterface?

**Mini-kata:** Skeleton of the constructor `LX225Driver(const std::string& port, int baudrate)` that opens the connection (5-6 lines, no need for protocol body).

---

### 13. Writing a Custom Controller

**Wiki:** [`ros2-control/13-writing-custom-controller.md`](../../ros2/ros2-control/13-writing-custom-controller.md)

**Rappel:** Inherits `controller_interface::ControllerInterface`. Implements lifecycle (`on_init`, `on_configure`, `on_activate`, `on_deactivate`) + `command_interface_configuration()` + `state_interface_configuration()` declaring what the controller wants to claim + `update(time, period)` that runs every tick. Plugin registered via `PLUGINLIB_EXPORT_CLASS` and a description XML in the package.

**Quick questions:**
- Which 3 config methods return the desired interface names?
- What does `update(time, period)` actually do?
- How do you register the class as a plugin?

**Mini-kata:** Skeleton of an `AlphaFilter` controller that reads position and writes filtered position — just method signatures (10-12 lines).

---

### 14. Controller Manager Internals

**Wiki:** [`ros2-control/14-controller-manager-internals.md`](../../ros2/ros2-control/14-controller-manager-internals.md)

**Rappel:** The manager owns the **ResourceManager** (which holds the hardware interfaces and arbitrates access — only one controller can claim a given command_interface at a time). It manages each controller's lifecycle (load → configure → activate → deactivate → cleanup → unload). `update()` execution order: by default load order, overridable via `chained_controllers`. At startup, parses the URDF, instantiates the hardware plugin, instantiates the declared controllers.

**Quick questions:**
- What does the ResourceManager do exactly?
- Can two controllers claim the same command_interface simultaneously?
- Where does the `update()` execution order of active controllers come from?

**Mini-kata:** Describe in 4-5 sentences the startup sequence of `ros2_control_node` when launching the full MoveIt launch file.

---

### 15. Lifecycle & State Machines

**Wiki:** [`ros2-control/15-lifecycle-state-machines.md`](../../ros2/ros2-control/15-lifecycle-state-machines.md)

**Rappel:** Hardware interfaces and controllers both follow the `rclcpp_lifecycle` state machine: Unconfigured → Inactive → Active → (back to Inactive or Finalized). Transitions: `configure`, `activate`, `deactivate`, `cleanup`, `shutdown`. Each transition can fail → `ErrorProcessing` state → custom error handler. CLI: `ros2 lifecycle set <node> <transition>`. Lets you isolate expensive init (in configure) from activation (fast, in activate).

**Quick questions:**
- Name the 4 main lifecycle states (Active included).
- What happens if `on_activate` returns `ERROR`?
- Why separate `configure` and `activate`?

**Mini-kata:** For a hardware_interface, list one operation that should go into each of `on_init`, `on_configure`, `on_activate`, `on_deactivate`.

---

### 16. Transmissions, Sensors & GPIO

**Wiki:** [`ros2-control/16-transmissions-sensors-gpio.md`](../../ros2/ros2-control/16-transmissions-sensors-gpio.md)

**Rappel:** Three abstractions beyond simple joints. **Transmissions**: mechanical couplings (gear ratio between actuator and joint, e.g. `joint_velocity = N × actuator_velocity`). **Sensors**: read-only interfaces (force/torque, IMU, external encoders). **GPIO**: digital/analog I/O (digital pin, analog read of a sensor). Each is declared in the URDF `<ros2_control>` block with its tag (`<transmission>`, `<sensor>`, `<gpio>`).

**Quick questions:**
- Concrete difference between a sensor and a state_interface on a joint?
- Why model a transmission instead of just multiplying inside the hardware_interface?
- What is a `<gpio>` for in a robot context?

**Mini-kata:** Describe in 3-4 sentences how you'd model a robot with a wheel encoder and a chassis-mounted IMU.

---

## `moveit/` — Manipulation

### 17. MoveIt Architecture

**Wiki:** [`moveit/17-moveit-architecture.md`](../../ros2/moveit/17-moveit-architecture.md)

**Rappel:** ros2_control = reflexes (50-100 Hz loop). MoveIt = brain (high-level planning, not real-time). The **move_group** node orchestrates: receives a goal pose → IK → planning → trajectory generation → sends the result via a `FollowJointTrajectory` action to a ros2_control controller. The **planning_scene** is the world state (robot + obstacles + ACM = Allowed Collision Matrix). The **SRDF** adds semantics on top of the URDF (planning groups, end-effectors, default poses, disabled collisions).

**Quick questions:**
- What does the move_group node do?
- ACM = what, and what for?
- SRDF vs URDF — what does the SRDF add?

**Mini-kata:** Trace in 5-6 sentences the flow of a "move arm to pose X" goal: from API call to wheel turning.

---

### 18. Configuration Space (C-Space)

**Wiki:** [`moveit/18-configuration-space.md`](../../ros2/moveit/18-configuration-space.md)

**Rappel:** The space of all possible configurations of a robot — for a 6-DOF arm it's ℝ⁶ (one point = a vector of 6 joint angles). Obstacles in task space transform into C-obstacles (forbidden regions of ℝ⁶). Complexity explodes with DOF (curse of dimensionality). Collision checking uses FCL or GJK to test if a configuration is collision-free. C-space distance is typically a weighted Euclidean metric.

**Quick questions:**
- Why does complexity explode with DOF?
- task space vs joint space — what's the difference?
- Which distance metric in configuration space?

**Mini-kata:** For a 7-DOF arm, give the C-space dimension and explain why a grid search over it is infeasible.

---

### 19. Motion Planning Algorithms

**Wiki:** [`moveit/19-motion-planning.md`](../../ros2/moveit/19-motion-planning.md)

**Rappel:** Sampling-based, because grid search is infeasible in high dim. **RRT**: sample a random point, extend the tree from the nearest node toward it. **RRT-Connect**: two trees (start + goal) that meet. **RRT***: asymptotically optimal (rewire). **PRM**: pre-computed roadmap, then A* query. OMPL is the lib that implements all of these. Narrow passages = classic hard problem. Path smoothing as a post-processing step (BSpline / shortcut).

**Quick questions:**
- Why sampling-based rather than grid?
- RRT vs PRM: when prefer one over the other?
- What does RRT* add over RRT?

**Mini-kata:** Write the RRT algorithm pseudocode (single-tree extend) in 8-10 lines.

---

### 20. Inverse Kinematics

**Wiki:** [`moveit/20-inverse-kinematics.md`](../../ros2/moveit/20-inverse-kinematics.md)

**Rappel:** "Given a desired end-effector pose, find joint angles." Forward kinematics = easy (chain of DH matrices). IK = hard (non-linear, possibly multiple solutions or none). **Analytical**: closed-form, possible for certain geometries (kinematics decoupling, 6-DOF). **Numerical**: iterative via Jacobian pseudoinverse `J⁺ = Jᵀ(JJᵀ)⁻¹`, or Damped Least Squares to handle singularities. **Redundancy** (7+ DOF) → null-space to optimize a secondary criterion.

**Quick questions:**
- Why DLS instead of pure pseudoinverse?
- How many IK solutions can a single pose have for a 6-DOF arm?
- What's a kinematic singularity?

**Mini-kata:** Sketch in 6-7 lines the iterative numerical IK loop (until convergence).

---

### 21. Trajectory Generation

**Wiki:** [`moveit/21-trajectory-generation.md`](../../ros2/moveit/21-trajectory-generation.md)

**Rappel:** Path = sequence of configurations (where to go). Trajectory = path + time + velocities + accelerations (when to be where). **Trapezoidal profile**: constant accel → cruise velocity → constant decel. **TOPP-RA**: optimizes timing under per-joint velocity/acceleration constraints. **Splines**: cubic (C²) or quintic (C⁴) for smooth interpolation. Execution via the `FollowJointTrajectory` action sent to a ros2_control controller.

**Quick questions:**
- Path vs trajectory — what's the difference?
- Quintic vs cubic spline — what does quintic add?
- Under which action does MoveIt send the trajectory to the controller?

**Mini-kata:** For a 0 → 1 rad move with vmax=1 rad/s and amax=2 rad/s², compute by hand the total time using a trapezoidal profile.

---

### 22. MoveIt Bringup

**Wiki:** [`moveit/22-moveit-bringup.md`](../../ros2/moveit/22-moveit-bringup.md)

**Rappel:** Three-package pattern: `<robot>_description` (URDF + meshes), `<robot>_moveit_config` (SRDF + kinematics + planning configs), `<robot>_bringup` (launch files that wire everything). Startup sequence: robot_state_publisher → ros2_control_node + controllers → move_group → RViz. Critical wiring: controllers declared in `moveit_controllers.yaml` must match the controllers loaded by ros2_control.

**Quick questions:**
- Name the 3 packages of the MoveIt bringup pattern.
- Which YAML file describes the controllers to MoveIt?
- In what order do components start in the launch sequence?

**Mini-kata:** Write a skeleton `launch.py` that includes the 4 sub-launch files in the correct order (5-6 Python lines).

---

### 23. MoveIt C++ API (MoveGroupInterface)

**Wiki:** [`moveit/23-moveit-cpp-api.md`](../../ros2/moveit/23-moveit-cpp-api.md)

**Rappel:** Programmatic interface to drive MoveIt from your own node. Base pattern: instantiate `MoveGroupInterface(node, "group_name")` → set goal (`setPoseTarget`, `setJointValueTarget`, `setNamedTarget`) → `plan(plan_msg)` → `execute(plan_msg)`. Multi-group (arm + gripper) needs one MoveGroupInterface per group. Classic pitfall: subscriber callbacks in the same thread as `plan()` → deadlock.

**Quick questions:**
- Name 3 types of goal targets MoveGroupInterface accepts.
- What's the canonical plan-execute sequence?
- Why does multi-group need multiple MoveGroupInterface instances?

**Mini-kata:** Write 8-10 lines of C++: create a `MoveGroupInterface` "manipulator", set a pose target, plan, execute if plan succeeded.

---

## Notes for the agent

- **At warm-up Step 1**: apply the weights from the table at the top of the file. Within the chosen block, pull from `progress/ros2.md` the concept with the oldest "Last seen" date.
- **Mini-kata**: in `intuition` mode, replace the kata with an oral articulation drill ("explain it without looking at the wiki…"). In `practice` mode, the user codes/writes in their editor, pastes, agent compares against the wiki.
- **Correction**: pinpoint precisely. Not "you forgot X" in the abstract, but "you swapped read/write order" or "you missed the claim on the state_interface".
- **Rappel**: the rappel above is your **support material** for Step 2 (today's lesson). Re-read it with the user out loud, don't rewrite it.
