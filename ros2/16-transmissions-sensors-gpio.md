# Part 16 — Transmissions, Sensors & GPIO

## Overview

ros2_control supports more than just joints with velocity/position commands. This part covers three additional hardware abstractions: **transmissions** (gear ratios, mechanical coupling), **sensors** (read-only interfaces), and **GPIOs** (general-purpose digital/analog I/O).

---

## 1. Transmissions

A transmission models the mechanical coupling between actuators and joints. In many robots, the motor shaft doesn't map 1:1 to the joint:

- **Gear reduction**: motor rotates 50 times for 1 joint rotation (ratio 50:1)
- **Differential coupling**: two motors drive two joints (e.g. wrist roll + pitch)
- **Belt/chain drive**: actuator position ≠ joint position

### URDF declaration

```xml
<ros2_control name="MyRobot" type="system">
  <joint name="joint1">
    <command_interface name="position"/>
    <state_interface name="position"/>
  </joint>
  <transmission name="joint1_transmission">
    <plugin>transmission_interface/SimpleTransmission</plugin>
    <actuator name="motor1" role="actuator1"/>
    <joint name="joint1" role="joint1">
      <mechanical_reduction>50.0</mechanical_reduction>
      <offset>0.0</offset>
    </joint>
  </transmission>
</ros2_control>
```

### What happens at runtime

```
Motor encoder reads 5000 counts
    |
    v
transmission.actuator_to_joint()
    |  joint_pos = actuator_pos / reduction
    |  joint_pos = 5000 / 50 = 100
    v
state_interface["joint1/position"] = 100

Controller writes command:
command_interface["joint1/position"] = 105
    |
    v
transmission.joint_to_actuator()
    |  actuator_cmd = joint_cmd * reduction
    |  actuator_cmd = 105 * 50 = 5250
    v
Send 5250 to motor
```

### Available transmission types

| Type | Use case |
|------|----------|
| `SimpleTransmission` | Single actuator ↔ single joint with gear ratio |
| `DifferentialTransmission` | Two actuators ↔ two joints (differential mechanism) |
| `FourBarLinkageTransmission` | Four-bar mechanical linkage |

---

## 2. Sensors

Sensors are read-only hardware components. They provide state interfaces but no command interfaces. Examples: IMU, force/torque sensor, camera encoder, temperature sensor.

### URDF declaration

```xml
<ros2_control name="IMU" type="sensor">
  <hardware>
    <plugin>my_hardware/IMUSensor</plugin>
    <param name="i2c_address">0x68</param>
  </hardware>
  <sensor name="imu_sensor">
    <state_interface name="orientation.x"/>
    <state_interface name="orientation.y"/>
    <state_interface name="orientation.z"/>
    <state_interface name="orientation.w"/>
    <state_interface name="angular_velocity.x"/>
    <state_interface name="angular_velocity.y"/>
    <state_interface name="angular_velocity.z"/>
    <state_interface name="linear_acceleration.x"/>
    <state_interface name="linear_acceleration.y"/>
    <state_interface name="linear_acceleration.z"/>
  </sensor>
</ros2_control>
```

### Hardware interface for sensors

Sensors inherit from `hardware_interface::SensorInterface` instead of `SystemInterface`:

```cpp
class IMUSensor : public hardware_interface::SensorInterface {
    // Same lifecycle: on_init, on_configure, on_activate, on_deactivate
    // Only read() — no write() method (sensors don't receive commands)
    hardware_interface::return_type read(
        const rclcpp::Time &time, const rclcpp::Duration &period) override;
};
```

### Interface types comparison

| Type | `read()` | `write()` | State interfaces | Command interfaces |
|------|----------|-----------|------------------|--------------------|
| `SystemInterface` | Yes | Yes | Yes | Yes |
| `ActuatorInterface` | Yes | Yes | Yes | Yes (single joint) |
| `SensorInterface` | Yes | No | Yes | No |

`SystemInterface` is the most general — it can handle multiple joints, sensors, and GPIOs. `ActuatorInterface` is for single-joint actuators. `SensorInterface` is for read-only devices.

---

## 3. GPIO (General-Purpose I/O)

GPIOs represent hardware I/O that doesn't map to a joint. Examples:
- LED strip (command: on/off, color)
- Emergency stop button (state: pressed/released)
- Vacuum gripper (command: activate/deactivate, state: pressure)
- Fan speed control

### URDF declaration

```xml
<ros2_control name="MyRobot" type="system">
  <!-- joints... -->
  <gpio name="vacuum_gripper">
    <command_interface name="activate"/>
    <state_interface name="pressure"/>
    <state_interface name="object_detected"/>
  </gpio>
  <gpio name="status_led">
    <command_interface name="red"/>
    <command_interface name="green"/>
    <command_interface name="blue"/>
  </gpio>
</ros2_control>
```

### Accessing GPIO in a controller

GPIO interfaces appear in the same `command_interfaces_` / `state_interfaces_` vectors as joint interfaces. The naming convention is `gpio_name/interface_name`:

```cpp
InterfaceConfiguration command_interface_configuration() const override {
    InterfaceConfiguration config;
    config.type = interface_configuration_type::INDIVIDUAL;
    config.names = {
        "joint1/velocity",           // joint command
        "vacuum_gripper/activate",   // GPIO command
        "status_led/red",            // GPIO command
        "status_led/green",
        "status_led/blue"
    };
    return config;
}
```

### In the hardware interface

GPIOs are handled in the same `read()` / `write()` methods as joints:

```cpp
hardware_interface::return_type write(...) override {
    // Write joint commands
    double vel = command_interfaces_["joint1/velocity"];
    send_velocity(vel);

    // Write GPIO commands
    bool activate = command_interfaces_["vacuum_gripper/activate"] > 0.5;
    set_gripper(activate);

    return hardware_interface::return_type::OK;
}
```

---

## 4. Combining Everything

A real robot often has all three in a single `SystemInterface`:

```xml
<ros2_control name="MyRobot" type="system">
  <hardware>
    <plugin>my_hardware/MyRobotHardware</plugin>
  </hardware>

  <!-- Actuated joints -->
  <joint name="shoulder">
    <command_interface name="position"/>
    <state_interface name="position"/>
    <state_interface name="velocity"/>
    <state_interface name="effort"/>
  </joint>

  <!-- Sensor -->
  <sensor name="force_torque">
    <state_interface name="force.x"/>
    <state_interface name="force.y"/>
    <state_interface name="force.z"/>
    <state_interface name="torque.x"/>
    <state_interface name="torque.y"/>
    <state_interface name="torque.z"/>
  </sensor>

  <!-- GPIO -->
  <gpio name="gripper">
    <command_interface name="close"/>
    <state_interface name="position"/>
  </gpio>

  <!-- Transmission -->
  <transmission name="shoulder_trans">
    <plugin>transmission_interface/SimpleTransmission</plugin>
    <actuator name="shoulder_motor" role="actuator1"/>
    <joint name="shoulder" role="joint1">
      <mechanical_reduction>100.0</mechanical_reduction>
    </joint>
  </transmission>
</ros2_control>
```

All interfaces (joints, sensors, GPIOs) end up in the ResourceManager's flat maps and can be loaned to any controller that requests them.

---

**Prev:** [Part 15 — Lifecycle & State Machines](15-lifecycle-state-machines.md)
**Next:** [Part 17 — MoveIt Architecture](17-moveit-architecture.md)
