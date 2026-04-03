# Part 4 — Custom Interfaces: Messages and Services

## 4.1 — Why Custom Interfaces?

ROS2 comes with built-in types like `std_msgs/String`, `example_interfaces/srv/AddTwoInts`. But real robots need custom data structures:
- A robot arm joint state with position + velocity + torque
- A sensor reading with timestamp + confidence + raw data
- A custom command with mode + target + constraints

Custom interfaces let you define your **own message and service types**.

---

## 4.2 — The .msg File

From `custom_interfaces/msg/MinimalInterface.msg`:

```
float64 a
bool b
string c
```

This is a **definition file** — a recipe that tells ROS2 code generators what fields the message has.

### Primitive types available:

| ROS2 Type | C++ Type | Python Type | Size in memory |
|---|---|---|---|
| `bool` | `bool` | `bool` | 1 byte |
| `int8` | `int8_t` | `int` | 1 byte |
| `int32` | `int32_t` | `int` | 4 bytes |
| `int64` | `int64_t` | `int` | 8 bytes |
| `float32` | `float` | `float` | 4 bytes |
| `float64` | `double` | `float` | 8 bytes |
| `string` | `std::string` | `str` | variable |

### Memory layout of a MinimalInterface message:

```
RAM (C++ object):
+--------+--------+--------+------------------------+
| a      | b      | pad    | c (std::string)        |
| 8 bytes| 1 byte | 7 bytes| 32 bytes (string obj)  |
| float64| bool   | align  | pointer to heap data   |
+--------+--------+--------+------------------------+

Total object size: ~48 bytes (varies by platform/compiler)
The string "Basic text" itself lives on the heap: 10 chars + null = 11 bytes
```

**Alignment**: CPUs read memory fastest when data is aligned to its natural boundary. A `float64` (8 bytes) must start at an address divisible by 8. The compiler adds **padding bytes** between `b` (1 byte) and the next 8-byte boundary.

---

## 4.3 — The .srv File

From `custom_interfaces/srv/MinimalService.srv`:

```
float64 a
float64 b
---
float64 c
```

This generates **two** C++ structs:
```cpp
// Auto-generated (simplified):
struct MinimalService_Request {
    double a;
    double b;
};

struct MinimalService_Response {
    double c;
};
```

---

## 4.4 — The Code Generation Pipeline

When you run `colcon build` on the `custom_interfaces` package, a lot happens:

```
Build pipeline:

MinimalInterface.msg          MinimalService.srv
        |                            |
        v                            v
   rosidl_generator_c          rosidl_generator_c
        |                            |
        v                            v
   C header (.h)               C header (.h)
   C source (.c)               C source (.c)
        |                            |
        v                            v
   rosidl_generator_cpp        rosidl_generator_cpp
        |                            |
        v                            v
   C++ header (.hpp)           C++ header (.hpp)
        |                            |
        v                            v
   rosidl_generator_py         rosidl_generator_py
        |                            |
        v                            v
   Python module (.py)         Python module (.py)
```

From `custom_interfaces/CMakeLists.txt`:
```cmake
find_package(rosidl_default_generators REQUIRED)

rosidl_generate_interfaces(${PROJECT_NAME}
  "msg/MinimalInterface.msg"
  "srv/MinimalService.srv"
)
```

`rosidl_generate_interfaces` triggers the whole pipeline. It generates:
1. **C code** — the base layer, used by both C++ and Python
2. **C++ headers** — what you `#include` in C++ nodes
3. **Python modules** — what you `import` in Python nodes
4. **Type support** — serialization/deserialization functions for DDS

### Where do generated files end up?

```
install/custom_interfaces/
  include/custom_interfaces/
    msg/
      minimal_interface.hpp          <-- C++ header you #include
      detail/
        minimal_interface__struct.hpp  <-- actual struct definition
        minimal_interface__traits.hpp  <-- type traits for DDS
    srv/
      minimal_service.hpp
  lib/python3.xx/site-packages/
    custom_interfaces/
      msg/
        _minimal_interface.py        <-- Python class you import
      srv/
        _minimal_service.py
```

---

## 4.5 — IDL: The Intermediate Language

Behind the scenes, `.msg` and `.srv` files are first converted to **IDL** (Interface Definition Language), the standard format from DDS:

```idl
// Auto-generated IDL from MinimalInterface.msg:
module custom_interfaces {
  module msg {
    struct MinimalInterface {
      double a;
      boolean b;
      string c;
    };
  };
};
```

This IDL is what DDS actually understands. The code generators read this IDL to produce C/C++/Python code.

---

## 4.6 — Using Custom Interfaces in Code

**C++ (`custom_interface_node.cpp`):**
```cpp
#include "custom_interfaces/msg/minimal_interface.hpp"  // auto-generated

auto msg = custom_interfaces::msg::MinimalInterface();
msg.a = 12.56;   // float64
msg.b = true;     // bool
msg.c = "Basic text";  // string
publisher_->publish(msg);
```

**Python (`custom_interface_node.py`):**
```python
from custom_interfaces.msg import MinimalInterface  # auto-generated

msg = MinimalInterface()
msg.a = 12.56
msg.b = True
msg.c = "Basic text"
self.publisher_.publish(msg)
```

The API is nearly identical. The generated code handles all the CDR serialization.

---

## 4.7 — CDR Serialization of Custom Messages

When `MinimalInterface(a=12.56, b=True, c="Basic text")` is published:

```
CDR binary on the wire:

Offset  Bytes                   Field
0x00    [71 3D 0A D7 A3 70 29 40]  a = 12.56 (IEEE 754 float64)
0x08    [01]                        b = true (1 byte)
0x09    [00 00 00]                  padding (align to 4-byte boundary)
0x0C    [0B 00 00 00]              string length = 11 (10 chars + null)
0x10    [42 61 73 69 63 20 74 65]  "Basic te"
0x18    [78 74 00]                 "xt\0"

Total: ~27 bytes
```

### IEEE 754 float64 deep-dive:

The number `12.56` is stored as a 64-bit floating point:

```
Sign (1 bit): 0 (positive)
Exponent (11 bits): 10000000010 (biased exponent = 1026, actual = 1026-1023 = 3)
Mantissa (52 bits): 1001 0010 0000 ... (fractional part)

Value = (-1)^0 x 1.mantissa x 2^3 = 1.57 x 8 = 12.56
```

This is how ALL computers store decimals — it's a hardware standard (IEEE 754), not a ROS2 thing.

---

## 4.8 — Package Dependencies

For another package to use your custom interfaces:

**In `package.xml`:**
```xml
<depend>custom_interfaces</depend>
```

**In `CMakeLists.txt`:**
```cmake
find_package(custom_interfaces REQUIRED)
ament_target_dependencies(my_node rclcpp custom_interfaces)
```

**In Python `package.xml`:**
```xml
<depend>custom_interfaces</depend>
```

`colcon build` uses these dependencies to determine build order: `custom_interfaces` is always built **before** packages that depend on it.

---

**Next:** [Part 5 — Parameters](05-parameters.md)

