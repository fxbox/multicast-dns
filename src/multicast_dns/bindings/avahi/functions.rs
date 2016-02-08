use super::types::*;
use super::enums::*;
use libc::{c_void, c_int, c_char, size_t};

#[link(name = "avahi-common")]
#[link(name = "avahi-client")]
#[allow(improper_ctypes)]
extern "C" {
    pub fn avahi_simple_poll_new() -> *mut AvahiSimplePoll;

    pub fn avahi_simple_poll_get(s: *mut AvahiSimplePoll) -> *mut AvahiPoll;

    pub fn avahi_client_new(poll_api: *const AvahiPoll,
                            flags: AvahiClientFlags,
                            callback: extern "C" fn(*mut AvahiClient,
                                                    AvahiClientState,
                                                    *mut c_void)
                                                   ,
                            userdata: *mut c_void,
                            error: *mut c_int)
                            -> *mut AvahiClient;

    pub fn avahi_service_browser_new(client: *mut AvahiClient,
                                     interface: c_int,
                                     protocol: c_int,
                                     le_type: *const c_char,
                                     domain: *const c_char,
                                     flags: AvahiLookupFlags,
                                     callback: ServiceBrowserCallback,
                                     userdata: *mut c_void)
                                     -> *mut AvahiServiceBrowser;

    pub fn avahi_simple_poll_loop(s: *mut AvahiSimplePoll) -> c_int;

    pub fn avahi_service_resolver_new(client: *mut AvahiClient,
                                      interface: c_int,
                                      protocol: c_int,
                                      name: *const c_char,
                                      le_type: *const c_char,
                                      domain: *const c_char,
                                      aprotocol: AvahiProtocol,
                                      flags: AvahiLookupFlags,
                                      callback: ServiceResolverCallback,
                                      userdata: *mut c_void)
                                      -> *mut AvahiServiceResolver;

    pub fn avahi_address_snprint(ret_s: *const c_char, length: size_t, a: *const AvahiAddress);

    pub fn avahi_string_list_to_string(l: *mut AvahiStringList) -> *const c_char;
    
    pub fn avahi_free(p: *mut c_void);
}