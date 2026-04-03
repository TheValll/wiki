# Part 5 — Parameters: Dynamic Node Configuration

## 5.1 — The Analogy

Imagine a machine with **dials and switches** on the front panel. You can turn a dial to change the speed, flip a switch to change the mode — without stopping the machine or rewriting its internal program.

Parameters are those dials and switches for your ROS2 nodes.

---

## 5.2 — What is a Parameter?

A parameter is a **named configuration value** stored inside a node. Unlike topics (data streams) or services (one-time calls), parameters are:

- **Persistent** within the node's lifetime
- **Typed** (string, int, float, bool, arrays)
- **Readable and writable** at runtime via CLI or other nodes
- **Declared** by the node (it knows what parameters it expects)

---

## 5.3 — The Code (C++ deep-dive)

From `cpp_pkg/src/parameters_node.cpp`:

```cpp
ParametersNode() : Node("parameters_node")
{
    this->declare_parameter("message", "Simple publisher");  // Declare with default
    publisher_ = this->create_publisher<String>("simple_topic", 10);
    timer_ = this->create_wall_timer(500ms, std::bind(&ParametersNode::publish_example, this));
}

void publish_example()
{
    auto msg = example_interfaces::msg::String();
    msg.data = this->get_parameter("message").as_string();  // Read at runtime
    publisher_->publish(msg);
}
```

### Step-by-step in memory:

**`declare_parameter("message", "Simple publisher")`:**
```
Node's internal parameter map (hash map on the HEAP):
+--------------------------------------------------+
|  parameters_ (std::unordered_map)                |
|  |                                                |
|  |  "message" --> ParameterValue {               |
|  |                  type: STRING                  |
|  |                  value: "Simple publisher"     |
|  |                }                               |
+--------------------------------------------------+
```

- The parameter is stored in a **hash map** inside the node
- `declare_parameter` does two things: (1) registers the name, (2) sets the default value
- If you try to `get_parameter` without declaring first, it throws an exception

**`get_parameter("message").as_string()`:**
1. Look up `"message"` in the hash map → O(1) average time
2. Return the `ParameterValue` object
3. `.as_string()` casts it to `std::string`

This happens **every 500ms** (each time the timer fires). So if someone changes the parameter at runtime, the next publish will use the new value.

---

## 5.4 — YAML Configuration Files

From `template_bringup/config/minimal_params.yaml`:

```yaml
parameters_node:           # <-- node name
  ros__parameters:         # <-- required key (double underscore!)
    message: "Hello World" # <-- parameter name: value
```

### Structure explained:

```
parameters_node:          # Must match the node name exactly
  ros__parameters:        # Magic key that ROS2 recognizes
    message: "Hello World"
    # You could add more:
    # rate: 10.0
    # debug: true
    # list_param: [1, 2, 3]
```

**Why `ros__parameters` with double underscore?**
It's a namespace convention. The double underscore `__` is a reserved separator in ROS2 to avoid collisions with user parameter names.

### YAML type mapping:

| YAML | ROS2 Type | Example |
|---|---|---|
| `"Hello World"` | string | `message: "Hello World"` |
| `42` | integer | `count: 42` |
| `3.14` | double | `rate: 3.14` |
| `true` / `false` | bool | `debug: true` |
| `[1, 2, 3]` | integer array | `ids: [1, 2, 3]` |

---

## 5.5 — Loading Parameters via Launch File

From `template_bringup/launch/simple_app.launch.xml`:

```xml
<launch>
  <node pkg="cpp_pkg" exec="parameters">
    <param from="$(find-pkg-share template_bringup)/config/minimal_params.yaml" />
  </node>
  <node pkg="cpp_pkg" exec="subscriber"/>
</launch>
```

### What happens at launch:

```
1. Launch system reads the XML
2. For "parameters" node:
   a. Finds the YAML file at the computed path
   b. Parses the YAML
   c. Passes parameters to the node via command-line arguments:
      --ros-args --params-file /path/to/minimal_params.yaml
3. Node starts, calls declare_parameter("message", "Simple publisher")
4. ROS2 sees that a YAML override exists: "message" = "Hello World"
5. The parameter value becomes "Hello World" (overrides the default)
```

So the priority is: **YAML file > default value in code**.

You could also set a parameter directly in the launch file:
```xml
<param name="message" value="Hello World" />
```

---

## 5.6 — Runtime Parameter Changes

Once a node is running, you can change parameters from the command line:

```bash
# List all parameters of a node
ros2 param list /parameters_node

# Get current value
ros2 param get /parameters_node message

# Change at runtime (takes effect on next get_parameter call)
ros2 param set /parameters_node message "New message!"
```

### Under the hood:

```
CLI: ros2 param set /parameters_node message "New message!"
  |
  v
Calls the node's built-in parameter service:
  /parameters_node/set_parameters  (hidden service, auto-created)
  |
  v
Updates the hash map:
  parameters_["message"] = ParameterValue("New message!")
  |
  v
Next timer callback:
  get_parameter("message") --> "New message!"
  publish("New message!")
```

Every node automatically gets parameter-related services:
- `/parameters_node/set_parameters`
- `/parameters_node/get_parameters`
- `/parameters_node/list_parameters`
- `/parameters_node/describe_parameters`

---

## 5.7 — Python Comparison

**Python (`parameters_node.py`):**
```python
self.declare_parameter("message", "Simple publisher")
msg.data = self.get_parameter("message").value  # .value instead of .as_string()
```

The only difference is `.value` (Python) vs `.as_string()` (C++). Python uses duck typing, so `.value` returns whatever type was stored.

---

**Next:** [Part 6 — Launch Files](06-launch-files.md)

