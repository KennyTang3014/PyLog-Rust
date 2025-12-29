# 🦀 PyLogRust

**High-Performance, Asynchronous Python Logging Powered by Rust.**

`PyLogRust` is a high-speed Python logging extension that offloads heavy I/O operations to a background Rust thread. It ensures your Python application remains lightning-fast even when capturing detailed tracebacks or writing to disk under high load.


## ⚡ Why PyLogRust? (The Problems We Solve)

Standard Python logging can become a bottleneck in high-throughput production environments. **PyLogRust** addresses three specific pain points:

### 1. The "Blocking I/O" Problem

* **Problem:** Writing logs to a file or printing to a console blocks the main Python thread. If the disk is slow, your API response time suffers.
* **Solution:** **Asynchronous Rust Core.** The Python side simply pushes data into a lock-free memory channel (microseconds). A separate Rust thread handles formatting and file writing in the background. **Your Python code never waits for the disk.**

### 2. The "Error Storm" Problem

* **Problem:** When a database goes down or a bug appears in a loop, logs are often flooded with thousands of identical error messages, filling up disk space and making debugging impossible.
* **Solution:** **Smart Throttling.** PyLogRust automatically detects duplicate errors within a configurable time window (e.g., 2 seconds) and drops them. You see the error once, but avoid the spam.

### 3. The "Missing Context" Problem

* **Problem:** An error log tells you *what* happened, but not *why* the server was slow or *which* user request triggered it.
* **Solution:** **System Snapshots & Trace Context.** Every error log automatically includes:
* **Hardware Health:** Real-time CPU and Memory usage snapshots at the moment of the crash.
* **Trace ID:** Automatic request tracking across nested functions.



---

## ✨ Features

* **🚀 Zero-Overhead Logging:** Python execution is decoupled from logging I/O via Rust channels.
* **🛡️ `@debug` Decorator:** Simple, zero-config error catching.
* **🚦 Smart Throttling:** Define a `throttle_sec` interval to deduplicate identical errors.
* **📊 System Metrics:** Auto-injects CPU load and RAM usage into every log entry.
* **🆔 Request Tracing:** Built-in context manager to track logs via Request IDs.
* **🎨 Colored Output:** High-visibility, colored logs in the terminal for easy debugging.
* **📂 Background File Logging:** robust file writing handled by Rust.
* **💥 Crash Control:** Choose to swallow errors (`crash=False`) or propagate them (`crash=True`).

---

## 🛠️ Installation

**Prerequisites:** You need `Rust` (cargo) and `Python` installed.

1. **Install the build tool:**
```bash
pip install maturin

```


2. **Build and install the package:**
Navigate to the project directory and run:
```bash
maturin develop

```



---

## 📖 Usage Guide

### 1. Initialization

You must initialize the Rust core once at the start of your application.

'''
pip install pylogrust
'''

```python
import pylogrust

# Initialize the logger
# log_name: Name of your service (e.g., "AuthService")
# file_path: Path to save logs (set to None if you only want console output)
# throttle_sec: Time in seconds to ignore duplicate errors (e.g., 2)
pylogrust.init(
    log_name="PaymentService", 
    file_path="app_errors.log", 
    throttle_sec=2
)

```

### 2. The `@debug` Decorator

Use the `@debug` decorator to automatically catch exceptions, log them with full context, and control program flow.

```python
from pylogrust import debug

# --- Mode 1: Safe Mode (Default) ---
# Logs the error via Rust but keeps the program running (returns None)
@debug(crash=False)
def risky_task(data):
    return 100 / data  # If data is 0, it logs but doesn't crash the app

# --- Mode 2: Strict Mode ---
# Logs the error immediately, then re-raises the exception (program crashes/stops)
@debug(crash=True)
def critical_task():
    raise ValueError("Critical DB Failure!")

```

### 3. Using Trace Context (Request IDs)

Track logs across multiple function calls belonging to the same request.

```python
from pylogrust import set_request_id, debug

def handle_web_request():
    # Generates a unique ID for this execution context
    set_request_id() 
    
    process_data()

@debug
def process_data():
    # If this fails, the log will contain the Request ID generated above.
    # This helps link the error back to the specific user request.
    print(1 / 0)

```

---

## 🏗️ Architecture

```mermaid
graph LR
    subgraph Python ["Python Main Thread"]
        UserCode[User Function] --> Decorator[@debug Decorator]
        Decorator -->|1. Capture Error & Context| PyO3[Rust Binding]
    end

    subgraph Rust ["Rust Core (Background)"]
        PyO3 -->|2. Send (Non-blocking)| Channel((Memory Channel))
        Channel -->|3. Async Receive| Worker[Background Worker]
        
        Worker -->|4. Check Throttling| Filter{Is Duplicate?}
        
        Filter -- No --> Sys[Fetch CPU/Mem Metrics]
        Sys --> Console[Colored Console Output]
        Sys --> File[File System I/O]
        
        Filter -- Yes --> Drop[Discard Log]
    end

```

---

## ⚙️ Configuration Options

| Parameter | Type | Description |
| --- | --- | --- |
| `log_name` | `str` | The label for your application in the logs. |
| `file_path` | `str` (Optional) | Path to the log file. If `None`, logs are only printed to stdout. |
| `throttle_sec` | `int` | The deduplication window in seconds. Identical errors within this window are ignored. |

---

## 📄 License

MIT License
