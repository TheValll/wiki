# Part 1 — ROS2 Architecture: Nodes, DDS & the Graph

## 1.1 — What is ROS2?

Imagine a **factory**. In this factory, there are many specialized workers:
- One worker reads temperature sensors
- One worker controls motors
- One worker does calculations
- One worker displays results on a screen

Each worker = **a Node** in ROS2.

These workers don't talk to each other face-to-face. They use an **internal messaging system** (like a bulletin board or pneumatic tubes). This messaging system = **DDS** (Data Distribution Service).

ROS2 is **not** an OS (despite the name "Robot Operating System"). It's a **communication framework**: a set of tools that let independent programs exchange data.

---

## 1.2 — A Node: What Is It Concretely?

A Node is a **process** (or part of a process) on your computer. When you start a node, the OS assigns it:

| OS Resource | Description |
|---|---|
| **PID** | A unique identity number (Process ID) |
| **RAM** | Memory to store its variables, callbacks, buffers |
| **Thread(s)** | One or more execution threads to process callbacks |
| **Network sockets** | Connections to communicate via DDS |

### Full C++ code — `minimal_node.cpp` (complete, compilable):

```cpp
#include "rclcpp/rclcpp.hpp"

class MinimalNode : public rclcpp::Node
{
    public:
        MinimalNode() : Node("minimal_node")
        {
            timer_ = this->create_wall_timer(
                std::chrono::seconds(1),
                std::bind(&MinimalNode::timer_callback, this));
        }

    private:
        void timer_callback()
        {
            RCLCPP_INFO(this->get_logger(), "Minimal node is running ...");
        }

        rclcpp::TimerBase::SharedPtr timer_;
};

int main(int argc, char **argv){
    rclcpp::init(argc, argv);                      // 1. Initialize the ROS2 context
    auto node = std::make_shared<MinimalNode>();    // 2. Create the node in memory
    rclcpp::spin(node);                            // 3. Infinite event-waiting loop
    rclcpp::shutdown();                            // 4. Cleanup
    return 0;
}
```

Build and run:
```bash
colcon build --packages-select cpp_pkg
source install/setup.bash
ros2 run cpp_pkg minimal_cpp_node
```

### What happens in memory, step by step?

**Step 1: `rclcpp::init(argc, argv)`**
```
Process RAM:
+-------------------------------------+
|  ROS2 Context (global object)       |
|  |-- DDS Participant created         |
|  |-- Default configuration           |
|  +-- Domain ID = 0 (default)        |
+-------------------------------------+
```
- Initializes the **global ROS2 context** in the process
- Creates a **DDS DomainParticipant**: this is the process's identity on the DDS network
- The **Domain ID** (default 0) is like a "radio channel": only nodes on the same domain ID can see each other

**Step 2: `std::make_shared<MinimalNode>()`**
```
Process RAM (HEAP):
+-------------------------------------+
|  MinimalNode (on the heap)          |
|  |-- name = "minimal_node"          |
|  |-- logger_ (for RCLCPP_INFO)      |
|  |-- timer_ (SharedPtr) ----------->|-- Timer (1 second)
|  |                                  |   |-- period = 1s
|  |                                  |   +-- callback = timer_callback()
|  +-- node_base_ (rclcpp internal)   |
|      |-- DDS Publisher(s)           |
|      |-- DDS Subscriber(s)          |
|      +-- Guard conditions           |
+-------------------------------------+

 shared_ptr reference count = 1
```

Key points:
- `std::make_shared` allocates the object on the **heap** (dynamic memory), not on the stack
- A **smart pointer** (`shared_ptr`) manages memory automatically: when the reference count drops to 0, the object is destroyed
- The **timer** is a separate object linked to the node, with a period (1s) and a pointer to the callback function

**Step 3: `rclcpp::spin(node)` — The main loop**

This is where the magic happens. `spin()` runs an **infinite loop**:

```
while (context_is_valid) {
    1. Wait for an event (timer expired, message received, etc.)
    2. Identify which callback to call
    3. Execute the callback
    4. Return to 1.
}
```

```
                    +----------------+
                    |    spin()      |
                    |    (loop)      |
                    +-------+--------+
                            |
              +-------------v--------------+
              |  wait_set.wait()           |
              |  (blocks the thread        |
              |   until an event occurs)   |
              +-------------+--------------+
                            | timer expired!
              +-------------v--------------+
              |  timer_callback()          |
              |  -> prints the message     |
              +-------------+--------------+
                            |
                     back to wait
```

- The thread **sleeps** when there's nothing to do (no CPU wasted)
- When the timer expires, the **wait_set** wakes up and executes `timer_callback()`
- This is **event-driven** programming, like a web server waiting for requests

**Step 4: `rclcpp::shutdown()`**
- Destroys the DDS context
- Releases all network resources
- Smart pointers free RAM automatically

---

## 1.3 — C++ vs Python Comparison

The Python code (`py_pkg/py_pkg/minimal_node.py`) does **exactly the same thing**:

| Concept | C++ | Python |
|---|---|---|
| Library | `rclcpp` | `rclpy` |
| Init | `rclcpp::init(argc, argv)` | `rclpy.init(args=args)` |
| Base class | `rclcpp::Node` | `rclpy.node.Node` |
| Create timer | `create_wall_timer(chrono::seconds(1), cb)` | `self.create_timer(1.0, cb)` |
| Spin | `rclcpp::spin(node)` | `rclpy.spin(node)` |
| Shutdown | `rclcpp::shutdown()` | `rclpy.shutdown()` |

**Under the hood difference:**
- In C++, `rclcpp` calls the C layer (`rcl`) then DDS directly. Very fast, no garbage collector.
- In Python, `rclpy` is a **wrapper**: Python calls C code under the hood via bindings. Slower (interpreter overhead ~10-100x), but simpler to write.
- Both can **communicate with each other** seamlessly because they speak the same DDS protocol.

---

## 1.4 — DDS: The Invisible Communication System

DDS (Data Distribution Service) is the **middleware** that transports messages between nodes. It's an industrial standard (OMG), not invented by ROS.

### How do nodes discover each other?

When you start a node, DDS sends **UDP multicast** packets on the local network:

```
Node A starts:
  -> Broadcasts: "Hi, I'm minimal_node, Domain 0"

Node B starts:
  -> Broadcasts: "Hi, I'm publisher_node, Domain 0"
  -> Receives A's message: "OK, I know minimal_node"

Node A:
  -> Receives B's message: "OK, I know publisher_node"
```

This is **auto-discovery**. No central server needed (unlike ROS1 which had a `rosmaster`).

### SPDP & SEDP — the two discovery protocols

Discovery happens in **two phases**:

**Phase 1 — SPDP** (Simple Participant Discovery Protocol):
- Each node broadcasts a "hello" packet via **UDP multicast** on a well-known address
- The multicast port is computed from the Domain ID:

```
port = 7400 + (domain_id * 250) + offset

Example with domain_id = 0:
  Discovery multicast port = 7400 + 0*250 + 0 = 7400
  Multicast address = 239.255.0.1

Example with domain_id = 42:
  Discovery multicast port = 7400 + 42*250 = 17900
```

This is why nodes on different Domain IDs can't see each other — they listen on different ports.

**Phase 2 — SEDP** (Simple Endpoint Discovery Protocol):
- Once two participants know each other (via SPDP), they exchange their **endpoint lists** via unicast
- Endpoints = publishers, subscribers, service servers, service clients
- This lets each node know "who publishes what" and "who subscribes to what"

```
Timeline:
  t=0.0s  Node A starts → SPDP broadcast "I exist, participant_A"
  t=0.1s  Node B starts → SPDP broadcast "I exist, participant_B"
  t=0.2s  A receives B's SPDP → stores B's address
  t=0.3s  SEDP unicast A→B: "I publish on /simple_topic (String)"
  t=0.3s  SEDP unicast B→A: "I subscribe to /simple_topic (String)"
  t=0.4s  DDS matches publisher ↔ subscriber → data flows
```

### Network memory layout:

```
+----------+     UDP Multicast (239.255.0.1)      +----------+
|  Node A  | <----------------------------------> |  Node B  |
|  PID 101 |     Port 7400 + Domain*250           |  PID 102 |
|          |                                       |          |
|  DDS     |     Data (after discovery):           |  DDS     |
| Particip.| <-- UDP Unicast or Shared Memory --> | Particip.|
+----------+                                       +----------+
```

- **Multicast** = a message sent to everyone on the network (for discovery)
- **Unicast** = a message sent to a specific recipient (for data afterwards)
- On the **same machine**, DDS can use **shared memory** instead of the network, which is much faster (no network copy, just a pointer to the same RAM area)

---

## 1.5 — The ROS2 Graph

The set of all nodes + their connections forms the **ROS2 graph**. You can visualize it:

```bash
ros2 node list                  # List all active nodes
ros2 node info /minimal_node    # Details of a node
```

With your repo, if you launch `minimal_cpp_node` and `publisher` at the same time:

```
ROS2 Graph:

  +---------------+          +----------------+
  | minimal_node  |          | publisher_node |
  |  (timer)      |          |  (timer +      |
  |               |          |   publisher)   |
  +---------------+          +----------------+
       |                          |
       +------- DDS Domain 0 ----+
         (know each other via discovery)
```

---

## 1.6 — The Build System: colcon, CMake, ament

### How does your C++ code become an executable?

```
Source (.cpp)  ->  CMake  ->  Compilation  ->  Binary executable
                    ^
              CMakeLists.txt
```

From your `CMakeLists.txt` (lines 26-27):
```cmake
add_executable(minimal_cpp_node src/minimal_node.cpp)
ament_target_dependencies(minimal_cpp_node rclcpp)
```

1. `add_executable`: tells CMake "compile `minimal_node.cpp` into an executable named `minimal_cpp_node`"
2. `ament_target_dependencies`: automatically adds rclcpp's headers and libraries (include paths, link flags)
3. `install(TARGETS ...)` (line 47): copies the executable to `install/cpp_pkg/lib/cpp_pkg/` so `ros2 run` can find it

### And in Python?

No compilation. The `setup.py` (line 26) declares **entry points**:
```python
"minimal_py_node = py_pkg.minimal_node:main"
```
This says: "when someone runs `ros2 run py_pkg minimal_py_node`, call the `main()` function from `py_pkg/minimal_node.py`"

### package.xml: the identity card

```xml
<depend>rclcpp</depend>                    <!-- I need rclcpp to build AND run -->
<build_type>ament_cmake</build_type>       <!-- I'm a CMake package -->
```

`colcon build` reads all `package.xml` files, determines the **build order** (dependencies first), then runs CMake/setuptools for each one.

---

## 1.7 — Full Visual Summary

```
+--------------- Your PC -----------------------------------------+
|                                                                  |
|  +--- Process 1 (PID 101) --------+                            |
|  |  rclcpp::init()                 |                            |
|  |  +------------------------+     |     DDS Discovery          |
|  |  |  "minimal_node"        |     |<----(UDP multicast)--------|
|  |  |  Timer 1s              |     |                            |
|  |  |  spin() -> event loop  |     |     DDS Data               |
|  |  +------------------------+     |<--(shared mem / UDP)-------|
|  |  rclcpp::shutdown()             |                            |
|  +---------------------------------+                            |
|                                                                  |
|  +--- Process 2 (PID 102) --------+                            |
|  |  rclpy.init()                   |                            |
|  |  +------------------------+     |     DDS Discovery          |
|  |  |  "minimal_node" (py)   |     |<----(UDP multicast)--------|
|  |  |  Timer 1s              |     |                            |
|  |  |  spin() -> event loop  |     |                            |
|  |  +------------------------+     |                            |
|  |  rclpy.shutdown()               |                            |
|  +---------------------------------+                            |
|                                                                  |
|  OS: manages RAM, threads, sockets for each process             |
+-----------------------------------------------------------------+
```

---

## 1.8 — Quick Reference

| Concept | Key Point |
|---|---|
| Node | A process (or part of one) with a name, timer(s), and DDS participant |
| `rclcpp::init()` | Creates the global ROS2 context + DDS DomainParticipant |
| `std::make_shared<Node>()` | Allocates the node on the **heap**, managed by smart pointer |
| `rclcpp::spin(node)` | Infinite event loop — sleeps until a callback is ready |
| `rclcpp::shutdown()` | Destroys DDS context, frees all resources |
| DDS Discovery | SPDP (multicast) finds participants, SEDP (unicast) matches endpoints |
| Domain ID | Like a radio channel — `port = 7400 + domain_id * 250` |
| Shared memory | Used when pub/sub are on the same machine (no network copy) |
| `colcon build` | Topological sort of packages, then CMake + make for each |
| `source install/setup.bash` | Sets `AMENT_PREFIX_PATH`, `LD_LIBRARY_PATH` so `ros2 run` works |

---

**Next:** [Part 2 — Topics & Pub/Sub](02-topics-pub-sub.md)

