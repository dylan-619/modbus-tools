<div align="center">
  <img src="src-tauri/icons/128x128@2x.png" alt="Modbus Tools Logo" width="128" />
  <h1>Modbus Tools</h1>
  <p>A modern, high-performance, and cross-platform Modbus Master Tool built with Tauri and Rust.</p>
  <p>一个基于 Tauri 和 Rust 构建的现代、高性能、跨平台 Modbus 主站调试工具。</p>
  
  <p>
    <img src="https://img.shields.io/badge/Tauri-2.0-blue?style=flat-square&logo=tauri" alt="Tauri" />
    <img src="https://img.shields.io/badge/Vue.js-3.0-4FC08D?style=flat-square&logo=vuedotjs" alt="Vue" />
    <img src="https://img.shields.io/badge/Rust-Backend-CE412B?style=flat-square&logo=rust" alt="Rust" />
    <img src="https://img.shields.io/badge/License-MIT-green?style=flat-square" alt="License" />
  </p>
</div>

---

## 🌍 Introduction | 简介

**Modbus Tools** is a cross-platform desktop application designed for industrial automation engineers and developers. It provides a sleek, dark-mode user interface for communicating with Modbus TCP and RTU devices. By leveraging Rust's multi-threading and Tauri's lightweight architecture, it guarantees extreme reliability and real-time performance without the bloat of traditional Electron apps.

**Modbus Tools** 是一款专为工业自动化工程师和开发者设计的跨平台桌面应用程序。它提供了一个现代化的深色模式用户界面，用于与 Modbus TCP 和 RTU 设备进行通信。通过利用 Rust 的多线程和 Tauri 的轻量级架构，它在保证极致可靠性和实时性能的同时，摆脱了传统 Electron 程序的臃肿。

## ✨ Features | 功能特性

### 🇺🇸 English
* **Dual Transport Support**: Connect via **Modbus TCP** or **Modbus RTU** (Serial port with customizable baud rate, parity, stop bits).
* **Comprehensive Read/Write**: Supports all standard function codes:
  * 01 Read Coils, 02 Read Discrete Inputs
  * 03 Read Holding Registers, 04 Read Input Registers
  * 05 Write Single Coil, 06 Write Single Register
  * 15 Write Multiple Coils, 16 Write Multiple Registers
* **Advanced Data Parsing**: Supports diverse byte orders (AB, BA, ABCD, BADC, etc.) and native data types (UInt16, Float32, Int64, etc.).
* **Real-time Traffic Logger**: Built-in Rx/Tx raw hex traffic monitoring panel with auto-scroll.
* **Auto-Polling**: Configurable interval-based polling for continuous data monitoring.
* **Ultra-Fast & Lightweight**: Negligible RAM and CPU footprint thanks to the Rust backend.

### 🇨🇳 中文
* **双协议支持**：支持通过 **Modbus TCP** 或 **Modbus RTU**（可配置波特率、校验位、停止位等串口参数）连接设备。
* **完整的读写支持**：支持所有标准功能码：
  * 01 读取线圈状态, 02 读取离散输入
  * 03 读取保持寄存器, 04 读取输入寄存器
  * 05 写单个线圈, 06 写单个寄存器
  * 15 写多个线圈, 16 写多个寄存器
* **高级数据解析**：支持多种字节序（AB, BA, ABCD, BADC 等）以及原生数据类型（UInt16, Float32, Int64 等）的解析。
* **实时流量监控**：内置 Rx/Tx 原始十六进制报文日志面板，支持自动滚动。
* **自动轮询监控**：支持配置自定义时间间隔，实现数据的持续自动读取和监控。
* **极致性能与轻量**：得益于 Rust 后端，极低的内存占用和 CPU 消耗。

## 🛠 Tech Stack | 技术栈

* **Backend / 核心引擎**: [Rust](https://www.rust-lang.org/), [Tauri 2](https://v2.tauri.app/), `tokio`, `serialport`
* **Frontend / 用户界面**: [Vue 3](https://vuejs.org/) (Composition API), [Pinia](https://pinia.vuejs.org/), Vanilla CSS
* **Build Tool / 构建工具**: [Vite](https://vitejs.dev/), `pnpm`

## 🚀 Getting Started | 快速开始

### Prerequisites / 准备工作
Ensure you have the following installed on your system:
请确保您的系统已安装以下环境：
* [Node.js](https://nodejs.org/) (v18+)
* [pnpm](https://pnpm.io/)
* [Rust](https://www.rust-lang.org/tools/install) (cargo)
* Prerequisites for Tauri (C++ build tools / Xcode command line tools based on your OS).

### Installation / 安装与启动

1. **Clone the repository | 克隆仓库**
   ```bash
   git clone https://github.com/dylan-619/modbus-tools.git
   cd modbus-tools
   ```

2. **Install dependencies | 安装依赖**
   ```bash
   pnpm install
   ```

3. **Run in development mode | 开发模式运行**
   ```bash
   pnpm tauri dev
   ```

4. **Build for production | 构建发行版本**
   ```bash
   pnpm tauri build
   ```

## 📸 Screenshots | 界面预览
*(Screenshots coming soon | 截图准备中...)*

## 📄 License | 开源协议

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
本项目基于 MIT 协议开源。
