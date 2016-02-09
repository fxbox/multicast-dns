use multicast_dns::bindings::avahi;
use multicast_dns::callback_handler::CallbackHandler;
use multicast_dns::callback_handler::SafeHandler;
use multicast_dns::callback_handler::ClientReference;
use multicast_dns::callback_handler::ServiceDescription;

use libc::{c_void, free};
use std::mem;

use std::ffi::CString;
use std::ffi::CStr;
use std::ptr;

pub struct MulticastDNS;

impl MulticastDNS {
    pub fn new() -> MulticastDNS {
        MulticastDNS
    }

    /// List all available service by type_name.
    pub fn list(&self, service_type: &str) {
        unsafe {
            let mut client_error_code: i32 = 0;

            let simple_poll = avahi::avahi_simple_poll_new();
            let poll = avahi::avahi_simple_poll_get(simple_poll);

            let client =
                avahi::avahi_client_new(poll,
                                        avahi::AvahiClientFlags::AVAHI_CLIENT_IGNORE_USER_CONFIG,
                                        *Box::new(CallbackHandler::client_callback),
                                        ptr::null_mut(),
                                        &mut client_error_code);

            // Check that we've created client successfully, otherwise try to resolve error
            // into human-readable string.
            if client.is_null() {
                let error_string = CStr::from_ptr(avahi::avahi_strerror(client_error_code));
                free(client as *mut c_void);
                panic!("Failed to create avahi client: {}",
                       error_string.to_str().unwrap());

            }

            let client_reference = ClientReference {
                client: client,
                handler: self,
            };

            // Let's search for service of requested type.
            let sb = avahi::avahi_service_browser_new(client,
                                                      avahi::AvahiIfIndex::AVAHI_IF_UNSPEC,
                                                      avahi::AvahiProtocol::AVAHI_PROTO_UNSPEC,
                                                      CString::new(service_type).unwrap().as_ptr(),
                                                      ptr::null_mut(),
                                                      avahi::AvahiLookupFlags::AVAHI_LOOKUP_NO_TXT,
                                                      *Box::new(CallbackHandler::browse_callback::<MulticastDNS>),
                                                      mem::transmute(&client_reference));

            avahi::avahi_simple_poll_loop(simple_poll);

            avahi::avahi_service_browser_free(sb);
            avahi::avahi_client_free(client);
            avahi::avahi_simple_poll_free(simple_poll);
        }
    }
}

impl SafeHandler for MulticastDNS {
    fn on_browse(&self) {}

    fn on_resolve(&self, service_description: ServiceDescription) {
        println!("New service discovered: {:?}", service_description);
    }
}
