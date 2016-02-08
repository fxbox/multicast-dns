use multicast_dns::bindings::avahi;

use libc::{c_void, c_int, c_char};
use std::mem;

use std::ffi::CString;
use std::ffi::CStr;
use std::ptr;

struct ServiceDescription {
    name: String,
    type_name: String,
    domain: String
}

impl ServiceDescription {
    fn new(name: String, type_name: String, domain: String) -> ServiceDescription {
        ServiceDescription {
            name: name,
            type_name: type_name,
            domain: domain
        }
    }
}

#[allow(unused_variables)]
extern fn client_callback(
    s: *mut avahi::AvahiClient,
    state: avahi::AvahiClientState,
    userdata: *mut c_void) {
}

#[allow(unused_variables)]
extern "C" fn browse_callback(
    b: *mut avahi::AvahiServiceBrowser,
    interface: c_int,
    protocol: c_int,
    event: avahi::AvahiBrowserEvent, 
    name: *const c_char,
    le_type: *const c_char,
    domain: *const c_char,
    flags: avahi::AvahiLookupResultFlags, 
    userdata: *mut c_void
) {
    match event {
        avahi::AvahiBrowserEvent::AVAHI_BROWSER_NEW => { 
            let service_description = unsafe {
                ServiceDescription::new(
                    CStr::from_ptr(name).to_string_lossy().into_owned(),
                    CStr::from_ptr(le_type).to_string_lossy().into_owned(),
                    CStr::from_ptr(domain).to_string_lossy().into_owned()
                )
            };
            
            let mdns = unsafe { 
                mem::transmute::<*mut c_void, MulticastDNS>(userdata)
            };
            mdns.on_new_service(service_description);
        }
        _ => println!("{:?}", event)
    }
}

pub struct MulticastDNS {
    _ptr: usize
}

impl MulticastDNS {
    pub fn new() -> MulticastDNS {
        // FIXME: Not sure how to make mem::transmute to love my empty struct :/
        MulticastDNS { _ptr: 0 }
    }
    
    fn on_new_service(&self, service_description: ServiceDescription) {
        println!(
            "New service discovered: {} {} {}",
            service_description.name,
            service_description.type_name,
            service_description.domain
        );
    }
    
    /// List all available service by type_name.
    pub fn list(self, service_type: String) {  
        unsafe {
            let mut error: i32 = 0;
            
            let simple_poll = avahi::avahi_simple_poll_new();

            let poll = avahi::avahi_simple_poll_get(simple_poll);

            let client = avahi::avahi_client_new(
                poll,
                avahi::AvahiClientFlags::AVAHI_CLIENT_IGNORE_USER_CONFIG,
                *Box::new(client_callback),
                ptr::null_mut(),
                &mut error
            );

            let _sb = avahi::avahi_service_browser_new(
                client,
                -1,
                -1,
                CString::new(service_type).unwrap().as_ptr(), 
                ptr::null_mut(),
                avahi::AvahiLookupFlags::AVAHI_LOOKUP_NO_TXT, 
                *Box::new(browse_callback),
                // We need reference to ourselves in the callback.
                mem::transmute(&self)
            );

            avahi::avahi_simple_poll_loop(simple_poll);
        }
    }
}