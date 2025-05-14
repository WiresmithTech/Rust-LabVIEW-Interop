//! Support for exchanging date and times. with LabVIEWs timestamp format.
//!
//! This includes binary formats, to and from 1904 epoch, unix (1970) epoch
//! and optionally chrono DateTime with the `chrono` feature.
//!

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LVTimeError {
    #[error("Cannot generate a chrono time as it is out of range.")]
    ChronoOutOfRange,
}

/// Mirrors the internal LabVIEW timestamp structure so
/// it can be passed back and forward.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LVTime {
    fractions: u64,
    seconds: i64,
}

///The Unix Epoch in LabVIEW epoch seconds for shifting timestamps between them.
///
/// This is the [`i64`] value. See also [`UNIX_EPOCH_IN_LV_SECONDS_F64`].
pub const UNIX_EPOCH_IN_LV_SECONDS_I64: i64 = 2082844800;

///The Unix Epoch in LabVIEW epoch seconds for shifting timestamps between them.
///
/// This is the [`f64`] value. See also [`UNIX_EPOCH_IN_LV_SECONDS_I64`].
pub const UNIX_EPOCH_IN_LV_SECONDS_F64: f64 = UNIX_EPOCH_IN_LV_SECONDS_I64 as f64;

impl LVTime {
    /// Extract the sub-second component as a floating point number.
    pub fn sub_seconds(&self) -> f64 {
        let fractional = self.to_parts().1;
        (fractional as f64) / 0xFFFF_FFFF_FFFF_FFFFu64 as f64
    }

    ///Extract the seconds component which is referenced to the LabVIEW epoch.
    #[inline]
    pub const fn seconds(&self) -> i64 {
        self.seconds
    }

    /// From a double precision number which is the seconds
    /// since the 1904 epoch used by LabVIEW
    pub fn from_lv_epoch(seconds: f64) -> Self {
        let (seconds, fractions) = (seconds / 1.0, seconds % 1.0);
        let integer_fractions = (fractions * 0xFFFF_FFFF_FFFF_FFFFu64 as f64) as u64;
        Self::from_parts(seconds as i64, integer_fractions)
    }

    /// Into a double precision number which is the seconds
    /// since the 1904 epoch used by LabVIEW.
    pub fn to_lv_epoch(&self) -> f64 {
        self.seconds() as f64 + self.sub_seconds()
    }

    /// To a double precision number which is the seconds since unix epoch.
    pub fn to_unix_epoch(&self) -> f64 {
        let lv_epoch = self.to_lv_epoch();
        lv_epoch - UNIX_EPOCH_IN_LV_SECONDS_F64
    }

    /// To a double precision number which is the seconds since unix epoch.
    pub fn from_unix_epoch(seconds: f64) -> Self {
        let lv_epoch = seconds + UNIX_EPOCH_IN_LV_SECONDS_F64;
        Self::from_lv_epoch(lv_epoch)
    }

    /// Build from the full seconds and fractional second parts.
    pub const fn from_parts(seconds: i64, fractions: u64) -> Self {
        Self {
            seconds,
            fractions,       
        }
    }

    /// Seperate out the u64 components.
    #[inline]
    pub const fn to_parts(&self) -> (i64, u64) {
        (self.seconds, self.fractions)
    }


    /// To little endian bytes.
    pub const fn to_le_bytes(&self) -> [u8; 16] {
        // Note the reversal here so it is like a u128.
        let littlest = self.fractions.to_le_bytes();
        let biggest = self.seconds.to_le_bytes();
        [
            littlest[0], littlest[1], littlest[2], littlest[3], littlest[4], littlest[5], littlest[6], littlest[7],
            biggest[0], biggest[1], biggest[2], biggest[3], biggest[4], biggest[5], biggest[6], biggest[7],
        ]
    }

    /// To big endian bytes.
    pub const fn to_be_bytes(&self) -> [u8; 16] {
        let biggest = self.seconds.to_be_bytes();
        let littlest = self.fractions.to_be_bytes();
        [
            biggest[0], biggest[1], biggest[2], biggest[3], biggest[4], biggest[5], biggest[6], biggest[7],
            littlest[0], littlest[1], littlest[2], littlest[3], littlest[4], littlest[5], littlest[6], littlest[7],
        ]
    }

    /// From little endian bytes.
    pub const fn from_le_bytes(bytes: [u8; 16]) -> Self {
        // Ugly but keeps this const compatible.
        let littlest = [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]];
        let biggest = [bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]];
        let fraction = u64::from_le_bytes(littlest);
        let seconds = i64::from_le_bytes(biggest);
        Self::from_parts(seconds, fraction)
    }

    /// From big endian bytes.
    pub const fn from_be_bytes(bytes: [u8; 16]) -> Self {
        let biggest = [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]];
        let littlest = [bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]];
        let fractions = u64::from_be_bytes(littlest);
        let seconds = i64::from_be_bytes(biggest);
        Self::from_parts(seconds, fractions)
    }
}

#[cfg(feature = "chrono")]
mod chrono {

    use super::*;
    use ::chrono::{DateTime, Utc};

    /// Get the chrono time from the LabVIEW time as a UTC value.
    ///
    /// From here you can convert to a specific timezone or naive values.
    impl TryFrom<&LVTime> for DateTime<Utc> {
        type Error = LVTimeError;

        fn try_from(value: &LVTime) -> Result<Self, Self::Error> {
            let seconds_for_time: i64 = value.seconds() - UNIX_EPOCH_IN_LV_SECONDS_I64;
            let nanoseconds = value.sub_seconds() * 1_000_000_000f64;
            Self::from_timestamp(seconds_for_time, nanoseconds as u32)
                .ok_or(LVTimeError::ChronoOutOfRange)
        }
    }
    
    /// Implementation for owned types as well. Probably rarer but kept for backwards
    /// compatability.
    impl TryFrom<LVTime> for DateTime<Utc> {
        type Error = LVTimeError;

        fn try_from(value: LVTime) -> Result<Self, Self::Error> {
            value.try_into() 
        }
    }

    /// Allow conversion from a chrono time to a LabVIEW time.
    impl From<DateTime<Utc>> for LVTime {
        fn from(value: DateTime<Utc>) -> Self {
            let seconds = value.timestamp();
            let nanoseconds = value.timestamp_subsec_nanos();
            let fractional = (nanoseconds as f64) / 1_000_000_000f64;
            Self::from_unix_epoch(seconds as f64 + fractional)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_from_parts() {
        let time = LVTime::from_parts(20, 0x8000_0000_0000_0000);
        assert_eq!((20, 0x8000_0000_0000_0000), time.to_parts());
    }

    #[test]
    fn test_to_from_lv_epoch_seconds() {
        let time = LVTime::from_parts(20, 0x8000_0000_0000_0000);
        assert_eq!(20.5f64, time.to_lv_epoch());
        assert_eq!(time, LVTime::from_lv_epoch(20.5f64));
    }

    #[test]
    fn test_to_from_unix_epoch() {
        let time = LVTime::from_parts(3758974472, 0x8000_0000_0000_0000);
        assert_eq!(1676129672.5f64, time.to_unix_epoch());
        assert_eq!(time, LVTime::from_unix_epoch(1676129672.5f64));
    }

    #[test]
    fn test_to_from_le_bytes() {
        let time = LVTime::from_parts(20, 0x8000_0000_0000_0000);
        let bytes = time.to_le_bytes();
        assert_eq!(
            bytes,
            [00, 00, 00, 00, 00, 00, 00, 0x80, 0x14, 00, 00, 00, 00, 00, 00, 00]
        );
        assert_eq!(time, LVTime::from_le_bytes(bytes));
    }

    #[test]
    fn test_to_from_be_bytes() {
        let time = LVTime::from_parts(20, 0x8000_0000_0000_0000);
        let bytes = time.to_be_bytes();
        assert_eq!(
            bytes,
            [00, 00, 00, 00, 00, 00, 00, 0x14, 0x80, 00, 00, 00, 00, 00, 00, 00]
        );
        assert_eq!(time, LVTime::from_be_bytes(bytes));
    }
}

#[cfg(test)]
#[cfg(feature = "chrono")]
mod chrono_tests {

    use super::{LVTime, UNIX_EPOCH_IN_LV_SECONDS_I64};
    use chrono::{DateTime, Utc};

    #[test]
    fn datetime_from_lv_time() {
        let date_time: DateTime<Utc> = LVTime::from_lv_epoch(3758974472.02440977f64)
            .try_into()
            .unwrap();
        let expected =
            DateTime::from_timestamp(3758974472 - UNIX_EPOCH_IN_LV_SECONDS_I64, 024409770).unwrap();
        assert_eq!(date_time, expected);
    }

    #[test]
    fn lv_time_from_datetime() {
        let lv_time = LVTime::from_lv_epoch(3758974472.02440977f64);
        let date_time: DateTime<Utc> = lv_time.try_into().unwrap();
        let lv_time_round_trip = date_time.into();
        assert_eq!(lv_time, lv_time_round_trip);
    }
}
