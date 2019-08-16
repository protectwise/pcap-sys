use log::*;
use std::env;
use std::path::PathBuf;

const PCAP_HEADER: &'static str = "pcap.h";

fn main() {
    let _ = env_logger::try_init();

    let cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = PathBuf::from(env::var("OUT_DIR").unwrap()); //set at build time

    let libpcap_path = if let Some(pcap_dir) = env::var_os("LIBPCAP_DIR") {
        PathBuf::from(pcap_dir).join(PCAP_HEADER)
    } else {
        locate_header::locate_header(
            PCAP_HEADER,
            Some(locate_header::Package {
                version: "1.5.3-11".to_owned(),
                name: "libpcap-devel".to_owned()
            })
        ).expect(&format!("Failed to find {}", PCAP_HEADER))
    };

    let libpcap_header = libpcap_path
        .to_str()
        .expect("No path provided")
        .to_string();

    info!(
        "Generating binding for libpcap using header {}",
        libpcap_header
    );

    let libpcap_include_dir = libpcap_path.parent()
        .expect("Could not get pcap file parent")
        .parent()
        .expect("Could not get pcap directory parent");

    let clang_args = [
        format!(
            "-I{}",
            libpcap_include_dir.to_str().expect("Failed to convert to string")
        ),
    ];

    let bindings = bindgen::Builder::default()
        .trust_clang_mangling(false)
        .clang_args(&clang_args)
        .header(libpcap_header.clone())
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the src/libpcap.rs file.
    bindings
        .write_to_file(output_dir.join("pcap.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed={}", libpcap_header);
    println!(
        "cargo:rerun-if-changed={}/build.rs",
        cargo_dir.to_str().expect("Failed to convert to string")
    );
}
