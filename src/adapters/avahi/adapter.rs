use std::cell::Cell;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::sync::mpsc;

use libc::c_void;

use bindings::avahi::*;
use discovery::discovery_manager::*;

use adapters::adapter::*;
use adapters::avahi::utils::*;
use adapters::avahi::callbacks::*;

pub struct AvahiAdapter {
    client: Cell<Option<*mut AvahiClient>>,
    poll: Cell<Option<*mut AvahiThreadedPoll>>,
    service_browser: Cell<Option<*mut AvahiServiceBrowser>>,
}

fn avahi_protocol_to_service_protocol(protocol: AvahiProtocol) -> ServiceProtocol {
    match protocol {
        AvahiProtocol::AVAHI_PROTO_INET => return ServiceProtocol::IPv4,
        AvahiProtocol::AVAHI_PROTO_INET6 => return ServiceProtocol::IPv6,
        AvahiProtocol::AVAHI_PROTO_UNSPEC => return ServiceProtocol::Uspecified,
    }
}

fn service_protocol_to_avahi_protocol(protocol: ServiceProtocol) -> AvahiProtocol {
    match protocol {
        ServiceProtocol::IPv4 => return AvahiProtocol::AVAHI_PROTO_INET,
        ServiceProtocol::IPv6 => return AvahiProtocol::AVAHI_PROTO_INET6,
        ServiceProtocol::Uspecified => return AvahiProtocol::AVAHI_PROTO_UNSPEC,
    }
}

fn name_fqdn_to_cname_rdata(name_fqdn: &str) -> Vec<u8> {
    let mut rdata: Vec<u8> = Vec::new();

    for part in name_fqdn.split(".") {
        rdata.push(part.len() as u8);
        rdata.extend_from_slice(part.as_bytes());
    }

    // Push NULL byte.
    rdata.push(0);

    rdata
}

impl AvahiAdapter {
    /// Creates `AvahiClient` instance for the provided `AvahiPoll` object. If there
    /// was an error while creating client, this method will `panic!`
    ///
    /// # Arguments
    ///
    /// * `poll` - Abstracted `AvahiPoll` object that we'd like to create client for.
    fn create_client(&self, poll: *mut AvahiPoll) {
        let mut client_error_code: i32 = 0;

        let avahi_client = unsafe {
            avahi_client_new(poll,
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

        self.client.set(Some(avahi_client));
    }

    /// Initializes `AvahiClient` and `AvahiPoll` objects and runs polling. If client
    /// has been already initialized, this method does nothing.
    fn initialize(&self) {
        if self.client.get().is_some() {
            return;
        }

        // AvahiClient works with abstracted poll object only, so we need both threaded
        // and abstracted polls.
        let (threaded_poll, abstracted_poll) = unsafe {
            let threaded_poll = avahi_threaded_poll_new();
            (threaded_poll, avahi_threaded_poll_get(threaded_poll))
        };

        self.create_client(abstracted_poll);

        let result_code = unsafe { avahi_threaded_poll_start(threaded_poll) };
        if result_code == -1 {
            panic!("Avahi threaded poll could not be started!");
        }

        self.poll.set(Some(threaded_poll));
    }

    fn destroy(&self) {
        let client = self.client.get();
        if client.is_some() {
            // This will remove service browser as well as resolver.
            unsafe {
                avahi_client_free(client.unwrap());
            }

            self.client.set(None);
            self.service_browser.set(None);
        }

        let poll = self.poll.get();
        if poll.is_some() {
            unsafe {
                avahi_threaded_poll_free(poll.unwrap());
            }

            self.poll.set(None);
        }
    }
}

impl DiscoveryAdapter for AvahiAdapter {
    fn start_discovery(&self, service_type: &str, listeners: DiscoveryListeners) {
        self.initialize();

        let (tx, rx) = mpsc::channel::<BrowseCallbackParameters>();

        let service_type = AvahiUtils::to_c_string(service_type.to_owned()).into_raw();

        let avahi_service_browser = unsafe {
            avahi_service_browser_new(self.client.get().unwrap(),
                                      AvahiIfIndex::AVAHI_IF_UNSPEC,
                                      AvahiProtocol::AVAHI_PROTO_UNSPEC,
                                      service_type,
                                      ptr::null_mut(),
                                      AvahiLookupFlags::AVAHI_LOOKUP_UNSPEC,
                                      *Box::new(AvahiCallbacks::browse_callback),
                                      mem::transmute(&tx.clone()))
        };

        self.service_browser.set(Some(avahi_service_browser));

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
                        protocol: avahi_protocol_to_service_protocol(a.protocol),
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
                _ => {
                    // println!("Default {:?}", a.event)
                }
            }
        }

        // Reconstruct string to properly free up memory.
        unsafe { CString::from_raw(service_type) };
    }

    fn resolve(&self, service: ServiceInfo, listeners: ResolveListeners) {
        let (tx, rx) = mpsc::channel::<ResolveCallbackParameters>();

        let avahi_service_resolver = unsafe {
            avahi_service_resolver_new(self.client.get().unwrap(),
                                       service.interface,
                                       service_protocol_to_avahi_protocol(service.protocol),
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
            protocol: avahi_protocol_to_service_protocol(raw_service.protocol),
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

    fn stop_discovery(&self) {
        self.destroy();
        // TODO: service_browser_free hangs for some reason - need to look into it.
        // let mut service_browser = self.service_browser.borrow_mut();
        // if service_browser.is_some() {
        //     unsafe {
        //         avahi_service_browser_free(service_browser.unwrap());
        //     }
        //     *service_browser = None;
        // }
    }
}

impl HostAdapter for AvahiAdapter {
    fn get_name(&self) -> String {
        self.initialize();

        let host_name_ptr = unsafe { avahi_client_get_host_name(self.client.get().unwrap()) };

        AvahiUtils::to_owned_string(host_name_ptr).unwrap()
    }

    fn get_name_fqdn(&self) -> String {
        self.initialize();

        let host_name_fqdn_ptr = unsafe {
            avahi_client_get_host_name_fqdn(self.client.get().unwrap())
        };

        AvahiUtils::to_owned_string(host_name_fqdn_ptr).unwrap()
    }

    fn set_name(&self, host_name: &str) -> String {
        self.initialize();

        if host_name == self.get_name() {
            return host_name.to_owned();
        }

        let client = self.client.get().unwrap();
        let host_name = AvahiUtils::to_c_string(host_name.to_owned()).into_raw();

        let result_code = unsafe { avahi_client_set_host_name(client, host_name) };

        if result_code != 0 {
            let error_string = AvahiUtils::to_owned_string(unsafe { avahi_strerror(result_code) });

            panic!("Failed set host name: {} (code {:?})",
                   error_string.unwrap(),
                   result_code);
        }

        // HACK: Waiting until host name is upgraded, to know for sure what name is assigned.
        // Name can differ from the one we set because of possible collisions.
        // We should wait for the moment when client starts upgrading and only then wait for the
        // RUNNING state.
        let mut registering_triggered = false;
        loop {
            let state = unsafe { avahi_client_get_state(client) };
            match state {
                AvahiClientState::AVAHI_CLIENT_S_REGISTERING => {
                    registering_triggered = true;
                }
                AvahiClientState::AVAHI_CLIENT_S_RUNNING => {
                    if registering_triggered {
                        break;
                    }
                }
                _ => {}
            }
        }

        // Reconstruct string to properly free up memory.
        unsafe { CString::from_raw(host_name) };

        self.get_name()
    }

    fn is_valid_name(&self, host_name: &str) -> bool {
        let host_name = AvahiUtils::to_c_string(host_name.to_owned()).into_raw();

        let is_valid = unsafe { avahi_is_valid_host_name(host_name) };

        // Reconstruct string to properly free up memory.
        unsafe { CString::from_raw(host_name) };

        is_valid == 1
    }

    fn get_alternative_name(&self, host_name: &str) -> String {
        let original_host_name = AvahiUtils::to_c_string(host_name.to_owned()).into_raw();

        let alternative_host_name_ptr = unsafe { avahi_alternative_host_name(original_host_name) };

        let alternative_host_name = AvahiUtils::to_owned_string(alternative_host_name_ptr);

        unsafe { avahi_free(alternative_host_name_ptr as *mut c_void) };

        // Reconstruct string to properly free up memory.
        unsafe { CString::from_raw(original_host_name) };

        alternative_host_name.unwrap()
    }

    fn add_name_alias(&self, host_name: &str) {
        self.initialize();

        if host_name == self.get_name() {
            return;
        }

        let client = self.client.get().unwrap();

        let entry_group = unsafe {
            avahi_entry_group_new(client,
                                  *Box::new(AvahiCallbacks::entry_group_callback),
                                  ptr::null_mut())
        };

        let rdata = &name_fqdn_to_cname_rdata(&self.get_name_fqdn());

        let host_name = AvahiUtils::to_c_string(host_name.to_owned()).into_raw();

        let result_code = unsafe {
            avahi_entry_group_add_record(entry_group,
                                         -1,
                                         AvahiProtocol::AVAHI_PROTO_UNSPEC,
                                         AvahiPublishFlags::AVAHI_PUBLISH_USE_MULTICAST,
                                         host_name,
                                         AvahiRecordClass::AVAHI_IN,
                                         AvahiRecordType::AVAHI_CNAME,
                                         60,
                                         rdata.as_ptr() as *mut _,
                                         rdata.len())
        };


        if result_code != 0 {
            let error_string = AvahiUtils::to_owned_string(unsafe { avahi_strerror(result_code) });

            panic!("Failed to add new entry group record: {} (code {:?})",
                   error_string.unwrap(),
                   result_code);
        }

        let result_code = unsafe { avahi_entry_group_commit(entry_group) };

        if result_code != 0 {
            let error_string = AvahiUtils::to_owned_string(unsafe { avahi_strerror(result_code) });

            panic!("Failed to commit new entry group record: {} (code {:?})",
                   error_string.unwrap(),
                   result_code);
        }

        // Reconstruct string to properly free up memory.
        unsafe { CString::from_raw(host_name) };
    }
}

impl Drop for AvahiAdapter {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl Adapter for AvahiAdapter {
    fn new() -> AvahiAdapter {
        AvahiAdapter {
            client: Cell::new(None),
            poll: Cell::new(None),
            service_browser: Cell::new(None),
        }
    }
}