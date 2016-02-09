#![feature(custom_derive)]
extern crate libc;

mod multicast_dns;

fn main() {
    let mdns_browser = multicast_dns::MulticastDNS::new();

    mdns_browser.list(&format!("_device-info._tcp"));
}