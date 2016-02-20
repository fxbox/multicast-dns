use std::cell::RefCell;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::sync::mpsc;

use libc::c_void;

use bindings::avahi::*;
use discovery::discovery_manager::*;

use api::api::API;
use api::avahi::avahi_utils::AvahiUtils;
use api::avahi::avahi_callbacks::*;

pub struct AvahiAPI {
    client: RefCell<Option<*mut AvahiClient>>,
    poll: RefCell<Option<*mut AvahiThreadedPoll>>,
    service_browser: RefCell<Option<*mut AvahiServiceBrowser>>,
}

impl AvahiAPI {
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
    fn initialize_client(&self) {
        let mut client = self.client.borrow_mut();

        if client.is_some() {
            return;
        }

        self.initialize_poll();

        let mut client_error_code: i32 = 0;
        let poll = self.poll.borrow().unwrap();

        let avahi_client = unsafe {
            avahi_client_new(avahi_threaded_poll_get(poll),
                             AvahiClientFlags::AVAHI_CLIENT_IGNORE_USER_CONFIG,
                             *Box::new(AvahiCallbacks::client_callback),
                             ptr::null_mut(),
                             &mut client_error_code)
        };

        // Check that we've created client successfully, otherwise try to resolve error
        // into human-readable string.
        if avahi_client.is_null() {
            let error_string = AvahiUtils::to_owned_string(unsafe {
                avahi_strerror(client_error_code)
            });

            panic!("Failed to create avahi client: {}", error_string.unwrap());
        }

        *client = Some(avahi_client);
    }

    fn initialize_poll(&self) {
        let mut poll = self.poll.borrow_mut();

        if poll.is_some() {
            return;
        }

        *poll = Some(unsafe { avahi_threaded_poll_new() });
    }

    fn start_polling(&self) {
        let poll = self.poll.borrow().unwrap();

        let result_code = unsafe { avahi_threaded_poll_start(poll) };
        if result_code == -1 {
            panic!("Avahi threaded poll could not be started!");
        }
    }
}

impl API for AvahiAPI {
    fn new() -> AvahiAPI {
        AvahiAPI {
            client: RefCell::new(None),
            poll: RefCell::new(None),
            service_browser: RefCell::new(None),
        }
    }

    fn start_browser(&self, service_type: &str, listeners: DiscoveryListeners) {
        self.initialize_client();

        let (tx, rx) = mpsc::channel::<BrowseCallbackParameters>();

        let avahi_service_browser = unsafe {
            avahi_service_browser_new(self.client.borrow().unwrap(),
                                      AvahiIfIndex::AVAHI_IF_UNSPEC,
                                      AvahiProtocol::AVAHI_PROTO_UNSPEC,
                                      AvahiUtils::string_to_ptr(service_type),
                                      ptr::null_mut(),
                                      AvahiLookupFlags::AVAHI_LOOKUP_UNSPEC,
                                      *Box::new(AvahiCallbacks::browse_callback),
                                      mem::transmute(&tx.clone()))
        };

        *self.service_browser.borrow_mut() = Some(avahi_service_browser);

        self.start_polling();

        for a in rx.iter() {
            match a.event {
                AvahiBrowserEvent::AVAHI_BROWSER_NEW => {
                    let service = ServiceInfo {
                        address: None,
                        domain: a.domain,
                        host_name: None,
                        interface: a.interface,
                        name: a.name,
                        port: 0,
                        protocol: a.protocol,
                        txt: None,
                        type_name: a.service_type,
                    };

                    if listeners.on_service_discovered.is_some() {
                        (*listeners.on_service_discovered.unwrap())(service);
                    }
                }
                AvahiBrowserEvent::AVAHI_BROWSER_ALL_FOR_NOW => {
                    if listeners.on_all_discovered.is_some() {
                        (*listeners.on_all_discovered.unwrap())();
                    }

                    break;
                }
                _ => println!("Default {:?}", a.event),
            }
        }
    }

    fn resolve(&self, service: ServiceInfo, listeners: ResolveListeners) {
        let (tx, rx) = mpsc::channel::<ResolveCallbackParameters>();

        let avahi_service_resolver = unsafe {
            avahi_service_resolver_new(self.client.borrow().unwrap(),
                                       service.interface,
                                       service.protocol,
                                       CString::new(service.name.unwrap()).unwrap().as_ptr(),
                                       CString::new(service.type_name.unwrap()).unwrap().as_ptr(),
                                       CString::new(service.domain.unwrap()).unwrap().as_ptr(),
                                       AvahiProtocol::AVAHI_PROTO_UNSPEC,
                                       AvahiLookupFlags::AVAHI_LOOKUP_UNSPEC,
                                       *Box::new(AvahiCallbacks::resolve_callback),
                                       mem::transmute(&tx))
        };

        let raw_service = rx.recv().unwrap();

        let service = ServiceInfo {
            address: raw_service.address,
            domain: raw_service.domain,
            host_name: raw_service.host_name,
            interface: raw_service.interface,
            name: raw_service.name,
            port: raw_service.port,
            protocol: raw_service.protocol,
            txt: raw_service.txt,
            type_name: raw_service.service_type,
        };

        if listeners.on_service_resolved.is_some() {
            (*listeners.on_service_resolved.unwrap())(service);
        }

        unsafe {
            avahi_service_resolver_free(avahi_service_resolver);
        }
    }

    fn stop_browser(&self) {
        let mut client = self.client.borrow_mut();
        let mut service_browser = self.service_browser.borrow_mut();
        if client.is_some() {
            // This will remove service browser as well as resolver.
            unsafe {
                avahi_client_free(client.unwrap());
            }

            *client = None;
            *service_browser = None
        }

        println!("Removed client");


        //         let mut service_browser = self.service_browser.borrow_mut();
        //         if service_browser.is_some() {
        //             unsafe {
        //                 avahi_service_browser_free(service_browser.unwrap());
        //             };
        //
        //
        //         }
        //
        //         println!("Removed service browser");

        let mut poll = self.poll.borrow_mut();
        if poll.is_some() {
            unsafe {
                avahi_threaded_poll_free(poll.unwrap());
            }

            *poll = None;
        }

        println!("Removed poll");
    }

    fn get_host_name(&self) -> Option<String> {
        self.initialize_client();

        let client = self.client.borrow().unwrap();

        let host_name_ptr = unsafe { avahi_client_get_host_name(client) };

        AvahiUtils::to_owned_string(host_name_ptr)
    }

    fn set_host_name(&self, host_name: &str) {
        self.initialize_client();

        if host_name == self.get_host_name().unwrap() {
            return;
        }

        let client = self.client.borrow().unwrap();

        let result_code = unsafe {
            avahi_client_set_host_name(client, AvahiUtils::string_to_ptr(host_name))
        };

        if result_code != 0 {
            let error_string = AvahiUtils::to_owned_string(unsafe { avahi_strerror(result_code) });

            panic!("Failed set host name: {}", error_string.unwrap());
        }
    }

    fn is_valid_host_name(&self, host_name: &str) -> bool {
        let host_name_ptr = AvahiUtils::string_to_ptr(host_name);

        let is_valid = unsafe { avahi_is_valid_host_name(host_name_ptr) };

        is_valid == 1
    }

    fn get_alternative_host_name(&self, host_name: &str) -> String {
        let original_host_name_ptr = AvahiUtils::string_to_ptr(host_name);

        let alternative_host_name_ptr = unsafe {
            avahi_alternative_host_name(original_host_name_ptr)
        };

        let alternative_host_name = AvahiUtils::to_owned_string(alternative_host_name_ptr);

        unsafe { avahi_free(alternative_host_name_ptr as *mut c_void) };

        alternative_host_name.unwrap()
    }
}