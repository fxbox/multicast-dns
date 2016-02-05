#![feature(custom_derive)]
extern crate libc;

mod multicast_dns;
use multicast_dns::lib::MulticastDNS;

fn print_service_name(service_name: &str) {
    println!("Service name {}", service_name);
}

fn main() {
    let mut mdns_browser = MulticastDNS::new();
    
    mdns_browser.list(
        format!("_http._tcp"),
        print_service_name
    );
}