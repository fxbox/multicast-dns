use std::mem;
use std::ffi::CStr;

use libc::{c_void, c_int, c_char};

use multicast_dns::bindings::avahi;

trait SafeHandler {
    fn on_browse(&self);
    fn on_resolve(&self);
}

struct ClientReference<'a, T: 'a>
    where T: SafeHandler
{
    client: *mut avahi::AvahiClient,
    handler: &'a T,
}

struct CallbackHandler<T> {
    handler: T,
}

impl<T> CallbackHandler<T> where T: SafeHandler
{
    pub fn new(value: T) -> CallbackHandler<T> {
        CallbackHandler { handler: value }
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
                                                  *Box::new(CallbackHandler::<T>::resolve_callback),
                                                  userdata);
            },
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
                
                handler.on_resolve();

//                 let service_description = ServiceDescription::new(address.to_str().unwrap(),
//                                                                   domain.to_str().unwrap(),
//                                                                   host_name.to_str().unwrap(),
//                                                                   name.to_str().unwrap(),
//                                                                   port,
//                                                                   le_type.to_str().unwrap());
// 
//                 mdns.on_new_service(service_description);
            }
        }
    }
}
