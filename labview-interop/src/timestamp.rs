/// Mirrors the internal LabVIEW timestamp structure so
/// it can be passed back and forward.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LVTime(u128);

///The Unix Epoch in LabVIEW epoch seconds for shifting timestamps between them.
pub const UNIX_EPOCH_IN_LV_SECONDS: f64 = 2082844800;

//todo:
// * from/to bytes

impl LVTime {
    /// From a double precision number which is the seconds
    /// since the 1904 epoch used by LabVIEW
    fn from_lv_epoch(seconds: f64) -> Self {
        let (seconds, fractions) = (seconds / 1.0, seconds % 1.0);
        let integer_fractions = (fractions * 0xFFFF_FFFF_FFFF_FFFFu64 as f64) as u64;
        Self::from_parts(seconds as u64, integer_fractions)
    }

    /// Into a double precision number which is the seconds
    /// since the 1904 epoch used by LabVIEW.
    fn to_lv_epoch(&self) -> f64 {
        let (seconds, fractions) = self.to_parts();
        let seconds: f64 = seconds as f64;
        //let fractions: f64 = fractions as f64;
        println!("{fractions}");
        let fractions: f64 = (fractions as f64) / 0xFFFF_FFFF_FFFF_FFFFu64 as f64;
        seconds + fractions
    }

    /// Build from the full seconds and fractional second parts.
    fn from_parts(seconds: u64, fractions: u64) -> Self {
        let time = (seconds as u128) << 64 | (fractions as u128);
        Self(time)
    }

    fn to_parts(&self) -> (u64, u64) {
        let seconds = (self.0 >> 64) as u64;
        let fractions = (self.0 & 0xFFFF_FFFF_FFFF_FFFF) as u64;
        (seconds, fractions)
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
}
