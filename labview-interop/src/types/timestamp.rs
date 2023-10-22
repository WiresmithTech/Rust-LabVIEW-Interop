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

// The LV Type consists really of a (i64, u64) but
// the effective storage type is u128 so I've kept
// it there for now.

/// Mirrors the internal LabVIEW timestamp structure so
/// it can be passed back and forward.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LVTime(u128);

///The Unix Epoch in LabVIEW epoch seconds for shifting timestamps between them.
pub const UNIX_EPOCH_IN_LV_SECONDS: f64 = 2082844800.0;

impl LVTime {
    /// Extract the sub-second component as a floating point number.
    pub fn sub_seconds(&self) -> f64 {
        let fractional = self.to_parts().1;
        (fractional as f64) / 0xFFFF_FFFF_FFFF_FFFFu64 as f64
    }

    ///Extract the seconds component which is referenced to the LabVIEW epoc.
    pub const fn seconds(&self) -> i64 {
        (self.0 >> 64) as i64
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
        lv_epoch - UNIX_EPOCH_IN_LV_SECONDS
    }

    /// To a double precision number which is the seconds since unix epoch.
    pub fn from_unix_epoch(seconds: f64) -> Self {
        let lv_epoch = seconds + UNIX_EPOCH_IN_LV_SECONDS;
        Self::from_lv_epoch(lv_epoch)
    }

    /// Build from the full seconds and fractional second parts.
    pub const fn from_parts(seconds: i64, fractions: u64) -> Self {
        let time = (seconds as u128) << 64 | (fractions as u128);
        Self(time)
    }

    /// Seperate out the u64 components.
    #[inline]
    pub const fn to_parts(&self) -> (i64, u64) {
        let fractions = (self.0 & 0xFFFF_FFFF_FFFF_FFFF) as u64;
        (self.seconds(), fractions)
    }

    /// Load from u128 which is the storage format
    #[inline]
    const fn from_u128(repr: u128) -> Self {
        Self(repr)
    }

    #[inline]
    const fn as_u128(&self) -> &u128 {
        &self.0
    }

    /// To little endian bytes.
    pub const fn to_le_bytes(&self) -> [u8; 16] {
        self.as_u128().to_le_bytes()
    }

    /// To big endian bytes.
    pub const fn to_be_bytes(&self) -> [u8; 16] {
        self.as_u128().to_be_bytes()
    }

    /// From little endian bytes.
    pub const fn from_le_bytes(bytes: [u8; 16]) -> Self {
        Self::from_u128(u128::from_le_bytes(bytes))
    }

    /// From big endian bytes.
    pub const fn from_be_bytes(bytes: [u8; 16]) -> Self {
        Self::from_u128(u128::from_be_bytes(bytes))
    }
}

#[cfg(feature = "chrono")]
mod chrono {

    use super::*;
    use ::chrono::{DateTime, NaiveDateTime, Utc};

    impl TryFrom<LVTime> for DateTime<Utc> {
        type Error = LVTimeError;

        fn try_from(value: LVTime) -> Result<Self, Self::Error> {
            let naive_time: NaiveDateTime = value.try_into()?;
            Ok(DateTime::<Utc>::from_utc(naive_time, Utc))
        }
    }

    impl TryFrom<LVTime> for NaiveDateTime {
        type Error = LVTimeError;

        fn try_from(value: LVTime) -> Result<Self, Self::Error> {
            let seconds_for_time: i64 = value.seconds() as i64 - UNIX_EPOCH_IN_LV_SECONDS as i64;
            let nanoseconds = value.sub_seconds() * 1_000_000_000f64;
            Self::from_timestamp_opt(seconds_for_time, nanoseconds as u32)
                .ok_or(LVTimeError::ChronoOutOfRange)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_from_parts() {
        let time = LVTime::from_parts(20, 0x8000_0000_0000_0000);
        assert_eq!(time.0, 0x14_8000_0000_0000_0000);
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
        assert_eq!(time.0, 0x14_8000_0000_0000_0000);
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
        assert_eq!(time.0, 0x14_8000_0000_0000_0000);
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

    use super::{LVTime, UNIX_EPOCH_IN_LV_SECONDS};
    use chrono::NaiveDateTime;
    use chrono::{DateTime, Utc};

    #[test]
    fn datetime_from_lv_time() {
        let date_time: DateTime<Utc> = LVTime::from_lv_epoch(3758974472.02440977f64)
            .try_into()
            .unwrap();
        let naive: NaiveDateTime = LVTime::from_lv_epoch(3758974472.02440977f64)
            .try_into()
            .unwrap();
        let expected_naive = NaiveDateTime::from_timestamp_opt(
            3758974472 - UNIX_EPOCH_IN_LV_SECONDS as i64,
            024409770,
        )
        .unwrap();
        let expected = DateTime::<Utc>::from_utc(expected_naive, Utc);
        assert_eq!(date_time, expected);
        assert_eq!(naive, expected_naive)
    }
}
