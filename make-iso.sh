#!/usr/bin/env bash

echo Creating ISO file

curl -SsLO https://ddn.stateprisma.com/limine.tar.gz || exit 1

mkdir limine
tar xvf limine.tar.gz -C limine
rm limine.tar.gz

(
  cd iso_root || exit 255
  mkdir -p EFI/BOOT
  cp ../limine/bin/BOOTAA64.EFI EFI/BOOT
  cp ../limine/bin/BOOTX64.EFI EFI/BOOT
  cp ../limine/bin/limine .
  cp ../limine.conf .
  cp ../target/k64.sys .

  cd ..
  xorriso -as mkisofs \
  		-no-emul-boot -boot-load-size 4 -boot-info-table \
  		--efi-boot limine \
  		-efi-boot-part --efi-boot-image --protective-msdos-label \
  		iso_root -o ferrux.iso
)
