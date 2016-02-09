use std::mem;
use std::ffi::CStr;

use libc::{c_void, c_int, c_char};

use multicast_dns::bindings::avahi;

#[derive(Debug)]
pub struct ServiceDescription<'a> {
    pub address: &'a str,
    pub domain: &'a str,
    pub host_name: &'a str,
    pub name: &'a str,
    pub port: u16,
    pub type_name: &'a str,
}

impl<'a> ServiceDescription<'a> {
    fn new(address: &'a str,
           domain: &'a str,
           host_name: &'a str,
           name: &'a str,
           port: u16,
           type_name: &'a str)
           -> ServiceDescription<'a> {
        ServiceDescription {
            address: address,
            domain: domain,
            host_name: host_name,
            name: name,
            port: port,
            type_name: type_name,
        }
    }
}

pub trait SafeHandler {
    fn on_browse(&self);
    fn on_resolve(&self, service_description: ServiceDescription);
}

pub struct ClientReference<'a, T: 'a>
    where T: SafeHandler
{
    pub client: *mut avahi::AvahiClient,
    pub handler: &'a T,
}

pub struct CallbackHandler;

impl CallbackHandler {
    #[allow(unused_variables)]
    pub extern "C" fn client_callback(s: *mut avahi::AvahiClient,
                                      state: avahi::AvahiClientState,
                                      userdata: *mut c_void) {
    }

    #[allow(unused_variables)]
    pub extern "C" fn browse_callback<T: SafeHandler>(b: *mut avahi::AvahiServiceBrowser,
                                                      interface: c_int,
                                                      protocol: c_int,
                                                      event: avahi::AvahiBrowserEvent,
                                                      name: *const c_char,
                                                      le_type: *const c_char,
                                                      domain: *const c_char,
                                                      flags: avahi::AvahiLookupResultFlags,
                                                      userdata: *mut c_void) {
        match event {
            avahi::AvahiBrowserEvent::AVAHI_BROWSER_NEW => unsafe {
                let client_reference = mem::transmute::<*mut c_void,
                                                        &mut ClientReference<T>>(userdata);
                avahi::avahi_service_resolver_new(client_reference.client,
                                                  interface,
                                                  protocol,
                                                  name,
                                                  le_type,
                                                  domain,
                                                  avahi::AvahiProtocol::AVAHI_PROTO_UNSPEC,
                                                  avahi::AvahiLookupFlags::AVAHI_LOOKUP_NO_TXT,
                                                  *Box::new(CallbackHandler::resolve_callback::<T>),
                                                  userdata);
            },
            _ => println!("{:?}", event),
        }
    }

    #[allow(unused_variables)]
    extern "C" fn resolve_callback<T: SafeHandler>(r: *mut avahi::AvahiServiceResolver,
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

                let (handler, address, domain, host_name, name, le_type) = unsafe {
                    avahi::avahi_address_snprint(address_vector,
                                                 avahi::AVAHI_ADDRESS_STR_MAX,
                                                 address);

                    (mem::transmute::<*mut c_void, &mut ClientReference<T>>(userdata).handler,
                     CStr::from_ptr(address_vector),
                     CStr::from_ptr(domain),
                     CStr::from_ptr(host_name),
                     CStr::from_ptr(name),
                     CStr::from_ptr(le_type))
                };

                let service_description = ServiceDescription::new(address.to_str().unwrap(),
                                                                  domain.to_str().unwrap(),
                                                                  host_name.to_str().unwrap(),
                                                                  name.to_str().unwrap(),
                                                                  port,
                                                                  le_type.to_str().unwrap());

                handler.on_resolve(service_description);
            }
        }
    }
}
