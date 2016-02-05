use super::types::*;
use super::enums::*;
use libc::{c_void, c_int, c_char};

#[link(name = "avahi-common")]
#[link(name = "avahi-client")]
#[allow(improper_ctypes)]
extern {
    pub fn avahi_simple_poll_new() -> *mut AvahiSimplePoll;
    
    pub fn avahi_simple_poll_get(s: *mut AvahiSimplePoll) -> *mut AvahiPoll;
    
    pub fn avahi_client_new(
        poll_api: *const AvahiPoll,
        flags: AvahiClientFlags,
        callback: extern fn(*mut AvahiClient, AvahiClientState, *mut c_void),
        userdata: *mut c_void,
        error: *mut c_int
    ) -> *mut AvahiClient;
                      
    pub fn avahi_service_browser_new(
        client: *mut AvahiClient,
        interface: c_int,
        protocol: c_int,
        le_type: *const c_char,
        domain: *const c_char,
        flags: AvahiLookupFlags,
        callback: ServiceBrowserCallback,
        userdata: *mut c_void
    ) -> *mut AvahiServiceBrowser;
        
    pub fn avahi_simple_poll_loop(s: *mut AvahiSimplePoll) -> c_int;
}