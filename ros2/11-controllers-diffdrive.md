# Part 11 — Controllers: DiffDrive & JointStateBroadcaster

## 11.1 — The Analogy

Think of a **car's steering system**:
- You turn the steering wheel (high-level input: "go left")
- The power steering system converts that into specific wheel angles (low-level output)

The DiffDriveController does the same: you say "move forward at 0.5 m/s and turn left at 0.3 rad/s" (`cmd_vel`), and it calculates the exact speed each wheel needs.

---

## 11.2 — Your Two Controllers

From `template_controllers.yaml`:

```yaml
controller_manager:
  ros__parameters:
    update_rate: 50

    joint_state_broadcaster:
      type: joint_state_broadcaster/JointStateBroadcaster

    diff_drive_controller:
      type: diff_drive_controller/DiffDriveController
```

| Controller | Purpose | Input | Output |
|---|---|---|---|
| **JointStateBroadcaster** | Publishes joint data to ROS2 topics | State interfaces (position, velocity) | `/joint_states` topic |
| **DiffDriveController** | Converts cmd_vel to wheel speeds | `/cmd_vel` topic | Command interfaces (wheel velocities) |

---

## 11.3 — JointStateBroadcaster

This is the simplest controller. It just reads and publishes:

```
Every cycle (50Hz):
  1. Read state interfaces:
     - base_left_wheel_joint/position  = 3.14 rad
     - base_left_wheel_joint/velocity  = 1.5 rad/s
     - base_right_wheel_joint/position = 2.87 rad
     - base_right_wheel_joint/velocity = 1.2 rad/s

  2. Publish to /joint_states topic:
     sensor_msgs/msg/JointState {
       header.stamp: <current time>
       name: ["base_left_wheel_joint", "base_right_wheel_joint"]
       position: [3.14, 2.87]
       velocity: [1.5, 1.2]
     }
```

RViz and robot_state_publisher subscribe to `/joint_states` to visualize the robot.

---

## 11.4 — Differential Drive Kinematics (The Math)

A differential drive robot has two independently driven wheels. By spinning them at different speeds, the robot can move forward, backward, or turn.

### The robot model:

```
        Front
          ^
          |
  +-------+-------+
  |               |
  |   base_link   |
  |               |
  O               O
left            right
wheel           wheel
  |<--- L/2 --->|<--- L/2 --->|
  |<---------- L ------------->|

  L = wheel_separation = 0.45m
  r = wheel_radius = 0.1m
```

### Forward kinematics: wheel speeds -> robot motion

Given left wheel angular velocity `w_L` and right wheel angular velocity `w_R`:

```
Linear velocity (forward speed):
  v = r * (w_R + w_L) / 2

Angular velocity (turning speed):
  omega = r * (w_R - w_L) / L

Where:
  r = wheel_radius = 0.1m
  L = wheel_separation = 0.45m
  w_L, w_R = wheel angular velocities (rad/s)
```

**Intuition:**
- Both wheels same speed → robot goes straight (`w_R - w_L = 0, omega = 0`)
- Right wheel faster → robot turns left (`w_R > w_L, omega > 0`)
- Wheels spin opposite → robot rotates in place (`w_R = -w_L, v = 0`)

### Inverse kinematics: robot motion -> wheel speeds

The DiffDriveController needs the **inverse** — given desired `v` and `omega`, compute wheel speeds:

```
w_L = (v - omega * L/2) / r
w_R = (v + omega * L/2) / r

Where:
  v = desired linear velocity (m/s) from cmd_vel.linear.x
  omega = desired angular velocity (rad/s) from cmd_vel.angular.z
```

### Example calculation:

```
Desired: v = 0.5 m/s, omega = 0.3 rad/s (gentle left turn)

w_L = (0.5 - 0.3 * 0.225) / 0.1 = (0.5 - 0.0675) / 0.1 = 4.325 rad/s
w_R = (0.5 + 0.3 * 0.225) / 0.1 = (0.5 + 0.0675) / 0.1 = 5.675 rad/s

Right wheel spins faster -> robot turns left. Correct!
```

### Another example: pure rotation (spin in place):

```
Desired: v = 0 m/s, omega = 1.0 rad/s

w_L = (0 - 1.0 * 0.225) / 0.1 = -2.25 rad/s (backward!)
w_R = (0 + 1.0 * 0.225) / 0.1 = +2.25 rad/s (forward)

Wheels spin opposite -> robot rotates in place. Correct!
```

---

## 11.5 — DiffDriveController Configuration

```yaml
diff_drive_controller:
  ros__parameters:
    left_wheel_names: ["base_left_wheel_joint"]
    right_wheel_names: ["base_right_wheel_joint"]
    wheel_separation: 0.45        # L (meters)
    wheel_radius: 0.1             # r (meters)
    odom_frame_id: "odom"
    base_frame_id: "base_footprint"
    pose_covariance_diagonal: [0.001, 0.001, 0.001, 0.001, 0.001, 0.01]
    twist_covariance_diagonal: [0.001, 0.001, 0.001, 0.001, 0.001, 0.01]
    enable_odom_tf: true
    publish_rate: 50.0
    linear.x.max_velocity: 1.0    # m/s
    linear.x.min_velocity: -1.0
    angular.z.max_velocity: 1.0   # rad/s
    angular.z.min_velocity: -1.0
```

### Parameters explained:

| Parameter | Meaning |
|---|---|
| `wheel_separation` | Distance between wheel centers (L) |
| `wheel_radius` | Wheel radius (r) |
| `odom_frame_id` | Name of the odometry frame |
| `base_frame_id` | Name of the robot's base frame |
| `pose_covariance_diagonal` | Uncertainty in position estimate (for sensor fusion) |
| `twist_covariance_diagonal` | Uncertainty in velocity estimate |
| `enable_odom_tf` | Publish odom -> base_footprint transform? |
| `linear.x.max_velocity` | Speed limit (m/s) |
| `angular.z.max_velocity` | Turn rate limit (rad/s) |

---

## 11.6 — Odometry

The DiffDriveController also computes **odometry** — the robot's estimated position in the world.

### The math (2D pose integration):

```
At each timestep, given v and omega:

  x(t) = x(t-1) + v * cos(theta) * dt
  y(t) = y(t-1) + v * sin(theta) * dt
  theta(t) = theta(t-1) + omega * dt

Where:
  (x, y) = position in the world
  theta = heading angle
  dt = timestep (0.02s at 50Hz)
```

### Example trajectory:

```
Start: x=0, y=0, theta=0 (facing forward)

Step 1: v=0.5, omega=0, dt=0.02
  x = 0 + 0.5 * cos(0) * 0.02 = 0.01m
  y = 0 + 0.5 * sin(0) * 0.02 = 0m
  theta = 0

Step 2: v=0.5, omega=0.3, dt=0.02
  x = 0.01 + 0.5 * cos(0) * 0.02 = 0.02m
  y = 0 + 0.5 * sin(0) * 0.02 = 0m
  theta = 0 + 0.3 * 0.02 = 0.006 rad

Step 3: v=0.5, omega=0.3, dt=0.02
  x = 0.02 + 0.5 * cos(0.006) * 0.02 = 0.03m
  y = 0 + 0.5 * sin(0.006) * 0.02 = 0.00006m  (starting to curve)
  theta = 0.006 + 0.006 = 0.012 rad
```

The robot traces a curve! Over time, the odometry drifts because of small integration errors and wheel slip.

### Covariance:

The `pose_covariance_diagonal` values represent **uncertainty** in the odometry:

```
Covariance matrix (6x6, only diagonal shown):
  [0.001, 0.001, 0.001, 0.001, 0.001, 0.01]
    x      y      z     roll   pitch   yaw

The yaw (0.01) has more uncertainty than x/y (0.001) because
rotation errors accumulate faster than position errors.
```

This is used by sensor fusion algorithms (like EKF) to combine odometry with other sensors (IMU, GPS, lidar).

---

## 11.7 — The Data Flow

```
/cmd_vel topic (Twist msg)
  linear.x = 0.5 m/s
  angular.z = 0.3 rad/s
       |
       v
+----------------------------+
| DiffDriveController        |
|                             |
| Inverse kinematics:        |
| w_L = (v - omega*L/2) / r  |
| w_R = (v + omega*L/2) / r  |
+----------------------------+
       |
       v
Command interfaces:
  left_wheel/velocity = 4.325 rad/s
  right_wheel/velocity = 5.675 rad/s
       |
       v
+----------------------------+
| HardwareInterface write()  |
| sends to servos            |
+----------------------------+
       |
       v (wheels spin)
       |
+----------------------------+
| HardwareInterface read()   |
| reads from servos          |
+----------------------------+
       |
       v
State interfaces:
  left_wheel/position, velocity
  right_wheel/position, velocity
       |
       v
+----------------------------+
| DiffDriveController        |
|                             |
| Forward kinematics:        |
| v = r*(w_R+w_L)/2          |
| omega = r*(w_R-w_L)/L      |
|                             |
| Odometry integration:      |
| x += v*cos(theta)*dt       |
| y += v*sin(theta)*dt       |
+----------------------------+
       |
       v
/odom topic (Odometry msg)
  position: x, y, theta
  velocity: v, omega
```

---

## 11.8 — Odometry Error Propagation

### Why odometry drifts over time

Odometry uses dead reckoning — it integrates small increments. Errors accumulate:

```
Error sources:
  1. Wheel slip (wheels don't grip perfectly)
  2. Wheel radius error (manufacturing tolerance)
  3. Wheel separation error (L not exact)
  4. Integration discretization (Euler method)
  5. Encoder resolution (quantization)

Position error growth:
  σ_x(t) ≈ σ_x(0) + k * √t

  The error grows with √t (random walk), not linearly.
  After 10 seconds, error is √10 ≈ 3.2x the per-step error.
  After 100 seconds, error is √100 = 10x.

Heading error is worse:
  σ_θ(t) ≈ k_θ * √t
  And heading error causes position error to grow LINEARLY:
    σ_x(t) ≈ v * σ_θ * t

  A small heading error amplified over distance = large position drift.
```

This is why robots use **sensor fusion** (odometry + IMU + lidar/GPS) — odometry alone drifts too much for long-distance navigation.

---

## 11.9 — Quick Reference

| Concept | Key Point |
|---|---|
| DiffDrive | Two wheels, independent speeds → linear + angular motion |
| Forward kinematics | `v = r*(w_R+w_L)/2`, `ω = r*(w_R-w_L)/L` |
| Inverse kinematics | `w_L = (v - ω*L/2)/r`, `w_R = (v + ω*L/2)/r` |
| `wheel_separation` (L) | Distance between wheel centers (meters) |
| `wheel_radius` (r) | Wheel radius (meters) |
| `/cmd_vel` | Input topic: `Twist` msg with `linear.x` and `angular.z` |
| `/odom` | Output topic: `Odometry` msg with pose + twist + covariance |
| Odometry integration | `x += v*cos(θ)*dt`, `y += v*sin(θ)*dt`, `θ += ω*dt` |
| Covariance | Diagonal matrix — uncertainty in [x, y, z, roll, pitch, yaw] |
| Odometry drift | Position error ∝ √t, heading error makes it worse |
| JointStateBroadcaster | Reads state interfaces → publishes `/joint_states` |
| Velocity limits | `linear.x.max_velocity`, `angular.z.max_velocity` |

---

**Next:** [Part 12 — Hardware Driver: LX-225](12-lx225-driver.md)

