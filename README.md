# PCF8563/BM8563 Real-Time Clock Rust Driver (pcf8563-dd)

[![Crates.io](https://img.shields.io/crates/v/pcf8563-dd.svg)](https://crates.io/crates/pcf8563-dd)
[![Docs.rs](https://docs.rs/pcf8563-dd/badge.svg)](https://docs.rs/pcf8563-dd)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses)

A `no_std` Rust driver for the NXP PCF8563 and compatible BM8563 real-time clock ICs. This driver leverages the `device-driver` crate with a declarative YAML manifest (`device.yaml`) for a robust, type-safe register map definition. The low-level API covers 100% of the PCF8563's registers, with `device.yaml` providing a comprehensive and accurate description of all registers and their fields verified against the official datasheet.

## Overview

The `pcf8563-dd` driver offers:

- **Declarative Configuration:** The PCF8563 register map is defined in [`device.yaml`](device.yaml), enabling `device-driver` to generate a type-safe, low-level register access API.
- **Unified Async/Blocking API:** Uses the [`bisync`](https://github.com/JM4ier/bisync) crate to provide both asynchronous (`Pcf8563Async`) and blocking (`Pcf8563`) drivers from the same codebase, with no feature flags required.
- **High-Level and Low-Level APIs:**
  - High-level methods simplify tasks like reading/setting date and time, configuring alarms, and managing the timer.
  - Low-level API (via the `ll` field) offers direct, type-safe access to all registers defined in `device.yaml`.
- **`no_std` and `no-alloc`:** Optimized for bare-metal and RTOS environments.
- **Optional Logging:** Supports `defmt` and the `log` facade for debugging.
- **Optional `rtcc` Traits (blocking):** Enable the `rtcc` feature to get `rtcc::DateTimeAccess` and `rtcc::Rtcc` implementations for the blocking driver.

## Features

- **Date and Time:** Read and set year (2000-2099), month, day, weekday, hours, minutes, and seconds.
- **Clock Integrity Detection:** Voltage-low (VL) flag indicates if clock data may be invalid due to power loss.
- **Century Flag:** Track century rollover for extended date range.
- **Alarm Function:** Programmable alarm with minute, hour, day, and weekday matching.
- **Countdown Timer:** 8-bit countdown timer with selectable clock frequencies (4096Hz, 64Hz, 1Hz, 1/60Hz).
- **Clock Output (CLKOUT):** Programmable square wave output (32.768kHz, 1024Hz, 32Hz, 1Hz).
- **Interrupt Support:** Configurable interrupts for alarm and timer events.
- **Low Power:** Designed for battery-backed operation.

## Supported Devices

- **PCF8563** - NXP real-time clock (original)
- **BM8563** - Compatible clone found in M5Stack devices

## Getting Started

1. **Add `pcf8563-dd` to `Cargo.toml`:**

   ```toml
   [dependencies]
   pcf8563-dd = "0.2.0"
   # For blocking usage (Pcf8563):
   embedded-hal = "1.0.0"
   # For async usage (Pcf8563Async):
   embedded-hal-async = "1.0.0"
   ```

2. **Instantiate the driver with your I2C bus:**

   - **Blocking:**
     ```rust
     use pcf8563_dd::{Pcf8563, DateTime, Alarm};

     let i2c_bus = /* your I2C bus */;
     let mut rtc = Pcf8563::new(i2c_bus);

     // Initialize the RTC
     rtc.init()?;

     // Check if clock data is valid
     let valid = rtc.is_clock_valid()?;

     // Set the date and time
     let dt = DateTime {
         year: 24,       // 2024
         month: 12,      // December
         day: 1,         // 1st
         weekday: 0,     // Sunday
         hours: 12,      // 12:00:00
         minutes: 0,
         seconds: 0,
     };
     rtc.set_datetime(&dt)?;

     // Read current date and time
     let current = rtc.get_datetime()?;
     ```

   - **Async:**
     ```rust
     use pcf8563_dd::{Pcf8563Async, DateTime};

     let i2c_bus = /* your async I2C bus */;
     let mut rtc = Pcf8563Async::new(i2c_bus);

     // Initialize the RTC
     rtc.init().await?;

     // Read current date and time
     let current = rtc.get_datetime().await?;
     ```

## High-Level API

### Date and Time

```rust
// Get current date/time
let dt = rtc.get_datetime()?;

// Set date/time
rtc.set_datetime(&DateTime {
    year: 24, month: 12, day: 25,
    weekday: 3, hours: 10, minutes: 30, seconds: 0,
})?;

// Set only time (preserves date)
rtc.set_time(&Time { hours: 14, minutes: 30, seconds: 0 })?;
```

### Clock Control

```rust
// Check clock validity (false if power was lost)
let valid = rtc.is_clock_valid()?;

// Clear the voltage-low flag after setting time
rtc.clear_voltage_low_flag()?;

// Stop/start the clock
rtc.set_clock_running(false)?; // Stop
rtc.set_clock_running(true)?;  // Start
```

### Alarm

```rust
use pcf8563_dd::Alarm;

// Set alarm for 12:30 on any day
let alarm = Alarm {
    minute: Some(30),
    hour: Some(12),
    day: None,      // Any day
    weekday: None,  // Any weekday
};
rtc.set_alarm(&alarm)?;
rtc.set_alarm_interrupt(true)?;

// Check and clear alarm flag
if rtc.get_alarm_flag()? {
    rtc.clear_alarm_flag()?;
}

// Disable alarm
rtc.disable_alarm()?;
```

### Timer

```rust
use pcf8563_dd::TimerFrequency;

// Configure 10-second countdown timer
rtc.set_timer_frequency(TimerFrequency::Freq1Hz)?;
rtc.set_timer_value(10)?;
rtc.set_timer_enabled(true)?;
rtc.set_timer_interrupt(true)?;

// Check timer status
let value = rtc.get_timer_value()?;
if rtc.get_timer_flag()? {
    rtc.clear_timer_flag()?;
}
```

### Clock Output

```rust
use pcf8563_dd::ClkoutFrequency;

// Enable 1Hz square wave output on CLKOUT pin
rtc.set_clkout_frequency(ClkoutFrequency::Freq1Hz)?;
rtc.set_clkout_enabled(true)?;
```

## Low-Level API Usage

The driver provides direct access to all PCF8563 registers through the low-level API via `rtc.ll`. This API is automatically generated from [`device.yaml`](device.yaml) and provides type-safe access to all register fields.

### Reading Registers

```rust
// Read control status registers
let ctrl1 = rtc.ll.control_status_1().read()?;
let is_stopped = ctrl1.stop();

let ctrl2 = rtc.ll.control_status_2().read()?;
let alarm_flag = ctrl2.af();
let timer_flag = ctrl2.tf();

// Read seconds register with voltage-low flag
let seconds = rtc.ll.seconds().read()?;
let vl = seconds.vl(); // Clock integrity flag
let sec_value = seconds.seconds_ten() * 10 + seconds.seconds_unit();
```

### Writing Registers

```rust
// Configure control status 2
rtc.ll.control_status_2().write(|w| {
    w.set_aie(true);  // Enable alarm interrupt
    w.set_tie(false); // Disable timer interrupt
    w.set_af(false);  // Clear alarm flag
    w.set_tf(false);  // Clear timer flag
})?;

// Set timer value
rtc.ll.timer().write(|w| {
    w.set_timer_value(60); // 60 counts
})?;
```

### Modifying Registers

Use `.modify()` to read-modify-write, preserving other fields:

```rust
// Enable timer without changing frequency
rtc.ll.timer_control().modify(|w| {
    w.set_te(true);
})?;
```

### Async Low-Level API

Append `_async` to method names for async usage:

```rust
let ctrl1 = rtc.ll.control_status_1().read_async().await?;

rtc.ll.timer_control().modify_async(|w| {
    w.set_te(true);
}).await?;
```

## Register Map

The complete PCF8563 register map is defined in [`device.yaml`](device.yaml):

| Address | Register | Description |
|---------|----------|-------------|
| 0x00 | Control Status 1 | TEST1, STOP, TESTC flags |
| 0x01 | Control Status 2 | Interrupt enables and flags (TI_TP, AF, TF, AIE, TIE) |
| 0x02 | Seconds | VL flag, seconds (0-59) |
| 0x03 | Minutes | Minutes (0-59) |
| 0x04 | Hours | Hours (0-23) |
| 0x05 | Days | Day of month (1-31) |
| 0x06 | Weekdays | Day of week (0-6) |
| 0x07 | Century/Months | Century flag, month (1-12) |
| 0x08 | Years | Year (0-99) |
| 0x09 | Minute Alarm | AE_M flag, minute alarm value |
| 0x0A | Hour Alarm | AE_H flag, hour alarm value |
| 0x0B | Day Alarm | AE_D flag, day alarm value |
| 0x0C | Weekday Alarm | AE_W flag, weekday alarm value |
| 0x0D | CLKOUT Control | FE (enable), FD (frequency) |
| 0x0E | Timer Control | TE (enable), TD (frequency) |
| 0x0F | Timer | Countdown timer value (0-255) |

## Examples

Examples for ESP32 using `esp-hal` are included. Both examples demonstrate high-level convenience methods and low-level register API usage.

- **Async Example:** [`examples/test_pcf8563_async.rs`](examples/test_pcf8563_async.rs)
  ```bash
  cargo run --release --example test_pcf8563_async --features defmt
  ```
- **Blocking Example:** [`examples/test_pcf8563_blocking.rs`](examples/test_pcf8563_blocking.rs)
  ```bash
  cargo run --release --example test_pcf8563_blocking --features defmt
  ```

## Feature Flags

- **`default = []`**: No default features; async and blocking drivers are always available.
- **`std`**: Enables `std` features for `thiserror`.
- **`log`**: Enables `log` facade logging.
- **`defmt`**: Enables `defmt` logging for embedded debugging.
- **`rtcc`**: Implements `rtcc::DateTimeAccess` and `rtcc::Rtcc` for the blocking driver.

## Timer Frequencies

The countdown timer supports four clock frequencies:

| Frequency | Period | Max Duration |
|-----------|--------|--------------|
| 4096 Hz | ~244 µs | ~62 ms |
| 64 Hz | ~15.6 ms | ~4 seconds |
| 1 Hz | 1 second | ~4.25 minutes |
| 1/60 Hz | 1 minute | ~4.25 hours |

## CLKOUT Frequencies

The clock output pin can generate square waves at:

- 32.768 kHz (default)
- 1024 Hz
- 32 Hz
- 1 Hz

## Contributing

Contributions are welcome! You can contribute by:

- Adding high-level convenience methods for additional features.
- Enhancing documentation with examples or clarifications.
- Reporting issues or suggesting improvements.
- Testing on different hardware platforms.

Please submit issues, fork the repository, and create pull requests.

## License

This project is dual-licensed under the [MIT License](LICENSE-MIT) or [Apache License 2.0](LICENSE-APACHE), at your option.
