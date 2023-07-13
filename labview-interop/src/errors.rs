#[repr(transparent)]
pub struct MgErr(i32);

pub type Result<T> = std::result::Result<T, MgErr>;

impl MgErr {
    pub const NO_ERROR: MgErr = MgErr(0);
    pub fn to_result<T>(self, success_value: T) -> Result<T> {
        if self.0 != 0 {
            Err(self)
        } else {
            Ok(success_value)
        }
    }
}
