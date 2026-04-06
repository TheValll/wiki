# Part 2 — Topics & Pub/Sub: Asynchronous Communication

## 2.1 — The Analogy

Imagine a **radio station**. The DJ (publisher) talks into the microphone and broadcasts on a specific frequency (topic). Anyone with a radio tuned to that frequency (subscriber) hears the message. The DJ doesn't know or care how many people are listening — could be 0, 1, or 1000.

This is the **publish-subscribe** pattern: the sender and receiver are **decoupled**.

---

## 2.2 — Topics: Named Message Channels

A topic is simply a **named channel** through which messages flow. In your repo:

```
Topic name: "simple_topic"
Message type: example_interfaces/msg/String
Direction: publisher_node --> simple_topic --> subscriber_node
```

Think of it like a pipe:
```
+----------------+     "simple_topic"      +----------------+
| publisher_node |  ===================>   | subscriber_node|
| (sends every   |     String msg          | (receives &    |
|  500ms)        |                         |  prints)       |
+----------------+                         +----------------+
```

Key properties:
- **Many-to-many**: multiple publishers can write to the same topic, multiple subscribers can read from it
- **Asynchronous**: the publisher doesn't wait for the subscriber — it just sends and moves on
- **Typed**: each topic has a fixed message type (you can't send an Integer on a String topic)

---

## 2.3 — The Publisher (C++ deep-dive)

### Full C++ code — `publisher_node.cpp` (complete, compilable):

```cpp
#include "rclcpp/rclcpp.hpp"
#include "example_interfaces/msg/string.hpp"

using namespace std::chrono_literals;

class PublisherNode : public rclcpp::Node
{
    public:
        PublisherNode() : Node("publisher")
        {
            publisher_ = this->create_publisher<example_interfaces::msg::String>(
                "simple_topic", 10);
            timer_ = this->create_wall_timer(
                500ms, std::bind(&PublisherNode::publish_example, this));
            RCLCPP_INFO(this->get_logger(), "Publisher has been started ...");
        }

    private:
        void publish_example()
        {
            auto msg = example_interfaces::msg::String();
            msg.data = "Simple publisher";
            publisher_->publish(msg);
        }

        rclcpp::Publisher<example_interfaces::msg::String>::SharedPtr publisher_;
        rclcpp::TimerBase::SharedPtr timer_;
};

int main(int argc, char **argv){
    rclcpp::init(argc, argv);
    auto node = std::make_shared<PublisherNode>();
    rclcpp::spin(node);
    rclcpp::shutdown();
    return 0;
}
```

### Breaking down `create_publisher`:

### Breaking down `create_publisher<String>("simple_topic", 10)`:

| Argument | Meaning |
|---|---|
| `<example_interfaces::msg::String>` | The **message type** — a template parameter |
| `"simple_topic"` | The **topic name** — a string identifier |
| `10` | The **queue depth** (QoS) — how many messages to buffer |

### What happens in memory when you create a publisher:

```
HEAP:
+---------------------------------------------+
|  PublisherNode                               |
|  |                                           |
|  |-- publisher_ (SharedPtr) --------------->|
|  |                                           |
|  |   Publisher<String> object                |
|  |   |-- topic_name = "simple_topic"        |
|  |   |-- QoS queue depth = 10               |
|  |   |-- DDS DataWriter (native handle)     |
|  |   +-- type_support_ (serializer ptr)     |
|  |                                           |
|  |-- timer_ (SharedPtr) ------------------>|
|  |   Timer (500ms, callback)                 |
|  +-------------------------------------------+
```

The **DDS DataWriter** is the real network-level object. When you call `publish(msg)`:

```
publish(msg) call chain:

  1. msg (C++ object)
     |
  2. Serialize to CDR binary format
     (CDR = Common Data Representation)
     |
  3. DDS DataWriter sends bytes
     |
  4. Transport layer:
     - Same process? -> direct pointer
     - Same machine? -> shared memory
     - Different machine? -> UDP
```

### The queue depth (10) explained:

```
Publisher sends messages faster than subscriber can process them:

Queue (size 10):
+---+---+---+---+---+---+---+---+---+---+
| 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |10 |
+---+---+---+---+---+---+---+---+---+---+
                                        ^
                                   If full, oldest
                                   message is DROPPED

This is a ring buffer in memory — fixed size, no dynamic allocation.
```

If the subscriber is too slow and the queue fills up, the **oldest messages are dropped**. This is the default "keep last" QoS policy.

---

## 2.4 — The Subscriber (C++ deep-dive)

### Full C++ code — `subscriber_node.cpp` (complete, compilable):

```cpp
#include "rclcpp/rclcpp.hpp"
#include "example_interfaces/msg/string.hpp"

using namespace std::placeholders;

class SubscriberNode : public rclcpp::Node
{
    public:
        SubscriberNode() : Node("subscriber")
        {
            subscriber_ = this->create_subscription<example_interfaces::msg::String>(
                "simple_topic", 10,
                std::bind(&SubscriberNode::callback_topic, this, _1));
            RCLCPP_INFO(this->get_logger(), "Subscriber has been started ...");
        }

    private:
        void callback_topic(const example_interfaces::msg::String::SharedPtr msg)
        {
            RCLCPP_INFO(this->get_logger(), "%s", msg->data.c_str());
        }

        rclcpp::Subscription<example_interfaces::msg::String>::SharedPtr subscriber_;
};

int main(int argc, char **argv){
    rclcpp::init(argc, argv);
    auto node = std::make_shared<SubscriberNode>();
    rclcpp::spin(node);
    rclcpp::shutdown();
    return 0;
}
```

### What happens when a message arrives:

```
Step-by-step in memory:

1. DDS DataReader receives bytes from network/shared-mem
   [55 00 00 00 10 00 00 00 53 69 6D 70 6C 65 ...]
    ^-- CDR-encoded binary data

2. Deserialize: CDR bytes --> C++ String object
   msg->data = "Simple publisher"

3. Message placed in subscriber's queue (ring buffer, depth 10)

4. spin() loop detects "new message available" in the wait_set

5. spin() calls callback_topic(msg)
   --> prints "Simple publisher"

6. Message SharedPtr ref count drops to 0 --> memory freed
```

### The `_1` placeholder:

`std::bind(&SubscriberNode::callback_topic, this, _1)` means:
- `this` = the object to call the method on
- `_1` = "the first argument will be filled in later" (the message)

It creates a **function object** (closure) stored on the heap.

---

## 2.5 — QoS (Quality of Service)

The `10` in `create_publisher(..., 10)` is shorthand for a QoS profile. Full QoS settings include:

| QoS Policy | Options | Default | Effect |
|---|---|---|---|
| **History** | Keep last / Keep all | Keep last | How many messages to store |
| **Depth** | 1, 10, 100... | 10 | Queue size (if keep last) |
| **Reliability** | Reliable / Best effort | Reliable | Guaranteed delivery vs speed |
| **Durability** | Volatile / Transient local | Volatile | Late-joiners get old messages? |

**Reliable** = TCP-like, messages are re-sent if lost (slower but guaranteed)
**Best effort** = UDP-like, fire-and-forget (faster but may lose messages)

For sensor data (camera, lidar), **best effort** is typical — you want the latest frame, not a delayed old one.
For commands (move robot), **reliable** is critical — you don't want to miss a "stop" command.

---

## 2.6 — Serialization: How Messages Travel

When a message is published, it must be converted to bytes. ROS2 uses **CDR** (Common Data Representation) from the CORBA standard.

For a `String` message with `data = "Simple publisher"`:

```
Memory layout of the CDR-encoded message:

Offset  Bytes           Meaning
0x00    [00 01 00 00]   CDR header (little-endian, version)
0x04    [11 00 00 00]   String length = 17 (16 chars + null)
0x08    [53 69 6D 70]   "Simp"
0x0C    [6C 65 20 70]   "le p"
0x10    [75 62 6C 69]   "ubli"
0x14    [73 68 65 72]   "sher"
0x18    [00 __ __ __]   null terminator + padding

Total: ~28 bytes on the wire for this message
```

This binary format is **language-independent**: a C++ publisher and a Python subscriber both understand it, because the serialization/deserialization code is auto-generated from the `.msg` file.

---

## 2.7 — C++ vs Python Comparison

**C++ Publisher (`publisher_node.cpp`):**
```cpp
publisher_ = this->create_publisher<example_interfaces::msg::String>("simple_topic", 10);
auto msg = example_interfaces::msg::String();
msg.data = "Simple publisher";
publisher_->publish(msg);
```

**Python Publisher (`publisher_node.py`):**
```python
self.publisher_ = self.create_publisher(String, "simple_topic", 10)
msg = String()
msg.data = "Simple publisher"
self.publisher_.publish(msg)
```

| Aspect | C++ | Python |
|---|---|---|
| Type specification | Template `<String>` | First argument `String` |
| Message creation | `auto msg = String()` | `msg = String()` |
| Performance | ~1-10 microseconds per publish | ~100-1000 microseconds |
| Use case | Real-time, high-frequency data | Prototyping, non-critical nodes |

Both produce **identical bytes on the wire** — a Python subscriber can read from a C++ publisher and vice versa.

---

## 2.8 — Full Data Flow Diagram

```
+------------------+                              +-------------------+
|  PublisherNode    |                              |  SubscriberNode   |
|                   |                              |                   |
|  timer (500ms)    |                              |  spin() loop      |
|       |           |                              |       |           |
|  timer_callback() |                              |  wait_set.wait()  |
|       |           |                              |       |           |
|  msg.data = "..." |                              |       |           |
|       |           |                              |       |           |
|  publish(msg)     |                              |       |           |
|       |           |                              |       |           |
|  serialize(CDR)   |                              |       |           |
|       |           |                              |       |           |
+-------+-----------+                              +-------+-----------+
        |                                                  ^
        |              DDS Transport Layer                  |
        |  (shared memory on same machine, UDP otherwise)  |
        +--------------------------------------------------+
                            |
                     deserialize(CDR)
                            |
                     callback_topic(msg)
                            |
                     RCLCPP_INFO(msg->data)
```

---

## 2.9 — Useful CLI Commands

```bash
ros2 topic list                          # List all active topics
ros2 topic info /simple_topic            # Show publishers/subscribers count
ros2 topic echo /simple_topic            # Print messages in real-time
ros2 topic hz /simple_topic              # Measure publish rate
ros2 topic pub /simple_topic example_interfaces/msg/String "data: hello"  # Publish from CLI
```

---

## 2.10 — The Math Behind Pub/Sub

### Ring buffer — how the queue works

The queue (depth `N`) is implemented as a **circular buffer** (ring buffer). It uses two pointers:

```
Memory layout (depth = 4):

  write_idx
      |
  +---v---+-------+-------+-------+
  | msg_3 | msg_0 | msg_1 | msg_2 |
  +-------+-------+---^---+-------+
                       |
                   read_idx

write_idx = (write_idx + 1) % N
read_idx  = (read_idx  + 1) % N
```

- **No dynamic allocation** — the buffer is pre-allocated with fixed size `N`
- When `write_idx` catches up with `read_idx`, the oldest message is **overwritten** (dropped)
- This is O(1) for both read and write — no shifting, no copying the array

### When are messages lost?

Messages are dropped when the publisher is faster than the subscriber:

```
Let:
  f_pub = publisher frequency (Hz)
  f_sub = subscriber processing frequency (Hz)
  N     = queue depth

If f_pub > f_sub, messages accumulate in the queue.

Time to fill the queue:
  t_fill = N / (f_pub - f_sub)

After t_fill, the oldest messages start being dropped.

Example:
  f_pub = 100 Hz, f_sub = 80 Hz, N = 10
  t_fill = 10 / (100 - 80) = 0.5 seconds

  After 0.5s, you lose 20 messages/second.

Drop rate = f_pub - f_sub = 20 msg/s (when queue is full)
```

### Nyquist and sensor topics

When subscribing to sensor data (camera, lidar, IMU), the **Nyquist-Shannon theorem** applies:

```
To reconstruct a signal of frequency f, you must sample at:

  f_sample >= 2 * f_signal

In ROS2 terms:
  f_subscriber >= 2 * f_max_change

Example: a motor vibrates at up to 50 Hz.
  Your subscriber must process at >= 100 Hz to capture all dynamics.

If f_sub < 2 * f_signal → aliasing (you see phantom low-frequency patterns
                                     that don't exist in reality)
```

In practice, aim for `f_sub >= 5-10x f_signal` for comfortable margin. Set your queue depth based on how much latency you can tolerate:

```
Latency introduced by the queue:
  t_latency_max = N / f_pub

Example: N = 10, f_pub = 100 Hz
  t_latency_max = 10 / 100 = 0.1 seconds = 100ms

For real-time control, use N = 1 (latest message only, no buffering).
```

---

## 2.11 — Quick Reference

| Concept | Key Point |
|---|---|
| Topic | Named channel with a fixed message type, many-to-many |
| Publisher | `create_publisher<Type>("topic", queue_depth)` |
| Subscriber | `create_subscription<Type>("topic", depth, callback)` |
| Queue depth | Ring buffer size — oldest dropped when full |
| `std::bind(fn, this, _1)` | Creates a callable that binds `this` and leaves 1 arg open |
| CDR serialization | Language-independent binary format — C++ and Python interop |
| QoS Reliable | Re-sends lost messages (like TCP) — for commands |
| QoS Best Effort | Fire-and-forget (like UDP) — for sensor streams |
| Transient Local | Late-joining subscribers get the last published message |
| Message loss | Occurs when `f_pub > f_sub` and queue fills in `N/(f_pub - f_sub)` seconds |
| Nyquist rule | Subscribe at `>= 2x` the signal frequency to avoid aliasing |
| Queue latency | `t_max = N / f_pub` — use depth 1 for real-time control |

---

**Next:** [Part 3 — Services](03-services.md)

