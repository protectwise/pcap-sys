use env_logger;
use log::*;
use pkg_config;

use std::env;
use std::path::PathBuf;

const PCAP_HEADER: &'static str = "pcap.h";

fn find_it(to_find: &str) -> Option<PathBuf> {
    env::var_os("PATH")
        .and_then(|paths| {
            env::split_paths(&paths)
                .filter_map(|dir| {
                    let full_path = dir.join(to_find);
                    if full_path.is_file() {
                        Some(full_path)
                    } else {
                        None
                    }
                })
                .next()
        })
        .or_else(|| {
            let include_path = PathBuf::from("/usr/include");
            let pcap_path = include_path.join(PCAP_HEADER);
            if pcap_path.exists() {
                Some(pcap_path)
            } else {
                None
            }
        })
}

fn locate_libpcap_header() -> Option<PathBuf> {
    let library = if let Ok(l) = pkg_config::Config::new()
        .atleast_version("1.5.3-11")
        .probe("libpcap-devel")
    {
        l
    } else {
        return None;
    };

    for include_path in library.include_paths {
        let pcap_path = include_path.join(PCAP_HEADER);
        if pcap_path.exists() {
            return Some(pcap_path);
        }
    }
    None
}

fn main() {
    let _ = env_logger::try_init();

    let cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = PathBuf::from(env::var("OUT_DIR").unwrap()); //set at build time

    let libpcap_path = if let Some(pcap_dir) = env::var_os("LIBPCAP_DIR") {
        PathBuf::from(pcap_dir).join(PCAP_HEADER)
    } else if let Some(p) = locate_libpcap_header() {
        p
    } else {
        find_it(PCAP_HEADER).expect("Failed to find pcap header in path")
    }
    .to_str()
    .expect("No path provided")
    .to_string();

    info!(
        "Generating binding for libpcap using header {}",
        libpcap_path
    );

    let bindings = bindgen::Builder::default()
        .trust_clang_mangling(false)
        .header(libpcap_path.clone())
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the src/libpcap.rs file.
    bindings
        .write_to_file(output_dir.join("pcap.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed={}", libpcap_path);
    println!(
        "cargo:rerun-if-changed={}/build.rs",
        cargo_dir.to_str().expect("Failed to convert to string")
    );
}
