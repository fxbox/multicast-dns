use super::types::*;
use super::enums::*;
use libc::{c_void, c_int, c_char, size_t};

#[link(name = "avahi-common")]
#[link(name = "avahi-client")]
#[link(name = "dbus-1")]
#[allow(improper_ctypes)]
extern "C" {
    /// Create a new main loop object.
    ///
    /// # Return value
    ///
    /// Main loop object - `AvahiSimplePoll`.
    pub fn avahi_simple_poll_new() -> *mut AvahiSimplePoll;

    /// Return the abstracted poll API object for this main loop object.
    /// The is will return the same pointer each time it is called.
    ///
    /// # Arguments
    ///
    /// * `simple_poll` - Main loop object returned from `avahi_simple_poll_new`.
    ///
    /// # Return value
    ///
    /// Abstracted poll API object - `AvahiPoll`.
    pub fn avahi_simple_poll_get(simple_poll: *mut AvahiSimplePoll) -> *mut AvahiPoll;

    /// Call `avahi_simple_poll_iterate` in a loop and return if it
    /// returns non-zero.
    ///
    /// # Arguments
    ///
    /// * `simple_poll` - Main loop object returned from `avahi_simple_poll_new`.
    ///
    /// # Return value
    ///
    /// Non-zero if `avahi_simple_poll_iterate` return non-zero value.
    pub fn avahi_simple_poll_loop(simple_poll: *mut AvahiSimplePoll) -> c_int;

    /// Free a main loop object.
    ///
    /// # Arguments
    ///
    /// * `simple_poll` - Main loop object returned from `avahi_simple_poll_new`.
    pub fn avahi_simple_poll_free(simple_poll: *mut AvahiSimplePoll);

    /// Creates a new client instance.
    ///
    /// # Arguments
    /// * `poll_api` - The abstract event loop API to use.
    /// * `flags` -	Some flags to modify the behaviour of the client library.
    /// * `callback` - A callback that is called whenever the state of the client changes.
    ///                This may be NULL. Please note that this function is called for the
    ///                first time from within the avahi_client_new() context! Thus, in the
    ///                callback you should not make use of global variables that are initialized
    ///                only after your call to avahi_client_new(). A common mistake is to store
    ///                the AvahiClient pointer returned by avahi_client_new() in a global
    ///                variable and assume that this global variable already contains the valid
    ///                pointer when the callback is called for the first time. A work-around for
    ///                this is to always use the AvahiClient pointer passed to the callback
    ///                function instead of the global pointer.
    /// * `userdata` - Some arbitrary user data pointer that will be passed to the callback.
    /// * `error` -	If creation of the client fails, this integer will contain the error cause.
    ///             May be NULL if you aren't interested in the reason why `avahi_client_new()`
    ///             has failed.
    ///
    /// # Return value
    ///
    /// New client instance - `AvahiClient`.
    pub fn avahi_client_new(poll_api: *const AvahiPoll,
                            flags: AvahiClientFlags,
                            callback: extern "C" fn(*mut AvahiClient,
                                                    AvahiClientState,
                                                    *mut c_void)
                                                   ,
                            userdata: *mut c_void,
                            error: *mut c_int)
                            -> *mut AvahiClient;


    /// Free a client instance.
    /// This will automatically free all associated browser, resolve and entry group objects.
    /// All pointers to such objects become invalid!
    ///
    /// # Arguments
    ///
    /// * `client` - Active `AvahiClient` instance.
    pub fn avahi_client_free(client: *mut AvahiClient);

    pub fn avahi_client_get_host_name(client: *mut AvahiClient) -> *const c_char;

    pub fn avahi_client_set_host_name(client: *mut AvahiClient, name: *const c_char) -> c_int;

    pub fn avahi_is_valid_host_name(host_name: *const c_char) -> c_int;

    pub fn avahi_alternative_host_name(host_name: *const c_char) -> *const c_char;

    pub fn avahi_client_get_host_name_fqdn(client: *mut AvahiClient) -> *const c_char;

    pub fn avahi_client_get_state(client: *mut AvahiClient) -> AvahiClientState;

    /// Browse for domains on the local network.
    ///
    /// # Arguments
    ///
    /// * `client` - Active `AvahiClient` instance.
    /// * `interface` - Numeric network interface index. Takes OS dependent values and
    ///                 the special constant AVAHI_IF_UNSPEC (-1).
    /// * `protocol` - Protocol family specification `AvahiProtocol`.
    /// * `domain` - Domain to look for.
    /// * `service_type` - The type of domain to browse for `AvahiDomainBrowserType`.
    /// * `flags` - Flags for lookup functions `AvahiLookupFlags`.
    /// * `callback` - `AvahiDomainBrowserCallback` callback to be called for every new
    ///                found service.
    /// * `userdata` - Some arbitrary user data pointer that will be passed to the callback.
    ///
    /// # Return value
    ///
    /// A domain browser `AvahiServiceBrowser` object.
    pub fn avahi_service_browser_new(client: *mut AvahiClient,
                                     interface: AvahiIfIndex,
                                     protocol: AvahiProtocol,
                                     service_type: *const c_char,
                                     domain: *const c_char,
                                     flags: AvahiLookupFlags,
                                     callback: ServiceBrowserCallback,
                                     userdata: *mut c_void)
                                     -> *mut AvahiServiceBrowser;

    /// Cleans up and frees an `AvahiServiceBrowser` object.
    ///
    /// # Arguments
    ///
    /// * `service_browser` - instance of `AvahiServiceBrowser`.
    pub fn avahi_service_browser_free(service_browser: *mut AvahiServiceBrowser) -> c_int;

    /// Create a new service resolver object.
    ///
    /// Please make sure to pass all the service data you received via
    /// `avahi_service_browser_new()` callback function, especially `interface`
    /// and `protocol`.
    ///
    /// # Arguments
    ///
    /// * `client` - Active `AvahiClient` instance.
    /// * `interface` - Interface argument received in `AvahiServiceBrowserCallback`.
    /// * `protocol` - The protocol argument specifies the protocol (IPv4 or IPv6)
    ///                to use as transport for the queries which are sent out by this
    ///                resolver. Generally, on `protocol` you should only pass what was
    ///                supplied to you as parameter to your `AvahiServiceBrowserCallback`.
    ///                Or, more technically speaking: protocol specifies if the mDNS
    ///                queries should be sent as UDP/IPv4 resp. UDP/IPv6 packets.
    /// * `name` - Name argument received in `AvahiServiceBrowserCallback`.
    /// * `service_type` - Service type argument received in `AvahiServiceBrowserCallback`.
    /// * `domain` - Domain argument received in `AvahiServiceBrowserCallback`.
    /// * `aprotocol` - The `aprotocol` argument specifies the adress family
    ///                 (IPv4 or IPv6) of the address of the service we are looking for.
    ///                 In `aprotocol` you should pass what your application code can deal
    ///                 with when connecting to the service. Or, more technically speaking:
    ///                 `aprotocol` specifies whether the query is for a A resp. AAAA
    ///                 resource record. AVAHI_PROTO_UNSPEC if your application can deal
    ///                 with both IPv4 and IPv6
    /// * `flags` - Flags for lookup functions `AvahiLookupFlags`.
    /// * `callback` - `ServiceResolverCallback` callback to be called for every new
    ///                resolved service.
    /// * `userdata` - Some arbitrary user data pointer that will be passed to the callback.
    ///
    /// # Return value
    ///
    /// A service resolver `AvahiServiceResolver` object.
    pub fn avahi_service_resolver_new(client: *mut AvahiClient,
                                      interface: c_int,
                                      protocol: AvahiProtocol,
                                      name: *const c_char,
                                      service_type: *const c_char,
                                      domain: *const c_char,
                                      aprotocol: AvahiProtocol,
                                      flags: AvahiLookupFlags,
                                      callback: ServiceResolverCallback,
                                      userdata: *mut c_void)
                                      -> *mut AvahiServiceResolver;

    pub fn avahi_service_resolver_free(resolver: *mut AvahiServiceResolver) -> c_int;

    pub fn avahi_address_snprint(ret_s: *const c_char, length: size_t, a: *const AvahiAddress);

    /// Convert the string list object to a single character string, seperated by spaces
    /// and enclosed in "". `avahi_free` should always be called for the result!
    /// This function doesn't work well with strings that contain NUL bytes.
    ///
    /// # Arguments
    ///
    /// * `string_list` - string list instance.
    ///
    /// # Return value
    ///
    /// Single character string, seperated by spaces and enclosed in "".
    pub fn avahi_string_list_to_string(string_list: *mut AvahiStringList) -> *const c_char;

    /// Free some memory.
    ///
    /// # Arguments
    ///
    /// * `pointer` - pointer to free memory for.
    pub fn avahi_free(pointer: *mut c_void);

    /// Return a human readable error string for the specified error code.
    ///
    /// # Arguments
    ///
    /// * `error` - Integer error code used by avahi.
    ///
    /// # Return value
    ///
    /// Human readable error string.
    pub fn avahi_strerror(error: c_int) -> *const c_char;

    /// Create a new main loop object (will be performed in a separate thread).
    ///
    /// # Return value
    ///
    /// Main loop object - `AvahiThreadedPoll`.
    pub fn avahi_threaded_poll_new() -> *mut AvahiThreadedPoll;

    /// Return the abstracted poll API object for this main loop object.
    /// The is will return the same pointer each time it is called.
    ///
    /// # Arguments
    ///
    /// * `threaded_poll` - Main loop object returned from `avahi_threaded_poll_new`.
    ///
    /// # Return value
    ///
    /// Abstracted poll API object - `AvahiPoll`.
    pub fn avahi_threaded_poll_get(threaded_poll: *mut AvahiThreadedPoll) -> *mut AvahiPoll;

    /// Start the event loop helper thread.
    ///
    /// After the thread has started you must make sure to access the event loop object
    /// (`AvahiThreadedPoll`, `AvahiPoll` and all its associated objects) synchronized,
    /// i.e. with proper locking. You may want to use `avahi_threaded_poll_lock` and
    /// `avahi_threaded_poll_unlock` for this, which will lock the the entire event loop.
    /// Please note that event loop callback functions are called from the event loop
    /// helper thread with that lock held, i.e. `avahi_threaded_poll_lock` calls are not
    /// required from event callbacks.
    ///
    /// # Arguments
    ///
    /// * `threaded_poll` - Main loop object returned from `avahi_threaded_poll_new`.
    pub fn avahi_threaded_poll_start(threaded_poll: *mut AvahiThreadedPoll) -> c_int;

    /// Request that the event loop quits and the associated thread stops.
    ///
    /// Call this from outside the helper thread if you want to shut it down.
    ///
    /// # Arguments
    ///
    /// * `threaded_poll` - Main loop object returned from `avahi_threaded_poll_new`.
    pub fn avahi_threaded_poll_stop(threaded_poll: *mut AvahiThreadedPoll) -> c_int;

    pub fn avahi_threaded_poll_quit(threaded_poll: *mut AvahiThreadedPoll) -> c_void;



    /// Free an event loop object.
    ///
    /// This will stop the associated event loop thread (if it is running).
    ///
    /// # Arguments
    ///
    /// * `threaded_poll` - Main loop object returned from `avahi_threaded_poll_new`.
    pub fn avahi_threaded_poll_free(threaded_poll: *mut AvahiThreadedPoll) -> c_void;

    pub fn avahi_entry_group_new(client: *mut AvahiClient,
                                 callback: AvahiEntryGroupCallback,
                                 userdata: *mut c_void)
                                 -> *mut AvahiEntryGroup;


    pub fn avahi_entry_group_add_record(group: *mut AvahiEntryGroup,
                                        interface: c_int,
                                        protocol: AvahiProtocol,
                                        flags: AvahiPublishFlags,
                                        name: *const c_char,
                                        record_class: AvahiRecordClass,
                                        record_type: AvahiRecordType,
                                        ttl: u32,
                                        rdata: *const c_void,
                                        size: usize)
                                        -> c_int;

    pub fn avahi_entry_group_commit(group: *mut AvahiEntryGroup) -> c_int;

    pub fn avahi_entry_group_get_state(group: *mut AvahiEntryGroup) -> c_int;
}