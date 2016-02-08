#[repr(C)]
#[allow(dead_code)]
pub enum AvahiClientFlags {
    AVAHI_CLIENT_IGNORE_USER_CONFIG,
    AVAHI_CLIENT_NO_FAIL,
}

#[repr(C)]
#[allow(dead_code)]
pub enum AvahiClientState {
    AVAHI_CLIENT_S_REGISTERING,
    AVAHI_CLIENT_S_RUNNING,
    AVAHI_CLIENT_S_COLLISION,
    AVAHI_CLIENT_FAILURE,
    AVAHI_CLIENT_CONNECTING,
}

#[repr(C)]
#[allow(dead_code)]
pub enum AvahiLookupFlags {
    AVAHI_LOOKUP_NO_TXT,
    AVAHI_LOOKUP_NO_ADDRESS,
}

#[repr(C)]
#[allow(dead_code)]
pub enum AvahiLookupResultFlags {
    AVAHI_LOOKUP_RESULT_CACHED,
    AVAHI_LOOKUP_RESULT_WIDE_AREA,
    AVAHI_LOOKUP_RESULT_MULTICAST,
    AVAHI_LOOKUP_RESULT_LOCAL,
    AVAHI_LOOKUP_RESULT_OUR_OWN,
    AVAHI_LOOKUP_RESULT_STATIC,
}

#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum AvahiBrowserEvent {
    AVAHI_BROWSER_NEW,
    AVAHI_BROWSER_REMOVE,
    AVAHI_BROWSER_CACHE_EXHAUSTED,
    AVAHI_BROWSER_ALL_FOR_NOW,
    AVAHI_BROWSER_FAILURE,
}

#[repr(C)]
pub enum AvahiProtocol {
    AVAHI_PROTO_INET = 0,
    AVAHI_PROTO_INET6 = 1,
    AVAHI_PROTO_UNSPEC = -1,
}

#[repr(C)]
pub enum AvahiResolverEvent {
    AVAHI_RESOLVER_FOUND,
    AVAHI_RESOLVER_FAILURE,
}
