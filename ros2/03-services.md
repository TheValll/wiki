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

From `cpp_pkg/src/server_node.cpp`:

```cpp
server_ = this->create_service<example_interfaces::srv::AddTwoInts>(
    "add_two_ints",
    std::bind(&ServerNode::callback_server, this, _1, _2));
```

The callback receives **two** arguments:
```cpp
void callback_server(
    const AddTwoInts::Request::SharedPtr req,    // _1 = the request
    const AddTwoInts::Response::SharedPtr res)    // _2 = the response to fill
{
    res->sum = req->a + req->b;  // Fill in the response
}
```

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

From `cpp_pkg/src/client_node.cpp`:

```cpp
client_ = this->create_client<example_interfaces::srv::AddTwoInts>("add_two_ints");
```

### The call sequence:

```cpp
void call_service(int a, int b)
{
    // 1. Wait until the server exists on the network
    while (!client_->wait_for_service(1s)){
        RCLCPP_WARN(this->get_logger(), "Waiting for the server ...");
    }

    // 2. Create the request object
    auto req = std::make_shared<AddTwoInts::Request>();
    req->a = a;
    req->b = b;

    // 3. Send request ASYNCHRONOUSLY and register a callback for the response
    client_->async_send_request(req,
        std::bind(&ClientNode::callback_response, this, _1));
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

**Next:** [Part 4 — Custom Interfaces](04-custom-interfaces.md)

