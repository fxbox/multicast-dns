use multicast_dns::bindings::avahi;

use libc::{c_void, c_int, c_char};
use std::mem;

use std::ffi::CString;
use std::ffi::CStr;
use std::ptr;

#[derive(Debug)]
pub struct AvahiResolveResult<'a> {
    pub name: &'a str,
    pub host_name: &'a str,
    pub address: &'a str,
    pub port: u16,
}

struct ServiceDescription {
    name: String,
    type_name: String,
    domain: String,
}

struct ClientReference<'a> {
    client: *mut avahi::AvahiClient,
    multicast_dns: &'a MulticastDNS,
}

impl ServiceDescription {
    fn new(name: String, type_name: String, domain: String) -> ServiceDescription {
        ServiceDescription {
            name: name,
            type_name: type_name,
            domain: domain,
        }
    }
}

#[allow(unused_variables)]
extern "C" fn client_callback(s: *mut avahi::AvahiClient,
                              state: avahi::AvahiClientState,
                              userdata: *mut c_void) {
}

#[allow(unused_variables)]
extern "C" fn browse_callback(b: *mut avahi::AvahiServiceBrowser,
                              interface: c_int,
                              protocol: c_int,
                              event: avahi::AvahiBrowserEvent,
                              name: *const c_char,
                              le_type: *const c_char,
                              domain: *const c_char,
                              flags: avahi::AvahiLookupResultFlags,
                              userdata: *mut c_void) {
    match event {
        avahi::AvahiBrowserEvent::AVAHI_BROWSER_NEW => {
            let service_description = unsafe {
                ServiceDescription::new(CStr::from_ptr(name).to_string_lossy().into_owned(),
                                        CStr::from_ptr(le_type).to_string_lossy().into_owned(),
                                        CStr::from_ptr(domain).to_string_lossy().into_owned())
            };

            let mdns = unsafe { mem::transmute::<*mut c_void, &mut ClientReference>(userdata) };
            mdns.multicast_dns.on_new_service(service_description);

            unsafe {
                avahi::avahi_service_resolver_new(mdns.client,
                                                  interface,
                                                  protocol,
                                                  name,
                                                  le_type,
                                                  domain,
                                                  avahi::AvahiProtocol::AVAHI_PROTO_UNSPEC,
                                                  avahi::AvahiLookupFlags::AVAHI_LOOKUP_NO_TXT,
                                                  *Box::new(resolve_callback),
                                                  userdata);
            }
        }
        _ => println!("{:?}", event),
    }
}

#[allow(unused_variables)]
extern "C" fn resolve_callback(r: *mut avahi::AvahiServiceResolver,
                               interface: c_int,
                               protocol: c_int,
                               event: avahi::AvahiResolverEvent,
                               name: *const c_char,
                               le_type: *const c_char,
                               domain: *const c_char,
                               host_name: *const c_char,
                               address: *const avahi::AvahiAddress,
                               port: u16,
                               txt: *mut avahi::AvahiStringList,
                               flags: avahi::AvahiLookupResultFlags,
                               userdata: *mut c_void) {

    match event {
        avahi::AvahiResolverEvent::AVAHI_RESOLVER_FAILURE => {
            println!("Failed to resolve");
        }

        avahi::AvahiResolverEvent::AVAHI_RESOLVER_FOUND => {
            let address_vector = Vec::with_capacity(avahi::AVAHI_ADDRESS_STR_MAX).as_ptr();

            let result = unsafe {
                avahi::avahi_address_snprint(address_vector, avahi::AVAHI_ADDRESS_STR_MAX, address);

                AvahiResolveResult {
                    name: CStr::from_ptr(name).to_str().unwrap(),
                    host_name: CStr::from_ptr(host_name).to_str().unwrap(),
                    address: CStr::from_ptr(address_vector).to_str().unwrap(),
                    port: port,
                }
            };

            println!("Resolved! {:?}", result);
        }
    }
}

pub struct MulticastDNS;

impl MulticastDNS {
    pub fn new() -> MulticastDNS {
        MulticastDNS
    }

    fn on_new_service(&self, service_description: ServiceDescription) {
        println!("New service discovered: {} {} {}",
                 service_description.name,
                 service_description.type_name,
                 service_description.domain);
    }

    /// List all available service by type_name.
    pub fn list(&mut self, service_type: String) {
        unsafe {
            let mut error: i32 = 0;

            let simple_poll = avahi::avahi_simple_poll_new();

            let poll = avahi::avahi_simple_poll_get(simple_poll);

            let client =
                avahi::avahi_client_new(poll,
                                        avahi::AvahiClientFlags::AVAHI_CLIENT_IGNORE_USER_CONFIG,
                                        *Box::new(client_callback),
                                        ptr::null_mut(),
                                        &mut error);
            let client_reference = ClientReference {
                client: client,
                multicast_dns: &self,
            };

            let _sb =
                avahi::avahi_service_browser_new(client,
                                                 -1,
                                                 -1,
                                                 CString::new(service_type).unwrap().as_ptr(),
                                                 ptr::null_mut(),
                                                 avahi::AvahiLookupFlags::AVAHI_LOOKUP_NO_TXT,
                                                 *Box::new(browse_callback),
                                                 mem::transmute(&client_reference));

            avahi::avahi_simple_poll_loop(simple_poll);
        }
    }
}