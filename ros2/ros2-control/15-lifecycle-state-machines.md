# Part 15 — Lifecycle & State Machines in ros2_control

## Overview

Both hardware interfaces and controllers in ros2_control follow a lifecycle state machine. Understanding the states and transitions is essential for writing robust plugins that handle errors, startup, and shutdown correctly.

---

## 1. The rclcpp_lifecycle State Machine

ros2_control components inherit from `rclcpp_lifecycle::LifecycleNode`. The full state machine from the ROS2 lifecycle spec:

```
                  ┌──────────────┐
                  │  UNCONFIGURED │
                  └──────┬───────┘
                         │ on_configure()
                         v
                  ┌──────────────┐
           ┌──────│   INACTIVE    │◄─────┐
           │      └──────┬───────┘      │
           │             │ on_activate() │ on_deactivate()
           │             v              │
           │      ┌──────────────┐      │
           │      │    ACTIVE     │──────┘
           │      └──────┬───────┘
           │             │ on_error() [automatic on failure]
           │             v
           │      ┌──────────────┐
           │      │   FINALIZED   │
           │      └──────────────┘
           │             ▲
           └─────────────┘
             on_cleanup() or on_shutdown()
```

### Primary states (stable)
- **Unconfigured**: Plugin loaded, constructor called, but no resources allocated
- **Inactive**: Configured and ready, but not processing
- **Active**: Running — `update()` / `read()` / `write()` being called
- **Finalized**: Terminal state, plugin about to be destroyed

### Transition callbacks
| Callback | From → To | Purpose |
|----------|-----------|---------|
| `on_init()` | — → Unconfigured | Called once at plugin load. Declare parameters. |
| `on_configure()` | Unconfigured → Inactive | Allocate resources, open connections, create pubs/subs |
| `on_activate()` | Inactive → Active | Initialize runtime state, start processing |
| `on_deactivate()` | Active → Inactive | Stop processing, keep resources |
| `on_cleanup()` | Inactive → Unconfigured | Release resources, close connections |
| `on_shutdown()` | Any → Finalized | Final cleanup before destruction |
| `on_error()` | Any → Finalized | Handle unrecoverable error |

### Return values

Each callback returns `CallbackReturn::SUCCESS` or `CallbackReturn::ERROR`. Returning ERROR aborts the transition — the component stays in its current state (or moves to error state).

---

## 2. Hardware Interface Lifecycle

Hardware interfaces (`SystemInterface`, `ActuatorInterface`, `SensorInterface`) follow the same pattern but with specific semantics:

```
on_init(HardwareInfo)
  └─ Parse URDF params (servo IDs, baudrate, port)
  └─ Validate configuration
  └─ Initialize member variables

on_configure()
  └─ Open serial port / CAN bus / EtherCAT connection
  └─ Send initialization commands to hardware
  └─ Verify communication

on_activate()
  └─ Enable actuators (send enable torque command)
  └─ Read initial state → set as initial command (smooth start)
  └─ Start accepting read()/write() calls

on_deactivate()
  └─ Disable actuators (send zero velocity or hold position)
  └─ Stop commanding hardware

on_cleanup()
  └─ Close serial port / bus connection
  └─ Free resources
```

### The HardwareInfo struct

Passed to `on_init()`, it contains everything parsed from the URDF `<ros2_control>` tag:

```cpp
struct HardwareInfo {
    std::string name;                    // "MobileBase"
    std::string type;                    // "system"
    std::string hardware_plugin_name;    // "mobile_base_hardware/MobileBaseHardware"
    std::map<std::string, std::string> hardware_parameters;  // servo_id, baudrate, port
    std::vector<ComponentInfo> joints;   // joint names + interfaces
    std::vector<ComponentInfo> sensors;
    std::vector<ComponentInfo> gpios;
};
```

---

## 3. Controller Lifecycle

Controllers have the same states but different typical patterns:

```
on_init()
  └─ auto_declare() parameters (joints, interface_name, gains...)
  └─ No resource allocation yet

on_configure()
  └─ Read declared parameters via get_node()->get_parameter()
  └─ Create subscribers/publishers (these use the executor, not RT thread)
  └─ Pre-allocate buffers (vectors, matrices)

on_activate()
  └─ state_interfaces_ and command_interfaces_ are now available
  └─ Read current state → initialize command buffer (avoid jumps)
  └─ Clear stale commands from subscriber buffer

on_deactivate()
  └─ Optionally send zero/hold command one last time
  └─ command_interfaces_ and state_interfaces_ are released after this
```

### Interface availability

**Critical detail:** `state_interfaces_` and `command_interfaces_` vectors are only populated between `on_activate()` and `on_deactivate()`. Accessing them in `on_configure()` will segfault.

```
on_init()      → state_interfaces_ EMPTY
on_configure() → state_interfaces_ EMPTY  ← don't touch!
on_activate()  → state_interfaces_ POPULATED ← safe to use
update()       → state_interfaces_ POPULATED ← safe to use
on_deactivate()→ state_interfaces_ ABOUT TO BE RELEASED
on_cleanup()   → state_interfaces_ EMPTY
```

---

## 4. Error Handling Strategies

### In hardware interfaces

```cpp
hardware_interface::return_type read(const rclcpp::Time &time, const rclcpp::Duration &period) {
    int bytes = serial_port_.read(buffer_, expected_size_);
    if (bytes != expected_size_) {
        // Option 1: Return error (triggers deactivation)
        return hardware_interface::return_type::ERROR;

        // Option 2: Use last known good value (more resilient)
        RCLCPP_WARN(get_logger(), "Read timeout, using last state");
        return hardware_interface::return_type::OK;
    }
    // parse buffer into state interfaces...
}
```

### In controllers

```cpp
controller_interface::return_type update(const rclcpp::Time &time, const rclcpp::Duration &period) {
    auto state = state_interfaces_[0].get_optional();
    if (!state.has_value()) {
        // Interface not available — hardware might have errored
        return controller_interface::return_type::ERROR;
    }
    // normal computation...
}
```

---

## 5. State Transitions via CLI

You can manually trigger transitions:

```bash
# Load + configure + activate (normal startup via spawner)
ros2 run controller_manager spawner my_controller

# Deactivate a running controller
ros2 control set_controller_state my_controller inactive

# Reactivate
ros2 control set_controller_state my_controller active

# Switch atomically (deactivate A, activate B in same cycle)
ros2 control switch_controllers --deactivate controller_a --activate controller_b
```

For hardware components:

```bash
# List hardware components and their states
ros2 control list_hardware_components

# Deactivate hardware
ros2 control set_hardware_component_state my_hardware inactive
```

---

## 6. Startup Sequence (Full Picture)

Putting it all together, here is the complete boot sequence when you launch ros2_control:

```
1. Launch file starts ros2_control_node
2. Controller Manager receives URDF via robot_description param
3. ResourceManager parses <ros2_control> URDF tags
4. For each hardware component:
   a. pluginlib loads the .so
   b. on_init(HardwareInfo) → parse params
   c. on_configure() → open connections
   d. on_activate() → enable actuators
   e. Register state/command interfaces in ResourceManager maps
5. Controller Manager starts the control loop timer
6. Spawner service calls arrive:
   a. load_controller → dlopen + on_init()
   b. configure_controller → on_configure()
   c. switch_controller → on_activate() (next cycle)
7. Control loop runs: read() → update() → write() at update_rate Hz
```

---

## 7. Quick Reference

| Concept | Key Point |
|---|---|
| Lifecycle states | Unconfigured → Inactive → **Active** → Inactive → Finalized |
| `on_init()` | Parse config, declare params — called once at load |
| `on_configure()` | Open connections, create pubs/subs, allocate memory |
| `on_activate()` | Interfaces become available — init runtime state |
| `on_deactivate()` | Stop commanding — interfaces released after this |
| `on_cleanup()` | Close connections, free resources → back to Unconfigured |
| `on_error()` | Unrecoverable failure → Finalized |
| Return `ERROR` | Aborts the transition — stays in current state |
| Interface availability | `state_interfaces_` only valid between `on_activate()` and `on_deactivate()` |
| HardwareInfo | Struct from URDF: joints, sensors, GPIOs, params, plugin name |
| CLI transitions | `ros2 control set_controller_state name active/inactive` |
| Atomic switch | `ros2 control switch_controllers --deactivate A --activate B` |

---

**Next:** [Part 16 — Transmissions, Sensors & GPIO](16-transmissions-sensors-gpio.md)
