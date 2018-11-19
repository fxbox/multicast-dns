#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
pub enum AvahiClientFlags {
    /// Don't read user configuration.
    AVAHI_CLIENT_IGNORE_USER_CONFIG,

    /// Don't fail if the daemon is not available when avahi_client_new() is called,
    /// instead enter AVAHI_CLIENT_CONNECTING state and wait for the daemon to appear.
    AVAHI_CLIENT_NO_FAIL,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug)]
pub enum AvahiClientState {
    AVAHI_CLIENT_S_REGISTERING = 1,
    AVAHI_CLIENT_S_RUNNING = 2,
    AVAHI_CLIENT_S_COLLISION = 3,
    AVAHI_CLIENT_FAILURE = 100,
    AVAHI_CLIENT_CONNECTING = 101,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
pub enum AvahiLookupFlags {
    AVAHI_LOOKUP_UNSPEC = 0,
    /// When doing service resolving, don't lookup TXT record.
    AVAHI_LOOKUP_NO_TXT,

    /// When doing service resolving, don't lookup A/AAAA record.
    AVAHI_LOOKUP_NO_ADDRESS,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug)]
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
#[allow(dead_code, non_camel_case_types)]
pub enum AvahiBrowserEvent {
    AVAHI_BROWSER_NEW,
    AVAHI_BROWSER_REMOVE,
    AVAHI_BROWSER_CACHE_EXHAUSTED,
    AVAHI_BROWSER_ALL_FOR_NOW,
    AVAHI_BROWSER_FAILURE,
}

#[repr(C)]
#[derive(Debug)]
#[allow(dead_code, non_camel_case_types)]
pub enum AvahiProtocol {
    /// IPv4.
    AVAHI_PROTO_INET = 0,

    /// IPv6.
    AVAHI_PROTO_INET6 = 1,

    /// Unspecified/all protocol(s).
    AVAHI_PROTO_UNSPEC = -1,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
pub enum AvahiIfIndex {
    /// Dummy variant to overcome [E0083].
    DUMMY = 0,
    /// Unspecified/all interface(s).
    AVAHI_IF_UNSPEC = -1,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug)]
pub enum AvahiResolverEvent {
    AVAHI_RESOLVER_FOUND,
    AVAHI_RESOLVER_FAILURE,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
pub enum AvahiDomainBrowserType {
    /// Browse for a list of available browsing domains.
    AVAHI_DOMAIN_BROWSER_BROWSE,

    /// Browse for the default browsing domain.
    AVAHI_DOMAIN_BROWSER_BROWSE_DEFAULT,

    /// Browse for a list of available registering domains.
    AVAHI_DOMAIN_BROWSER_REGISTER,

    /// Browse for the default registering domain.
    AVAHI_DOMAIN_BROWSER_REGISTER_DEFAULT,

    /// Legacy browse domain - see DNS-SD spec for more information.
    AVAHI_DOMAIN_BROWSER_BROWSE_LEGACY,

    AVAHI_DOMAIN_BROWSER_MAX,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug)]
pub enum AvahiEntryGroupState {
    AVAHI_ENTRY_GROUP_UNCOMMITED,
    AVAHI_ENTRY_GROUP_REGISTERING,
    AVAHI_ENTRY_GROUP_ESTABLISHED,
    AVAHI_ENTRY_GROUP_COLLISION,
    AVAHI_ENTRY_GROUP_FAILURE,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug)]
pub enum AvahiPublishFlags {
    AVAHI_PUBLISH_UNIQUE = 1,
    AVAHI_PUBLISH_NO_PROBE = 2,
    AVAHI_PUBLISH_NO_ANNOUNCE = 4,
    AVAHI_PUBLISH_ALLOW_MULTIPLE = 8,
    AVAHI_PUBLISH_NO_REVERSE = 16,
    AVAHI_PUBLISH_NO_COOKIE = 32,
    AVAHI_PUBLISH_UPDATE = 64,
    AVAHI_PUBLISH_USE_WIDE_AREA = 128,
    AVAHI_PUBLISH_USE_MULTICAST = 256,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug)]
pub enum AvahiRecordClass {
    AVAHI_IN = 1,
}

#[repr(C)]
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug)]
pub enum AvahiRecordType {
    AVAHI_A = 1,
    AVAHI_NS = 2,
    AVAHI_CNAME = 5,
    AVAHI_SOA = 6,
    AVAHI_PTR = 12,
    AVAHI_HINFO = 13,
    AVAHI_MX = 15,
    AVAHI_TXT = 16,
    AVAHI_AAA = 28,
    AVAHI_SRV = 33,
}
