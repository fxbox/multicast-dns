# Multicast DNS

[![Build Status](https://travis-ci.org/fxbox/multicast-dns.svg?branch=master)](https://travis-ci.org/fxbox/multicast-dns)

```multicust_dns``` - is essentially a Rust wrapper around Avahi that internally uses AvahiDaemon to manage host name and browse services on the local network.

Requires ```avahi-common```, ```avahi-client``` and ```dbus-1``` libs to compile sucessfully.

For non-linux platforms that don't have required avahi libs, fake implementation is used. 

See [Multicast DNS Utils](https://github.com/fxbox/multicast-dns-utils) command line app as an example.

Examples (see `./examples` folder):

```rust
extern crate multicast_dns;
use multicast_dns::host::HostManager;

fn main() {
    let host_name = format!("custom-host");

    let host_manager = HostManager::new();

    if !host_manager.is_valid_name(&host_name).unwrap() {
        panic!("Host name `{}` is not a valid host name!", &host_name);
    }

    // The `new_host_name` can be different from the one we are trying to set,
    // due to possible collisions that may happen.
    let new_host_name = host_manager.set_name(&host_name).unwrap();

    println!("New host name is: {:?}", &new_host_name);
}
```

or

```rust
extern crate multicast_dns;
use multicast_dns::discovery::*;

fn main() {
    let service_type = format!("_device-info._tcp");
    
    let discovery_manager = DiscoveryManager::new();

    let on_service_resolved = |service: ServiceInfo| {
        println!("Service resolved: {:?}", service);
    };

    let on_service_discovered = |service: ServiceInfo| {
        println!("Service discovered: {:?}", service);

        let resolve_listeners = ResolveListeners {
            on_service_resolved: Some(&on_service_resolved),
        };

        discovery_manager.resolve_service(service, resolve_listeners);
    };

    let on_all_discovered = || {
        println!("All services has been discovered");
    };

    let discovery_listeners = DiscoveryListeners {
        on_service_discovered: Some(&on_service_discovered),
        on_all_discovered: Some(&on_all_discovered),
    };

    discovery_manager.discover_services(&service_type, discovery_listeners);
}
```

Look at [RFC 6762](https://tools.ietf.org/html/rfc6762) and [RFC 6763](https://tools.ietf.org/html/rfc6763) for the standard specifications.

Also one can take a look at [Service Name and Transport Protocol Port Number Registry](http://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml) to see currently available and registered services.