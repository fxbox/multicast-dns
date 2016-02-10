Requires ```avahi-common``` and ```avahi-client``` libs 


Examples:
```
multicast-dns -t _http._tcp

multicast-dns -t _scanner._tcp
```

Look at [RFC 6762](https://tools.ietf.org/html/rfc6762) and [RFC 6763](https://tools.ietf.org/html/rfc6763) for the standard specifications.

Also one can take a look at [Service Name and Transport Protocol Port Number Registry](http://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml) to see currently available and registered services.