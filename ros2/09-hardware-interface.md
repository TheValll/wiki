# Part 9 — Hardware Interface: The Bridge to Physical Hardware

## 9.1 — The Analogy

The hardware interface is like a **translator** at a diplomatic meeting. On one side, the controllers speak "ros2_control language" (command/state interfaces — plain doubles). On the other side, the hardware speaks "serial protocol" (bytes over a wire). The translator converts between the two.

---

## 9.2 — The SystemInterface Class

Your hardware interface inherits from `hardware_interface::SystemInterface`.

From `mobile_base_hardware_interface.hpp`:

```cpp
class MobileBaseHardware : public hardware_interface::SystemInterface {
public:
    CallbackReturn on_init(const HardwareInfo & info) override;
    CallbackReturn on_configure(const State & previous_state) override;
    CallbackReturn on_activate(const State & previous_state) override;
    CallbackReturn on_deactivate(const State & previous_state) override;
    return_type read(const Time & time, const Duration & period) override;
    return_type write(const Time & time, const Duration & period) override;

private:
    std::shared_ptr<LX225Driver> driver_;
    int servo_id_;
    int baudrate_;
    std::string port_name_;

    double hw_positions_[2]  = {0.0, 0.0};   // Left, Right wheel position (rad)
    double hw_velocities_[2] = {0.0, 0.0};   // Left, Right wheel velocity (rad/s)
    double hw_commands_[2]   = {0.0, 0.0};    // Left, Right wheel target velocity
};
```

### Hardware interface types:

| Type | Description | Example |
|---|---|---|
| `SystemInterface` | Multiple joints controlled together | Your mobile base (2 wheels) |
| `ActuatorInterface` | Single joint/actuator | A single motor |
| `SensorInterface` | Read-only (no commands) | An encoder, IMU |

Your robot uses `SystemInterface` because both wheels are on the same serial bus.

---

## 9.3 — Memory Layout

```
HEAP:
+------------------------------------------------------------+
| MobileBaseHardware object                                   |
|                                                              |
| driver_ (shared_ptr) -----> LX225Driver object              |
|                              |-- serial port handle         |
|                              |-- baudrate = 115200          |
|                              +-- servo_id = 6              |
|                                                              |
| servo_id_ = 6                                               |
| baudrate_ = 115200                                          |
| port_name_ = "/dev/ttyUSB0"                                |
|                                                              |
| hw_positions_[2]:                                           |
|   [0] = 3.14  (left wheel, radians)                        |
|   [1] = 2.87  (right wheel, radians)                       |
|                                                              |
| hw_velocities_[2]:                                          |
|   [0] = 1.5   (left wheel, rad/s)                          |
|   [1] = 1.2   (right wheel, rad/s)                         |
|                                                              |
| hw_commands_[2]:                                            |
|   [0] = 1.5   (left wheel target, rad/s)                   |
|   [1] = 1.2   (right wheel target, rad/s)                  |
+------------------------------------------------------------+

These arrays are the LOCAL COPIES of state/command data.
The Controller Manager's shared interfaces point to these values.
```

---

## 9.4 — Lifecycle: The State Machine

The hardware interface follows a **lifecycle** pattern — it goes through states in order:

```
                    on_init()
  [UNCONFIGURED] ──────────> [CONFIGURED]
                                  |
                           on_configure()
                                  |
                           on_activate()
                                  v
                              [ACTIVE]
                           read()/write()
                           called every cycle
                                  |
                          on_deactivate()
                                  v
                             [INACTIVE]
                                  |
                           on_cleanup()
                                  v
                           [UNCONFIGURED]
```

### What each step does in your code:

**`on_init()` — Parse configuration:**
```cpp
CallbackReturn MobileBaseHardware::on_init(const HardwareInfo & info)
{
    // 1. Call parent init (validates URDF hardware info)
    SystemInterface::on_init(info);

    // 2. Read parameters from URDF <param> tags
    servo_id_  = std::stoi(info_.hardware_parameters.at("servo_id"));   // "6"
    baudrate_  = std::stoi(info_.hardware_parameters.at("baudrate"));   // "115200"
    port_name_ = info_.hardware_parameters.at("port");                   // "/dev/ttyUSB0"

    // 3. Zero out all state arrays
    for (size_t i = 0; i < 2; i++) {
        hw_positions_[i] = hw_velocities_[i] = hw_commands_[i] = 0.0;
    }

    // 4. Create the driver object (but don't open the port yet)
    driver_ = std::make_shared<LX225Driver>(port_name_, baudrate_, servo_id_);

    return CallbackReturn::SUCCESS;
}
```

The `info_` object comes from parsing the URDF's `<ros2_control>` tag. It contains everything declared in the xacro file.

**`on_configure()` — Open hardware connection:**
```cpp
CallbackReturn MobileBaseHardware::on_configure(const State & previous_state)
{
    if (driver_->init() != 0)    // Opens serial port, sets baudrate
        return CallbackReturn::ERROR;
    return CallbackReturn::SUCCESS;
}
```

This is where the serial port actually opens. If it fails (port doesn't exist, permission denied), the hardware goes to ERROR state.

**`on_activate()` — Ready to run:**
```cpp
CallbackReturn MobileBaseHardware::on_activate(const State & previous_state)
{
    hw_positions_[0] = hw_positions_[1] = 0.0;
    hw_velocities_[0] = hw_velocities_[1] = 0.0;
    return CallbackReturn::SUCCESS;
}
```

Resets positions to zero. After this, `read()` and `write()` will be called every cycle.

**`on_deactivate()` — Stop and cleanup:**
```cpp
CallbackReturn MobileBaseHardware::on_deactivate(const State & previous_state)
{
    driver_->close_LX225();   // Returns servo to default position, closes port
    return CallbackReturn::SUCCESS;
}
```

---

## 9.5 — The read() Function

Called every cycle (50 Hz) BEFORE controllers update:

```cpp
return_type MobileBaseHardware::read(const Time & time, const Duration & period)
{
    // Integrate velocity to get position: position += velocity * dt
    for (int i = 0; i < 2; i++) {
        hw_positions_[i] += hw_commands_[i] * period.seconds();
        hw_velocities_[i] = hw_commands_[i];
    }

    // Push values to the state interfaces (shared with controllers)
    set_state("base_left_wheel_joint/position", hw_positions_[0]);
    set_state("base_left_wheel_joint/velocity", hw_velocities_[0]);
    set_state("base_right_wheel_joint/position", hw_positions_[1]);
    set_state("base_right_wheel_joint/velocity", hw_velocities_[1]);

    return return_type::OK;
}
```

### The math: position integration

```
position(t) = position(t-1) + velocity * dt

Where:
  dt = period.seconds() = 0.02s (at 50Hz)
  velocity = hw_commands_ (the commanded velocity, assumed = actual)

Example:
  position_prev = 3.14 rad
  velocity = 1.5 rad/s
  dt = 0.02s

  position_new = 3.14 + 1.5 * 0.02 = 3.14 + 0.03 = 3.17 rad
```

This is **Euler integration** — the simplest numerical integration method. It assumes velocity is constant during each timestep.

**Note:** In this implementation, `read()` doesn't actually read from the physical hardware. It estimates position from the commanded velocity. A real implementation would read encoder values from the servo.

---

## 9.6 — The write() Function

Called every cycle AFTER controllers update:

```cpp
return_type MobileBaseHardware::write(const Time & time, const Duration & period)
{
    // Get commands from the controller (via command interfaces)
    double cmd_left  = get_command("base_left_wheel_joint/velocity");
    double cmd_right = get_command("base_right_wheel_joint/velocity");

    // NaN safety check
    hw_commands_[0] = std::isnan(cmd_left)  ? 0.0 : cmd_left;
    hw_commands_[1] = std::isnan(cmd_right) ? 0.0 : cmd_right;

    return return_type::OK;
}
```

### Why check for NaN?

When a controller is first loaded but hasn't received any commands yet, the command interface value may be **NaN** (Not a Number). Sending NaN to a servo would be dangerous, so we default to 0.0.

```
NaN in IEEE 754:
  Sign: X  Exponent: 11111111111  Mantissa: non-zero
  Special bit pattern that means "this is not a valid number"

  std::isnan(x) checks this bit pattern.
```

**Note:** This implementation stores the commands but doesn't actually send them to the serial port. A complete implementation would call `driver_->set_position()` or a velocity command here.

---

## 9.7 — The Plugin Registration

At the bottom of `mobile_base_hardware_interface.cpp`:

```cpp
#include "pluginlib/class_list_macros.hpp"
PLUGINLIB_EXPORT_CLASS(
    mobile_base_hardware::MobileBaseHardware,
    hardware_interface::SystemInterface)
```

This macro tells the **pluginlib** system: "This shared library contains a class `MobileBaseHardware` that implements `SystemInterface`."

The Controller Manager can then load this plugin **by name at runtime** — it doesn't need to know about it at compile time.

---

## 9.8 — Full Cycle Diagram

```
50Hz loop:
+----------------------------------------------------------------+
|                                                                  |
|  1. read()                                                       |
|     hw_positions_[i] += hw_commands_[i] * 0.02                  |
|     set_state("left/position", hw_positions_[0])                |
|     set_state("left/velocity", hw_velocities_[0])               |
|     set_state("right/position", hw_positions_[1])               |
|     set_state("right/velocity", hw_velocities_[1])              |
|                                                                  |
|  2. Controllers update()                                         |
|     JointStateBroadcaster: reads states, publishes /joint_states |
|     DiffDriveController: reads /cmd_vel, writes wheel commands   |
|                                                                  |
|  3. write()                                                      |
|     cmd_left = get_command("left/velocity")                      |
|     cmd_right = get_command("right/velocity")                    |
|     hw_commands_[0] = cmd_left                                   |
|     hw_commands_[1] = cmd_right                                  |
|     (would send to servo here)                                   |
|                                                                  |
+--- repeat in 20ms ---------------------------------------------|
```

---

**Next:** [Part 10 — ros2_control URDF](10-ros2-control-urdf.md)

