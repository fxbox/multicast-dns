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
use adapters::avahi::errors::Error as AvahiError;
use adapters::errors::Error as AdapterError;

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
    /// was an error while creating client, corresponding error will be returned.
    ///
    /// # Arguments
    ///
    /// * `poll` - Abstracted `AvahiPoll` object that we'd like to create client for.
    fn create_client(&self, poll: *mut AvahiPoll) -> Result<(), AvahiError> {
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
        if client_error_code != 0 || avahi_client.is_null() {
            return Err(AvahiError::fromErrorCode(client_error_code));
        }

        for message in self.client_channel.receiver.iter() {
            if let AvahiClientState::AVAHI_CLIENT_S_RUNNING = message.state {
                break;
            }
        }

        self.client.set(Some(avahi_client));

        debug!("Client is created.");
        Ok(())
    }

    /// Initializes `AvahiClient` and `AvahiPoll` objects and runs polling. If client
    /// has been already initialized, this method does nothing.
    fn initialize(&self) -> Result<(), AvahiError> {
        if self.client.get().is_some() {
            return Ok(());
        }

        debug!("New client initialization is requested.");

        // AvahiClient works with abstracted poll object only, so we need both threaded
        // and abstracted polls.
        let (threaded_poll, abstracted_poll) = unsafe {
            let threaded_poll = avahi_threaded_poll_new();
            (threaded_poll, avahi_threaded_poll_get(threaded_poll))
        };

        debug!("Threaded poll is created.");

        try!(self.create_client(abstracted_poll));

        let result_code = unsafe { avahi_threaded_poll_start(threaded_poll) };
        if result_code != 0 {
            return Err(AvahiError::fromErrorCode(result_code));
        }

        self.poll.set(Some(threaded_poll));

        Ok(())
    }

    fn destroy(&self) {
        debug!("Avahi adapter is going to be dropped.");

        let client = self.client.get();
        if client.is_some() {
            let avahi_poll = self.poll.get().unwrap();
            let avahi_client = client.unwrap();

            unsafe {
                avahi_threaded_poll_stop(avahi_poll);
                debug!("Avahi threaded poll has been stopped successfully.");

                // Free memory from our custom userdata.
                Box::from_raw((*avahi_client).userdata);

                // This will remove service browser as well as resolver.
                avahi_client_free(avahi_client);
                debug!("Avahi client has been destroyed successfully.");

                avahi_threaded_poll_free(avahi_poll);
                debug!("Avahi threaded poll has been destroyed successfully.");
            }

            self.poll.set(None);
            self.client.set(None);
            self.service_browser.set(None);

            debug!("Avahi adapter has been dropped successfully.");
        }
    }
}

impl DiscoveryAdapter for AvahiAdapter {
    fn start_discovery(&self, service_type: &str, listeners: DiscoveryListeners)
        -> Result<(), AdapterError>{
        debug!("Discovery started for the service: {}.", service_type);

        try!(self.initialize());

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
                    error!("Service browser failed: {}", AvahiError::fromErrorCode(error_code));
                }
                _ => {}
            }
        }

        // Reconstruct string to properly free up memory.
        unsafe { CString::from_raw(service_type) };

        Ok(())
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
            let avahi_service_browser = service_browser.unwrap();
            unsafe {
                // Free memory from our custom userdata.
                Box::from_raw((*avahi_service_browser).userdata);

                avahi_service_browser_free(avahi_service_browser);
                debug!("Avahi service browser has been destroyed successfully.");
            }

            self.service_browser.set(None);
            self.service_browser_channel.sender.send(None).unwrap();
        }
    }
}

impl HostAdapter for AvahiAdapter {
    fn get_name(&self) -> Result<String, AdapterError> {
        debug!("Host name is requested.");

        try!(self.initialize());

        let host_name_ptr = unsafe { avahi_client_get_host_name(self.client.get().unwrap()) };

        AvahiUtils::to_owned_string(host_name_ptr)
            .ok_or(AdapterError::Internal("Name is not available".to_owned()))

        Ok(host_name)
    }

    fn get_name_fqdn(&self) -> Result<String, AdapterError> {
        debug!("Host name FQDN is requested.");

        try!(self.initialize());

        let host_name_fqdn_ptr = unsafe {
            avahi_client_get_host_name_fqdn(self.client.get().unwrap())
        };

        AvahiUtils::to_owned_string(host_name_fqdn_ptr)
            .ok_or(AdapterError::Internal("Name is not available".to_owned()))
    }

    fn set_name(&self, host_name: &str) -> Result<String, AdapterError> {
        debug!("Host name change (-> {}) is requested.", host_name);

        try!(self.initialize());
        let current_host_name = try!(self.get_name());

        if host_name == current_host_name {
            debug!("No need to change name, name is already set.");
            return Ok(host_name.to_owned());
        }

        let client = self.client.get().unwrap();
        let host_name = AvahiUtils::to_c_string(host_name.to_owned()).into_raw();

        let result_code = unsafe { avahi_client_set_host_name(client, host_name) };
        if result_code != 0 {
            return Err(From::from(AvahiError::fromErrorCode(result_code)));
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

    fn is_valid_name(&self, host_name: &str) -> Result<bool, AdapterError> {
        debug!("Host name {:?} validation is requested.", host_name);

        let host_name = AvahiUtils::to_c_string(host_name.to_owned()).into_raw();

        let is_valid = unsafe { avahi_is_valid_host_name(host_name) } == 1;

        debug!("Host name is valid: {:?}.", is_valid);

        // Reconstruct string to properly free up memory.
        unsafe { CString::from_raw(host_name) };

        Ok(is_valid)
    }

    fn get_alternative_name(&self, host_name: &str) -> Result<String, AdapterError> {
        let original_host_name = AvahiUtils::to_c_string(host_name.to_owned()).into_raw();

        let alternative_host_name_ptr = unsafe { avahi_alternative_host_name(original_host_name) };

        let alternative_host_name = AvahiUtils::to_owned_string(alternative_host_name_ptr);

        unsafe { avahi_free(alternative_host_name_ptr as *mut c_void) };

        // Reconstruct string to properly free up memory.
        unsafe { CString::from_raw(original_host_name) };

        alternative_host_name
            .ok_or(AdapterError::Internal("Name is not available".to_owned()))
    }

    fn add_name_alias(&self, host_name: &str) -> Result<(), AdapterError> {
        try!(self.initialize());

        let current_host_name = try!(self.get_name());
        if host_name == current_host_name {
            return Ok(());
        }

        let client = self.client.get().unwrap();

        let entry_group = unsafe {
            avahi_entry_group_new(client,
                                  *Box::new(AvahiCallbacks::entry_group_callback),
                                  ptr::null_mut())
        };

        let rdata = &name_fqdn_to_cname_rdata(&try!(self.get_name_fqdn()));

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
            let error = AvahiError::fromErrorCode(result_code);
            error!("Failed to add new entry group record: {}", error);
            return Err(From::from(error));
        }

        let result_code = unsafe { avahi_entry_group_commit(entry_group) };
        if result_code != 0 {
            let error = AvahiError::fromErrorCode(result_code);
            error!("Failed to commit new entry group record: {}", error);
            return Err(From::from(error));
        }

        // Reconstruct string to properly free up memory.
        unsafe { CString::from_raw(host_name) };

        Ok(())
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