# Part 7 — URDF & Visualization: Describing a Robot

## 7.1 — The Analogy

A URDF is like a **blueprint** of your robot. It tells the computer:
- What parts (links) the robot has
- How they connect together (joints)
- How big and heavy each part is (geometry, mass, inertia)
- What they look like (visual) and what they bump into (collision)

Without a URDF, ROS2 doesn't know what your robot looks like or how it moves.

---

## 7.2 — URDF Structure: Links and Joints

A URDF is a **tree** of links connected by joints:

```
                base_footprint (virtual, on the ground)
                      |
                 [base_joint] (fixed)
                      |
                  base_link (the chassis box)
                /     |     \          \
  [right_wheel] [left_wheel] [caster]  [second_link]
   (continuous)  (continuous)  (fixed)      |
       |              |          |     [second_third]
  right_wheel   left_wheel   caster        |
    _link         _link      _link     third_link
                                           |
                                    [camera_optical]
                                           |
                                    camera_link_optical
```

- **Link** = a rigid body (a physical part)
- **Joint** = the connection between two links (how one moves relative to the other)

---

## 7.3 — Joint Types

| Type | Motion | Example in your repo |
|---|---|---|
| `fixed` | No movement | `base_joint` (footprint to base) |
| `continuous` | Infinite rotation (like a wheel) | `base_right_wheel_joint` |
| `revolute` | Rotation with limits (like an elbow) | Not in this repo |
| `prismatic` | Linear sliding (like a piston) | Not in this repo |

From `mobile_base.xacro`:
```xml
<joint name="base_right_wheel_joint" type="continuous">
    <parent link="base_link" />
    <child link="right_wheel_link" />
    <origin xyz="${-base_length/4.0} ${-(base_width + wheel_length) / 2.0} 0" rpy="0 0 0" />
    <axis xyz="0 1 0" />   <!-- rotation around Y axis -->
</joint>
```

### Breakdown:
- `parent` / `child`: the wheel is attached to the base
- `origin xyz`: position of the joint relative to the parent (offset from base center)
- `axis xyz="0 1 0"`: the wheel rotates around the **Y axis**

---

## 7.4 — Xacro: Macros for URDF

Raw URDF is verbose and repetitive. **Xacro** (XML Macros) adds:

### Properties (variables):
```xml
<xacro:property name="wheel_radius" value="0.1" />
<!-- Use with ${wheel_radius} -->
```

### Macros (reusable templates):
```xml
<xacro:macro name="wheel_link" params="prefix">
    <link name="${prefix}_wheel_link">
        <visual>
            <geometry>
                <cylinder radius="${wheel_radius}" length="${wheel_length}" />
            </geometry>
            <origin xyz="0 0 0" rpy="${pi / 2.0} 0 0" />
        </visual>
    </link>
</xacro:macro>

<!-- Use it: generates two links from one template -->
<xacro:wheel_link prefix="right" />
<xacro:wheel_link prefix="left" />
```

### Include (file splitting):
From `my_robot.urdf.xacro`:
```xml
<xacro:include filename="common_properties.xacro" />
<xacro:include filename="mobile_base.xacro" />
<xacro:include filename="mobile_base.ros2_control.xacro" />
```

This splits the robot description into logical files:
- `common_properties.xacro` — shared materials (colors)
- `mobile_base.xacro` — physical structure
- `mobile_base.ros2_control.xacro` — hardware interface config

Xacro is processed at build/launch time: `xacro file.xacro` outputs pure URDF XML.

---

## 7.5 — Link Properties: Visual, Collision, Inertial

Each link can have three sub-elements:

```xml
<link name="base_link">
    <!-- 1. VISUAL: what it looks like in RViz/Gazebo -->
    <visual>
        <geometry>
            <box size="0.6 0.4 0.2" />   <!-- length, width, height -->
        </geometry>
        <origin xyz="0 0 0.1" rpy="0 0 0" />
        <material name="green" />
    </visual>

    <!-- 2. COLLISION: what the physics engine uses for contacts -->
    <collision>
        <geometry>
            <box size="0.6 0.4 0.2" />   <!-- often same as visual, sometimes simpler -->
        </geometry>
        <origin xyz="0 0 0.1" rpy="0 0 0" />
    </collision>

    <!-- 3. INERTIAL: mass and inertia (needed for physics simulation) -->
    <inertial>
        <mass value="5.0" />
        <inertia ixx="..." ixy="0" ixz="0" iyy="..." iyz="0" izz="..." />
    </inertial>
</link>
```

---

## 7.6 — Inertia Matrices (The Math)

The **inertia matrix** describes how a body resists rotation. It's a 3x3 symmetric matrix:

```
     [ Ixx  Ixy  Ixz ]
I =  [ Ixy  Iyy  Iyz ]
     [ Ixz  Iyz  Izz ]
```

For simple shapes, there are standard formulas. Your repo uses macros for these:

### Box (base_link): m=5kg, dimensions 0.6 x 0.4 x 0.2

```
Ixx = (m/12) * (y^2 + z^2) = (5/12) * (0.4^2 + 0.2^2) = (5/12) * 0.20 = 0.0833 kg.m^2
Iyy = (m/12) * (x^2 + z^2) = (5/12) * (0.6^2 + 0.2^2) = (5/12) * 0.40 = 0.1667 kg.m^2
Izz = (m/12) * (x^2 + y^2) = (5/12) * (0.6^2 + 0.4^2) = (5/12) * 0.52 = 0.2167 kg.m^2
```

### Cylinder (wheel): m=1kg, radius=0.1, height=0.05

```
Ixx = Iyy = (m/12) * (3*r^2 + h^2) = (1/12) * (3*0.01 + 0.0025) = 0.00271 kg.m^2
Izz = (m/2) * r^2 = 0.5 * 0.01 = 0.005 kg.m^2
```

### Sphere (caster wheel): m=0.5kg, radius=0.05

```
Ixx = Iyy = Izz = (2/5) * m * r^2 = 0.4 * 0.5 * 0.0025 = 0.0005 kg.m^2
```

**Why does this matter?**
- The physics simulator (Gazebo) uses inertia to calculate how the robot reacts to forces
- Wrong inertia → robot behaves unrealistically (flips over, jitters, slides)
- `Ixy = Iyz = Ixz = 0` for symmetric shapes aligned with their axes

### Physical intuition:
- **Ixx** = resistance to rotation around X axis. Larger if mass is far from X axis.
- Think of a figure skater: arms out = high inertia (spins slowly), arms in = low inertia (spins fast)

---

## 7.7 — Origin and RPY (Roll-Pitch-Yaw)

```xml
<origin xyz="0 0 0.1" rpy="0 0 0" />
```

| Parameter | Meaning |
|---|---|
| `xyz` | Translation: x=forward, y=left, z=up (in meters) |
| `rpy` | Rotation: Roll (X), Pitch (Y), Yaw (Z) (in radians) |

### The wheel rotation trick:

Cylinders in URDF are created **vertically** (along Z). But wheels roll along Y. So:
```xml
<origin xyz="0 0 0" rpy="${pi/2} 0 0" />
```
This rotates the cylinder 90 degrees around X, making it horizontal.

```
Before rpy:        After rpy="${pi/2} 0 0":
    |                    ___
    |               ---|   |---
    |                  |___|
  (vertical)         (horizontal, rolls along Y)
```

---

## 7.8 — TF2: The Transform Tree

`robot_state_publisher` reads the URDF and broadcasts **transforms** on the `/tf` topic. These transforms tell every node "where is each link relative to every other link."

```
TF Tree (matches the URDF joint tree):

  base_footprint
       |
    base_link         (xyz: 0, 0, 0.1)
    /    |     \
right  left   caster  (each at their xyz offset)
wheel  wheel  wheel
```

Any node can ask: "Where is `right_wheel_link` relative to `base_footprint`?" and TF2 chains the transforms:

```
T(base_footprint -> right_wheel) = T(base_footprint -> base_link) * T(base_link -> right_wheel)
```

This is **matrix multiplication** of 4x4 homogeneous transformation matrices:

```
T = [ R  t ]    R = 3x3 rotation matrix
    [ 0  1 ]    t = 3x1 translation vector

Full 4x4 matrix:
    [ r11  r12  r13  tx ]
T = [ r21  r22  r23  ty ]
    [ r31  r32  r33  tz ]
    [  0    0    0    1 ]
```

### RPY to rotation matrix

The rotation matrix R is built from Roll (X), Pitch (Y), Yaw (Z):

```
R = Rz(yaw) * Ry(pitch) * Rx(roll)

Rx(a) = [ 1    0       0    ]     Ry(b) = [ cos(b)  0  sin(b) ]
        [ 0  cos(a)  -sin(a)]             [   0     1    0    ]
        [ 0  sin(a)   cos(a)]             [-sin(b)  0  cos(b) ]

Rz(c) = [ cos(c)  -sin(c)  0 ]
        [ sin(c)   cos(c)  0 ]
        [   0        0     1 ]
```

### Example: wheel rotation `rpy="${pi/2} 0 0"`

```
roll = pi/2, pitch = 0, yaw = 0

Rx(pi/2) = [ 1   0    0  ]
           [ 0   0   -1  ]
           [ 0   1    0  ]

This maps: Y → Z, Z → -Y (the cylinder flips from vertical to horizontal)
```

### Chaining transforms — how TF2 works

To find the pose of `right_wheel_link` in the `base_footprint` frame:

```
T_footprint_wheel = T_footprint_base * T_base_wheel

Where:
  T_footprint_base = from URDF: base_joint (xyz="0 0 0.1", rpy="0 0 0")
  T_base_wheel     = from URDF: base_right_wheel_joint (xyz="-0.15 -0.225 0")

Multiplying 4x4 matrices:
  T_footprint_wheel = [ I  | 0, 0, 0.1 ] * [ I  | -0.15, -0.225, 0 ]
                      [ 0  |     1      ]   [ 0  |        1         ]

                    = [ I  | -0.15, -0.225, 0.1 ]
                      [ 0  |          1          ]

Result: the right wheel is at (-0.15, -0.225, 0.1) relative to base_footprint.
```

---

## 7.9 — Gazebo Integration

The URDF includes Gazebo-specific plugins:

```xml
<gazebo>
    <plugin filename="gz-sim-diff-drive-system" name="gz::sim::systems::DiffDrive">
        <left_joint>base_left_wheel_joint</left_joint>
        <right_joint>base_right_wheel_joint</right_joint>
        <wheel_separation>${base_width + wheel_length}</wheel_separation>
        <wheel_radius>${wheel_radius}</wheel_radius>
    </plugin>
</gazebo>
```

This tells Gazebo: "This robot has differential drive — spinning the wheels should move it."

---

## 7.10 — Quick Reference

| Concept | Key Point |
|---|---|
| URDF | XML description of a robot: links (bodies) + joints (connections) |
| Link | Rigid body with visual, collision, and inertial properties |
| Joint types | `fixed`, `continuous` (infinite rotation), `revolute` (limited), `prismatic` (linear) |
| Xacro | XML macros — `<xacro:property>`, `<xacro:macro>`, `<xacro:include>` |
| `origin xyz rpy` | Position (meters) + orientation (radians: roll/pitch/yaw) |
| Inertia (box) | `Ixx = m/12 * (y² + z²)` — similar for Iyy, Izz |
| Inertia (cylinder) | `Ixx = Iyy = m/12 * (3r² + h²)`, `Izz = m/2 * r²` |
| Inertia (sphere) | `Ixx = Iyy = Izz = 2/5 * m * r²` |
| TF2 | Transform tree — chains 4x4 matrices to find any frame relative to any other |
| `R = Rz * Ry * Rx` | RPY → rotation matrix (applied in Z-Y-X order) |
| robot_state_publisher | Reads URDF, publishes `/tf` transforms |
| Gazebo plugin | `<gazebo><plugin>` — adds physics behavior (diff drive, sensors, etc.) |

---

**Next:** [Part 8 — ros2_control Architecture](../ros2-control/08-ros2-control-architecture.md)

