use std::ffi::CStr;
use std::ffi::CString;
use libc::{c_char, c_void};

use bindings::avahi::*;

pub struct AvahiUtils;

impl AvahiUtils {
    pub fn to_c_string(r_string: String) -> CString {
        CString::new(r_string).unwrap()
    }

    pub fn to_owned_string(c_string: *const c_char) -> Option<String> {
        if c_string.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(c_string) }.to_string_lossy().into_owned())
        }
    }

    pub fn parse_address(address: *const AvahiAddress) -> Option<String> {
        if address.is_null() {
            None
        } else {
            let address_vector = Vec::with_capacity(AVAHI_ADDRESS_STR_MAX);
            unsafe { avahi_address_snprint(address_vector.as_ptr(), AVAHI_ADDRESS_STR_MAX, address) };

            AvahiUtils::to_owned_string(address_vector.as_ptr())
        }
    }

    pub fn parse_txt(txt: *mut AvahiStringList) -> Option<String> {
        if txt.is_null() {
            None
        } else {
            unsafe {
                let txt_pointer = avahi_string_list_to_string(txt);
                let txt = AvahiUtils::to_owned_string(txt_pointer);
                avahi_free(txt_pointer as *mut c_void);

                txt
            }
        }
    }
}
