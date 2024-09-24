build-debug-amd64:
    cargo build --target=x86_64-unknown-none

build-debug-aarch64:
    cargo build --target=aarch64-unknown-none

make-iso-amd64: build-debug-amd64
    rm -rf "iso_root"
    mkdir "iso_root"
    cp target/x86_64-unknown-none/debug/k64 target/k64.sys
    ./make-iso.sh

make-iso-aarch64: build-debug-aarch64
    rm -rf "iso_root"
    mkdir "iso_root"
    cp target/aarch64-unknown-none/debug/k64 target/k64.sys
    ./make-iso.sh

clean:
    rm -rf "iso_root" "limine"
    cargo clean
