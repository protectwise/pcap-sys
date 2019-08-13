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
