#[repr(transparent)]
pub struct LVBool(u8);

pub const LV_FALSE: LVBool = LVBool(0);
pub const LV_TRUE: LVBool = LVBool(1);

#[repr(transparent)]
pub struct Bool32(i32);

pub const FALSE: Bool32 = Bool32(0);
pub const TRUE: Bool32 = Bool32(1);
