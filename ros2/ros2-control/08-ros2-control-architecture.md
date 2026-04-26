# Part 8 — ros2_control Architecture: The Big Picture

## 8.1 — The Analogy

Imagine a **factory control room**:
- The **Controller Manager** is the supervisor — it orchestrates everything
- **Controllers** are the engineers — they compute what each machine should do
- **Hardware Interfaces** are the operators at the machines — they read gauges and turn knobs
- The **control loop** is the clock on the wall — every tick, everyone does their job

ros2_control separates "what to do" (controllers) from "how to talk to hardware" (hardware interfaces).

---

## 8.2 — Why ros2_control?

Without ros2_control, every robot driver is a custom mess:
```
BAD: Each robot has its own way of doing things
  Robot A driver --> custom topic --> custom controller node
  Robot B driver --> different topic --> different controller
  Robot C driver --> yet another approach
```

With ros2_control:
```
GOOD: Standard interface for everyone
  Any hardware --> standard HW interface --> standard controller

  Swap the robot? Just change the hardware interface plugin.
  Swap the controller? Just load a different one.
  The rest stays the same.
```

---

## 8.3 — The Three Layers

```
+----------------------------------------------------------+
|                    YOUR APPLICATION                        |
|  (ros2 topic pub /cmd_vel geometry_msgs/Twist ...)        |
+----------------------------------------------------------+
          |  /cmd_vel topic                    ^ /odom topic
          v                                    |
+----------------------------------------------------------+
|              CONTROLLER LAYER                              |
|                                                            |
|  +----------------------+  +---------------------------+  |
|  | DiffDriveController  |  | JointStateBroadcaster     |  |
|  | (reads cmd_vel,      |  | (reads joint states,      |  |
|  |  computes wheel      |  |  publishes /joint_states)  |  |
|  |  velocities)         |  |                           |  |
|  +----------+-----------+  +-------------+-------------+  |
|             |                            ^                 |
+----------------------------------------------------------+
|             | command interfaces   state interfaces |       |
|             v                          |                   |
+----------------------------------------------------------+
|              HARDWARE INTERFACE LAYER                       |
|                                                            |
|  +------------------------------------------------------+ |
|  | MobileBaseHardware (SystemInterface)                  | |
|  |                                                        | |
|  | write(): send velocity commands to servos              | |
|  | read():  read position/velocity from servos            | |
|  +------------------------------------------------------+ |
+----------------------------------------------------------+
              |                            ^
              v                            |
+----------------------------------------------------------+
|              PHYSICAL HARDWARE                              |
|  LX-225 Servos via UART (/dev/ttyUSB0)                    |
+----------------------------------------------------------+
```

---

## 8.4 — The Control Loop

The Controller Manager runs a **fixed-rate loop** (50 Hz in your config):

```yaml
# From template_controllers.yaml
controller_manager:
  ros__parameters:
    update_rate: 50   # 50 Hz = every 20ms
```

Every 20ms, the following happens **in order**:

```
Time 0.000s:  +-------+       +----------+       +-------+
              | read()|  -->  | update() |  -->  | write()|
              +-------+       +----------+       +-------+
              Hardware        Controllers         Hardware
              reads           compute new         sends
              sensors         commands            commands

Time 0.020s:  (same cycle repeats)
Time 0.040s:  (same cycle repeats)
...
```

### Step by step:

```
1. READ (Hardware Interface)
   - Ask hardware: "What is the current wheel position and velocity?"
   - Store in state interfaces (shared memory)

2. UPDATE (Controllers)
   - DiffDriveController reads cmd_vel topic
   - Computes: "left wheel needs 1.5 rad/s, right wheel needs 1.2 rad/s"
   - Writes to command interfaces (shared memory)

   - JointStateBroadcaster reads state interfaces
   - Publishes joint positions/velocities to /joint_states topic

3. WRITE (Hardware Interface)
   - Read command interfaces
   - Send velocity commands to LX-225 servos via serial
```

### Timing diagram:

```
     |-------- 20ms (1/50Hz) --------|
     |                                |
     v                                v
  [read] [update] [write]         [read] [update] [write]
  |-----|---------|------|         |-----|---------|------|

  If any step takes too long, the loop falls behind (real-time violation).
  That's why hardware interfaces should be FAST (no blocking I/O).
```

---

## 8.5 — State and Command Interfaces

The bridge between controllers and hardware is through **interfaces** — named double values in shared memory.

From `template_controllers.yaml`:
```yaml
diff_drive_controller:
  ros__parameters:
    left_wheel_names: ["base_left_wheel_joint"]
    right_wheel_names: ["base_right_wheel_joint"]
    wheel_separation: 0.45
    wheel_radius: 0.1
```

This means the controller expects these interfaces to exist:

```
COMMAND interfaces (controller writes, hardware reads):
  base_left_wheel_joint/velocity   --> double (rad/s)
  base_right_wheel_joint/velocity  --> double (rad/s)

STATE interfaces (hardware writes, controller reads):
  base_left_wheel_joint/position   --> double (radians)
  base_left_wheel_joint/velocity   --> double (rad/s)
  base_right_wheel_joint/position  --> double (radians)
  base_right_wheel_joint/velocity  --> double (rad/s)
```

### In memory:

```
Shared Memory (just plain doubles):

  Command buffer:
  +----------------------------+----------------------------+
  | left_wheel/velocity: 1.5   | right_wheel/velocity: 1.2  |
  +----------------------------+----------------------------+
       ^                             ^
       |                             |
  DiffDriveController writes    DiffDriveController writes
  Hardware Interface reads      Hardware Interface reads

  State buffer:
  +----------------------------+----------------------------+
  | left_wheel/position: 3.14  | right_wheel/position: 2.87 |
  | left_wheel/velocity: 1.5   | right_wheel/velocity: 1.2  |
  +----------------------------+----------------------------+
       ^                             ^
       |                             |
  Hardware Interface writes     Hardware Interface writes
  Controllers read              Controllers read
```

No DDS, no serialization, no network — just **direct memory access**. This is why ros2_control can be real-time.

---

## 8.6 — The Controller Manager

The Controller Manager is the node that runs the entire loop. From your launch file:

```xml
<node pkg="controller_manager" exec="ros2_control_node">
    <param from="$(var controller_manager_path)" />
</node>
<node pkg="controller_manager" exec="spawner" args="joint_state_broadcaster" />
<node pkg="controller_manager" exec="spawner" args="diff_drive_controller" />
```

- `ros2_control_node`: the main process running the control loop
- `spawner`: a helper that asks the Controller Manager to load and activate a controller

### Controller lifecycle:

```
unconfigured --> inactive --> active
                              |
                              | (running in the control loop)
                              |
                          deactivate
                              |
                              v
                          inactive --> finalized
```

Only **active** controllers are called in the update step.

---

## 8.7 — The Full Launch Sequence

From `ros2_control/launch/display.launch.xml`:

```
ros2 launch ros2_control display.launch.xml

Timeline:
  t=0s   robot_state_publisher starts
         - Receives URDF (with ros2_control tags)
         - Publishes TF transforms

  t=0s   ros2_control_node starts
         - Reads URDF from robot_description
         - Finds <ros2_control> tag
         - Loads MobileBaseHardware plugin
         - Calls on_init(), on_configure()
         - Starts 50Hz control loop

  t=1s   spawner: joint_state_broadcaster
         - Controller Manager loads the broadcaster
         - on_activate() called
         - Now publishes /joint_states every loop

  t=1s   spawner: diff_drive_controller
         - Controller Manager loads the diff drive
         - on_activate() called
         - Now listens to /cmd_vel and computes wheel speeds

  t=1s   rviz2 starts
         - Subscribes to /tf and /joint_states
         - Displays the robot
```

---

## 8.8 — The Math Behind Real-Time Control

### Fixed-rate loop timing

The control loop runs at a fixed frequency `f`:

```
f = update_rate = 50 Hz
dt = 1/f = 1/50 = 0.02s = 20ms

Each cycle budget:
  t_read + t_update + t_write <= dt

Example:
  t_read   = 0.5ms  (read servo position via serial)
  t_update = 0.1ms  (compute wheel velocities)
  t_write  = 0.5ms  (send commands via serial)
  t_total  = 1.1ms  << 20ms budget  ← OK, plenty of margin
```

### Jitter and stability

**Jitter** = variation in the actual loop period. If the nominal period is 20ms but actual periods vary between 18ms and 22ms, jitter = 2ms.

```
Jitter ratio = jitter / dt

Rule of thumb:
  jitter_ratio < 10%  → acceptable for most robots
  jitter_ratio < 1%   → required for high-precision servo control
  jitter_ratio > 25%  → control loop is unreliable, may cause instability

Example: 50Hz loop (dt=20ms)
  jitter = 2ms → ratio = 10% → borderline acceptable
  jitter = 5ms → ratio = 25% → PROBLEM: controllers assume constant dt

Why it matters:
  Euler integration: position += velocity * dt
  If dt varies, the integration accumulates error proportional to jitter.
  Odometry drift ∝ Σ|dt_actual - dt_nominal|
```

### Why shared memory (not topics)?

The command/state interfaces are plain `double*` pointers — no serialization, no DDS, no copying:

```
Topic-based communication:
  Controller → serialize → DDS → deserialize → Hardware
  Overhead: ~50-500 μs per message

Shared memory (ros2_control):
  Controller → write double* → Hardware reads same pointer
  Overhead: ~0.01 μs (just a memory load instruction)

Speedup: 5000-50000x faster
```

This is why ros2_control can achieve real-time performance.

---

## 8.9 — Quick Reference

| Concept | Key Point |
|---|---|
| ros2_control | Framework separating controllers from hardware interfaces |
| Controller Manager | Node that runs the read → update → write loop |
| Control loop | Fixed-rate: `dt = 1/update_rate`, default 50Hz = 20ms |
| Command interface | `joint/type` (e.g., `left_wheel/velocity`) — controller writes, HW reads |
| State interface | `joint/type` (e.g., `left_wheel/position`) — HW writes, controller reads |
| Shared memory | Plain `double*` pointers — no DDS overhead |
| Controller lifecycle | unconfigured → inactive → **active** (only active ones run) |
| `spawner` | Helper that loads and activates a controller |
| Jitter rule | Keep < 10% of `dt` for stable control |
| `template_controllers.yaml` | Configures update rate, controller types, wheel params |

---

**Next:** [Part 9 — Hardware Interface](09-hardware-interface.md)

