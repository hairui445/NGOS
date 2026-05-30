
```markdown
# NGOS Runtime Middleware

NGOS (Next Generation Omni Scheduler) is an MVP AI Runtime Compatibility Middleware. It standardizes tensor metadata, memory ownership, device discovery, plugin ABI, runtime dispatch, certification, benchmarking, and Python SDK access.

This project intentionally stays within the first-stage MVP:

- Rust runtime core.
- Stable C ABI for vendor plugins.
- Real CPU runtime plugin.
- CUDA capability-detection plugin that returns `Unsupported` when CUDA runtime libraries are unavailable.
- PyO3 Python SDK crate.
- `ngos-cert` certification CLI.
- `ngos-bench` benchmark CLI.
- Documentation-first specs in `docs/`.

## Build

```bash
cargo build --workspace
cargo test --workspace
```

## Certify CPU plugin

```bash
cargo build --workspace
cargo run -p ngos-cert -- target/debug/libngos_cpu_plugin.so
```

On macOS the extension is `.dylib`; on Windows it is `.dll`.

## Benchmark

```bash
cargo run -p ngos-bench -- --iterations 10000 --elements 1024
```

## Python SDK

The Python binding is implemented with PyO3, not direct `ctypes`.

```bash
cd bindings/python
python -m pip install maturin
python -m maturin develop
python examples/python_example.py
```

## Verified in this Devin session

- `cargo fmt --all`
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo build --workspace`
- `ngos-cert` against `libngos_cpu_plugin.so`
- `ngos-bench --iterations 1000 --elements 128`
- PyO3 SDK install with `maturin develop` and `examples/python_example.py`
- Go control plane compiles with `go test ./...`

Note: CUDA is implemented as an ABI-compatible capability detection plugin. On machines without CUDA driver libraries it reports `Unsupported` instead of pretending execution succeeded.
```
# NGOS Runtime Middleware

NGOS (Next Generation Omni Scheduler) 是一款高性能 AI 运行时兼容性中间件。它为 AI 框架提供了标准化的张量元数据（Tensor Metadata）、内存所有权管理、设备发现（Device Discovery）、插件 ABI 规范、运行时调度（Runtime Dispatch）、认证（Certification）、基准测试（Benchmarking）以及 Python SDK 支持。

## 项目愿景

NGOS 的核心目标是解决跨硬件供应商的 AI 运行时兼容性问题，通过定义统一的 ABI，实现不同后端（CPU, CUDA, Ascend 等）的无缝集成。

## 架构概览



## 功能特性

- **高性能运行时**：基于 Rust 构建，保证内存安全与极致性能。
- **标准化 ABI**：定义稳定的 C ABI，支持动态加载 vendor 插件。
- **全生态支持**：提供 PyO3 构建的 Python SDK，无需复杂的 ctypes。
- **工程化工具链**：
    - `ngos-cert`：用于验证插件是否符合 ABI 规范的认证 CLI。
    - `ngos-bench`：标准化的性能基准测试工具。
- **文档先行**：所有核心接口均配有详细的架构文档。

## 快速上手

### 1. 构建项目
确保已安装 Rust 开发环境（推荐 `rustc 1.80+`），在项目根目录下运行：
```bash
cargo build --workspace

