NGOS Runtime Middleware

**NGOS (Next Generation Omni Scheduler)** is a high-performance AI runtime compatibility middleware. It standardizes tensor metadata, memory ownership, device discovery, plugin ABI, runtime dispatch, certification, benchmarking, and provides robust Python SDK access.

**NGOS (Next Generation Omni Scheduler)** 是一款高性能 AI 运行时兼容性中间件。它为 AI 框架提供了标准化的张量元数据（Tensor Metadata）、内存所有权管理、设备发现（Device Discovery）、插件 ABI 规范、运行时调度（Runtime Dispatch）、认证（Certification）、基准测试（Benchmarking）以及 Python SDK 支持。

---

## Architecture Overview / 架构概览

The NGOS architecture is designed for modularity, utilizing a Rust-based core to orchestrate vendor-specific plugins via a stable C ABI.
NGOS 采用模块化设计，核心由 Rust 构建，并通过稳定的 C ABI 编排供应商插件，确保了高性能与可扩展性。

## Key Features / 功能特性

* **High-Performance Runtime / 高性能运行时**: Built with Rust for memory safety and zero-cost abstractions. / 基于 Rust 构建，兼顾内存安全与零成本抽象。
* **Stable ABI / 标准化 ABI**: Defines a stable C ABI for dynamic loading of vendor plugins (CPU, CUDA, etc.). / 定义了稳定的 C ABI，支持 CPU、CUDA 等供应商插件的动态加载。
* **Ecosystem Integration / 全生态支持**: Includes a native Python SDK powered by PyO3. / 提供由 PyO3 驱动的原生 Python SDK。
* **Engineering Toolchain / 工程化工具链**:
* `ngos-cert`: CLI for plugin ABI validation. / 用于插件 ABI 合规性验证的 CLI。
* `ngos-bench`: Standardized performance benchmarking. / 标准化性能基准测试工具。



## Quick Start / 快速上手

### 1. Prerequisites / 环境准备

* Rust `1.80+`
* Python `3.9+` (for bindings)

### 2. Build / 构建项目

```bash
# Build all workspace members
cargo build --workspace
# Run tests
cargo test --workspace

```

### 3. Certify Plugins / 插件认证

Ensure your plugins comply with the NGOS ABI:
确保插件符合 NGOS ABI 规范：

```bash
cargo run -p ngos-cert -- target/debug/libngos_cpu_plugin.so

```

*(Note: Use `.dylib` on macOS or `.dll` on Windows.)*

### 4. Python SDK

```bash
cd bindings/python
python -m pip install maturin
python -m maturin develop
python examples/python_example.py

```

---

## Verified Status / 项目状态

* **Core**: Rust runtime core verified.
* **Plugins**: CPU and CUDA (capability-detection) plugins implemented.
* **Benchmarking**: `ngos-bench` validated with 1000+ iterations.
* **Bindings**: PyO3 SDK confirmed stable.

---

## License / 协议

Licensed under the [Apache License, Version 2.0](https://www.google.com/search?q=LICENSE).

---