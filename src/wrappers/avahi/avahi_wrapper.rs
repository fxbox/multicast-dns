use std::cell::RefCell;
use std::ffi::CStr;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use libc::{c_char, c_void, c_int, free};

use bindings::avahi::*;
use wrappers::wrapper::Wrapper;
use service_discovery::service_discovery_manager::*;
use wrappers::avahi::avahi_utils::AvahiUtils;


#[derive(Debug)]
struct BrowseCallbackParameters {
    event: AvahiBrowserEvent,
    interface: i32,
    protocol: i32,
    name: Option<String>,
    service_type: Option<String>,
    domain: Option<String>,
    flags: AvahiLookupResultFlags,
}

#[derive(Debug)]
struct ResolveCallbackParameters {
    event: AvahiResolverEvent,
    address: Option<String>,
    interface: i32,
    port: u16,
    protocol: i32,
    name: Option<String>,
    service_type: Option<String>,
    domain: Option<String>,
    host_name: Option<String>,
    txt: Option<String>,
    flags: AvahiLookupResultFlags,
}

#[allow(unused_variables)]
extern "C" fn client_callback(s: *mut AvahiClient,
                              state: AvahiClientState,
                              userdata: *mut c_void) {
    println!("Client state changed: {:?}", state);
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

    let sender = unsafe {
        mem::transmute::<*mut c_void, &Sender<BrowseCallbackParameters>>(userdata)
    };

    let parameters = BrowseCallbackParameters {
        event: event,
        interface: interface,
        protocol: protocol,
        name: AvahiUtils::parse_c_string(name),
        service_type: AvahiUtils::parse_c_string(service_type),
        domain: AvahiUtils::parse_c_string(domain),
        flags: flags,
    };

    sender.send(parameters).unwrap();
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

    let sender = unsafe {
        mem::transmute::<*mut c_void, &Sender<ResolveCallbackParameters>>(userdata)
    };

    let parameters = ResolveCallbackParameters {
        event: event,
        address: AvahiUtils::parse_address(address),
        interface: interface,
        protocol: protocol,
        port: port,
        host_name: AvahiUtils::parse_c_string(host_name),
        name: AvahiUtils::parse_c_string(name),
        service_type: AvahiUtils::parse_c_string(service_type),
        domain: AvahiUtils::parse_c_string(domain),
        txt: AvahiUtils::parse_txt(txt),
        flags: flags,
    };

    sender.send(parameters).unwrap();
}

pub struct AvahiWrapper {
    client: RefCell<Option<*mut AvahiClient>>,
    poll: RefCell<Option<*mut AvahiThreadedPoll>>,
    service_browser: RefCell<Option<*mut AvahiServiceBrowser>>,
}

impl AvahiWrapper {
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

impl Wrapper for AvahiWrapper {
    fn new() -> AvahiWrapper {
        AvahiWrapper {
            client: RefCell::new(None),
            poll: RefCell::new(None),
            service_browser: RefCell::new(None),
        }
    }

    fn start_browser(&self, service_type: &str, listeners: DiscoveryListeners) {
        self.initialize_poll();
        self.initialize_client();

        let (tx, rx) = channel::<BrowseCallbackParameters>();

        let userdata = unsafe {
            mem::transmute::<&Sender<BrowseCallbackParameters>, *mut c_void>(&tx)
        };

        let avahi_service_browser = unsafe {
            avahi_service_browser_new(self.client.borrow().unwrap(),
                                      AvahiIfIndex::AVAHI_IF_UNSPEC,
                                      AvahiProtocol::AVAHI_PROTO_UNSPEC,
                                      CString::new(service_type).unwrap().as_ptr(),
                                      ptr::null_mut(),
                                      AvahiLookupFlags::AVAHI_LOOKUP_UNSPEC,
                                      *Box::new(browse_callback),
                                      userdata)
        };

        *self.service_browser.borrow_mut() = Some(avahi_service_browser);

        self.start_polling();

        for a in rx.iter() {
            match a.event {
                AvahiBrowserEvent::AVAHI_BROWSER_NEW => {
                    let service = ServiceDescription {
                        address: &"",
                        domain: &a.domain.unwrap(),
                        host_name: &"",
                        interface: a.interface,
                        name: &a.name.unwrap(),
                        port: 0,
                        protocol: a.protocol,
                        txt: &"",
                        type_name: service_type,
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

    fn resolve(&self, service: ServiceDescription, listeners: ResolveListeners) {
        let (tx, rx) = channel::<ResolveCallbackParameters>();

        let userdata = unsafe {
            mem::transmute::<&Sender<ResolveCallbackParameters>, *mut c_void>(&tx)
        };

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
                                       userdata)
        };

        let raw_service = rx.recv().unwrap();

        let service = ServiceDescription {
            address: &raw_service.address.unwrap(),
            domain: &raw_service.domain.unwrap(),
            host_name: &raw_service.host_name.unwrap(),
            interface: raw_service.interface,
            name: &raw_service.name.unwrap(),
            port: raw_service.port,
            protocol: raw_service.protocol,
            txt: &raw_service.txt.unwrap(),
            type_name: &raw_service.service_type.unwrap(),
        };

        if listeners.on_service_resolved.is_some() {
            (*listeners.on_service_resolved.unwrap())(service);
        }

        unsafe {
            avahi_service_resolver_free(avahi_service_resolver);
        }
    }

    fn stop_browser(&self) {
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
}