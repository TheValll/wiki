# Part 10 — ros2_control URDF: Declaring Hardware in the Robot Description

## 10.1 — The Analogy

If the URDF is the **blueprint** of the robot's physical structure, the ros2_control URDF tags are the **wiring diagram** — they describe which electronic boards control which joints, and what signals (commands/states) are available.

---

## 10.2 — The `<ros2_control>` Tag

From `mobile_base.ros2_control.xacro`:

```xml
<ros2_control name="MobileBase" type="system">
    <hardware>
        <plugin>mobile_base_hardware/MobileBaseHardware</plugin>
        <param name="servo_id">6</param>
        <param name="baudrate">115200</param>
        <param name="port">/dev/ttyUSB0</param>
    </hardware>
    <joint name="base_right_wheel_joint">
        <command_interface name="velocity" />
        <state_interface  name="velocity" />
        <state_interface  name="position" />
    </joint>
    <joint name="base_left_wheel_joint">
        <command_interface name="velocity" />
        <state_interface  name="velocity" />
        <state_interface  name="position" />
    </joint>
</ros2_control>
```

### Element by element:

**`<ros2_control name="MobileBase" type="system">`**
- `name` = a human-readable identifier
- `type` = matches the interface type:

| URDF type | C++ base class | Use case |
|---|---|---|
| `system` | `SystemInterface` | Multiple joints together |
| `actuator` | `ActuatorInterface` | Single joint |
| `sensor` | `SensorInterface` | Read-only device |

**`<hardware><plugin>mobile_base_hardware/MobileBaseHardware</plugin>`**
- This is the **plugin name** that pluginlib uses to find and load the class
- Format: `namespace/ClassName`
- The Controller Manager will dynamically load the shared library at runtime

**`<param name="servo_id">6</param>`**
- Custom parameters passed to `on_init()` via `info_.hardware_parameters`
- These are key-value string pairs — the hardware interface parses them

**`<joint name="base_right_wheel_joint">`**
- Must match a joint name in the URDF kinematic description (`mobile_base.xacro`)
- Links the ros2_control interface to the physical joint

**`<command_interface name="velocity" />`**
- Declares: "This joint accepts velocity commands"
- Creates a `double` in the Controller Manager's shared memory
- Full interface name: `base_right_wheel_joint/velocity`

**`<state_interface name="velocity" />` and `<state_interface name="position" />`**
- Declares: "This joint reports its velocity and position"
- Creates two `double` values readable by controllers

---

## 10.3 — How It All Connects

```
URDF parsing at startup:

1. Controller Manager reads robot_description parameter
2. Finds <ros2_control> tags
3. For each tag:
   a. Reads type="system"
   b. Loads plugin "mobile_base_hardware/MobileBaseHardware"
   c. Parses <param> tags -> fills HardwareInfo.hardware_parameters
   d. Parses <joint> tags -> fills HardwareInfo.joints[]
   e. For each joint, creates command/state interface slots in shared memory

Result in memory:
+----------------------------------------------------------+
| Controller Manager Shared Memory                          |
|                                                            |
| Command Interfaces:                                       |
|   "base_right_wheel_joint/velocity" --> double* (addr X)  |
|   "base_left_wheel_joint/velocity"  --> double* (addr Y)  |
|                                                            |
| State Interfaces:                                         |
|   "base_right_wheel_joint/velocity" --> double* (addr A)  |
|   "base_right_wheel_joint/position" --> double* (addr B)  |
|   "base_left_wheel_joint/velocity"  --> double* (addr C)  |
|   "base_left_wheel_joint/position"  --> double* (addr D)  |
+----------------------------------------------------------+
```

---

## 10.4 — The Plugin System (pluginlib)

### How does the Controller Manager find your class?

Three pieces work together:

**1. The XML descriptor (`my_robot_hardware_interface.xml`):**
```xml
<library path="ros2_control_hardware_template">
  <class name="mobile_base_hardware/MobileBaseHardware"
         type="mobile_base_hardware::MobileBaseHardware"
         base_class_type="hardware_interface::SystemInterface">
    <description>Basic description</description>
  </class>
</library>
```

| Field | Meaning |
|---|---|
| `path` | Shared library filename (without `lib` prefix and `.so` suffix) |
| `name` | Plugin name (used in URDF `<plugin>` tag) |
| `type` | Full C++ class name with namespace |
| `base_class_type` | The parent class it implements |

**2. The CMakeLists.txt registration:**
```cmake
pluginlib_export_plugin_description_file(hardware_interface my_robot_hardware_interface.xml)
```
This tells pluginlib: "Register this XML descriptor under the `hardware_interface` package."

**3. The C++ macro:**
```cpp
PLUGINLIB_EXPORT_CLASS(mobile_base_hardware::MobileBaseHardware, hardware_interface::SystemInterface)
```

### Loading at runtime:

```
Controller Manager:
  1. Reads URDF: plugin = "mobile_base_hardware/MobileBaseHardware"
  2. Asks pluginlib: "Where is this plugin?"
  3. pluginlib reads XML descriptors registered under hardware_interface
  4. Finds: library = "ros2_control_hardware_template"
  5. dlopen("libros2_control_hardware_template.so")   <-- dynamic library loading
  6. Looks for the PLUGINLIB_EXPORT_CLASS symbol
  7. Creates instance of MobileBaseHardware
  8. Calls on_init(info) with the parsed HardwareInfo
```

This is **dynamic loading** — the Controller Manager doesn't link against your library at compile time. It loads it at runtime using `dlopen()` (Linux) or `LoadLibrary()` (Windows).

---

## 10.5 — The CMakeLists.txt Explained

From `ros2_control_hardware_template/CMakeLists.txt`:

```cmake
# Build as shared library (not executable!)
add_library(${PROJECT_NAME} SHARED
  src/mobile_base_hardware_interface.cpp
)

# Link against ros2_control dependencies
ament_target_dependencies(${PROJECT_NAME}
  rclcpp rclcpp_lifecycle hardware_interface pluginlib)

# Register the plugin XML with pluginlib
pluginlib_export_plugin_description_file(
  hardware_interface my_robot_hardware_interface.xml)

# Install the .so library
install(TARGETS ${PROJECT_NAME} DESTINATION lib)

# Export so other packages can find it
ament_export_libraries(${PROJECT_NAME})
```

Key difference from a node: **`add_library(SHARED)`** instead of `add_executable`. This creates a `.so` (Linux) or `.dll` (Windows) file, not a standalone program.

---

## 10.6 — Mock Components for Testing

Notice the commented-out line in the xacro:

```xml
<!-- <plugin>mock_components/GenericSystem</plugin>
     <param name="calculate_dynamics">true</param> -->
```

`mock_components/GenericSystem` is a **fake hardware interface** provided by ros2_control. It:
- Accepts any command interfaces
- Echoes commands back as states (command velocity = state velocity)
- Requires no physical hardware

This is invaluable for testing: you can develop and test controllers without a real robot.

---

## 10.7 — File Organization Summary

```
my_robot.urdf.xacro
  |-- includes common_properties.xacro (colors)
  |-- includes mobile_base.xacro (links, joints, geometry)
  +-- includes mobile_base.ros2_control.xacro (hardware config)

The Controller Manager reads the COMBINED output and uses:
  - <link> and <joint> tags --> robot_state_publisher (TF)
  - <ros2_control> tags --> Controller Manager (hardware + controllers)
```

---

**Next:** [Part 11 — Controllers: DiffDrive](11-controllers-diffdrive.md)

