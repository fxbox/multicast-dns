use std::mem;
use std::ffi::CStr;

use libc::{c_void, c_int, c_char};

use bindings::avahi;
use service_discovery::service_discovery_manager::ServiceDiscoveryManager;
use service_discovery::service_discovery_manager::ServiceDescription;

pub trait DiscoveryEventHandler {
    fn on_service_discovered(&self, service_description: ServiceDescription);
    fn on_service_resolved(&self, service_description: ServiceDescription);
}

pub struct UserData<'a, T>
    where T: ServiceDiscoveryManager + 'a
{
    client: *mut avahi::AvahiClient,
    manager: &'a T,
    sink: &'a Sink<ServiceDescription<'a>>,
}

pub struct CallbackHandler;

impl CallbackHandler {
    #[allow(unused_variables)]
    pub extern "C" fn client_callback(s: *mut avahi::AvahiClient,
                                      state: avahi::AvahiClientState,
                                      userdata: *mut c_void) {
    }

    #[allow(unused_variables)]
    pub extern "C" fn browse_callback<T: DiscoveryEventHandler>(b: *mut avahi::AvahiServiceBrowser,
                                                      interface: c_int,
                                                      protocol: c_int,
                                                      event: avahi::AvahiBrowserEvent,
                                                      name: *const c_char,
                                                      service_type: *const c_char,
                                                      domain: *const c_char,
                                                      flags: avahi::AvahiLookupResultFlags,
                                                      userdata: *mut c_void) where T: ServiceDiscoveryManager {
        match event {
            avahi::AvahiBrowserEvent::AVAHI_BROWSER_NEW => unsafe {
                let client_reference = mem::transmute::<*mut c_void, &mut UserData<T>>(userdata);

                client_reference.manager.on_service_discovered(ServiceDescription {
                    address: &"",
                    domain: CStr::from_ptr(domain).to_str().unwrap(),
                    host_name: &"",
                    name: CStr::from_ptr(name).to_str().unwrap(),
                    port: 0,
                    txt: &"",
                    type_name: CStr::from_ptr(service_type).to_str().unwrap(),
                });

                // Theoretically we should not try to resolve automatically, instead it should
                // be decided in `on_service_discovered` callback.
                avahi::avahi_service_resolver_new(client_reference.client,
                                                  interface,
                                                  protocol,
                                                  name,
                                                  service_type,
                                                  domain,
                                                  avahi::AvahiProtocol::AVAHI_PROTO_UNSPEC,
                                                  avahi::AvahiLookupFlags::AVAHI_LOOKUP_UNSPEC,
                                                  *Box::new(CallbackHandler::resolve_callback::<T>),
                                                  userdata);
            },
            _ => println!("{:?}", event),
        }
    }

    #[allow(unused_variables)]
    extern "C" fn resolve_callback<T: DiscoveryEventHandler>(r: *mut avahi::AvahiServiceResolver,
                                                             interface: c_int,
                                                             protocol: c_int,
                                                             event: avahi::AvahiResolverEvent,
                                                             name: *const c_char,
                                                             service_type: *const c_char,
                                                             domain: *const c_char,
                                                             host_name: *const c_char,
                                                             address: *const avahi::AvahiAddress,
                                                             port: u16,
                                                             txt: *mut avahi::AvahiStringList,
                                                             flags: avahi::AvahiLookupResultFlags,
                                                             userdata: *mut c_void)
        where T: ServiceDiscoveryManager
    {
        match event {
            avahi::AvahiResolverEvent::AVAHI_RESOLVER_FAILURE => {
                println!("Failed to resolve");
            }

            avahi::AvahiResolverEvent::AVAHI_RESOLVER_FOUND => {
                let address_vector = Vec::with_capacity(avahi::AVAHI_ADDRESS_STR_MAX).as_ptr();

                let (manager, address, domain, host_name, name, service_type, txt) = unsafe {
                    avahi::avahi_address_snprint(address_vector,
                                                 avahi::AVAHI_ADDRESS_STR_MAX,
                                                 address);

                    let txt_pointer = avahi::avahi_string_list_to_string(txt);
                    let txt = CStr::from_ptr(txt_pointer).to_string_lossy().into_owned();
                    avahi::avahi_free(txt_pointer as *mut c_void);

                    (mem::transmute::<*mut c_void, &mut UserData<T>>(userdata).manager,
                     CStr::from_ptr(address_vector),
                     CStr::from_ptr(domain),
                     CStr::from_ptr(host_name),
                     CStr::from_ptr(name),
                     CStr::from_ptr(service_type),
                     txt)
                };

                manager.on_service_resolved(ServiceDescription {
                    address: address.to_str().unwrap(),
                    domain: domain.to_str().unwrap(),
                    host_name: host_name.to_str().unwrap(),
                    name: name.to_str().unwrap(),
                    port: port,
                    type_name: service_type.to_str().unwrap(),
                    txt: &txt,
                });
            }
        }
    }
}
