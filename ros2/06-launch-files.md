# Part 6 — Launch Files: Orchestrating the System

## 6.1 — The Analogy

A launch file is like a **conductor's score** for an orchestra. It tells:
- Which musicians (nodes) should play
- When they start
- What instrument settings (parameters) to use
- Where to sit (namespaces)

Without a launch file, you'd need to open 5+ terminals and type `ros2 run ...` in each one. A launch file does it all with one command.

---

## 6.2 — Launch File Formats

ROS2 supports 3 formats:

| Format | Extension | Syntax | Your repo uses |
|---|---|---|---|
| XML | `.launch.xml` | Declarative tags | Yes |
| Python | `.launch.py` | Programmatic (full Python) | No |
| YAML | `.launch.yaml` | Key-value pairs | No |

XML is the simplest to read. Python is the most flexible (if/else logic, loops). Your repo uses XML.

---

## 6.3 — Simple Launch File

From `template_bringup/launch/simple_app.launch.xml`:

```xml
<launch>
  <node pkg="cpp_pkg" exec="parameters">
    <param from="$(find-pkg-share template_bringup)/config/minimal_params.yaml" />
  </node>
  <node pkg="cpp_pkg" exec="subscriber"/>
</launch>
```

### Line by line:

| Line | Meaning |
|---|---|
| `<launch>` | Root tag — everything inside is part of the launch |
| `<node pkg="cpp_pkg" exec="parameters">` | Start the `parameters` executable from the `cpp_pkg` package |
| `<param from="...yaml" />` | Load parameters from the YAML file |
| `$(find-pkg-share template_bringup)` | **Substitution**: resolves to the install path of `template_bringup` |
| `<node pkg="cpp_pkg" exec="subscriber"/>` | Start the `subscriber` executable |
| `</launch>` | End |

### What happens when you run it:

```bash
ros2 launch template_bringup simple_app.launch.xml
```

```
Launch system:
  1. Parse XML file
  2. Resolve substitutions:
     $(find-pkg-share template_bringup)
       --> /home/user/ros2_ws/install/template_bringup/share/template_bringup
  3. Fork process 1: parameters node
     Command: ros2 run cpp_pkg parameters --ros-args --params-file /path/to/yaml
  4. Fork process 2: subscriber node
     Command: ros2 run cpp_pkg subscriber
  5. Monitor both processes (restart if crash, forward logs)
```

### In terms of OS processes:

```
Terminal:
  ros2 launch template_bringup simple_app.launch.xml
      |
      |-- PID 100: launch process (Python, manages everything)
           |
           |-- fork() --> PID 101: parameters_node
           |                (publishes "Hello World" every 500ms)
           |
           |-- fork() --> PID 102: subscriber_node
                            (prints received messages)

When you Ctrl+C the launch, it sends SIGINT to PID 101 and 102.
```

---

## 6.4 — Complex Launch File

From `basic_description/launch/display.launch.xml`:

```xml
<launch>
  <let name="urdf_path" value="$(find-pkg-share basic_description)/urdf/basic_urdf.urdf.xacro" />
  <let name="rviz_config_path" value="$(find-pkg-share basic_description)/rviz/urdf_config.rviz" />
  <let name="gazebo_path" value="$(find-pkg-share basic_description)/config/gazebo_bridge.yaml" />
  <let name="world_path" value="$(find-pkg-share basic_description)/worlds/basic_world.sdf" />
  <let name="models_path" value="$(find-pkg-share basic_description)/models" />

  <set_env name="GZ_SIM_RESOURCE_PATH" value="$(var models_path)" />

  <node pkg="robot_state_publisher" exec="robot_state_publisher">
    <param name="robot_description" value="$(command 'xacro $(var urdf_path)')" />
  </node>

  <include file="$(find-pkg-share ros_gz_sim)/launch/gz_sim.launch.py">
    <arg name="gz_args" value="$(var world_path) -r" />
  </include>

  <node pkg="ros_gz_sim" exec="create" args="-topic robot_description" />
  <node pkg="ros_gz_bridge" exec="parameter_bridge">
    <param name="config_file" value="$(var gazebo_path)" />
  </node>
  <node pkg="rviz2" exec="rviz2" output="screen" args="-d $(var rviz_config_path)" />
</launch>
```

### Key elements:

**`<let>` — Variable declaration:**
```xml
<let name="urdf_path" value="..." />
```
Creates a launch-time variable. Reusable via `$(var urdf_path)`.

**`<set_env>` — Environment variable:**
```xml
<set_env name="GZ_SIM_RESOURCE_PATH" value="$(var models_path)" />
```
Sets an OS environment variable so Gazebo knows where to find model files.

**`$(command 'xacro ...')` — Command substitution:**
```xml
<param name="robot_description" value="$(command 'xacro $(var urdf_path)')" />
```
Runs `xacro` at launch time to process the URDF template and produces the final XML string. The result is passed as the `robot_description` parameter.

**`<include>` — Nesting launch files:**
```xml
<include file="$(find-pkg-share ros_gz_sim)/launch/gz_sim.launch.py">
    <arg name="gz_args" value="$(var world_path) -r" />
</include>
```
Includes another launch file (Gazebo's launch file) and passes arguments to it.

---

## 6.5 — Substitutions Reference

| Substitution | Meaning | Example |
|---|---|---|
| `$(find-pkg-share pkg)` | Install path of a package | `$(find-pkg-share cpp_pkg)` |
| `$(var name)` | Value of a `<let>` variable | `$(var urdf_path)` |
| `$(command 'cmd')` | Output of a shell command | `$(command 'xacro file.xacro')` |
| `$(env VAR)` | Value of an environment variable | `$(env HOME)` |

Substitutions are resolved **at launch time**, not at build time.

---

## 6.6 — How Launch Files Get Installed

From `template_bringup/CMakeLists.txt`:

```cmake
install(DIRECTORY
  launch config
  DESTINATION share/${PROJECT_NAME}/
)
```

This copies the `launch/` and `config/` directories into:
```
install/template_bringup/share/template_bringup/launch/
install/template_bringup/share/template_bringup/config/
```

Without this `install()` call, `ros2 launch` can't find the files.

---

## 6.7 — The Full Picture for display.launch.xml

```
ros2 launch basic_description display.launch.xml
  |
  |-- robot_state_publisher     (publishes TF transforms from URDF)
  |     param: robot_description = <full URDF XML string>
  |
  |-- gz_sim (Gazebo)           (physics simulator)
  |     world: basic_world.sdf
  |     env: GZ_SIM_RESOURCE_PATH = .../models
  |
  |-- ros_gz_sim create         (spawns the robot in Gazebo)
  |     reads robot_description topic
  |
  |-- parameter_bridge          (bridges Gazebo <-> ROS2 topics)
  |     config: gazebo_bridge.yaml
  |
  |-- rviz2                     (3D visualization)
        config: urdf_config.rviz
```

5 processes, all managed by one command.

---

## 6.8 — Under the Hood: Process Management

### How does `ros2 launch` spawn nodes?

The launch system is a Python program that uses `subprocess.Popen` (on Linux, backed by `fork()` + `exec()`):

```
ros2 launch (PID 100)
  |
  |-- subprocess.Popen(["ros2", "run", "cpp_pkg", "parameters", ...])
  |     → OS: fork() → new process PID 101
  |     → OS: exec() → replaces with the node binary
  |
  |-- subprocess.Popen(["ros2", "run", "cpp_pkg", "subscriber"])
  |     → OS: fork() → new process PID 102
  |     → OS: exec() → replaces with the node binary
  |
  |-- Event loop: monitors PID 101, 102
  |     - SIGCHLD → a child exited (crash?)
  |     - SIGINT  → Ctrl+C → forward to all children
  |     - SIGTERM → clean shutdown request
```

### Signal propagation on Ctrl+C:

```
User presses Ctrl+C
  → kernel sends SIGINT to the foreground process group
  → launch process catches SIGINT
  → sends SIGINT to each child process
  → each child: rclcpp::shutdown() → cleanup → exit(0)
  → launch process waits for all children to exit
  → launch process exits
```

### Node startup time — why order matters

Nodes are started nearly simultaneously, but DDS discovery takes time:

```
t=0.00s  Launch spawns robot_state_publisher
t=0.01s  Launch spawns rviz2
t=0.05s  robot_state_publisher publishes /tf
t=0.10s  rviz2 starts, subscribes to /tf
t=0.15s  DDS matches publisher ↔ subscriber
t=0.20s  rviz2 receives first /tf → displays robot

If rviz2 starts before robot_state_publisher, it just waits
for the DDS match. No special ordering needed (decoupled!).
```

---

## 6.9 — Quick Reference

| Concept | Key Point |
|---|---|
| Launch file | Starts multiple nodes with one command |
| XML format | `<launch><node pkg="..." exec="..."/></launch>` |
| Python format | `LaunchDescription([Node(...), ...])` — more flexible |
| `<let>` | Declare a launch-time variable, use with `$(var name)` |
| `$(find-pkg-share pkg)` | Resolves to the package's install/share path |
| `$(command 'cmd')` | Runs a shell command, uses its stdout as the value |
| `<param from="file.yaml"/>` | Loads parameters from YAML |
| `<include file="...">` | Includes another launch file |
| `<set_env>` | Sets an OS environment variable |
| Install directive | `install(DIRECTORY launch config DESTINATION share/${PROJECT_NAME}/)` |
| Signal handling | Ctrl+C → SIGINT → forwarded to all child processes |

---

**Next:** [Part 7 — URDF & Visualization](07-urdf-visualization.md)

