use copy_to_output::copy_to_output;

fn main() {
    println!("cargo:rerun-if-changed=resources/source2viewer/Source2Viewer-CLI");
    println!("cargo:rerun-if-changed=lib/libmapdata.so");
    let profile = std::env::var("PROFILE").unwrap();

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={manifest_dir}/lib");
    println!("cargo:rustc-link-lib=mapdata");
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/lib");

    let _ = copy_to_output("resources/source2viewer", &profile);
    let _ = copy_to_output("lib", &profile);
}
