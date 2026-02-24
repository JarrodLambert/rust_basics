# Rust Embedded & Systems Resource Index

A curated list of libraries, tools, and documentation for current and future projects.

---

## 🎹 Ableton Push 2 Bridge
* **[push2_display (Crate)](https://docs.rs/push2_display)**: The core Rust driver for the Push 2 screen. Implements `embedded-graphics`.
* **[push2_pong (GitHub)](https://github.com/mbracher/push2_pong)**: A full Pong implementation. Great reference for real-time frame buffering and USB transfers.
* **[embedded-graphics (Docs)](https://docs.rs/embedded-graphics)**: The universal 2D graphics library for `no_std` Rust. Essential for drawing UI elements.
* **[BetterDisplay (Tool)](https://github.com/waydabber/BetterDisplay)**: Utility for creating the $960 \times 160$ virtual display on macOS.

## 🍓 Raspberry Pi 4B Bare-Metal
* **[Rust Raspberry Pi Tutorials](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)**: The definitive guide. Tutorials 1–5 cover the exact groundwork needed for Pi 4 revival.
* **[Low-level RPi4 (Sean Lawless)](https://sean-lawless.github.io/computersystems/)**: Deep-dive into Pi 4 specific memory mapping ($0xFE00\_0000$ base address).
* **[Official Pi Firmware](https://github.com/raspberrypi/firmware/tree/master/boot)**: Source for `start4.elf` and `fixup4.dat` files required for booting.

## ⚡ ESP32 (S3 & C6) & Embassy
* **[The Rust on ESP Book](https://docs.espressif.com/projects/rust/book/)**: Official Espressif guide for toolchain setup (including Xtensa for your S3).
* **[Awesome ESP Rust](https://github.com/esp-rs/awesome-esp-rust)**: Curated list of templates and crates specifically for the ESP ecosystem.
* **[Embassy on ESP: Getting Started](https://dev.to/theembeddedrustacean/embassy-on-esp-getting-started-27fi)**: Walkthrough for using the async executor on ESP32 chips.

## 🛠️ Tooling & Cross-Compiling
* **[The Embedded Rust Book](https://docs.rust-embedded.org/book/)**: The "Bible" for `no_std`, HALs, and embedded Rust philosophy.
* **[Cargo Cross (Tool)](https://github.com/cross-rs/cross)**: Docker-based cross-compilation. Essential for building on your Mac for Linux/ARM targets.
* **[Launchd.info](https://www.launchd.info/)**: Reference for creating macOS background services (`.plist` files) for scripts like your Screenshot Sweeper.

## 🎓 Rust Theory & Practice
* **[Comprehensive Rust (Google)](https://google.github.io/comprehensive-rust/bare-metal.html)**: Excellent "Bare Metal" section covering concurrency and traits.
* **[Rustlings](https://github.com/rust-lang/rustlings)**: Interactive exercises to sharpen your understanding of Ownership and Traits.
