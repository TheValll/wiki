# ROS2 Learning Wiki

A structured deep-dive into ROS2, from basic concepts to ros2_control internals.

---

## Syllabus

### Block A: ROS2 Basics

| # | Part | Key Concepts | Files |
|---|------|-------------|-------|
| 1 | [Nodes, DDS & the Graph](parts/01-nodes-dds-graph.md) | Processes, DDS discovery, spin loop, memory layout | `minimal_node.cpp`, `minimal_node.py` |
| 2 | [Topics & Pub/Sub](parts/02-topics-pub-sub.md) | Async messaging, queues, QoS, serialization | `publisher_node`, `subscriber_node` |
| 3 | [Services: Request/Reply](parts/03-services.md) | Sync communication, client-server, executors | `server_node`, `client_node` |
| 4 | [Custom Interfaces](parts/04-custom-interfaces.md) | .msg/.srv files, IDL, CDR serialization, codegen | `MinimalInterface.msg`, `MinimalService.srv` |
| 5 | [Parameters](parts/05-parameters.md) | Parameter server, YAML config, dynamic reconfigure | `parameters_node`, `minimal_params.yaml` |
| 6 | [Launch Files](parts/06-launch-files.md) | Process orchestration, arguments, composition | `simple_app.launch.xml`, `display.launch.xml` |
| 7 | [URDF & Visualization](parts/07-urdf-visualization.md) | Xacro, links, joints, inertia matrices, TF2 | `basic_urdf.urdf.xacro`, `mobile_base.xacro` |

### Block B: ROS2 Control

| # | Part | Key Concepts | Files |
|---|------|-------------|-------|
| 8 | [ros2_control Architecture](parts/08-ros2-control-architecture.md) | Controller Manager, real-time loop, read/update/write | `template_controllers.yaml` |
| 9 | [Hardware Interface](parts/09-hardware-interface.md) | SystemInterface, lifecycle, state/command interfaces | `mobile_base_hardware_interface.hpp/.cpp` |
| 10 | [ros2_control URDF](parts/10-ros2-control-urdf.md) | `<ros2_control>` tags, plugins, pluginlib | `mobile_base.ros2_control.xacro` |
| 11 | [Controllers: DiffDrive](parts/11-controllers-diffdrive.md) | Differential drive kinematics, odometry math | `template_controllers.yaml` |
| 12 | [Hardware Driver: LX-225](parts/12-lx225-driver.md) | UART, serial protocol, baudrate, servo commands | `LX225Driver.hpp`, `lx225_test.cpp` |

### Block C: ros2_control Deep Dive

| # | Part | Key Concepts | Files |
|---|------|-------------|-------|
| 13 | [Writing a Custom Controller](parts/13-writing-custom-controller.md) | Controller plugin, interface config, update loop, pluginlib | `basic_controller.hpp/.cpp`, `basic_controller_plugin.xml` |
| 14 | [Controller Manager Internals](parts/14-controller-manager-internals.md) | ResourceManager, interface claiming, URDF parsing, memory layout | — |
| 15 | [Lifecycle & State Machines](parts/15-lifecycle-state-machines.md) | rclcpp_lifecycle, transitions, error handling, startup sequence | — |
| 16 | [Transmissions, Sensors & GPIO](parts/16-transmissions-sensors-gpio.md) | Gear ratios, SensorInterface, GPIO interfaces, combined systems | — |
