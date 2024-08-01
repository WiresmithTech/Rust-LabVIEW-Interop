use labview_interop::{
    errors::LVStatusCode,
    labview_layout,
    memory::UPtr,
    sync::{LVUserEvent, Occurence},
    types::LStrOwned,
};

#[no_mangle]
pub extern "C" fn generate_event_3(lv_user_event: *mut LVUserEvent<i32>) -> LVStatusCode {
    let event = unsafe { *lv_user_event };
    let result = event.post(&mut 3);
    result.into()
}

labview_layout!(
    pub struct UserEventClusterOwned {
        eventno: i32,
        id: LStrOwned,
    }
);

#[no_mangle]
pub extern "C" fn generate_event_cluster(
    lv_user_event: UPtr<LVUserEvent<UserEventClusterOwned>>,
) -> LVStatusCode {
    let mystr = LStrOwned::from_data(b"Hello World!").unwrap();
    let mut eventdata = UserEventClusterOwned {
        eventno: 2,
        id: mystr,
    };
    let result = lv_user_event.post(&mut eventdata);

    result.into()
}

#[no_mangle]
pub extern "C" fn generate_occurence(occurence: *mut Occurence) -> LVStatusCode {
    let result = unsafe { (*occurence).set() };
    result.into()
}
