use std::cell::RefCell;
use std::ffi::CStr;
use std::ffi::CString;
use std::mem;
use std::ptr;

use libc::{c_char, c_void, c_int, free};

use bindings::avahi::*;
use service_discovery::service_discovery_manager::ServiceDescription;
use service_discovery::service_discovery_manager::DiscoveryListener;

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
    let listener: &mut DiscoveryListener = unsafe { mem::transmute(userdata) };

    match event {
        AvahiBrowserEvent::AVAHI_BROWSER_NEW => unsafe {
            let service_description = ServiceDescription {
                address: &"",
                domain: CStr::from_ptr(domain).to_str().unwrap(),
                host_name: &"",
                interface: interface,
                name: CStr::from_ptr(name).to_str().unwrap(),
                port: 0,
                protocol: protocol,
                txt: &"",
                type_name: CStr::from_ptr(service_type).to_str().unwrap(),
            };

            (*listener.on_service_found)(service_description);
        },
        AvahiBrowserEvent::AVAHI_BROWSER_ALL_FOR_NOW => {
            (*listener.on_all_discovered)();
        }
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

            let (address, domain, host_name, name, service_type, txt) = unsafe {
                avahi_address_snprint(address_vector, AVAHI_ADDRESS_STR_MAX, address);

                let txt_pointer = avahi_string_list_to_string(txt);
                let txt = CStr::from_ptr(txt_pointer).to_string_lossy().into_owned();
                avahi_free(txt_pointer as *mut c_void);

                (CStr::from_ptr(address_vector),
                 CStr::from_ptr(domain),
                 CStr::from_ptr(host_name),
                 CStr::from_ptr(name),
                 CStr::from_ptr(service_type),
                 txt)
            };

            let service_description = ServiceDescription {
                address: address.to_str().unwrap(),
                domain: domain.to_str().unwrap(),
                host_name: host_name.to_str().unwrap(),
                interface: interface,
                name: name.to_str().unwrap(),
                port: port,
                protocol: protocol,
                txt: &txt,
                type_name: service_type.to_str().unwrap(),
            };

            let callback: Box<Box<Fn(ServiceDescription)>> = unsafe {
                Box::from_raw(userdata as *mut _)
            };

            callback(service_description);
        }
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

    pub fn start_browser(&self, service_type: &str, mut listener: DiscoveryListener) {
        self.initialize_poll();
        self.initialize_client();

        let avahi_service_browser = unsafe {
            avahi_service_browser_new(self.client.borrow().unwrap(),
                                      AvahiIfIndex::AVAHI_IF_UNSPEC,
                                      AvahiProtocol::AVAHI_PROTO_UNSPEC,
                                      CString::new(service_type).unwrap().as_ptr(),
                                      ptr::null_mut(),
                                      AvahiLookupFlags::AVAHI_LOOKUP_UNSPEC,
                                      *Box::new(browse_callback),
                                      mem::transmute(&mut listener))
        };

        *self.service_browser.borrow_mut() = Some(avahi_service_browser);

        self.start_polling();
    }

    pub fn resolve<F>(&self, service: ServiceDescription, callback: F)
        where F: Fn(ServiceDescription)
    {
        let callback: Box<Box<Fn(ServiceDescription)>> = Box::new(Box::new(callback));

        let avahi_service_resolver = unsafe {
            avahi_service_resolver_new(self.client.borrow().unwrap(),
                                       service.interface,
                                       service.protocol,
                                       CString::new(service.name).unwrap().as_ptr(),
                                       CString::new(service.type_name).unwrap().as_ptr(),
                                       CString::new(service.domain).unwrap().as_ptr(),
                                       AvahiProtocol::AVAHI_PROTO_UNSPEC,
                                       AvahiLookupFlags::AVAHI_LOOKUP_UNSPEC,
                                       *Box::new(resolve_callback),
                                       Box::into_raw(callback) as *mut c_void)
        };

        *self.service_resolver.borrow_mut() = Some(avahi_service_resolver);
    }

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