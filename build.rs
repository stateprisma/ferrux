use std::env;

fn main() {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    match arch.as_str() {
        "x86_64" => {
            // Tell cargo to pass the Linker script to the Linker...
            println!("cargo:rustc-link-arg=-TLinker/x86_64.ld");
            // ...and to re-run if it changes.
            println!("cargo:rerun-if-changed=Linker/x86_64.ld");
        }
        "aarch64" => {
            // Tell cargo to pass the Linker script to the Linker...
            println!("cargo:rustc-link-arg=-TLinker/aarch64.ld");
            // ...and to re-run if it changes.
            println!("cargo:rerun-if-changed=Linker/aarch64.ld");
        }
        _ => panic!("Unsupported target ISA"),
    }
}