# Envy

![Language](https://img.shields.io/badge/language-Rust-orange?style=flat-square)
![License](https://img.shields.io/badge/license-BSD_2_Clause-blue?style=flat-square)
![Status](https://img.shields.io/badge/status-active_development-green?style=flat-square)

**Envy** is a lightweight, multi-threaded web server written in Rust.

Inspired by Nginx, this project is built from scratch to explore the low-level implementations of the HTTP/1.1 protocol, TCP networking, and memory management in Rust. It avoids high-level web frameworks (like Actix or Rocket) in favor of using standard library features and minimal dependencies to handle requests.

## ✨ Features

* **Robust Configuration:** Auto-generates a default `config.toml` if missing.
* **Custom Logging:** Real-time console logging with precise timestamps.
* **Multi-threading:** Handles multiple connections concurrently (currently thread-per-request, moving to Thread Pool).
* **Minimalistic:** Built with a focus on simplicity and understanding the core mechanics of a web server.

## 🗺️ Roadmap

The goal is to evolve Envy from a simple TCP listener into a capable static file server.

### Phase 1: Foundation (Core Networking) ✅
- [x] TCP Listener initialization.
- [x] Connection handling loop.
- [x] Configuration system (TOML parsing with Serde).
- [x] Custom logging implementation with `chrono`.
- [x] Fault tolerance: Auto-creation of configuration files.

### Phase 2: HTTP Parsing & Static Content ✅
- [x] **Request Parsing:** Extracting HTTP Method (GET), URI, and Version.
- [x] **Static File Serving:** Reading and serving files from the `root_dir`.
- [x] **MIME Types:** Automatic detection of content types (`.html`, `.css`, `.png`, etc.).
- [x] **Response Construction:** Proper formatting of HTTP 200/404 headers and body.

### Phase 3: Performance & Security (Current Focus) 🚧
- [ ] **Thread Pool:** Implementing a fixed-size thread pool to prevent DoS via resource exhaustion.
- [ ] **Security:** Path traversal protection (preventing access outside `root_dir`).
- [ ] **Graceful Shutdown:** Handling signals (Ctrl+C) to close sockets cleanly.

### Phase 4: Future Improvements 🔮
- [ ] **Async I/O:** Migration to `Tokio` for non-blocking operations.
- [ ] **Virtual Hosts:** Support for multiple domains on a single port.

## 🚀 Getting Started

### Prerequisites
You need [Rust and Cargo](https://rustup.rs/) installed on your machine.

### Installation

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/sashagoncharov19/envy.git
    cd envy
    ```

2.  **Run the server:**
    ```bash
    cargo run
    ```
    *On the first run, the server will automatically generate a `config.toml` file.*

3.  **Test it:**
    Open your browser and navigate to: `http://localhost:3550`

## ⚙️ Configuration

Envy uses a `config.toml` file for setup. You can modify it to change the server's behavior:

```toml
address = "0.0.0.0"     # IP address to bind to
port = 3550             # Port to listen on
root_dir = "./public"   # Directory serving static files
