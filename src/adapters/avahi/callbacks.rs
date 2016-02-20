use libc::{c_char, c_int, c_void};

use std::mem;
use std::sync::mpsc;

use bindings::avahi::*;

use adapters::avahi::utils::*;

pub struct AvahiCallbacks;

#[derive(Debug)]
pub struct BrowseCallbackParameters {
    pub event: AvahiBrowserEvent,
    pub interface: i32,
    pub protocol: i32,
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
    pub protocol: i32,
    pub name: Option<String>,
    pub service_type: Option<String>,
    pub domain: Option<String>,
    pub host_name: Option<String>,
    pub txt: Option<String>,
    pub flags: AvahiLookupResultFlags,
}

impl AvahiCallbacks {
    #[allow(unused_variables)]
    pub extern "C" fn client_callback(s: *mut AvahiClient,
                                      state: AvahiClientState,
                                      userdata: *mut c_void) {
        println!("Client state changed: {:?}", state);
    }

    #[allow(unused_variables)]
    pub extern "C" fn browse_callback(service_browser: *mut AvahiServiceBrowser,
                                      interface: c_int,
                                      protocol: c_int,
                                      event: AvahiBrowserEvent,
                                      name: *const c_char,
                                      service_type: *const c_char,
                                      domain: *const c_char,
                                      flags: AvahiLookupResultFlags,
                                      userdata: *mut c_void) {
        println!("Browse callback is called: {:?}", event);

        let parameters = BrowseCallbackParameters {
            event: event,
            interface: interface,
            protocol: protocol,
            name: AvahiUtils::to_owned_string(name),
            service_type: AvahiUtils::to_owned_string(service_type),
            domain: AvahiUtils::to_owned_string(domain),
            flags: flags,
        };

        let sender: &mpsc::Sender<BrowseCallbackParameters> = unsafe { mem::transmute(userdata) };
        println!("Before send");
        sender.send(parameters).unwrap();
        println!("After send");
    }

    #[allow(unused_variables)]
    pub extern "C" fn resolve_callback(r: *mut AvahiServiceResolver,
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
        println!("Resolve callback is called: {:?}", event);

        let parameters = ResolveCallbackParameters {
            event: event,
            address: AvahiUtils::parse_address(address),
            interface: interface,
            protocol: protocol,
            port: port,
            host_name: AvahiUtils::to_owned_string(host_name),
            name: AvahiUtils::to_owned_string(name),
            service_type: AvahiUtils::to_owned_string(service_type),
            domain: AvahiUtils::to_owned_string(domain),
            txt: AvahiUtils::parse_txt(txt),
            flags: flags,
        };

        let sender: &mpsc::Sender<ResolveCallbackParameters> = unsafe { mem::transmute(userdata) };
        sender.send(parameters).unwrap();
    }
}