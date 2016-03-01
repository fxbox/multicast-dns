use super::enums::*;
use libc::{c_void, c_int, c_char};

/// A main loop object.
/// Main loops of this type aren't very flexible since they only support a single wakeup type. 
#[repr(C)]
pub struct AvahiSimplePoll;

#[repr(C)]
pub struct AvahiThreadedPoll;

#[repr(C)]
pub struct AvahiPoll;

#[repr(C)]
pub struct DBusConnection;

#[repr(C)]
pub struct AvahiEntryGroup;

#[repr(C)]
pub struct AvahiDomainBrowser;

#[repr(C)]
pub struct AvahiServiceBrowser;

#[repr(C)]
pub struct AvahiServiceTypeBrowser;

#[repr(C)]
pub struct AvahiServiceResolver;

#[repr(C)]
pub struct AvahiHostNameResolver;

#[repr(C)]
pub struct AvahiAddressResolver;

#[repr(C)]
pub struct AvahiRecordBrowser;

#[repr(C)]
pub struct AvahiAddress;

#[repr(C)]
pub struct AvahiStringList;

#[repr(C)]
pub struct AvahiClient {
    poll_api: *const AvahiPoll,
    bus: *const DBusConnection,
    error: u16,
    state: AvahiClientState,
    flags: AvahiClientFlags,
    version_string: *const c_char,
    host_name: *const c_char,
    host_name_fqdn: *const c_char,
    domain_name: *const c_char,
    local_service_cookie: u32,
    local_service_cookie_valid: u16,
    callback: extern "C" fn(*const AvahiClient, AvahiClientState, *const c_void),
    userdata: *const c_void,
    groups: *const AvahiEntryGroup,
    domain_browsers: *const AvahiDomainBrowser,
    service_browsers: *const AvahiServiceBrowser,
    service_type_browsers: *const AvahiServiceTypeBrowser,
    service_resolvers: *const AvahiServiceResolver,
    hsot_name_resolvers: *const AvahiHostNameResolver,
    address_resolvers: *const AvahiAddressResolver,
    record_browsers: *const AvahiRecordBrowser,
}


pub type ServiceBrowserCallback = extern "C" fn(*mut AvahiServiceBrowser,
                                                c_int,
                                                AvahiProtocol,
                                                AvahiBrowserEvent,
                                                *const c_char,
                                                *const c_char,
                                                *const c_char,
                                                AvahiLookupResultFlags,
                                                *mut c_void)
                                               ;

pub type ServiceResolverCallback = extern "C" fn(*mut AvahiServiceResolver,
                                                 c_int,
                                                 AvahiProtocol,
                                                 AvahiResolverEvent,
                                                 *const c_char,
                                                 *const c_char,
                                                 *const c_char,
                                                 *const c_char,
                                                 *const AvahiAddress,
                                                 u16,
                                                 *mut AvahiStringList,
                                                 AvahiLookupResultFlags,
                                                 *mut c_void)
                                                ;

pub type AvahiEntryGroupCallback = extern "C" fn(*mut AvahiEntryGroup,
                                                 AvahiEntryGroupState,
                                                 *mut c_void)
                                                ;

pub static AVAHI_ADDRESS_STR_MAX: usize = 4 * 8 + 7 + 1; // 1 is for NUL