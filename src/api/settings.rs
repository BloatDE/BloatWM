use crate::*;

use std::os::raw::*;

use ctor::ctor;

static mut SETTINGS: Option<Settings> = None;

#[ctor]
unsafe fn ctor() {
    SETTINGS = Some(Default::default());
}

#[no_mangle]
unsafe extern "C" fn settings_get_bar_on_top_by_default() -> bool {
    SETTINGS.unwrap().bar_on_top_by_default()
}

#[no_mangle]
unsafe extern "C" fn settings_set_bar_on_top_by_default(value: bool) {
    SETTINGS.unwrap().bar_on_top_by_default_set(value);
}

#[no_mangle]
unsafe extern "C" fn settings_get_border_width() -> c_int {
    SETTINGS.unwrap().border_width()
}

#[no_mangle]
unsafe extern "C" fn settings_set_border_width(value: c_int) {
    SETTINGS.unwrap().border_width_set(value);
}

#[no_mangle]
unsafe extern "C" fn settings_get_default_clients_in_master() -> c_int {
    SETTINGS.unwrap().default_clients_in_master()
}

#[no_mangle]
unsafe extern "C" fn settings_set_default_clients_in_master(value: c_int) {
    SETTINGS.unwrap().default_clients_in_master_set(value);
}

#[no_mangle]
unsafe extern "C" fn settings_get_default_master_area_factor() -> c_float {
    SETTINGS.unwrap().default_master_area_factor()
}

#[no_mangle]
unsafe extern "C" fn settings_set_default_master_area_factor(value: c_float) {
    SETTINGS.unwrap().default_master_area_factor_set(value);
}
