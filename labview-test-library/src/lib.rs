use labview_interop::types::LVTime;

#[no_mangle]
pub extern "C" fn timestamp_to_epoch(timestamp: *const LVTime) -> f64 {
    unsafe { (*timestamp).to_lv_epoch() }
}

#[no_mangle]
pub extern "C" fn timestamp_from_epoch(seconds_since_epoch: f64, timestamp: *mut LVTime) {
    let timestamp = unsafe { timestamp.as_mut().unwrap() };
    *timestamp = LVTime::from_lv_epoch(seconds_since_epoch);
}

#[no_mangle]
pub extern "C" fn timestamp_from_le_bytes(bytes: *const u8, timestamp: *mut LVTime) {
    // Safety: for this simple test we can assume we have the right size bytes.
    let mut buf = [0u8; 16];
    let byte_slice = unsafe { std::slice::from_raw_parts(bytes, 16) };
    buf.copy_from_slice(byte_slice);
    unsafe {
        *timestamp = LVTime::from_le_bytes(buf);
    }
}

#[no_mangle]
pub extern "C" fn timestamp_from_be_bytes(bytes: *const u8, timestamp: *mut LVTime) {
    // Safety: for this simple test we can assume we have the right size bytes.
    let mut buf = [0u8; 16];
    let byte_slice = unsafe { std::slice::from_raw_parts(bytes, 16) };
    buf.copy_from_slice(byte_slice);
    unsafe {
        *timestamp = LVTime::from_be_bytes(buf);
    }
}
