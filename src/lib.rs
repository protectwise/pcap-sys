#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

#[link(name = "pcap")]
extern {}

include!(concat!(env!("OUT_DIR"), "/pcap.rs"));
