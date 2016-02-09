use super::types::*;
use super::enums::*;
use libc::{c_void, c_int, c_char, size_t};

#[link(name = "avahi-common")]
#[link(name = "avahi-client")]
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

    pub fn avahi_simple_poll_loop(s: *mut AvahiSimplePoll) -> c_int;
    pub fn avahi_simple_poll_free(s: *mut AvahiSimplePoll);

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
    ///                this is to always use the AvahiClient pointer passed to the callback function
    ///                instead of the global pointer.
    /// * `userdata` - Some arbitrary user data pointer that will be passed to the callback function.
    /// * `error` -	If creation of the client fails, this integer will contain the error cause.
    ///             May be NULL if you aren't interested in the reason why avahi_client_new() failed.
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


    pub fn avahi_client_free(client: *mut AvahiClient);

    pub fn avahi_service_browser_new(client: *mut AvahiClient,
                                     interface: c_int,
                                     protocol: c_int,
                                     le_type: *const c_char,
                                     domain: *const c_char,
                                     flags: AvahiLookupFlags,
                                     callback: ServiceBrowserCallback,
                                     userdata: *mut c_void)
                                     -> *mut AvahiServiceBrowser;

    pub fn avahi_service_resolver_new(client: *mut AvahiClient,
                                      interface: c_int,
                                      protocol: c_int,
                                      name: *const c_char,
                                      le_type: *const c_char,
                                      domain: *const c_char,
                                      aprotocol: AvahiProtocol,
                                      flags: AvahiLookupFlags,
                                      callback: ServiceResolverCallback,
                                      userdata: *mut c_void)
                                      -> *mut AvahiServiceResolver;

    pub fn avahi_service_browser_free(b: *mut AvahiServiceBrowser) -> c_int;

    pub fn avahi_address_snprint(ret_s: *const c_char, length: size_t, a: *const AvahiAddress);

    pub fn avahi_string_list_to_string(l: *mut AvahiStringList) -> *const c_char;

    pub fn avahi_free(p: *mut c_void);

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
}