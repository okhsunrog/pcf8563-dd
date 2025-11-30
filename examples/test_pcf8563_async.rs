#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::{
    Async,
    i2c::master::{Config as I2cConfig, Error as I2cError, I2c},
    interrupt::software::SoftwareInterruptControl,
    time::Rate,
    timer::timg::TimerGroup,
};
use panic_rtt_target as _;
use pcf8563_dd::{Alarm, ClkoutFrequency, DateTime, Pcf8563Async, RtcError, TimerFrequency};
use rtt_target::rtt_init_defmt;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(_spawner: Spawner) {
    rtt_init_defmt!();
    info!("Init!");

    let p = esp_hal::init(esp_hal::Config::default());

    let timg0 = TimerGroup::new(p.TIMG0);
    let sw_ints = SoftwareInterruptControl::new(p.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_ints.software_interrupt0);

    let config: I2cConfig = I2cConfig::default().with_frequency(Rate::from_khz(400));
    let i2c = I2c::new(p.I2C0, config)
        .unwrap()
        .with_sda(p.GPIO6)
        .with_scl(p.GPIO7)
        .into_async();

    test_pcf8563(i2c).await.unwrap();

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[rustfmt::skip]
async fn test_pcf8563(i2c: I2c<'_, Async>) -> Result<(), RtcError<I2cError>> {
    // Create PCF8563 instance
    let mut rtc = Pcf8563Async::new(i2c);

    info!("=== High-Level API Examples ===");

    // Initialize the RTC
    rtc.init().await?;
    info!("RTC initialized");

    // Check clock validity
    let valid = rtc.is_clock_valid().await?;
    info!("Clock valid: {}", valid);

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
    rtc.set_datetime(&dt).await?;
    info!("Date/time set to: 20{:02}-{:02}-{:02} {:02}:{:02}:{:02}",
          dt.year, dt.month, dt.day, dt.hours, dt.minutes, dt.seconds);

    // Read back the date and time
    let current = rtc.get_datetime().await?;
    info!("Current date/time: 20{:02}-{:02}-{:02} {:02}:{:02}:{:02}",
          current.year, current.month, current.day,
          current.hours, current.minutes, current.seconds);

    // Set an alarm for 12:05
    let alarm = Alarm {
        minute: Some(5),
        hour: Some(12),
        day: None,
        weekday: None,
    };
    rtc.set_alarm(&alarm).await?;
    rtc.set_alarm_interrupt(true).await?;
    info!("Alarm set for 12:05");

    // Configure CLKOUT
    rtc.set_clkout_frequency(ClkoutFrequency::Freq1Hz).await?;
    rtc.set_clkout_enabled(true).await?;
    info!("CLKOUT enabled at 1 Hz");

    // Configure timer for 10 second countdown
    rtc.set_timer_frequency(TimerFrequency::Freq1Hz).await?;
    rtc.set_timer_value(10).await?;
    rtc.set_timer_enabled(true).await?;
    info!("Timer started: 10 seconds countdown");

    info!("=== Low-Level API Examples ===");

    // Read control status 1 using low-level API
    let ctrl1 = rtc.ll.control_status_1().read_async().await?;
    info!("Control Status 1 - STOP: {}, TEST1: {}", ctrl1.stop(), ctrl1.test1());

    // Read control status 2 using low-level API
    let ctrl2 = rtc.ll.control_status_2().read_async().await?;
    info!("Control Status 2 - AIE: {}, TIE: {}, AF: {}, TF: {}",
          ctrl2.aie(), ctrl2.tie(), ctrl2.af(), ctrl2.tf());

    // Read seconds register using low-level API
    let seconds = rtc.ll.seconds().read_async().await?;
    info!("Seconds register - VL: {}, value: {}",
          seconds.vl(),
          (seconds.seconds_ten() * 10 + seconds.seconds_unit()) as u8);

    // Modify timer control using low-level API
    rtc.ll.timer_control().modify_async(|w| {
        w.set_te(true);
        w.set_td(TimerFrequency::Freq64Hz);
    }).await?;
    info!("Timer frequency changed to 64 Hz via LL API");

    info!("PCF8563 test complete!");

    Ok(())
}
