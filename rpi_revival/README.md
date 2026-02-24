# Raspberry Pi 4B Bare-Metal Revival

A diagnostic suite written in Rust to validate and revive "dead" Raspberry Pi 4B hardware.

## Phase 1: The Boot Laboratory (SD Setup)
* [ ] **1.1 EEPROM Recovery:** Use the official Raspberry Pi Imager to create an "EEPROM Recovery" SD card. Run it on the Pi to ensure the onboard bootloader isn't corrupted.
* [ ] **1.2 Clean Slate:** Format a microSD card as **FAT32** (MBR partition table).
- [ ] **1.3 Essential Firmware:** Download and copy the following from the [official firmware repo](https://github.com/raspberrypi/firmware/tree/master/boot):
    * `bootcode.bin`
    * `start4.elf`
    * `fixup4.dat`
- [ ] **1.4 Config File:** Create `config.txt` with:
    ```ini
    arm_64bit=1
    kernel=kernel8.img
    enable_uart=1
    ```

## Phase 2: The Rust Toolchain (AArch64)
- [ ] **2.1 Target Install:** Add the bare-metal 64-bit ARM target: 
    `rustup target add aarch64-unknown-none`
- [ ] **2.2 Panic Handler:** Implement a `#![no_std]` panic handler that simply loops forever.
- [ ] **2.3 Linker Script:** Create `layout.ld` to place the code at the Pi's entry point: **0x80,000**.



## Phase 3: The "Heartbeat" (GPIO Blink)
- [ ] **3.1 Peripheral Mapping:** Define the GPIO Register offsets (Base: `0xFE20_0000`).
- [ ] **3.2 ACT LED ID:** Identify the ACT LED pin for the Pi 4B (It is **GPIO 42**).
- [ ] **3.3 The Blinky:** Write a "Naked" function to initialize GPIO 42 as output and toggle it using a software delay loop.
- [ ] **3.4 Binary Conversion:** Use `rust-objcopy` to convert your ELF to a raw `kernel8.img`.

## Phase 4: The "Voice" (UART Debugging)
- [ ] **4.1 Mini-UART Init:** Configure the Mini-UART (GPIO 14/15) at 115200 baud.
- [ ] **4.2 The Hello World:** Create a function to push raw ASCII bytes to the UART transmit register.
- [ ] **4.3 Serial Link:** Connect a USB-to-TTL cable from your Mac to the Pi's pins (GND, TX, RX) and listen using `screen` or `Serial Tools`.



## Phase 5: Hardware Stress Test (Diagnostics)
- [ ] **5.1 RAM Probe:** Write a script to write/read patterns to a 1MB chunk of high memory to check for RAM defects.
- [ ] **5.2 Core Check:** Use the `MPIDR_EL1` register to identify which of the 4 cores is running and report it via Serial.
- [ ] **5.3 Continuous Monitor:** Create a loop that reports system uptime and "Heartbeat" status to the Serial console.
