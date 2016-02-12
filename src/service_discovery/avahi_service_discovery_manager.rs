use bindings::avahi::*;
use service_discovery::service_discovery_manager::*;
use service_discovery::avah_wrapper::*;

use libc::{c_char, c_void, c_int, free};
use std::mem;

use std::ffi::CString;
use std::ffi::CStr;
use std::ptr;
struct UserData<'a, T>
    where T: ServiceDiscoveryManager + 'a
{
    client: *mut AvahiClient,
    manager: &'a AvahiServiceDiscoveryManager,
    callback: &'a fn(manager: &T, service_description: ServiceDescription),
}

#[allow(unused_variables)]
extern "C" fn client_callback(s: *mut AvahiClient,
                              state: AvahiClientState,
                              userdata: *mut c_void) {
}

#[allow(unused_variables)]
extern "C" fn browse_callback(b: *mut AvahiServiceBrowser,
                              interface: c_int,
                              protocol: c_int,
                              event: AvahiBrowserEvent,
                              name: *const c_char,
                              service_type: *const c_char,
                              domain: *const c_char,
                              flags: AvahiLookupResultFlags,
                              userdata: *mut c_void) {
    match event {
        AvahiBrowserEvent::AVAHI_BROWSER_NEW => unsafe {
            let client_reference =
                mem::transmute::<*mut c_void,
                                 &mut UserData<AvahiServiceDiscoveryManager>>(userdata);

            let service_description = ServiceDescription {
                address: &"",
                domain: CStr::from_ptr(domain).to_str().unwrap(),
                host_name: &"",
                interface: interface,
                name: CStr::from_ptr(name).to_str().unwrap(),
                port: 0,
                txt: &"",
                type_name: CStr::from_ptr(service_type).to_str().unwrap(),
            };

            (*client_reference.callback)(client_reference.manager, service_description);


            // Theoretically we should not try to resolve automatically, instead it should
            // be decided in `on_service_discovered` callback.
            avahi_service_resolver_new(client_reference.client,
                                       interface,
                                       protocol,
                                       name,
                                       service_type,
                                       domain,
                                       AvahiProtocol::AVAHI_PROTO_UNSPEC,
                                       AvahiLookupFlags::AVAHI_LOOKUP_UNSPEC,
                                       *Box::new(resolve_callback),
                                       userdata);
        },
        _ => println!("{:?}", event),
    }
}

#[allow(unused_variables)]
extern "C" fn resolve_callback(r: *mut AvahiServiceResolver,
                               interface: c_int,
                               protocol: c_int,
                               event: AvahiResolverEvent,
                               name: *const c_char,
                               service_type: *const c_char,
                               domain: *const c_char,
                               host_name: *const c_char,
                               address: *const AvahiAddress,
                               port: u16,
                               txt: *mut AvahiStringList,
                               flags: AvahiLookupResultFlags,
                               userdata: *mut c_void) {
    match event {
        AvahiResolverEvent::AVAHI_RESOLVER_FAILURE => {
            println!("Failed to resolve");
        }

        AvahiResolverEvent::AVAHI_RESOLVER_FOUND => {
            let address_vector = Vec::with_capacity(AVAHI_ADDRESS_STR_MAX).as_ptr();

            let (manager, address, domain, host_name, name, service_type, txt) = unsafe {
                avahi_address_snprint(address_vector, AVAHI_ADDRESS_STR_MAX, address);

                let txt_pointer = avahi_string_list_to_string(txt);
                let txt = CStr::from_ptr(txt_pointer).to_string_lossy().into_owned();
                avahi_free(txt_pointer as *mut c_void);

                (mem::transmute::<*mut c_void,
                                  &mut UserData<AvahiServiceDiscoveryManager>>(userdata)
                     .manager,
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
                interface: interface,
                name: name.to_str().unwrap(),
                port: port,
                type_name: service_type.to_str().unwrap(),
                txt: &txt,
            });
        }
    }
}


pub struct AvahiServiceDiscoveryManager {
    wrapper: AvahiWrapper,
}

impl AvahiServiceDiscoveryManager {
    fn on_service_resolved(&self, service_description: ServiceDescription) {
        println!("Service resolved: {:?}", service_description);
    }
}


impl AvahiServiceDiscoveryManager {
    /// Creates `AvahiClient` instance for the provided `AvahiPoll` object.
    ///
    /// # Arguments
    ///
    /// * `poll` - Abstracted `AvahiPoll` object that we'd like to create client for.
    ///
    /// # Return value
    ///
    /// Initialized `AvahiClient` instance, if there was an error while creating
    /// client, this method will `panic!`.
    unsafe fn create_client(&self, poll: *const AvahiPoll) -> *mut AvahiClient {
        let mut client_error_code: i32 = 0;

        let client = avahi_client_new(poll,
                                      AvahiClientFlags::AVAHI_CLIENT_IGNORE_USER_CONFIG,
                                      *Box::new(client_callback),
                                      ptr::null_mut(),
                                      &mut client_error_code);
        // Check that we've created client successfully, otherwise try to resolve error
        // into human-readable string.
        if client.is_null() {
            free(client as *mut c_void);

            let error_string = CStr::from_ptr(avahi_strerror(client_error_code));

            panic!("Failed to create avahi client: {}",
                   error_string.to_str().unwrap());
        }

        return client;
    }
}

impl ServiceDiscoveryManager for AvahiServiceDiscoveryManager {
    fn new() -> AvahiServiceDiscoveryManager {
        AvahiServiceDiscoveryManager { wrapper: AvahiWrapper::new() }
    }

    fn discover_services<F>(&self, service_type: &str, callback: F)
        where F: FnMut(ServiceDescription)
    {
        self.wrapper.start_browser(service_type, callback);
    }

    fn stop_service_discovery(&self) {
        self.wrapper.stop_browser();
    }

    //     fn discover_services<T>(&self,
    //                             service_type: &str,
    //                             callback: fn(manager: &T, service_description: ServiceDescription))
    //         where T: ServiceDiscoveryManager
    //     {
    //         unsafe {
    //             let poll = avahi_threaded_poll_new();
    //
    //             // Get abstracted api for the Simple Poll object.
    //             let client = self.create_client(avahi_threaded_poll_get(poll));
    //
    //             let user_data = UserData {
    //                 client: client,
    //                 manager: &self,
    //                 callback: &callback,
    //             };
    //
    //             // Let's search for service of requested type.
    //             let _sb = avahi_service_browser_new(client,
    //                                                 AvahiIfIndex::AVAHI_IF_UNSPEC,
    //                                                 AvahiProtocol::AVAHI_PROTO_UNSPEC,
    //                                                 CString::new(service_type).unwrap().as_ptr(),
    //                                                 ptr::null_mut(),
    //                                                 AvahiLookupFlags::AVAHI_LOOKUP_UNSPEC,
    //                                                 *Box::new(browse_callback),
    //                                                 mem::transmute(&user_data));
    //
    //             let result_code = avahi_threaded_poll_start(poll);
    //
    //             if result_code == -1 {
    //                 panic!("Avahi threaded poll could not be started!");
    //             }
    //         }
    //     }

    /// List all available service by type_name.
    //     fn discover_services_sync(&self, service_type: &str) {
    //         unsafe {
    //             let poll = avahi::avahi_simple_poll_new();
    //
    //             // Get abstracted api for the Simple Poll object.
    //             let client = self.create_client(avahi::avahi_simple_poll_get(poll));
    //
    //             let user_data = UserData {
    //                 client: client,
    //                 manager: &self,
    //             };
    //
    //             // Let's search for service of requested type.
    //             let sb = avahi::avahi_service_browser_new(client,
    //                                                       avahi::AvahiIfIndex::AVAHI_IF_UNSPEC,
    //                                                       avahi::AvahiProtocol::AVAHI_PROTO_UNSPEC,
    //                                                       CString::new(service_type).unwrap().as_ptr(),
    //                                                       ptr::null_mut(),
    //                                                       avahi::AvahiLookupFlags::AVAHI_LOOKUP_UNSPEC,
    //                                                       *Box::new(browse_callback),
    //                                                       mem::transmute(&user_data));
    //
    //             avahi::avahi_simple_poll_loop(poll);
    //
    //             avahi::avahi_service_browser_free(sb);
    //             avahi::avahi_client_free(client);
    //             avahi::avahi_simple_poll_free(poll);
    //         }
    //     }

    fn resolve_service(&self, service_description: ServiceDescription) {
        panic!("Not Implemented!");
    }
}