# Part 13 — Writing a Custom Controller

## Overview

In Parts 8-11 we used existing controllers (JointStateBroadcaster, DiffDriveController). Now we build one from scratch. A controller is a pluginlib plugin that the Controller Manager loads at runtime via `dlopen()`. It reads state interfaces, computes new commands, and writes to command interfaces — all within the real-time loop.

---

## 1. Controller Lifecycle

A controller inherits from `controller_interface::ControllerInterface`, which itself inherits from `rclcpp_lifecycle::LifecycleNode`. The lifecycle states mirror the hardware interface:

```
[unconfigured] --on_configure()--> [inactive] --on_activate()--> [active]
                                       ^                            |
                                       |----on_deactivate()---------|
```

| Callback | When it runs | What to do |
|----------|-------------|------------|
| `on_init()` | Plugin is first loaded | Declare parameters (`auto_declare`), store member defaults |
| `on_configure()` | `configure` transition | Read parameters, create subscribers/publishers, allocate memory |
| `on_activate()` | `activate` transition | Initialize command buffer from current state (smooth start) |
| `on_deactivate()` | `deactivate` transition | Stop commanding, release resources |
| `update()` | Every control cycle (e.g. 50 Hz) | Read state interfaces, compute, write command interfaces |

**Key rule:** `update()` runs in the real-time thread. No memory allocation, no blocking calls, no DDS communication inside `update()`. Subscribers and publishers created in `on_configure()` are fine — the DDS callbacks run in a separate executor thread.

---

## 2. Interface Configuration

The Controller Manager needs to know which joints/interfaces your controller requires. You declare this by overriding two methods:

```cpp
controller_interface::InterfaceConfiguration command_interface_configuration() const override;
controller_interface::InterfaceConfiguration state_interface_configuration() const override;
```

Each returns an `InterfaceConfiguration` with:
- `type`: `INDIVIDUAL` (explicit list), `ALL` (claim everything), or `NONE`
- `names`: vector of `"joint_name/interface_name"` strings (e.g. `"base_left_wheel_joint/velocity"`)

**What happens under the hood:**
1. Controller Manager calls your `command_interface_configuration()`
2. It looks up each name in its internal `ResourceManager` map
3. It loans you direct pointers to the shared `double` values
4. These appear in `command_interfaces_` and `state_interfaces_` (inherited vectors)

```
Controller                    ResourceManager                Hardware Interface
    |                              |                              |
    |-- command_interface_config ->|                              |
    |<- loan command_interfaces ---|                              |
    |                              |                              |
    |   update() writes to         |                              |
    |   command_interfaces_[i] --->|--- same memory address ----->| write() reads it
    |                              |                              |
    |   state_interfaces_[i] <----|--- same memory address <-----| read() writes it
```

**No DDS, no serialization, no copies.** The controller and hardware interface share the same `double*` in memory. This is why ros2_control achieves real-time performance.

---

## 3. The update() Method

```cpp
controller_interface::return_type update(const rclcpp::Time &time, const rclcpp::Duration &period) override;
```

This is called every cycle by the Controller Manager's main loop:

```
while (running) {
    hardware.read(time, period);       // HW writes to state doubles
    controller.update(time, period);   // Controller reads state, writes command
    hardware.write(time, period);      // HW reads command doubles, sends to actuators
}
```

**Parameters:**
- `time`: current ROS clock time
- `period`: duration since last `update()` call (typically 20ms at 50Hz)

**Return value:** `return_type::OK` or `return_type::ERROR`. Returning ERROR triggers deactivation.

### Accessing interfaces

```cpp
// Read state (position, velocity, etc.)
double position = state_interfaces_[0].get_optional().value();

// Write command
command_interfaces_[0].set_value(new_velocity);
```

The indices match the order you declared in `state_interface_configuration()` / `command_interface_configuration()`.

---

## 4. Receiving External Commands

Controllers often subscribe to a topic to receive setpoints from the user or a planner. The pattern:

1. **Create subscriber in `on_configure()`** — this runs outside the real-time loop
2. **Store received data in a member variable** — the callback runs in the executor thread
3. **Read the stored data in `update()`** — the real-time thread

```cpp
// on_configure()
command_subscriber_ = get_node()->create_subscription<FloatArray>(
    "joints_command", 10,
    [this](const FloatArray::SharedPtr msg) {
        // Executor thread — NOT real-time
        stored_command_ = msg->data;
    });

// update()
// Real-time thread — just read stored_command_
double cmd = stored_command_[i];
```

**Thread safety note:** For production controllers, use `realtime_tools::RealtimeBuffer` to safely pass data between the executor thread (subscriber callback) and the real-time thread (`update()`). A simple member variable works for learning but can cause torn reads in multi-threaded scenarios.

---

## 5. Plugin Registration

For the Controller Manager to discover your controller at runtime, you need three things:

### a) The PLUGINLIB_EXPORT_CLASS macro

At the bottom of your `.cpp` file:
```cpp
#include "pluginlib/class_list_macros.hpp"
PLUGINLIB_EXPORT_CLASS(MyNamespace::MyController, controller_interface::ControllerInterface)
```

This generates a factory function that `dlopen()` / `dlsym()` will find.

### b) The plugin XML descriptor

```xml
<library path="my_controller_lib">
  <class name="my_namespace/MyController"
         type="MyNamespace::MyController"
         base_class_type="controller_interface::ControllerInterface">
    <description>My custom controller</description>
  </class>
</library>
```

- `path`: the shared library name (without `lib` prefix and `.so` suffix)
- `name`: the string you use in YAML config (`type: my_namespace/MyController`)
- `type`: the actual C++ fully-qualified class name

### c) CMakeLists.txt export

```cmake
pluginlib_export_plugin_description_file(controller_interface my_plugin.xml)
```

This installs the XML so pluginlib can find it at runtime.

---

## 6. Loading a Custom Controller

In your controller YAML config:

```yaml
controller_manager:
  ros__parameters:
    update_rate: 50
    my_controller:
      type: basic_controller/BasicController

my_controller:
  ros__parameters:
    joints: ["joint1", "joint2"]
    interface_name: "velocity"
    coefficient: 0.8
```

Then spawn it:
```bash
ros2 run controller_manager spawner my_controller
```

The Controller Manager will:
1. `dlopen()` the shared library
2. Call the factory function to instantiate your class
3. Call `on_init()` → `on_configure()` → `on_activate()`
4. Start calling `update()` every cycle

---

## 7. Common Pitfalls

| Mistake | Why it breaks | Fix |
|---------|--------------|-----|
| Allocating memory in `update()` | Breaks real-time guarantee (malloc can block) | Pre-allocate in `on_configure()` or `on_activate()` |
| Plugin XML `type` doesn't match C++ namespace | `dlsym()` can't find the factory | Ensure `type=` matches `PLUGINLIB_EXPORT_CLASS` exactly |
| Missing `;` after class definition | Cryptic compiler errors about non-member functions | Always `}; // class` |
| Using `::` instead of `/` in `#include` | File not found | `#include "pkg/header.hpp"` |
| Forgetting `ClassName::` in `.cpp` definitions | Functions are free functions, not methods — can't access `this` | Always qualify: `MyClass::on_init()` |
| Wrong interface name in config | Controller Manager can't loan the interface | Match URDF `<command_interface name="...">` exactly |

---

## 8. Complete Example: Alpha Filter Controller

Here is a complete custom controller from the repo that implements a **low-pass alpha filter** — it smoothly follows incoming commands instead of jumping to them instantly.

### Full C++ code — `basic_controller.cpp`:

```cpp
// on_init(): declare parameters
joint_names_ = auto_declare<std::vector<std::string>>("joints", {});
interface_name_ = auto_declare<std::string>("interface_name", "position");
coefficient_ = auto_declare<double>("coefficient", 0.8);

// on_configure(): subscribe to external commands
command_subscriber_ = get_node()->create_subscription<FloatArray>(
    "joints_command", 10,
    [this](const FloatArray::SharedPtr msg) {
        if (msg->data.size() == joint_names_.size()) {
            appCommand_.clear();
            for (auto cmd : msg->data)
                appCommand_.push_back(cmd);
        }
    });

// on_activate(): initialize commands from current state (no jump)
for (int i = 0; i < (int)joint_names_.size(); i++) {
    appCommand_.push_back(state_interfaces_[i].get_optional().value());
}

// update(): the alpha filter
for (int i = 0; i < (int)joint_names_.size(); i++) {
    double state = state_interfaces_[i].get_optional().value();
    double cmd = appCommand_[i];
    double new_cmd = cmd * coefficient_ + state * (1 - coefficient_);
    command_interfaces_[i].set_value(new_cmd);
}
```

### The alpha filter — math explanation

```
new_cmd = α * target + (1 - α) * current_state

Where:
  α = coefficient (0 to 1)
  target = desired command from the subscriber
  current_state = actual joint state from hardware

α = 1.0 → new_cmd = target         (instant jump, no filtering)
α = 0.0 → new_cmd = current_state  (never moves)
α = 0.8 → 80% target + 20% current (fast but smooth)
α = 0.2 → 20% target + 80% current (slow and very smooth)
```

This is a **first-order IIR (Infinite Impulse Response) filter**, also called an **exponential moving average**:

```
Step response (α = 0.8, target = 1.0, start = 0.0):

  Step 0: new = 0.8*1.0 + 0.2*0.0 = 0.80
  Step 1: new = 0.8*1.0 + 0.2*0.8 = 0.96
  Step 2: new = 0.8*1.0 + 0.2*0.96 = 0.992
  Step 3: new = 0.8*1.0 + 0.2*0.992 = 0.998

Convergence: after n steps, error = (1-α)^n * initial_error
  n steps to reach 99%: n = log(0.01) / log(1-α)
  For α=0.8: n = log(0.01)/log(0.2) = 2.86 → ~3 steps

At 50Hz with α=0.8: reaches 99% in 3 * 20ms = 60ms
```

---

## 9. Quick Reference

| Concept | Key Point |
|---|---|
| Controller base | `controller_interface::ControllerInterface` (inherits LifecycleNode) |
| `on_init()` | Declare parameters with `auto_declare<T>()` |
| `on_configure()` | Create subscribers/publishers, allocate memory |
| `on_activate()` | Interfaces available — init commands from current state |
| `update(time, period)` | Called every cycle — **no malloc, no blocking** |
| Interface config | Return `INDIVIDUAL` + list of `"joint/type"` names |
| Command interface | **Exclusive** — only one controller can claim it |
| State interface | **Shared** — multiple controllers can read |
| Plugin registration | `PLUGINLIB_EXPORT_CLASS(Class, ControllerInterface)` + XML + CMake |
| Spawner | `ros2 run controller_manager spawner my_controller` |
| Alpha filter | `new = α*target + (1-α)*current` — convergence in `log(ε)/log(1-α)` steps |
| RealtimeBuffer | Thread-safe way to pass subscriber data to `update()` |

---

**Next:** [Part 14 — Controller Manager Internals](14-controller-manager-internals.md)
