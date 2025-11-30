#![cfg_attr(not(any(test, feature = "std")), no_std)]
//! # PCF8563/BM8563 Real-Time Clock Driver
//!
//! This crate provides a bisync-based driver for the PCF8563 and BM8563 real-time clock ICs,
//! built upon the `device-driver` crate for robust, declarative register definitions via a
//! YAML manifest. It supports both asynchronous (`async`) and blocking operation through a
//! unified API, using the [`bisync`](https://docs.rs/bisync) crate for seamless compatibility
//! with both `embedded-hal` and `embedded-hal-async` traits.
//!
//! ## Features
//!
//! *   **Declarative Register Map:** Full device configuration defined in `device.yaml`.
//! *   **Unified Async/Blocking Support:** Write your code once and use it in both async and blocking contexts via bisync.
//! *   **Type-Safe API:** High-level functions for reading/setting date and time
//!     and a generated low-level API (`ll`) for direct register access.
//! *   **Full RTC Functionality:** Date/time, alarms, timer, and clock output control.
//! *   **Optional `rtcc` Traits (blocking):** Enable the `rtcc` feature to implement
//!     [`rtcc::DateTimeAccess`](https://docs.rs/rtcc/latest/rtcc/trait.DateTimeAccess.html)
//!     and [`rtcc::Rtcc`](https://docs.rs/rtcc/latest/rtcc/trait.Rtcc.html) on the blocking driver.
//! *   **`defmt` and `log` Integration:** Optional support for logging and debugging.
//!
//! ## Getting Started
//!
//! To use the driver, instantiate `Pcf8563` (blocking) or `Pcf8563Async` (async) with your I2C bus implementation:
//!
//! ```rust,no_run
//! # use embedded_hal::i2c::I2c;
//! # use pcf8563_dd::Pcf8563;
//! let i2c_bus = todo!();
//! let mut rtc = Pcf8563::new(i2c_bus);
//!
//! let datetime = rtc.get_datetime()?;
//! # Ok::<(), pcf8563_dd::RtcError<std::io::Error>>(())
//! ```
//!
//! For async environments, use `Pcf8563Async` (re-exported from the `asynchronous` module):
//!
//! ```rust,no_run
//! # use embedded_hal_async::i2c::I2c;
//! # use pcf8563_dd::Pcf8563Async;
//! let i2c_bus = todo!();
//! let mut rtc = Pcf8563Async::new(i2c_bus);
//!
//! let datetime = rtc.get_datetime().await?;
//! # Ok::<(), pcf8563_dd::RtcError<std::io::Error>>(())
//! ```
//!
//! For a detailed register map, please refer to the `device.yaml` file in the
//! [repository](https://github.com/okhsunrog/pcf8563-dd).
//!
//! ## Supported Devices
//!
//! - **PCF8563** - NXP real-time clock (original)
//! - **BM8563** - Compatible clone found in M5Stack devices

#[macro_use]
pub(crate) mod fmt;

use thiserror::Error;

device_driver::create_device!(device_name: Pcf8563LowLevel, manifest: "device.yaml");

/// PCF8563/BM8563 I2C address (7-bit)
pub const PCF8563_I2C_ADDR: u8 = 0x51;

#[derive(Debug, Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcError<I2cErr> {
    #[error("I2C error")]
    I2c(I2cErr),
    #[error("Invalid input data")]
    InvalidInputData,
}

/// Date and time structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DateTime {
    /// Year (0-99, represents 2000-2099 or 1900-1999 based on century flag)
    pub year: u8,
    /// Month (1-12)
    pub month: u8,
    /// Day of month (1-31)
    pub day: u8,
    /// Weekday (0-6, typically 0=Sunday)
    pub weekday: u8,
    /// Hours (0-23)
    pub hours: u8,
    /// Minutes (0-59)
    pub minutes: u8,
    /// Seconds (0-59)
    pub seconds: u8,
}

/// Time-only structure (for clock applications without calendar)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Time {
    /// Hours (0-23)
    pub hours: u8,
    /// Minutes (0-59)
    pub minutes: u8,
    /// Seconds (0-59)
    pub seconds: u8,
}

/// Alarm configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Alarm {
    /// Minute alarm (0-59), None if disabled
    pub minute: Option<u8>,
    /// Hour alarm (0-23), None if disabled
    pub hour: Option<u8>,
    /// Day alarm (1-31), None if disabled
    pub day: Option<u8>,
    /// Weekday alarm (0-6), None if disabled
    pub weekday: Option<u8>,
}

pub struct Pcf8563Interface<I2CBus> {
    i2c_bus: I2CBus,
}

impl<I2CBus> Pcf8563Interface<I2CBus> {
    pub fn new(i2c_bus: I2CBus) -> Self {
        Self { i2c_bus }
    }
}

#[path = "."]
mod asynchronous {
    use bisync::asynchronous::*;
    use device_driver::AsyncRegisterInterface as RegisterInterface;
    use embedded_hal_async::i2c::I2c;
    mod driver;
    pub use driver::*;
}
pub use asynchronous::Pcf8563 as Pcf8563Async;

#[path = "."]
mod blocking {
    use bisync::synchronous::*;
    use device_driver::RegisterInterface;
    use embedded_hal::i2c::I2c;
    #[allow(clippy::duplicate_mod)]
    mod driver;
    pub use driver::*;
}
pub use blocking::Pcf8563;

/// Convert BCD to decimal
#[inline]
pub(crate) fn bcd_to_dec(bcd: u8) -> u8 {
    (bcd & 0x0F) + ((bcd >> 4) * 10)
}

/// Convert decimal to BCD
#[inline]
pub(crate) fn dec_to_bcd(dec: u8) -> u8 {
    ((dec / 10) << 4) | (dec % 10)
}
