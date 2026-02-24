# Ableton Push 2 Virtual Display Bridge

A Rust-based utility to mirror a macOS virtual display to the Ableton Push 2 hardware via USB Bulk transfers.

## Project Roadmap

### Phase 1: The "Ghost" Setup (Environment)
- [ ] **1.1 Virtual Display:** Install *BetterDisplay* and create a virtual screen.
- [ ] **1.2 Constraints:** Lock the virtual screen resolution to exactly **960 x 160**.
- [ ] **1.3 USB Discovery:** Confirm the Mac sees the Push 2's Vendor ID (`2982`) and Product ID (`1967`) using `system_profiler SPUSBDataType`.
- [ ] **1.4 Permission Check:** Ensure the terminal/IDE has "Screen Recording" permissions in macOS System Settings.

### Phase 2: The "Pixel Scraper" (Software)
- [ ] **2.1 Frame Capture:** Write a Rust script using `scrap` or `core-graphics` to capture a frame from the virtual monitor.
- [ ] **2.2 Buffer Validation:** Verify the captured buffer length matches $960 \times 160 \times 4$ (RGBA) bytes.
- [ ] **2.3 Single Frame Save:** Export the captured buffer as a `.png` to verify the "Scraper" logic.

### Phase 3: The "Translator" (Data Processing)
- [ ] **3.1 BGR565 Converter:** Implement a function to downsample 32-bit RGBA to 16-bit BGR565.
- [ ] **3.2 Bit-Shifting Logic:** Apply specific bit-shifting for Push 2's little-endian pixel format.
- [ ] **3.3 Header Implementation:** Construct the mandatory 20-byte USB frame header.



### Phase 4: The "Handshake" (Hardware)
- [ ] **4.1 Device Claiming:** Use `rusb` to claim the Push 2 USB Interface (Interface 1).
- [ ] **4.2 Static Push:** Successfully transmit a solid-color buffer (e.g., all blue) to the Bulk Endpoint.
- [ ] **4.3 Image Push:** Successfully transmit the frame captured in Phase 2.3 to the hardware.

### Phase 5: The "Bridge" (Integration)
- [ ] **5.1 The Main Loop:** Integrate the Scraper and the Pusher into a single execution loop.
- [ ] **5.2 Timing Control:** Use `std::time` to cap the loop (30 or 60 FPS) to manage CPU load.
- [ ] **5.3 Optimization:** Implement a "diff" check to only push frames when pixels have changed.

### Phase 6: The "Polish" (UX)
- [ ] **6.1 Auto-Discovery:** Enable the app to automatically find and claim the Push 2 on startup.
- [ ] **6.2 CLI Controls:** Add arguments for brightness control and toggle modes.
