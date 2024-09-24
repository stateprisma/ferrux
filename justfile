build-debug-all:
    build-debug-amd64
    build-debug-aarch64

build-debug-amd64:
    cargo build --target=x86_64-unknown-none
    cp target/x86_64-unknown-none/debug/k64 target/x86_64-unknown-none/debug/k64.sys

build-debug-aarch64:
    cargo build --target=aarch64-unknown-none
    cp target/aarch64-unknown-none/debug/k64 target/x86_64-unknown-none/debug/k64.sys
