use env_logger;
use log::*;

use std::env;
use std::path::PathBuf;

fn find_it(to_find: &str) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).filter_map(|dir| {
            let full_path = dir.join(to_find);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        }).next()
    })
}

fn main() {
    let _ = env_logger::try_init();

    let cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = PathBuf::from(env::var("OUT_DIR").unwrap()); //set at build time

    let libpcap_path = if let Some(d) = find_it("pcap.h") {
        d
    } else if let Some(pcap_dir) = env::var_os("LIBPCAP_DIR") {
        PathBuf::from(pcap_dir).join("pcap.h")
    } else {
        PathBuf::from("/usr/include/pcap.h")
    }.to_str().expect("No path provided").to_string();

    info!("Generating binding for libpcap using header {}", libpcap_path);

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
    println!("cargo:rerun-if-changed={}/build.rs", cargo_dir.to_str().expect("Failed to convert to string"));
}