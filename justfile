default:
  @just --list

build-debug-amd64:
    cargo build --target=x86_64-unknown-none

build-debug-aarch64:
    cargo build --target=aarch64-unknown-none

make-iso-amd64: build-debug-amd64
    cp target/x86_64-unknown-none/debug/k64 target/k64.sys
    ./make-iso.sh

make-iso-aarch64: build-debug-aarch64
    cp target/aarch64-unknown-none/debug/k64 target/k64.sys
    ./make-iso.sh

run-amd64: make-iso-amd64
    #!/usr/bin/env bash
    if [ ! -f "ovmf_amd64.fd" ]; then
      echo "Downloading OVMF"
      curl -SL 'https://ddn.stateprisma.com/RELEASEX64_OVMF.fd' -o ovmf_amd64.fd
    else
      echo "Found OVMF image already"
    fi
    qemu-system-x86_64 -enable-kvm -M q35 -cpu host -m 2G -bios ovmf_amd64.fd -cdrom ferrux.iso -boot d -serial stdio

clean:
    rm -rf "iso_root" "limine"
    cargo clean
