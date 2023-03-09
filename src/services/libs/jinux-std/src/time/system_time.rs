use core::time::Duration;

use crate::prelude::*;
use jinux_frame::time::get_real_time;
use time::{Date, Month, PrimitiveDateTime, Time};

/// This struct corresponds to `SystemTime` in Rust std.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SystemTime(PrimitiveDateTime);

impl SystemTime {
    /// The unix epoch, which represents 1970-01-01 00:00:00
    pub const UNIX_EPOCH: SystemTime = SystemTime::unix_epoch();

    const fn unix_epoch() -> Self {
        // 1970-01-01 00:00:00
        let date = Date::__from_ordinal_date_unchecked(1970, 1);
        let time = Time::__from_hms_nanos_unchecked(0, 0, 0, 0);
        SystemTime(PrimitiveDateTime::new(date, time))
    }

    /// Returns the current system time
    pub fn now() -> Self {
        let system_time = get_real_time();
        // The get real time result should always be valid
        convert_system_time(system_time).unwrap()
    }

    /// Add a duration to self. If the result does not exceed inner bounds return Some(t), else return None.
    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        let duration = convert_to_time_duration(duration);
        self.0.checked_add(duration).map(|inner| SystemTime(inner))
    }

    /// Substract a duration from self. If the result does not exceed inner bounds return Some(t), else return None.
    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        let duration = convert_to_time_duration(duration);
        self.0.checked_sub(duration).map(|inner| SystemTime(inner))
    }

    /// Returns the duration since an earlier time. Return error if `earlier` is later than self.
    pub fn duration_since(&self, earlier: &SystemTime) -> Result<Duration> {
        if self.0 < earlier.0 {
            return_errno_with_message!(
                Errno::EINVAL,
                "duration_since can only accept an earlier time"
            );
        }
        let duration = self.0 - earlier.0;
        Ok(convert_to_core_duration(duration))
    }

    /// Return the difference between current time and the time when self was created.
    /// Return Error if current time is earlier than creating time.
    /// The error can happen if self was created by checked_add.
    pub fn elapsed(&self) -> Result<Duration> {
        let now = SystemTime::now();
        now.duration_since(self)
    }
}

/// convert jinux_frame::time::Time to System time
fn convert_system_time(system_time: jinux_frame::time::Time) -> Result<SystemTime> {
    let month = match Month::try_from(system_time.month) {
        Ok(month) => month,
        Err(_) => return_errno_with_message!(Errno::EINVAL, "unknown month in system time"),
    };
    let date = match Date::from_calendar_date(system_time.year as _, month, system_time.day) {
        Ok(date) => date,
        Err(_) => return_errno_with_message!(Errno::EINVAL, "Invalid system date"),
    };
    let time_ = match Time::from_hms(system_time.hour, system_time.minute, system_time.second) {
        Ok(time_) => time_,
        Err(_) => return_errno_with_message!(Errno::EINVAL, "Invalid system time"),
    };
    Ok(SystemTime(PrimitiveDateTime::new(date, time_)))
}

/// FIXME: need to further check precision loss
/// convert core::time::Duration to time::Duration
const fn convert_to_time_duration(duration: Duration) -> time::Duration {
    let seconds = duration.as_secs() as i64;
    let nanoseconds = duration.subsec_nanos() as i32;
    time::Duration::new(seconds, nanoseconds)
}

/// FIXME: need to further check precision loss
/// convert time::Duration to core::time::Duration
const fn convert_to_core_duration(duration: time::Duration) -> Duration {
    let seconds = duration.whole_seconds() as u64;
    let nanoseconds = duration.subsec_nanoseconds() as u32;
    Duration::new(seconds, nanoseconds)
}