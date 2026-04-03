# Part 12 — Hardware Driver: Serial Communication with the LX-225

## 12.1 — The Analogy

The LX-225 driver is like a **telephone operator** from the old days. You pick up the phone (open the serial port), dial a number (send a command), wait for a reply (read the response), and hang up (close the port). The "language" spoken on the wire is a specific protocol with exact formatting rules.

---

## 12.2 — What is the LX-225?

The LX-225 is a **serial bus servo** — a small motor with:
- A built-in motor controller
- A position feedback sensor
- A serial communication interface (UART)
- A unique ID on the bus (so multiple servos share one wire)

```
Your PC                     Serial Bus
  |                           |
  | USB-to-UART adapter       |
  | (/dev/ttyUSB0)            |
  |                           |
  +--[ TX/RX ]--+--[ Servo ID=6 ]
                 |
                 +--[ Servo ID=7 ]  (if you had more)
                 |
                 +--[ Servo ID=8 ]
```

---

## 12.3 — UART: The Physical Layer

UART (Universal Asynchronous Receiver/Transmitter) is a simple serial protocol.

### Signal on the wire:

```
Idle (HIGH) ___
                \___________/    \_/    \_____/    \_/    \_/    ________
                 Start  D0   D1  D2   D3  D4  D5  D6  D7  Stop
                 bit                                          bit

One byte = 1 start bit + 8 data bits + 1 stop bit = 10 bits total
```

### Configuration in the driver:

```cpp
serial_.set_option(boost::asio::serial_port_base::baud_rate(baudrate));    // 115200
serial_.set_option(boost::asio::serial_port_base::character_size(8));       // 8 data bits
serial_.set_option(boost::asio::serial_port_base::parity(parity::none));    // no parity
serial_.set_option(boost::asio::serial_port_base::stop_bits(stop_bits::one)); // 1 stop bit
serial_.set_option(boost::asio::serial_port_base::flow_control(flow_control::none));
```

This is called **8N1** (8 data bits, No parity, 1 stop bit) — the most common UART configuration.

### Baudrate math:

```
Baudrate = 115200 bits/second

Each byte = 10 bits (start + 8 data + stop)
Throughput = 115200 / 10 = 11520 bytes/second

Time per byte = 1/11520 = 86.8 microseconds

A 30-character command takes: 30 * 86.8us = 2.6ms
```

---

## 12.4 — The LX225Driver Class

From `LX225Driver.hpp`:

```cpp
class LX225Driver {
private:
    std::string port_name;          // "/dev/ttyUSB0"
    int baudrate;                    // 115200
    int servo_id;                    // 6
    boost::asio::io_context io;      // Async I/O event loop
    boost::asio::serial_port serial_; // The serial port handle

    void send_command(const std::string& command);

public:
    LX225Driver(const std::string& port, int baudrate, int id = 6);
    int init();
    void close_LX225();
    int get_command_servo_position();
    void set_position(int position, int time_ms = 1000);
    void default_position();
};
```

### Memory layout:

```
HEAP:
+---------------------------------------------------+
| LX225Driver object                                 |
|                                                     |
| port_name = "/dev/ttyUSB0" (std::string, heap)     |
| baudrate = 115200                                   |
| servo_id = 6                                        |
|                                                     |
| io (boost::asio::io_context)                        |
|   +-- internal event loop state                    |
|                                                     |
| serial_ (boost::asio::serial_port)                  |
|   +-- file descriptor (int, from OS)               |
|   +-- OS kernel buffer for TX (outgoing bytes)     |
|   +-- OS kernel buffer for RX (incoming bytes)     |
+---------------------------------------------------+

The OS kernel manages the actual hardware:
  +-- UART controller (hardware on the motherboard)
  +-- TX buffer (ring buffer, typically 16-64 bytes)
  +-- RX buffer (ring buffer)
  +-- Interrupt handler (fires when byte received)
```

---

## 12.5 — Initialization

```cpp
int init() {
    serial_.open(port_name);
    serial_.set_option(baud_rate(baudrate));
    serial_.set_option(character_size(8));
    serial_.set_option(parity(parity::none));
    serial_.set_option(stop_bits(stop_bits::one));
    serial_.set_option(flow_control(flow_control::none));

    std::this_thread::sleep_for(std::chrono::seconds(3));  // Wait for servo to boot
    default_position();  // Move to center (500)
    return 0;
}
```

### What happens at the OS level:

```
serial_.open("/dev/ttyUSB0"):
  1. Linux kernel: open() system call
  2. Kernel finds the USB-to-UART device driver
  3. Returns a file descriptor (e.g., fd=7)
  4. Configures UART hardware registers:
     - Baudrate divisor for 115200
     - 8N1 format
     - Disable hardware flow control

3-second sleep:
  The servo needs time to initialize after power-on.
  During this time, the servo's internal MCU boots up.

default_position():
  Moves servo to position 500 (center of 0-1000 range)
```

---

## 12.6 — Sending Commands

```cpp
void send_command(const std::string& command) {
    boost::asio::write(serial_, boost::asio::buffer(command));
}

void set_position(int position, int time_ms = 1000) {
    if (position < 0) position = 0;
    if (position > 1000) position = 1000;
    std::string cmd = "bus_servo.run(" + std::to_string(servo_id) + "," +
                      std::to_string(position) + "," + std::to_string(time_ms) + ")\r\n";
    send_command(cmd);
}
```

### Command format:

```
Command string: "bus_servo.run(6,800,1000)\r\n"

Breakdown:
  bus_servo.run  = function name (text-based protocol)
  6              = servo ID
  800            = target position (0-1000 range)
  1000           = duration in ms (move over 1 second)
  \r\n           = carriage return + newline (end of command)
```

### Position range:

```
Position value:  0 -------- 500 -------- 1000
Physical angle:  0 deg      ~150 deg      ~300 deg

The servo maps the 0-1000 integer range to its physical rotation range.
Position 500 = center/default position.
```

### On the wire:

```
Bytes sent (ASCII):
  'b'  'u'  's'  '_'  's'  'e'  'r'  'v'  'o'  '.'  'r'  'u'  'n'
  0x62 0x75 0x73 0x5F 0x73 0x65 0x72 0x76 0x6F 0x2E 0x72 0x75 0x6E
  '('  '6'  ','  '8'  '0'  '0'  ','  '1'  '0'  '0'  '0'  ')'  '\r' '\n'
  0x28 0x36 0x2C 0x38 0x30 0x30 0x2C 0x31 0x30 0x30 0x30 0x29 0x0D 0x0A

Total: 27 bytes
Time on wire: 27 * 86.8us = 2.3ms
```

---

## 12.7 — Reading Position

```cpp
int get_command_servo_position()
{
    // 1. Flush any stale data in the RX buffer
    tcflush(handle, TCIFLUSH);

    // 2. Send position query command
    std::string cmd = "bus_servo.get_position(" + std::to_string(servo_id) + ")\r\n";
    send_command(cmd);

    // 3. Wait for servo to respond
    std::this_thread::sleep_for(std::chrono::milliseconds(500));

    // 4. Read response
    boost::asio::streambuf buf;
    boost::asio::read_until(serial_, buf, "\n");

    // 5. Parse: find the first line that doesn't contain "bus_servo" (echo)
    // 6. Extract digits from that line
    // 7. Convert to integer and return
}
```

### The read sequence:

```
Timeline:
  t=0ms     Send: "bus_servo.get_position(6)\r\n"
  t=0-3ms   Bytes travel to servo
  t=3-50ms  Servo processes command
  t=50ms    Servo sends back: "bus_servo.get_position(6)\r\n800\r\n"
  t=500ms   Driver reads the response (after sleep)

Response parsing:
  Line 1: "bus_servo.get_position(6)"  --> skip (echo)
  Line 2: "800"                        --> extract digits --> return 800
```

### Why `tcflush(TCIFLUSH)`?

```
The RX buffer might contain leftover bytes from previous commands:

  RX Buffer before flush: [0x38 0x30 0x30 0x0D 0x0A ...]
                           (stale "800\r\n" from an old read)

  tcflush(TCIFLUSH): clears the RX buffer

  RX Buffer after flush: [empty]

  Now we can send a new command and get a fresh response.
```

---

## 12.8 — The Test Program

From `lx225_test.cpp`:

```cpp
int main() {
    LX225Driver servo("/dev/ttyUSB0", 115200, 6);

    servo.init();                          // Open port, center servo
    servo.set_position(800, 1000);         // Move to 800 over 1 second
    sleep(1500ms);                         // Wait for move to complete
    int pos = servo.get_command_servo_position();  // Read current position
    servo.close_LX225();                   // Return to default, close port

    return 0;
}
```

### Full timeline:

```
  t=0s      init(): open port, configure 8N1 @ 115200
  t=0s      3s sleep (servo boot time)
  t=3s      default_position(): move to 500
  t=4s      set_position(800, 1000): move to 800 over 1s
  t=5.5s    get_command_servo_position(): query, expect ~800
  t=6s      close_LX225(): move back to 500, close port
  t=7s      done
```

---

## 12.9 — Connecting to ros2_control

In the hardware interface (`mobile_base_hardware_interface.cpp`):

```
on_init():     Creates LX225Driver object
on_configure(): Calls driver_->init() (opens serial port)
on_activate():  Resets positions
read():         Currently estimates position (could read from servo)
write():        Currently stores commands (could send to servo)
on_deactivate(): Calls driver_->close_LX225()
```

The current implementation is a **template** — it stores commands locally but doesn't fully drive the servos in the control loop. A complete implementation would:
- In `write()`: convert rad/s to servo position commands and call `set_position()`
- In `read()`: call `get_command_servo_position()` and convert to radians

---

## 12.10 — Hardware Deep-Dive: USB-to-UART

```
USB to UART conversion:

  Your PC's USB Port                     Servo
  +------------------+     +---------+     +-------+
  | USB Host         |-----| CH340 / |-----| LX-225|
  | Controller       | USB | CP2102  | TTL | UART  |
  | (PCI device)     |     | chip    |     | RX/TX |
  +------------------+     +---------+     +-------+
       |                        |
    USB protocol           UART signals
    (packets, endpoints,   (just voltage
     differential signals)  HIGH/LOW on wires)

  Linux sees this as /dev/ttyUSB0
  The CH340/CP2102 chip translates between USB packets and UART signals.
```

---

**Next:** [Part 13 — Writing a Custom Controller](13-writing-custom-controller.md)

