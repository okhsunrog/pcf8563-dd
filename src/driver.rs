use super::{I2c, RegisterInterface, bisync, only_async, only_sync};
use crate::{
    Alarm, ClkoutFrequency, DateTime, PCF8563_I2C_ADDR, Pcf8563Interface, Pcf8563LowLevel,
    RtcError, Time, TimerFrequency, bcd_to_dec, dec_to_bcd,
};

#[bisync]
impl<I2CBus, E> RegisterInterface for Pcf8563Interface<I2CBus>
where
    I2CBus: I2c<Error = E>,
    E: core::fmt::Debug,
{
    type AddressType = u8;
    type Error = RtcError<E>;
    async fn read_register(
        &mut self,
        address: u8,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.i2c_bus
            .write_read(PCF8563_I2C_ADDR, &[address], data)
            .await
            .map_err(RtcError::I2c)
    }
    async fn write_register(
        &mut self,
        address: u8,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        let mut buffer = [0u8; 9]; // Max: address + 8 bytes for datetime
        if (1 + data.len()) > buffer.len() {
            return Err(RtcError::InvalidInputData);
        }
        buffer[0] = address;
        buffer[1..1 + data.len()].copy_from_slice(data);
        self.i2c_bus
            .write(PCF8563_I2C_ADDR, &buffer[..1 + data.len()])
            .await
            .map_err(RtcError::I2c)
    }
}

pub struct Pcf8563<
    I2CImpl: RegisterInterface<AddressType = u8, Error = RtcError<I2CBusErr>>,
    I2CBusErr: core::fmt::Debug,
> {
    pub ll: Pcf8563LowLevel<I2CImpl>,
    _marker: core::marker::PhantomData<I2CBusErr>,
}

impl<I2CBus, E> Pcf8563<Pcf8563Interface<I2CBus>, E>
where
    I2CBus: I2c<Error = E>,
    E: core::fmt::Debug,
{
    pub fn new(i2c: I2CBus) -> Self {
        Self {
            ll: Pcf8563LowLevel::new(Pcf8563Interface::new(i2c)),
            _marker: core::marker::PhantomData,
        }
    }
}

include!("bisync_helpers.rs");

impl<I2CImpl, I2CBusErr> Pcf8563<I2CImpl, I2CBusErr>
where
    I2CImpl: RegisterInterface<AddressType = u8, Error = RtcError<I2CBusErr>>,
    I2CBusErr: core::fmt::Debug,
{
    // =========================================================================
    // Date and Time
    // =========================================================================

    /// Get the current date and time
    ///
    /// Reads all 7 time/date registers in one operation as recommended by datasheet.
    #[bisync]
    pub async fn get_datetime(&mut self) -> Result<DateTime, RtcError<I2CBusErr>> {
        let mut op_sec = self.ll.seconds();
        let seconds_reg = read_internal(&mut op_sec).await?;

        let mut op_min = self.ll.minutes();
        let minutes_reg = read_internal(&mut op_min).await?;

        let mut op_hr = self.ll.hours();
        let hours_reg = read_internal(&mut op_hr).await?;

        let mut op_day = self.ll.days();
        let days_reg = read_internal(&mut op_day).await?;

        let mut op_wd = self.ll.weekdays();
        let weekdays_reg = read_internal(&mut op_wd).await?;

        let mut op_mon = self.ll.century_months();
        let months_reg = read_internal(&mut op_mon).await?;

        let mut op_yr = self.ll.years();
        let years_reg = read_internal(&mut op_yr).await?;

        Ok(DateTime {
            seconds: bcd_to_dec(
                (seconds_reg.seconds_ten() << 4) as u8 | seconds_reg.seconds_unit() as u8,
            ),
            minutes: bcd_to_dec(
                (minutes_reg.minutes_ten() << 4) as u8 | minutes_reg.minutes_unit() as u8,
            ),
            hours: bcd_to_dec((hours_reg.hours_ten() << 4) as u8 | hours_reg.hours_unit() as u8),
            day: bcd_to_dec((days_reg.days_ten() << 4) as u8 | days_reg.days_unit() as u8),
            weekday: weekdays_reg.weekday() as u8,
            month: bcd_to_dec(
                (months_reg.months_ten() << 4) as u8 | months_reg.months_unit() as u8,
            ),
            year: bcd_to_dec((years_reg.years_ten() << 4) as u8 | years_reg.years_unit() as u8),
        })
    }

    /// Set the date and time
    ///
    /// Sets all 7 time/date registers in sequence. Also clears the VL flag.
    #[bisync]
    pub async fn set_datetime(&mut self, dt: &DateTime) -> Result<(), RtcError<I2CBusErr>> {
        // Validate input
        if dt.year > 99
            || dt.month < 1
            || dt.month > 12
            || dt.weekday > 6
            || dt.day < 1
            || dt.day > 31
            || dt.hours > 23
            || dt.minutes > 59
            || dt.seconds > 59
        {
            return Err(RtcError::InvalidInputData);
        }

        let seconds_bcd = dec_to_bcd(dt.seconds);
        let minutes_bcd = dec_to_bcd(dt.minutes);
        let hours_bcd = dec_to_bcd(dt.hours);
        let days_bcd = dec_to_bcd(dt.day);
        let months_bcd = dec_to_bcd(dt.month);
        let years_bcd = dec_to_bcd(dt.year);

        // Write seconds (also clears VL flag)
        let mut op_sec = self.ll.seconds();
        write_internal(&mut op_sec, |r| {
            r.set_vl(false);
            r.set_seconds_ten(seconds_bcd >> 4);
            r.set_seconds_unit(seconds_bcd & 0x0F);
        })
        .await?;

        let mut op_min = self.ll.minutes();
        write_internal(&mut op_min, |r| {
            r.set_minutes_ten(minutes_bcd >> 4);
            r.set_minutes_unit(minutes_bcd & 0x0F);
        })
        .await?;

        let mut op_hr = self.ll.hours();
        write_internal(&mut op_hr, |r| {
            r.set_hours_ten(hours_bcd >> 4);
            r.set_hours_unit(hours_bcd & 0x0F);
        })
        .await?;

        let mut op_day = self.ll.days();
        write_internal(&mut op_day, |r| {
            r.set_days_ten(days_bcd >> 4);
            r.set_days_unit(days_bcd & 0x0F);
        })
        .await?;

        let mut op_wd = self.ll.weekdays();
        write_internal(&mut op_wd, |r| {
            r.set_weekday(dt.weekday);
        })
        .await?;

        let mut op_mon = self.ll.century_months();
        write_internal(&mut op_mon, |r| {
            r.set_months_ten(months_bcd >> 4);
            r.set_months_unit(months_bcd & 0x0F);
        })
        .await?;

        let mut op_yr = self.ll.years();
        write_internal(&mut op_yr, |r| {
            r.set_years_ten(years_bcd >> 4);
            r.set_years_unit(years_bcd & 0x0F);
        })
        .await?;

        Ok(())
    }

    /// Set only the time (hours, minutes, seconds)
    #[bisync]
    pub async fn set_time(&mut self, time: &Time) -> Result<(), RtcError<I2CBusErr>> {
        if time.hours > 23 || time.minutes > 59 || time.seconds > 59 {
            return Err(RtcError::InvalidInputData);
        }

        let seconds_bcd = dec_to_bcd(time.seconds);
        let minutes_bcd = dec_to_bcd(time.minutes);
        let hours_bcd = dec_to_bcd(time.hours);

        let mut op_sec = self.ll.seconds();
        write_internal(&mut op_sec, |r| {
            r.set_vl(false);
            r.set_seconds_ten(seconds_bcd >> 4);
            r.set_seconds_unit(seconds_bcd & 0x0F);
        })
        .await?;

        let mut op_min = self.ll.minutes();
        write_internal(&mut op_min, |r| {
            r.set_minutes_ten(minutes_bcd >> 4);
            r.set_minutes_unit(minutes_bcd & 0x0F);
        })
        .await?;

        let mut op_hr = self.ll.hours();
        write_internal(&mut op_hr, |r| {
            r.set_hours_ten(hours_bcd >> 4);
            r.set_hours_unit(hours_bcd & 0x0F);
        })
        .await?;

        Ok(())
    }

    // =========================================================================
    // Clock Integrity (Voltage Low Detection)
    // =========================================================================

    /// Check if clock integrity is guaranteed
    ///
    /// Returns `false` if the VL (Voltage Low) flag is set, indicating
    /// the clock data may be invalid due to power loss.
    #[bisync]
    pub async fn is_clock_valid(&mut self) -> Result<bool, RtcError<I2CBusErr>> {
        let mut op = self.ll.seconds();
        let reg = read_internal(&mut op).await?;
        Ok(!reg.vl())
    }

    /// Clear the voltage-low flag
    ///
    /// Should be called after setting the time to indicate clock is valid.
    #[bisync]
    pub async fn clear_voltage_low_flag(&mut self) -> Result<(), RtcError<I2CBusErr>> {
        let mut op = self.ll.seconds();
        modify_internal(&mut op, |r| r.set_vl(false)).await
    }

    // =========================================================================
    // Century Flag
    // =========================================================================

    /// Get the century flag
    ///
    /// Returns `true` if century is X+1 (e.g., 2100s), `false` if century is X (e.g., 2000s)
    #[bisync]
    pub async fn get_century_flag(&mut self) -> Result<bool, RtcError<I2CBusErr>> {
        let mut op = self.ll.century_months();
        let reg = read_internal(&mut op).await?;
        Ok(reg.century())
    }

    /// Set the century flag
    #[bisync]
    pub async fn set_century_flag(&mut self, century: bool) -> Result<(), RtcError<I2CBusErr>> {
        let mut op = self.ll.century_months();
        modify_internal(&mut op, |r| r.set_century(century)).await
    }

    // =========================================================================
    // Clock Control
    // =========================================================================

    /// Start or stop the RTC clock
    #[bisync]
    pub async fn set_clock_running(&mut self, running: bool) -> Result<(), RtcError<I2CBusErr>> {
        let mut op = self.ll.control_status_1();
        modify_internal(&mut op, |r| r.set_stop(!running)).await
    }

    /// Check if the RTC clock is running
    #[bisync]
    pub async fn is_clock_running(&mut self) -> Result<bool, RtcError<I2CBusErr>> {
        let mut op = self.ll.control_status_1();
        let reg = read_internal(&mut op).await?;
        Ok(!reg.stop())
    }

    // =========================================================================
    // Alarm
    // =========================================================================

    /// Get the current alarm configuration
    #[bisync]
    pub async fn get_alarm(&mut self) -> Result<Alarm, RtcError<I2CBusErr>> {
        let mut op_min = self.ll.minute_alarm();
        let min_reg = read_internal(&mut op_min).await?;

        let mut op_hr = self.ll.hour_alarm();
        let hr_reg = read_internal(&mut op_hr).await?;

        let mut op_day = self.ll.day_alarm();
        let day_reg = read_internal(&mut op_day).await?;

        let mut op_wd = self.ll.weekday_alarm();
        let wd_reg = read_internal(&mut op_wd).await?;

        Ok(Alarm {
            minute: if min_reg.ae_m() {
                None
            } else {
                Some(bcd_to_dec(
                    (min_reg.minute_alarm_ten() << 4) as u8 | min_reg.minute_alarm_unit() as u8,
                ))
            },
            hour: if hr_reg.ae_h() {
                None
            } else {
                Some(bcd_to_dec(
                    (hr_reg.hour_alarm_ten() << 4) as u8 | hr_reg.hour_alarm_unit() as u8,
                ))
            },
            day: if day_reg.ae_d() {
                None
            } else {
                Some(bcd_to_dec(
                    (day_reg.day_alarm_ten() << 4) as u8 | day_reg.day_alarm_unit() as u8,
                ))
            },
            weekday: if wd_reg.ae_w() {
                None
            } else {
                Some(wd_reg.weekday_alarm() as u8)
            },
        })
    }

    /// Set the alarm configuration
    ///
    /// Set a field to `Some(value)` to enable that alarm component,
    /// or `None` to disable it.
    #[bisync]
    pub async fn set_alarm(&mut self, alarm: &Alarm) -> Result<(), RtcError<I2CBusErr>> {
        // Minute alarm
        let mut op_min = self.ll.minute_alarm();
        write_internal(&mut op_min, |r| {
            if let Some(min) = alarm.minute {
                let bcd = dec_to_bcd(min);
                r.set_ae_m(false); // Enable
                r.set_minute_alarm_ten(bcd >> 4);
                r.set_minute_alarm_unit(bcd & 0x0F);
            } else {
                r.set_ae_m(true); // Disable
            }
        })
        .await?;

        // Hour alarm
        let mut op_hr = self.ll.hour_alarm();
        write_internal(&mut op_hr, |r| {
            if let Some(hr) = alarm.hour {
                let bcd = dec_to_bcd(hr);
                r.set_ae_h(false); // Enable
                r.set_hour_alarm_ten(bcd >> 4);
                r.set_hour_alarm_unit(bcd & 0x0F);
            } else {
                r.set_ae_h(true); // Disable
            }
        })
        .await?;

        // Day alarm
        let mut op_day = self.ll.day_alarm();
        write_internal(&mut op_day, |r| {
            if let Some(day) = alarm.day {
                let bcd = dec_to_bcd(day);
                r.set_ae_d(false); // Enable
                r.set_day_alarm_ten(bcd >> 4);
                r.set_day_alarm_unit(bcd & 0x0F);
            } else {
                r.set_ae_d(true); // Disable
            }
        })
        .await?;

        // Weekday alarm
        let mut op_wd = self.ll.weekday_alarm();
        write_internal(&mut op_wd, |r| {
            if let Some(wd) = alarm.weekday {
                r.set_ae_w(false); // Enable
                r.set_weekday_alarm(wd);
            } else {
                r.set_ae_w(true); // Disable
            }
        })
        .await?;

        Ok(())
    }

    /// Disable all alarm components
    #[bisync]
    pub async fn disable_alarm(&mut self) -> Result<(), RtcError<I2CBusErr>> {
        self.set_alarm(&Alarm::default()).await
    }

    /// Check if alarm flag is set (alarm has triggered)
    #[bisync]
    pub async fn get_alarm_flag(&mut self) -> Result<bool, RtcError<I2CBusErr>> {
        let mut op = self.ll.control_status_2();
        let reg = read_internal(&mut op).await?;
        Ok(reg.af())
    }

    /// Clear the alarm flag
    #[bisync]
    pub async fn clear_alarm_flag(&mut self) -> Result<(), RtcError<I2CBusErr>> {
        let mut op = self.ll.control_status_2();
        modify_internal(&mut op, |r| r.set_af(false)).await
    }

    /// Enable or disable alarm interrupt
    #[bisync]
    pub async fn set_alarm_interrupt(&mut self, enable: bool) -> Result<(), RtcError<I2CBusErr>> {
        let mut op = self.ll.control_status_2();
        modify_internal(&mut op, |r| r.set_aie(enable)).await
    }

    /// Check if alarm interrupt is enabled
    #[bisync]
    pub async fn is_alarm_interrupt_enabled(&mut self) -> Result<bool, RtcError<I2CBusErr>> {
        let mut op = self.ll.control_status_2();
        let reg = read_internal(&mut op).await?;
        Ok(reg.aie())
    }

    // =========================================================================
    // Timer
    // =========================================================================

    /// Set the timer countdown value (0-255)
    #[bisync]
    pub async fn set_timer_value(&mut self, value: u8) -> Result<(), RtcError<I2CBusErr>> {
        let mut op = self.ll.timer();
        write_internal(&mut op, |r| r.set_timer_value(value)).await
    }

    /// Get the current timer countdown value
    #[bisync]
    pub async fn get_timer_value(&mut self) -> Result<u8, RtcError<I2CBusErr>> {
        let mut op = self.ll.timer();
        let reg = read_internal(&mut op).await?;
        Ok(reg.timer_value() as u8)
    }

    /// Set the timer source clock frequency
    #[bisync]
    pub async fn set_timer_frequency(
        &mut self,
        freq: TimerFrequency,
    ) -> Result<(), RtcError<I2CBusErr>> {
        let mut op = self.ll.timer_control();
        modify_internal(&mut op, |r| r.set_td(freq)).await
    }

    /// Get the timer source clock frequency
    #[bisync]
    pub async fn get_timer_frequency(&mut self) -> Result<TimerFrequency, RtcError<I2CBusErr>> {
        let mut op = self.ll.timer_control();
        let reg = read_internal(&mut op).await?;
        Ok(reg.td())
    }

    /// Enable or disable the timer
    #[bisync]
    pub async fn set_timer_enabled(&mut self, enable: bool) -> Result<(), RtcError<I2CBusErr>> {
        let mut op = self.ll.timer_control();
        modify_internal(&mut op, |r| r.set_te(enable)).await
    }

    /// Check if timer is enabled
    #[bisync]
    pub async fn is_timer_enabled(&mut self) -> Result<bool, RtcError<I2CBusErr>> {
        let mut op = self.ll.timer_control();
        let reg = read_internal(&mut op).await?;
        Ok(reg.te())
    }

    /// Check if timer flag is set (timer has triggered)
    #[bisync]
    pub async fn get_timer_flag(&mut self) -> Result<bool, RtcError<I2CBusErr>> {
        let mut op = self.ll.control_status_2();
        let reg = read_internal(&mut op).await?;
        Ok(reg.tf())
    }

    /// Clear the timer flag
    #[bisync]
    pub async fn clear_timer_flag(&mut self) -> Result<(), RtcError<I2CBusErr>> {
        let mut op = self.ll.control_status_2();
        modify_internal(&mut op, |r| r.set_tf(false)).await
    }

    /// Enable or disable timer interrupt
    #[bisync]
    pub async fn set_timer_interrupt(&mut self, enable: bool) -> Result<(), RtcError<I2CBusErr>> {
        let mut op = self.ll.control_status_2();
        modify_internal(&mut op, |r| r.set_tie(enable)).await
    }

    /// Check if timer interrupt is enabled
    #[bisync]
    pub async fn is_timer_interrupt_enabled(&mut self) -> Result<bool, RtcError<I2CBusErr>> {
        let mut op = self.ll.control_status_2();
        let reg = read_internal(&mut op).await?;
        Ok(reg.tie())
    }

    /// Set timer interrupt mode (level or pulse)
    #[bisync]
    pub async fn set_timer_interrupt_pulse_mode(
        &mut self,
        pulse: bool,
    ) -> Result<(), RtcError<I2CBusErr>> {
        let mut op = self.ll.control_status_2();
        modify_internal(&mut op, |r| r.set_ti_tp(pulse)).await
    }

    // =========================================================================
    // Clock Output
    // =========================================================================

    /// Enable or disable the CLKOUT output
    #[bisync]
    pub async fn set_clkout_enabled(&mut self, enable: bool) -> Result<(), RtcError<I2CBusErr>> {
        let mut op = self.ll.clkout_control();
        modify_internal(&mut op, |r| r.set_fe(enable)).await
    }

    /// Check if CLKOUT is enabled
    #[bisync]
    pub async fn is_clkout_enabled(&mut self) -> Result<bool, RtcError<I2CBusErr>> {
        let mut op = self.ll.clkout_control();
        let reg = read_internal(&mut op).await?;
        Ok(reg.fe())
    }

    /// Set the CLKOUT frequency
    #[bisync]
    pub async fn set_clkout_frequency(
        &mut self,
        freq: ClkoutFrequency,
    ) -> Result<(), RtcError<I2CBusErr>> {
        let mut op = self.ll.clkout_control();
        modify_internal(&mut op, |r| r.set_fd(freq)).await
    }

    /// Get the CLKOUT frequency setting
    #[bisync]
    pub async fn get_clkout_frequency(&mut self) -> Result<ClkoutFrequency, RtcError<I2CBusErr>> {
        let mut op = self.ll.clkout_control();
        let reg = read_internal(&mut op).await?;
        Ok(reg.fd())
    }

    // =========================================================================
    // Initialization
    // =========================================================================

    /// Initialize the RTC with default settings
    ///
    /// - Clears all control bits
    /// - Clears voltage-low flag
    /// - Disables all alarms
    /// - Sets timer to lowest frequency (1/60 Hz) for power saving
    #[bisync]
    pub async fn init(&mut self) -> Result<(), RtcError<I2CBusErr>> {
        // Clear control status 1
        let mut op1 = self.ll.control_status_1();
        write_internal(&mut op1, |r| {
            r.set_test1(false);
            r.set_stop(false);
            r.set_testc(false);
        })
        .await?;

        // Clear control status 2
        let mut op2 = self.ll.control_status_2();
        write_internal(&mut op2, |r| {
            r.set_ti_tp(false);
            r.set_af(false);
            r.set_tf(false);
            r.set_aie(false);
            r.set_tie(false);
        })
        .await?;

        // Clear voltage-low flag
        self.clear_voltage_low_flag().await?;

        // Disable all alarms
        self.disable_alarm().await?;

        // Set timer to lowest frequency for power saving
        self.set_timer_frequency(TimerFrequency::Freq160Hz).await?;

        Ok(())
    }
}
