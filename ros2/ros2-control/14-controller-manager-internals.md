# Part 14 — Controller Manager Internals

## Overview

The Controller Manager is the central orchestrator of ros2_control. It owns the real-time loop, manages controller lifecycles, and brokers access to hardware interfaces through the ResourceManager. This part explores what happens inside.

---

## 1. The Real-Time Control Loop

The Controller Manager runs a single timer callback at the configured `update_rate`:

```
┌─────────────────────────────────────────────────────┐
│                  Control Loop (50 Hz)                │
│                                                     │
│  1. resource_manager_->read(time, period)           │
│     └─ calls read() on every loaded HW component    │
│                                                     │
│  2. for each active controller:                     │
│        controller->update(time, period)             │
│                                                     │
│  3. resource_manager_->write(time, period)          │
│     └─ calls write() on every loaded HW component   │
│                                                     │
│  4. Check for pending lifecycle transitions         │
│     └─ activate/deactivate queued controllers       │
└─────────────────────────────────────────────────────┘
```

### Timing budget

At 50 Hz, each cycle has a **20 ms budget**. If `read() + update() + write()` exceeds this, the loop overruns and the next cycle starts late. The Controller Manager logs a warning but does not skip cycles.

```
Timeline (20ms window):
|-- read (1ms) --|-- update (2ms) --|-- write (1ms) --|-- idle (16ms) --|
```

Real-time kernels (`PREEMPT_RT`) reduce jitter by ensuring the timer callback is scheduled promptly. Without it, other processes can delay the loop by milliseconds.

---

## 2. ResourceManager

The ResourceManager is the bookkeeper for all hardware interfaces. It:

1. **Parses the URDF** `<ros2_control>` tags at startup
2. **Loads hardware plugins** via pluginlib (`dlopen`)
3. **Stores all state and command interface values** as `double` in flat maps
4. **Loans interfaces** to controllers when they activate

### Interface storage

```
ResourceManager internal maps:
  state_interfaces_:
    "base_left_wheel_joint/position"  → double* (0x7f...)
    "base_left_wheel_joint/velocity"  → double* (0x7f...)
    "base_right_wheel_joint/position" → double* (0x7f...)
    "base_right_wheel_joint/velocity" → double* (0x7f...)

  command_interfaces_:
    "base_left_wheel_joint/velocity"  → double* (0x7f...)
    "base_right_wheel_joint/velocity" → double* (0x7f...)
```

### Interface claiming

Command interfaces are **exclusive**: only one controller can claim a given command interface at a time. If `diff_drive_controller` already claims `base_left_wheel_joint/velocity`, a second controller requesting the same interface will fail to activate.

State interfaces are **shared**: multiple controllers can read the same state simultaneously.

```
diff_drive_controller ──claims──> "base_left_wheel_joint/velocity" (command) ✓
my_controller ────────claims──> "base_left_wheel_joint/velocity" (command) ✗ CONFLICT

joint_state_broadcaster ──reads──> "base_left_wheel_joint/velocity" (state) ✓
diff_drive_controller ────reads──> "base_left_wheel_joint/velocity" (state) ✓  (shared)
```

---

## 3. Controller Lifecycle Management

The Controller Manager handles lifecycle transitions asynchronously. When you run:

```bash
ros2 run controller_manager spawner my_controller
```

The spawner makes service calls to the Controller Manager:

```
spawner                          Controller Manager
   |                                    |
   |-- /load_controller (srv) -------->|  dlopen() + instantiate + on_init()
   |<-- OK ----------------------------|
   |                                    |
   |-- /configure_controller (srv) --->|  on_configure()
   |<-- OK ----------------------------|
   |                                    |
   |-- /switch_controller (srv) ------>|  queue activation
   |<-- OK ----------------------------|
   |                                    |
   |  (next control cycle)              |  on_activate() + start calling update()
```

### Switch controller

`/switch_controller` is the most complex service. It supports:
- **Activating** controllers (transition from inactive to active)
- **Deactivating** controllers (active to inactive)
- **Switching** — deactivate one set and activate another atomically

The `strictness` parameter controls behavior:
- `BEST_EFFORT`: activate what you can, skip failures
- `STRICT`: all-or-nothing — if any controller fails, none switch

**Atomic switch** is critical for safety. Example: switching from position control to velocity control. You want to deactivate the position controller and activate the velocity controller in the same cycle, with no gap where neither is running.

---

## 4. Controller Execution Order

When multiple controllers are active, `update()` is called in the order they were activated. There is no explicit priority system.

If controller B depends on controller A's output (chaining), activate A first:

```bash
ros2 run controller_manager spawner controller_a
ros2 run controller_manager spawner controller_b
```

### Controller chaining (advanced)

ros2_control supports **chained controllers** where one controller's output feeds another's input via "reference interfaces". Example:

```
PID Controller ──writes──> reference_interface ──read by──> Effort Controller ──writes──> command_interface
```

This allows cascaded control loops (position → velocity → effort) within the same real-time cycle.

---

## 5. The ros2_control Node

The Controller Manager runs as a ROS2 node (`ros2_control_node`). It exposes:

### Services
| Service | Purpose |
|---------|---------|
| `/controller_manager/list_controllers` | List all loaded controllers and their states |
| `/controller_manager/list_controller_types` | List all available controller plugins |
| `/controller_manager/load_controller` | Load a controller plugin |
| `/controller_manager/unload_controller` | Unload a controller |
| `/controller_manager/configure_controller` | Trigger `on_configure()` |
| `/controller_manager/switch_controller` | Activate/deactivate controllers |
| `/controller_manager/list_hardware_interfaces` | List all available interfaces |
| `/controller_manager/list_hardware_components` | List hardware components and states |

### Parameters
| Parameter | Default | Purpose |
|-----------|---------|---------|
| `update_rate` | 100 | Control loop frequency in Hz |
| `<controller_name>.type` | — | Plugin name for each controller |

### CLI shortcuts

The `ros2 control` CLI wraps these services:

```bash
ros2 control list_controllers          # → calls list_controllers service
ros2 control list_hardware_interfaces  # → calls list_hardware_interfaces service
ros2 control list_controller_types     # → calls list_controller_types service
```

---

## 6. URDF Parsing Flow

At startup, the full chain is:

```
Launch file
    |
    v
robot_state_publisher (receives URDF via param)
    |
    v
ros2_control_node (receives same URDF via param)
    |
    v
Controller Manager::init()
    |
    v
ResourceManager::load_urdf(urdf_string)
    |
    ├── Parse all <ros2_control> tags
    ├── For each <hardware>:
    │     ├── pluginlib::ClassLoader::createInstance(plugin_name)
    │     ├── hardware->on_init(HardwareInfo)  // pass URDF params
    │     └── Register state/command interfaces
    └── Build interface maps
```

The `HardwareInfo` struct passed to `on_init()` contains everything from the URDF:
- Joint names
- Interface names and min/max values
- Plugin parameters (`<param>` tags)
- Sensor and GPIO definitions

---

## 7. Memory Layout at Runtime

```
Process: ros2_control_node
├── Main thread (control loop)
│   ├── ResourceManager
│   │   ├── state_interface doubles  ← HW read() writes here
│   │   └── command_interface doubles ← Controller update() writes here
│   ├── Hardware plugins (loaded via dlopen)
│   │   └── MobileBaseHardware instance
│   └── Controller plugins (loaded via dlopen)
│       ├── JointStateBroadcaster instance
│       └── DiffDriveController instance
│
├── Executor thread(s)
│   ├── Service callbacks (load/switch/list controllers)
│   ├── Subscriber callbacks (controller topics)
│   └── Publisher callbacks (joint_states, odom, tf)
│
└── Shared memory
    └── double values for each interface (the actual data)
```

The control loop thread and the executor thread(s) share the process. Interface doubles live in the process heap. No IPC, no serialization — just pointer dereferencing.

---

## 8. Quick Reference

| Concept | Key Point |
|---|---|
| Controller Manager | Node that owns the real-time loop + manages controller lifecycles |
| ResourceManager | Stores all interface `double*` pointers, loans them to controllers |
| Control loop | `read()` → `update()` (all active controllers) → `write()` at `update_rate` Hz |
| Command interfaces | **Exclusive** — one controller at a time |
| State interfaces | **Shared** — multiple readers OK |
| Spawner | Calls `load → configure → switch_controller` services |
| Atomic switch | Deactivate + activate in the same cycle (no gap) |
| `STRICT` mode | All-or-nothing switch — if one fails, none switch |
| Controller order | `update()` called in activation order |
| Chained controllers | Output → reference interface → input of next controller |
| `ros2 control` CLI | `list_controllers`, `list_hardware_interfaces`, `switch_controllers` |
| URDF parsing | `ResourceManager::load_urdf()` → parse tags → dlopen plugins → register interfaces |

---

**Next:** [Part 15 — Lifecycle & State Machines](15-lifecycle-state-machines.md)
