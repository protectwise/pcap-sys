# pcap-sys

Bindgen generated code from installed libpcap. If environment variable `LIBPCAP_DIR` is not set, will use [pkg-config](https://docs.rs/crate/pkg-config/0.3.14) to locate libpcap. As a last resort, it will fall back to a path search.

