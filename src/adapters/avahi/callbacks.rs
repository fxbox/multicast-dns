use libc::{c_char, c_int, c_void};

use std::sync::mpsc;

use bindings::avahi::*;

use adapters::avahi::utils::*;

pub struct AvahiCallbacks;

#[derive(Debug)]
pub struct ClientCallbackParameters {
    pub state: AvahiClientState,
}

#[derive(Debug)]
pub struct BrowseCallbackParameters {
    pub event: AvahiBrowserEvent,
    pub interface: i32,
    pub protocol: AvahiProtocol,
    pub name: Option<String>,
    pub service_type: Option<String>,
    pub domain: Option<String>,
    pub flags: AvahiLookupResultFlags,
}

#[derive(Debug)]
pub struct ResolveCallbackParameters {
    pub event: AvahiResolverEvent,
    pub address: Option<String>,
    pub interface: i32,
    pub port: u16,
    pub protocol: AvahiProtocol,
    pub name: Option<String>,
    pub service_type: Option<String>,
    pub domain: Option<String>,
    pub host_name: Option<String>,
    pub txt: Option<String>,
    pub flags: AvahiLookupResultFlags,
}

impl AvahiCallbacks {
    #[allow(unused_variables)]
    pub extern "C" fn client_callback(
        client: *const AvahiClient,
        state: AvahiClientState,
        userdata: *const c_void,
    ) {
        let parameters = ClientCallbackParameters { state };

        debug!("Client state has changed: {:?}.", parameters);

        let sender: Box<mpsc::Sender<ClientCallbackParameters>> =
            unsafe { Box::from_raw(userdata as *mut _) };

        sender.send(parameters).unwrap();

        // Leak pointer to the sender so that it can be reused later.
        Box::into_raw(sender);
    }

    #[allow(unused_variables)]
    pub extern "C" fn browse_callback(
        service_browser: *const AvahiServiceBrowser,
        interface: c_int,
        protocol: AvahiProtocol,
        event: AvahiBrowserEvent,
        name: *const c_char,
        service_type: *const c_char,
        domain: *const c_char,
        flags: AvahiLookupResultFlags,
        userdata: *const c_void,
    ) {
        let parameters = BrowseCallbackParameters {
            event,
            interface,
            protocol,
            name: AvahiUtils::to_owned_string(name),
            service_type: AvahiUtils::to_owned_string(service_type),
            domain: AvahiUtils::to_owned_string(domain),
            flags,
        };

        debug!("Service state has changed: {:?}.", parameters);

        let sender: Box<mpsc::Sender<Option<BrowseCallbackParameters>>> =
            unsafe { Box::from_raw(userdata as *mut _) };

        sender.send(Some(parameters)).unwrap();

        // Leak pointer to the sender so that it can be reused later.
        Box::into_raw(sender);
    }

    #[allow(unused_variables)]
    pub extern "C" fn resolve_callback(
        r: *const AvahiServiceResolver,
        interface: c_int,
        protocol: AvahiProtocol,
        event: AvahiResolverEvent,
        name: *const c_char,
        service_type: *const c_char,
        domain: *const c_char,
        host_name: *const c_char,
        address: *const AvahiAddress,
        port: u16,
        txt: *mut AvahiStringList,
        flags: AvahiLookupResultFlags,
        userdata: *const c_void,
    ) {
        let parameters = ResolveCallbackParameters {
            event,
            address: AvahiUtils::parse_address(address),
            interface,
            protocol,
            port,
            host_name: AvahiUtils::to_owned_string(host_name),
            name: AvahiUtils::to_owned_string(name),
            service_type: AvahiUtils::to_owned_string(service_type),
            domain: AvahiUtils::to_owned_string(domain),
            txt: AvahiUtils::parse_txt(txt),
            flags,
        };

        debug!("Service resolution state has changed: {:?}.", parameters);

        let sender: Box<mpsc::Sender<ResolveCallbackParameters>> =
            unsafe { Box::from_raw(userdata as *mut _) };

        sender.send(parameters).unwrap();
    }

    #[allow(unused_variables)]
    pub extern "C" fn entry_group_callback(
        group: *const AvahiEntryGroup,
        state: AvahiEntryGroupState,
        userdata: *const c_void,
    ) {
        debug!("Entry group state has changed to {:?}.", state);
    }
}
