fn main() {
    let target = std::env::var("TARGET").unwrap();

    // Only set the linker script for the bare-metal target
    if target == "aarch64-unknown-none" {
        println!("cargo:rustc-link-arg=-Tlink.ld");
    }
}
