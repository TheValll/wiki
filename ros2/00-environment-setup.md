# Part 0 — Environment, Workspace & Essential Commands

## 0.1 — ROS2 Workspace Architecture

A ROS2 workspace is a directory with a specific structure:

```
my_workspace/
├── src/                    ← Your packages (source code)
│   ├── cpp_pkg/            ← C++ package
│   ├── py_pkg/             ← Python package
│   └── custom_interfaces/  ← Message/service definitions
├── build/                  ← Compilation files (generated)
├── install/                ← Installed binaries and libs (generated)
├── log/                    ← Build logs (generated)
└── colcon_defaults.yaml    ← Build configuration (optional)
```

Only `src/` is version-controlled (git). The other 3 directories are **generated** by `colcon build`.

---

## 0.2 — Anatomy of a C++ Package

Every C++ package contains at minimum:

```
cpp_pkg/
├── CMakeLists.txt     ← Compilation instructions
├── package.xml        ← Package identity card (name, deps, type)
├── src/               ← Source code (.cpp)
└── include/           ← Headers (.hpp) — optional
```

### package.xml — the identity card

```xml
<?xml version="1.0"?>
<package format="3">
  <name>cpp_pkg</name>
  <version>0.0.0</version>
  <description>My ROS2 package</description>

  <buildtool_depend>ament_cmake</buildtool_depend>   <!-- Build tool -->
  <depend>rclcpp</depend>                             <!-- C++ ROS2 dependency -->
  <depend>example_interfaces</depend>                 <!-- Standard messages/services -->

  <export>
    <build_type>ament_cmake</build_type>
  </export>
</package>
```

### CMakeLists.txt — the compilation plan

```cmake
cmake_minimum_required(VERSION 3.8)
project(cpp_pkg)

# Find dependencies
find_package(ament_cmake REQUIRED)
find_package(rclcpp REQUIRED)
find_package(example_interfaces REQUIRED)

# Create an executable from a source file
add_executable(minimal_cpp_node src/minimal_node.cpp)
ament_target_dependencies(minimal_cpp_node rclcpp)

# Install the executable so ros2 run can find it
install(TARGETS minimal_cpp_node
  DESTINATION lib/${PROJECT_NAME}
)

ament_package()
```

### Under the hood — what does `ament_target_dependencies` do?

It's a shortcut that does 3 things:

```
ament_target_dependencies(minimal_cpp_node rclcpp)

is equivalent to:
  target_include_directories(minimal_cpp_node ... /opt/ros/humble/include/rclcpp/)
  target_link_libraries(minimal_cpp_node ... librclcpp.so)
  target_compile_definitions(minimal_cpp_node ... )
```

It automatically resolves include paths and libraries to link against.

---

## 0.3 — Building with colcon

### Build commands

```bash
# Build the ENTIRE workspace
colcon build

# Build a SINGLE package (much faster)
colcon build --packages-select cpp_pkg

# Build with visible console output
colcon build --event-handlers console_direct+

# Build in debug mode (symbols for gdb)
colcon build --cmake-args -DCMAKE_BUILD_TYPE=Debug
```

### What does `colcon build` do under the hood?

```
colcon build
    |
    ├─ 1. Reads all package.xml files in src/
    |
    ├─ 2. Resolves build order (dependency graph)
    |     If B depends on A → build A first
    |
    |     Order = topological sort of the graph:
    |     custom_interfaces → cpp_pkg → template_bringup
    |
    ├─ 3. For each package (in order):
    |     ├─ cmake -S src/cpp_pkg -B build/cpp_pkg
    |     ├─ make -j$(nproc)        ← parallel compilation
    |     └─ make install DESTDIR=install/cpp_pkg
    |
    └─ 4. Generates install/setup.bash (environment variables)
```

### Topological sort — simple math

The dependency graph is a **DAG** (Directed Acyclic Graph). Kahn's algorithm gives the build order:

```
Input: Dependency graph
  custom_interfaces ← (no local ROS2 deps)
  cpp_pkg ← depends on custom_interfaces
  template_bringup ← depends on cpp_pkg

Algorithm:
  1. Find all nodes with no incoming edges → [custom_interfaces]
  2. Remove that node from the graph, add to the queue
  3. Repeat → [custom_interfaces, cpp_pkg, template_bringup]

This is a topological sort: complexity O(V + E)
  V = number of packages, E = number of dependencies
```

---

## 0.4 — Sourcing the Environment

After compilation, executables are in `install/`. For the system to find them:

```bash
# Source the workspace (must be done in EVERY new terminal)
source install/setup.bash
```

### What does `source install/setup.bash` do?

It modifies the shell's environment variables:

```
Before source:
  PATH = /usr/bin:/bin
  AMENT_PREFIX_PATH = /opt/ros/humble

After source:
  PATH = /usr/bin:/bin
  AMENT_PREFIX_PATH = /home/user/ws/install/cpp_pkg:/opt/ros/humble
  LD_LIBRARY_PATH += /home/user/ws/install/cpp_pkg/lib
  PYTHONPATH += /home/user/ws/install/py_pkg/lib/python3/dist-packages
```

Without this, `ros2 run cpp_pkg minimal_cpp_node` won't find the executable.

**Classic trap**: forgetting to re-source after a `colcon build`. The terminal uses the old version.

---

## 0.5 — Creating a New Package

```bash
# C++ package (ament_cmake)
cd ~/ros2_ws/src
ros2 pkg create --build-type ament_cmake --dependencies rclcpp example_interfaces my_pkg

# Python package (ament_python)
ros2 pkg create --build-type ament_python --dependencies rclpy my_pkg_py
```

This command auto-generates `package.xml`, `CMakeLists.txt` (or `setup.py`), and the folder structure.

---

## 0.6 — Running and Inspecting Nodes

### Run a node

```bash
# Run a single node
ros2 run cpp_pkg minimal_cpp_node

# Run with a name remap
ros2 run cpp_pkg minimal_cpp_node --ros-args -r __node:=my_node

# Run with a parameter override
ros2 run cpp_pkg parameters --ros-args -p message:="Hello"
```

### Run a launch file (multiple nodes)

```bash
ros2 launch template_bringup simple_app.launch.py
ros2 launch basic_description display.launch.xml
```

---

## 0.7 — Essential CLI Commands

### Nodes

```bash
ros2 node list                          # List active nodes
ros2 node info /publisher               # Node details (topics, services)
```

### Topics

```bash
ros2 topic list                         # List topics
ros2 topic list -t                      # List with message types
ros2 topic info /simple_topic           # Number of pub/sub
ros2 topic echo /simple_topic           # Print messages in real-time
ros2 topic hz /simple_topic             # Publish frequency
ros2 topic bw /simple_topic             # Bandwidth
ros2 topic pub /simple_topic example_interfaces/msg/String "data: hello"
```

### Services

```bash
ros2 service list                       # List services
ros2 service type /add_two_ints         # Service type
ros2 service call /add_two_ints example_interfaces/srv/AddTwoInts "{a: 5, b: 3}"
```

### Parameters

```bash
ros2 param list                         # List parameters by node
ros2 param get /parameters_node message # Read a parameter
ros2 param set /parameters_node message "New message"  # Modify at runtime
ros2 param dump /parameters_node        # YAML dump of all parameters
```

### Interfaces (messages/services)

```bash
ros2 interface list                     # All available types
ros2 interface show example_interfaces/msg/String      # Message structure
ros2 interface show example_interfaces/srv/AddTwoInts  # Service structure
```

### Graph

```bash
rqt_graph                               # Graphical visualization (GUI)
ros2 doctor                             # System diagnostics
```

---

## 0.8 — Quick Reference

| Action | Command |
|---|---|
| Build everything | `colcon build` |
| Build one package | `colcon build --packages-select pkg` |
| Source environment | `source install/setup.bash` |
| Create C++ package | `ros2 pkg create --build-type ament_cmake --dependencies rclcpp pkg` |
| Run a node | `ros2 run pkg node` |
| Run a launch file | `ros2 launch pkg file.launch.py` |
| List nodes | `ros2 node list` |
| List topics | `ros2 topic list` |
| Echo a topic | `ros2 topic echo /topic` |
| Topic frequency | `ros2 topic hz /topic` |
| Call a service | `ros2 service call /srv type "{fields}"` |
| Read a parameter | `ros2 param get /node param` |
| Set a parameter | `ros2 param set /node param value` |

---

**Next:** [Part 1 — Nodes, DDS & the Graph](01-nodes-dds-graph.md)
