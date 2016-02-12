use std::mem;

use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

use libc::{c_char, c_void, c_int, free};

use bindings::avahi::*;
use service_discovery::service_discovery_manager::ServiceDescription;
use std::cell::RefCell;

#[allow(unused_variables)]
extern "C" fn client_callback(s: *mut AvahiClient,
                              state: AvahiClientState,
                              userdata: *mut c_void) {
}

#[allow(unused_variables)]
extern "C" fn browse_callback(service_browser: *mut AvahiServiceBrowser,
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
            let callback: &mut &mut FnMut(ServiceDescription) = mem::transmute(userdata);

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

            callback(service_description);
        },
        _ => println!("{:?}", event),
    }
}

pub struct AvahiWrapper {
    client: RefCell<Option<*mut AvahiClient>>,
    poll: RefCell<Option<*mut AvahiThreadedPoll>>,
    service_browser: RefCell<Option<*mut AvahiServiceBrowser>>,
    service_resolver: RefCell<Option<*mut AvahiServiceResolver>>,
}

impl AvahiWrapper {
    pub fn new() -> AvahiWrapper {
        AvahiWrapper {
            client: RefCell::new(None),
            poll: RefCell::new(None),
            service_browser: RefCell::new(None),
            service_resolver: RefCell::new(None),
        }
    }

    pub fn start_browser<F>(&self, service_type: &str, mut callback: F)
        where F: FnMut(ServiceDescription)
    {
        self.initialize_poll();
        self.initialize_client();

        let mut callback: &mut FnMut(ServiceDescription) = &mut callback;

        let avahi_service_browser = unsafe {
            avahi_service_browser_new(self.client.borrow().unwrap(),
                                      AvahiIfIndex::AVAHI_IF_UNSPEC,
                                      AvahiProtocol::AVAHI_PROTO_UNSPEC,
                                      CString::new(service_type).unwrap().as_ptr(),
                                      ptr::null_mut(),
                                      AvahiLookupFlags::AVAHI_LOOKUP_UNSPEC,
                                      *Box::new(browse_callback),
                                      mem::transmute(&mut callback))
        };

        *self.service_browser.borrow_mut() = Some(avahi_service_browser);

        self.start_polling();
    }

    // pub fn resolve(&self, service: ServiceDescription, mut callback: F)
    //     where F: FnMut(ServiceDescription)
    // {
    //     let avahi_service_resolver = unsafe {
    //         avahi_service_resolver_new(self.client.borrow().unwrap(),
    //                                interface,
    //                                protocol,
    //                                name,
    //                                service_type,
    //                                domain,
    //                                AvahiProtocol::AVAHI_PROTO_UNSPEC,
    //                                AvahiLookupFlags::AVAHI_LOOKUP_UNSPEC,
    //                                *Box::new(resolve_callback),
    //                                userdata)
    //                                }
    // }

    pub fn stop_browser(&self) {
        let mut service_browser = self.service_browser.borrow_mut();
        if service_browser.is_some() {
            unsafe {
                avahi_service_browser_free(service_browser.unwrap());
            };

            *service_browser = None
        }

        let mut client = self.client.borrow_mut();
        if client.is_some() {
            unsafe {
                avahi_client_free(client.unwrap());
            }

            *client = None;
        }

        let mut poll = self.poll.borrow_mut();
        if poll.is_some() {
            unsafe {
                // avahi_threaded_poll_quit(poll.unwrap());
                avahi_threaded_poll_free(poll.unwrap());
            }

            *poll = None;
        }
    }

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
        let mut client_error_code: i32 = 0;
        let poll = self.poll.borrow().unwrap();

        let avahi_client = unsafe {
            avahi_client_new(avahi_threaded_poll_get(poll),
                             AvahiClientFlags::AVAHI_CLIENT_IGNORE_USER_CONFIG,
                             *Box::new(client_callback),
                             ptr::null_mut(),
                             &mut client_error_code)
        };

        // Check that we've created client successfully, otherwise try to resolve error
        // into human-readable string.
        if avahi_client.is_null() {
            let error_string = unsafe {
                free(avahi_client as *mut c_void);
                CStr::from_ptr(avahi_strerror(client_error_code))
            };

            panic!("Failed to create avahi client: {}",
                   error_string.to_str().unwrap());
        }

        *self.client.borrow_mut() = Some(avahi_client);
    }

    fn initialize_poll(&self) {
        let avahi_poll = unsafe { avahi_threaded_poll_new() };

        *self.poll.borrow_mut() = Some(avahi_poll);
    }

    fn start_polling(&self) {
        let poll = self.poll.borrow().unwrap();

        let result_code = unsafe { avahi_threaded_poll_start(poll) };
        if result_code == -1 {
            panic!("Avahi threaded poll could not be started!");
        }
    }
}