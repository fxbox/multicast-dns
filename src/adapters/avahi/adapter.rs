use std::cell::Cell;
use std::ffi::CString;
use std::ptr;
use std::sync::mpsc;

use libc::c_void;

use bindings::avahi::*;
use discovery::discovery_manager::*;

use adapters::adapter::*;
use adapters::avahi::utils::*;
use adapters::avahi::callbacks::*;

pub struct Channel<T> {
    pub receiver: mpsc::Receiver<T>,
    pub sender: mpsc::Sender<T>,
}

pub struct AvahiAdapter {
    poll: Cell<Option<*mut AvahiThreadedPoll>>,

    client: Cell<Option<*mut AvahiClient>>,
    client_channel: Channel<ClientCallbackParameters>,

    service_browser: Cell<Option<*mut AvahiServiceBrowser>>,
    service_browser_channel: Channel<Option<BrowseCallbackParameters>>,
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

        let sender = Box::new(self.client_channel.sender.clone());
        let avahi_client = unsafe {
            avahi_client_new(poll,
                             AvahiClientFlags::AVAHI_CLIENT_IGNORE_USER_CONFIG,
                             *Box::new(AvahiCallbacks::client_callback),
                             Box::into_raw(sender) as *mut _ as *mut c_void,
                             &mut client_error_code)
        };

        // Check that we've created client successfully, otherwise try to resolve error
        // into human-readable string.
        if avahi_client.is_null() {
            let error_string = AvahiUtils::to_owned_string(unsafe {
                avahi_strerror(client_error_code)
            });

            panic!("Failed to create avahi client: {}.", error_string.unwrap());
        }

        for message in self.client_channel.receiver.iter() {
            if let AvahiClientState::AVAHI_CLIENT_S_RUNNING = message.state {
                break;
            }
        }

        self.client.set(Some(avahi_client));

        debug!("Client is created.");
    }

    /// Initializes `AvahiClient` and `AvahiPoll` objects and runs polling. If client
    /// has been already initialized, this method does nothing.
    fn initialize(&self) {
        if self.client.get().is_some() {
            return;
        }

        debug!("New client initialization is requested.");

        // AvahiClient works with abstracted poll object only, so we need both threaded
        // and abstracted polls.
        let (threaded_poll, abstracted_poll) = unsafe {
            let threaded_poll = avahi_threaded_poll_new();
            (threaded_poll, avahi_threaded_poll_get(threaded_poll))
        };

        debug!("Threaded poll is created.");

        self.create_client(abstracted_poll);

        let result_code = unsafe { avahi_threaded_poll_start(threaded_poll) };
        if result_code == -1 {
            panic!("Avahi threaded poll could not be started!");
        }

        self.poll.set(Some(threaded_poll));
    }

    fn destroy(&self) {
        debug!("Adapter is going to be destroyed.");

        let client = self.client.get();
        if client.is_some() {
            // This will remove service browser as well as resolver.
            unsafe {
                let avahi_client = client.unwrap();

                // Free memory from our custom userdata.
                Box::from_raw((*avahi_client).userdata);

                avahi_client_free(avahi_client);
            }

            self.client.set(None);
            self.service_browser.set(None);

            debug!("Client instance is destroyed.");
        }

        let poll = self.poll.get();
        if poll.is_some() {
            unsafe {
                avahi_threaded_poll_free(poll.unwrap());
            }

            self.poll.set(None);

            debug!("Poll instance is destroyed.");
        }
    }
}

impl DiscoveryAdapter for AvahiAdapter {
    fn start_discovery(&self, service_type: &str, listeners: DiscoveryListeners) {
        debug!("Discovery started for the service: {}.", service_type);

        self.initialize();

        let service_type = AvahiUtils::to_c_string(service_type.to_owned()).into_raw();
        let sender = Box::new(self.service_browser_channel.sender.clone());

        let avahi_service_browser = unsafe {
            avahi_service_browser_new(self.client.get().unwrap(),
                                      AvahiIfIndex::AVAHI_IF_UNSPEC,
                                      AvahiProtocol::AVAHI_PROTO_UNSPEC,
                                      service_type,
                                      ptr::null_mut(),
                                      AvahiLookupFlags::AVAHI_LOOKUP_UNSPEC,
                                      *Box::new(AvahiCallbacks::browse_callback),
                                      Box::into_raw(sender) as *mut _ as *mut c_void)
        };

        self.service_browser.set(Some(avahi_service_browser));

        for message in self.service_browser_channel.receiver.iter() {
            if message.is_none() {
                break;
            }

            let parameters = message.unwrap();

            match parameters.event {
                AvahiBrowserEvent::AVAHI_BROWSER_NEW => {
                    let service = ServiceInfo {
                        address: None,
                        domain: parameters.domain,
                        host_name: None,
                        interface: parameters.interface,
                        name: parameters.name,
                        port: 0,
                        protocol: avahi_protocol_to_service_protocol(parameters.protocol),
                        txt: None,
                        type_name: parameters.service_type,
                    };

                    if listeners.on_service_discovered.is_some() {
                        (*listeners.on_service_discovered.unwrap())(service);
                    }
                }
                AvahiBrowserEvent::AVAHI_BROWSER_ALL_FOR_NOW => {
                    if listeners.on_all_discovered.is_some() {
                        (*listeners.on_all_discovered.unwrap())();
                    }
                }
                AvahiBrowserEvent::AVAHI_BROWSER_FAILURE => {
                    let error_code = unsafe { avahi_client_errno(self.client.get().unwrap()) };

                    if error_code != 0 {
                        let error_string = AvahiUtils::to_owned_string(unsafe {
                            avahi_strerror(error_code)
                        });

                        error!("Service browser failed: {} (code {:?})",
                               error_string.unwrap(),
                               error_code);
                    } else {
                        error!("Service browser failed because of unknown reason.");
                    }
                }
                _ => {}
            }
        }

        // Reconstruct string to properly free up memory.
        unsafe { CString::from_raw(service_type) };
    }

    fn resolve(&self, service: ServiceInfo, listeners: ResolveListeners) {
        debug!("Resolution is requested for service: {:?}.", service);

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
                                       Box::into_raw(Box::new(tx)) as *mut _ as *mut c_void)
        };

        for message in rx.iter() {
            if let AvahiResolverEvent::AVAHI_RESOLVER_FOUND = message.event {
                let service = ServiceInfo {
                    address: message.address,
                    domain: message.domain,
                    host_name: message.host_name,
                    interface: message.interface,
                    name: message.name,
                    port: message.port,
                    protocol: avahi_protocol_to_service_protocol(message.protocol),
                    txt: message.txt,
                    type_name: message.service_type,
                };

                if listeners.on_service_resolved.is_some() {
                    (*listeners.on_service_resolved.unwrap())(service);
                }

                break;
            }
        }

        unsafe {
            avahi_service_resolver_free(avahi_service_resolver);
        }
    }

    fn stop_discovery(&self) {
        let service_browser = self.service_browser.get();
        if service_browser.is_some() {
            unsafe {
                let avahi_service_browser = service_browser.unwrap();

                // Free memory from our custom userdata.
                Box::from_raw((*avahi_service_browser).userdata);

                avahi_service_browser_free(avahi_service_browser);
            }
            self.service_browser.set(None);

            self.service_browser_channel.sender.send(None).unwrap();
        }
    }
}

impl HostAdapter for AvahiAdapter {
    fn get_name(&self) -> String {
        debug!("Host name is requested.");

        self.initialize();

        let host_name_ptr = unsafe { avahi_client_get_host_name(self.client.get().unwrap()) };
        let host_name = AvahiUtils::to_owned_string(host_name_ptr).unwrap();

        debug!("Host name is {}.", host_name);

        host_name
    }

    fn get_name_fqdn(&self) -> String {
        debug!("Host name FQDN is requested.");

        self.initialize();

        let host_name_fqdn_ptr = unsafe {
            avahi_client_get_host_name_fqdn(self.client.get().unwrap())
        };

        let host_name_fqdn = AvahiUtils::to_owned_string(host_name_fqdn_ptr).unwrap();

        debug!("Host name FQDN is {}.", host_name_fqdn);

        host_name_fqdn
    }

    fn set_name(&self, host_name: &str) -> String {
        debug!("Host name change (-> {}) is requested.", host_name);

        self.initialize();

        if host_name == self.get_name() {
            debug!("No need to change name, name is already set.");
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

        debug!("Waiting for the name to be applied.");

        for message in self.client_channel.receiver.iter() {
            if let AvahiClientState::AVAHI_CLIENT_S_RUNNING = message.state {
                break;
            }
        }

        debug!("Host name is successfully updated.");

        // Reconstruct string to properly free up memory.
        unsafe { CString::from_raw(host_name) };

        self.get_name()
    }

    fn is_valid_name(&self, host_name: &str) -> bool {
        debug!("Host name {:?} validation is requested.", host_name);

        let host_name = AvahiUtils::to_c_string(host_name.to_owned()).into_raw();

        let is_valid = unsafe { avahi_is_valid_host_name(host_name) } == 1;

        debug!("Host name is valid: {:?}.", is_valid);

        // Reconstruct string to properly free up memory.
        unsafe { CString::from_raw(host_name) };

        is_valid
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
        let (client_sender, client_receiver) = mpsc::channel::<ClientCallbackParameters>();

        let (service_browser_sender, service_browser_receiver) =
            mpsc::channel::<Option<BrowseCallbackParameters>>();

        AvahiAdapter {
            poll: Cell::new(None),

            client: Cell::new(None),
            client_channel: Channel {
                receiver: client_receiver,
                sender: client_sender,
            },

            service_browser: Cell::new(None),
            service_browser_channel: Channel {
                receiver: service_browser_receiver,
                sender: service_browser_sender,
            },
        }
    }
}