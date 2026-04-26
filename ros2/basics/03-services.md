# Part 3 — Services: Request/Reply (Synchronous Communication)

## 3.1 — The Analogy

If topics are like a **radio station** (broadcast, no reply), services are like a **phone call**:
- You dial a number (send a request)
- Someone picks up and answers (processes and returns a response)
- You wait until you get the answer

This is **synchronous communication** — the client sends a request and expects a response back.

---

## 3.2 — Topics vs Services

| Feature | Topics | Services |
|---|---|---|
| Pattern | Publish/Subscribe | Request/Response |
| Direction | One-way (broadcast) | Two-way (ask and answer) |
| Timing | Asynchronous (fire & forget) | Synchronous (wait for reply) |
| Multiplicity | Many-to-many | Many clients, ONE server |
| Use case | Continuous data streams (sensors) | One-time computations, queries |

**When to use what:**
- Camera frames at 30Hz → **Topic**
- "Calculate the sum of 3+7" → **Service**
- Robot odometry at 50Hz → **Topic**
- "Set the robot's mode to IDLE" → **Service**

---

## 3.3 — The Service Interface

From `custom_interfaces/srv/MinimalService.srv`:
```
float64 a
float64 b
---
float64 c
```

The `---` separator divides:
- **Request** (what the client sends): `a` and `b`
- **Response** (what the server returns): `c`

Your repo uses the built-in `example_interfaces/srv/AddTwoInts`:
```
int64 a
int64 b
---
int64 sum
```

---

## 3.4 — The Server (C++ deep-dive)

### Full C++ code — `server_node.cpp` (complete, compilable):

```cpp
#include "rclcpp/rclcpp.hpp"
#include "example_interfaces/srv/add_two_ints.hpp"

using namespace std::placeholders;

class ServerNode : public rclcpp::Node
{
    public:
        ServerNode() : Node("server_node")
        {
            server_ = this->create_service<example_interfaces::srv::AddTwoInts>(
                "add_two_ints",
                std::bind(&ServerNode::callback_server, this, _1, _2));
            RCLCPP_INFO(this->get_logger(), "Server node is running ...");
        }

    private:
        void callback_server(
            const example_interfaces::srv::AddTwoInts::Request::SharedPtr req,
            const example_interfaces::srv::AddTwoInts::Response::SharedPtr res)
        {
            res->sum = req->a + req->b;
        }

        rclcpp::Service<example_interfaces::srv::AddTwoInts>::SharedPtr server_;
};

int main(int argc, char **argv){
    rclcpp::init(argc, argv);
    auto node = std::make_shared<ServerNode>();
    rclcpp::spin(node);
    rclcpp::shutdown();
    return 0;
}
```

The callback receives **two** arguments:
- `_1` = the **request** (read-only, filled by the client)
- `_2` = the **response** (empty, you fill it in)

### What happens in memory when a request arrives:

```
Server Node (HEAP):
+------------------------------------------------+
|  ServerNode                                     |
|  |-- server_ (SharedPtr) ----> Service object   |
|      |-- service_name = "add_two_ints"          |
|      |-- DDS DataReader (for requests)          |
|      +-- DDS DataWriter (for responses)         |
+------------------------------------------------+

When request arrives:
1. DDS DataReader receives CDR bytes
2. Deserialize --> Request object {a: 3, b: 7}
3. Allocate Response object {sum: 0}
4. Call callback_server(req, res)
5. After callback: res->sum = 10
6. Serialize Response to CDR bytes
7. DDS DataWriter sends response back
```

Notice: a service uses **two DDS entities** under the hood — a reader for requests and a writer for responses. It's actually two topics hidden behind a nice API:
```
Hidden topic: "add_two_ints/_request"   (client --> server)
Hidden topic: "add_two_ints/_response"  (server --> client)
```

---

## 3.5 — The Client (C++ deep-dive)

### Full C++ code — `client_node.cpp` (complete, compilable):

```cpp
#include "rclcpp/rclcpp.hpp"
#include "example_interfaces/srv/add_two_ints.hpp"

using namespace std::chrono_literals;
using namespace std::placeholders;

class ClientNode : public rclcpp::Node
{
    public:
        ClientNode() : Node("client_node")
        {
            client_ = this->create_client<example_interfaces::srv::AddTwoInts>(
                "add_two_ints");
            RCLCPP_INFO(this->get_logger(), "Client node is running ...");
        }

        void call_service(int a, int b)
        {
            while (!client_->wait_for_service(1s)){
                RCLCPP_WARN(this->get_logger(), "Waiting for the server ...");
            }

            auto req = std::make_shared<example_interfaces::srv::AddTwoInts::Request>();
            req->a = a;
            req->b = b;

            client_->async_send_request(req,
                std::bind(&ClientNode::callback_response, this, _1));
        }

    private:
        void callback_response(
            rclcpp::Client<example_interfaces::srv::AddTwoInts>::SharedFuture future)
        {
            auto res = future.get();
            RCLCPP_INFO(get_logger(), "Sum: %d", (int)res->sum);
        }

        rclcpp::Client<example_interfaces::srv::AddTwoInts>::SharedPtr client_;
};

int main(int argc, char **argv){
    rclcpp::init(argc, argv);
    auto node = std::make_shared<ClientNode>();
    node->call_service(3, 7);
    node->call_service(10, 3);
    node->call_service(5, 4);
    rclcpp::spin(node);
    rclcpp::shutdown();
    return 0;
}
```

### Why `async_send_request` instead of a blocking call?

If you used a synchronous call, the thread would **block** inside `spin()`, creating a **deadlock**:

```
DEADLOCK scenario (if synchronous):

spin() thread:
  |-- wait for events
  |-- call_service() called
  |     |-- send request
  |     |-- BLOCK waiting for response...  <-- STUCK HERE
  |     |
  |     X-- Can't process response callback because
  |         the spin thread is blocked!

  DEADLOCK! The thread is waiting for something
  that it itself needs to process.
```

`async_send_request` avoids this: it sends the request and returns immediately. The response will be handled later by `spin()` when it arrives.

### The Future pattern:

```cpp
void callback_response(Client<AddTwoInts>::SharedFuture future)
{
    auto res = future.get();  // Get the response (already arrived at this point)
    RCLCPP_INFO(get_logger(), "Sum: %d", (int)res->sum);
}
```

A **Future** is a container for a value that will be available "in the future". When the callback fires, the future is already resolved.

```
Timeline:
  t=0.000s  async_send_request(req)  --> sends request, returns Future
  t=0.001s  spin() continues processing other events
  t=0.005s  response arrives from server
  t=0.005s  spin() calls callback_response(future)
  t=0.005s  future.get() --> returns Response{sum: 10}
```

---

## 3.6 — Python Comparison

**Python Server (`server_node.py`):**
```python
self.server_ = self.create_service(AddTwoInts, "add_two_ints", self.callback_server)

def callback_server(self, req, res):
    res.sum = req.a + req.b
    return res   # <-- Python must explicitly return the response
```

**Python Client (`client_node.py`):**
```python
future = self.client_.call_async(req)
future.add_done_callback(partial(self.callback_response, req))
```

Key difference: Python uses `functools.partial` instead of `std::bind`:
```python
# partial(self.callback_response, req) creates:
# A function that calls self.callback_response(req, future)
# where `req` is "baked in" and `future` is passed when the callback fires
```

This lets the Python client log `f"{req.a} + {req.b} = {res.sum}"` — it remembers what was asked.

---

## 3.7 — The `main()` Sequence

```cpp
int main(int argc, char **argv){
    rclcpp::init(argc, argv);
    auto node = std::make_shared<ClientNode>();
    node->call_service(3, 7);    // Queue request 1
    node->call_service(10, 3);   // Queue request 2
    node->call_service(5, 4);    // Queue request 3
    rclcpp::spin(node);          // Now process everything
    rclcpp::shutdown();
    return 0;
}
```

The 3 calls happen **before** `spin()`. They just queue the requests. The actual sending and response handling happens inside `spin()`.

```
Timeline:
  call_service(3,7)   --> "I want to send this when spin starts"
  call_service(10,3)  --> "Queue this too"
  call_service(5,4)   --> "And this"
  spin()              --> NOW: send all 3, wait for responses, call callbacks
```

---

## 3.8 — Full Flow Diagram

```
  ClientNode                    Network (DDS)                ServerNode
  ==========                    =============                ==========
       |                                                          |
  call_service(3,7)                                               |
       |                                                          |
  async_send_request -----> [Request{a:3, b:7}] ------>  callback_server
       |                                                     res->sum = 10
       |                    [Response{sum:10}]  <------       return
       |                          |                               |
  callback_response <-------------+                               |
  future.get() = {sum:10}                                         |
  print "Sum: 10"                                                 |
```

---

## 3.9 — Useful CLI Commands

```bash
ros2 service list                              # List all active services
ros2 service type /add_two_ints                # Show the service type
ros2 service call /add_two_ints example_interfaces/srv/AddTwoInts "{a: 5, b: 3}"
```

---

## 3.10 — The Math Behind Service Latency

### Round-trip time decomposition

A service call's total latency can be broken down:

```
T_total = T_serialize_req + T_transport_req + T_deserialize_req
        + T_compute
        + T_serialize_res + T_transport_res + T_deserialize_res

Simplified:
  T_total = 2 * T_serialize + 2 * T_transport + T_compute

Typical values (same machine, shared memory):
  T_serialize   ≈ 1-10 μs    (depends on message size)
  T_transport   ≈ 5-50 μs    (shared mem) or 0.1-1 ms (network)
  T_compute     ≈ application-dependent

Example — AddTwoInts on the same machine:
  T_total ≈ 2*5μs + 2*20μs + 1μs ≈ 51 μs ≈ 0.05 ms
```

### Why async matters — throughput vs latency

With **synchronous** calls (blocking), your maximum throughput is:

```
throughput_sync = 1 / T_total

Example: T_total = 1ms → max 1000 calls/second
```

With **async** calls (non-blocking), you can have multiple requests in-flight:

```
throughput_async = N_concurrent / T_total

Example: 10 concurrent requests, T_total = 1ms
  → up to 10,000 calls/second
```

This is why `async_send_request` exists — it allows **pipelining** of requests while the spin loop continues processing other events.

### Timeout math

`wait_for_service(1s)` blocks for up to 1 second. If the server isn't found:

```
Total wait before giving up = N_retries * timeout

If you retry forever (while loop), the client blocks indefinitely.
In production, add a max retry count:

  max_wait = max_retries * timeout_per_retry
  Example: 5 retries * 1s = 5s max wait before error
```

---

## 3.11 — Quick Reference

| Concept | Key Point |
|---|---|
| Service | Request/response pattern — like a function call over the network |
| Server | `create_service<Type>("name", callback)` — callback gets `(req, res)` |
| Client | `create_client<Type>("name")` — calls `async_send_request(req, cb)` |
| `_1, _2` placeholders | `std::bind` placeholders for the request and response |
| Hidden topics | Service = 2 DDS topics: `name/_request` + `name/_response` |
| Async pattern | **Required** to avoid deadlock — never block inside `spin()` |
| Future | Container for a value that will arrive later — `.get()` retrieves it |
| `wait_for_service(1s)` | Blocks until the server is discovered on DDS |
| Round-trip time | `T ≈ 2*T_serialize + 2*T_transport + T_compute` |
| Throughput | Sync: `1/T_total` — Async: `N_concurrent/T_total` |

---

**Next:** [Part 4 — Custom Interfaces](04-custom-interfaces.md)

